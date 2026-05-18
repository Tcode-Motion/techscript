// ── Native Anime Module (timeline MVP) ───────────────────────────────
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::{NativeFnObj, Value};

thread_local! {
    static ANIME_CTX: RefCell<Vec<AnimStep>> = RefCell::new(Vec::new());
}

#[derive(Clone)]
struct AnimStep {
    action: String,
    target: String,
    duration: f64,
    ease: String,
}

pub fn register(globals: &mut HashMap<String, Value>) {
    for (name, func) in [
        ("__anime_timeline", timeline_begin as fn(&[Value]) -> Result<Value, String>),
        ("__anime_move", anime_move),
        ("__anime_fade", anime_fade),
        ("__anime_run", anime_run),
    ] {
        globals.insert(
            name.into(),
            Value::NativeFunction(Rc::new(NativeFnObj {
                name: name.to_string(),
                func: Box::new(func),
            })),
        );
    }
}

fn timeline_begin(args: &[Value]) -> Result<Value, String> {
    let _name = args.first().map(|v| v.display_string()).unwrap_or_else(|| "start".into());
    ANIME_CTX.with(|ctx| ctx.borrow_mut().clear());
    Ok(Value::None)
}

fn anime_move(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("anime_move needs target and position".into());
    }
    let target = args[0].display_string();
    let position = args.get(1).map(|v| v.display_string()).unwrap_or_default();
    let duration = args.get(2).and_then(|v| v.as_f64()).unwrap_or(1.0);
    let ease = args.get(3).map(|v| v.display_string()).unwrap_or_else(|| "linear".into());
    ANIME_CTX.with(|ctx| {
        ctx.borrow_mut().push(AnimStep {
            action: format!("move to {}", position),
            target,
            duration,
            ease,
        });
    });
    Ok(Value::None)
}

fn anime_fade(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("anime_fade needs target".into());
    }
    let target = args[0].display_string();
    let duration = args.get(2).and_then(|v| v.as_f64()).unwrap_or(0.5);
    ANIME_CTX.with(|ctx| {
        ctx.borrow_mut().push(AnimStep {
            action: "fade".into(),
            target,
            duration,
            ease: "linear".into(),
        });
    });
    Ok(Value::None)
}

fn anime_run(_args: &[Value]) -> Result<Value, String> {
    let steps = ANIME_CTX.with(|ctx| ctx.borrow().clone());
    println!("🎬 Anime timeline ({} steps):", steps.len());
    for (i, step) in steps.iter().enumerate() {
        println!(
            "  {}. {} {} over {:.1}s ease {}",
            i + 1,
            step.action,
            step.target,
            step.duration,
            step.ease
        );
        std::thread::sleep(std::time::Duration::from_millis((step.duration * 200.0) as u64));
    }
    println!("Timeline complete.");
    Ok(Value::None)
}
