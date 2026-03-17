// ── TechScript Built-in Functions & Module Namespaces ────────────────
// Pure std Rust — no external crate dependencies beyond clap/rustyline/colored
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::{Value, NativeFnObj};

#[macro_export]
macro_rules! native {
    ($name:expr, $f:expr) => {
        crate::value::Value::NativeFunction(std::rc::Rc::new(crate::value::NativeFnObj {
            name: $name.to_string(),
            func: Box::new($f),
        }))
    };
}

pub fn make_module(entries: Vec<(&str, Value)>) -> Value {
    let mut map: HashMap<String, Value> = HashMap::new();
    for (k, v) in entries { map.insert(k.to_string(), v); }
    Value::Map(Rc::new(RefCell::new(map)))
}

pub fn register_builtins(globals: &mut HashMap<String, Value>) {
    register_io(globals);
    register_core(globals);
    register_type_conv(globals);
    register_string_utils(globals);
    register_list_utils(globals);
    crate::stdlib::register_all(globals);
}

// ── I/O ──────────────────────────────────────────────────────────────
fn register_io(globals: &mut HashMap<String, Value>) {
    globals.insert("print".into(), native!("print", |args| {
        use std::io::Write;
        let s: Vec<String> = args.iter().map(|a| a.display_string()).collect();
        print!("{}", s.join(" "));
        std::io::stdout().flush().ok();
        Ok(Value::None)
    }));
    globals.insert("say".into(), native!("say", |args| {
        let s: Vec<String> = args.iter().map(|a| a.display_string()).collect();
        println!("{}", s.join(" "));
        Ok(Value::None)
    }));
    globals.insert("write".into(), native!("write", |args| {
        use std::io::Write;
        let s: Vec<String> = args.iter().map(|a| a.display_string()).collect();
        print!("{}", s.join(""));
        std::io::stdout().flush().ok();
        Ok(Value::None)
    }));
    globals.insert("debug".into(), native!("debug", |args| {
        if let Some(v) = args.first() { eprintln!("[DEBUG] {:?}", v); }
        Ok(Value::None)
    }));
    globals.insert("log".into(), native!("log", |args| {
        if let Some(v) = args.first() { println!("[LOG] {}", v.display_string()); }
        Ok(Value::None)
    }));
    globals.insert("warn".into(), native!("warn", |args| {
        if let Some(v) = args.first() { eprintln!("[WARN] {}", v.display_string()); }
        Ok(Value::None)
    }));
    globals.insert("error".into(), native!("error", |args| {
        if let Some(v) = args.first() { eprintln!("[ERROR] {}", v.display_string()); }
        Ok(Value::None)
    }));
    globals.insert("clear".into(), native!("clear", |_| {
        print!("\x1B[2J\x1B[1;1H");
        Ok(Value::None)
    }));
    globals.insert("format".into(), native!("format", |args| {
        if args.is_empty() { return Ok(Value::String(Rc::new(String::new()))); }
        let mut result = args[0].display_string();
        for (i, arg) in args.iter().skip(1).enumerate() {
            result = result.replacen(&format!("{{{}}}", i), &arg.display_string(), 1);
        }
        Ok(Value::String(Rc::new(result)))
    }));
}

// ── Core Utilities ────────────────────────────────────────────────────
fn register_core(globals: &mut HashMap<String, Value>) {
    globals.insert("assert".into(), native!("assert", |args| {
        let cond = args.first().map(|v| v.is_truthy()).unwrap_or(false);
        if !cond {
            let msg = args.get(1).map(|v| v.display_string()).unwrap_or("Assertion failed".into());
            return Err(msg);
        }
        Ok(Value::None)
    }));
    globals.insert("panic".into(), native!("panic", |args| {
        let msg = args.first().map(|v| v.display_string()).unwrap_or("panic!".into());
        eprintln!("PANIC: {}", msg);
        std::process::exit(1);
    }));
    globals.insert("sleep".into(), native!("sleep", |args| {
        let ms = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0);
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
        Ok(Value::None)
    }));
    globals.insert("time".into(), native!("time", |_| {
        Ok(Value::Float(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs_f64()))
    }));
    globals.insert("time_ms".into(), native!("time_ms", |_| {
        Ok(Value::Int(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64))
    }));
    globals.insert("exit".into(), native!("exit", |args| {
        let code = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0);
        std::process::exit(code as i32);
    }));
    globals.insert("version".into(), native!("version", |_| {
        Ok(Value::String(Rc::new("1.0.4".into())))
    }));
    globals.insert("callable".into(), native!("callable", |args| {
        let r = matches!(args.first(), Some(Value::NativeFunction(_)) | Some(Value::Closure(_)));
        Ok(Value::Bool(r))
    }));
}

// ── Type Conversion ────────────────────────────────────────────────────
fn register_type_conv(globals: &mut HashMap<String, Value>) {
    globals.insert("int".into(), native!("int", |args| {
        match args.first() {
            Some(Value::Int(i)) => Ok(Value::Int(*i)),
            Some(Value::Float(f)) => Ok(Value::Int(*f as i64)),
            Some(Value::String(s)) => Ok(Value::Int(s.parse::<i64>().unwrap_or(0))),
            Some(Value::Bool(b)) => Ok(Value::Int(if *b { 1 } else { 0 })),
            _ => Ok(Value::Int(0)),
        }
    }));
    globals.insert("float".into(), native!("float", |args| {
        match args.first() {
            Some(Value::Int(i)) => Ok(Value::Float(*i as f64)),
            Some(Value::Float(f)) => Ok(Value::Float(*f)),
            Some(Value::String(s)) => Ok(Value::Float(s.parse::<f64>().unwrap_or(0.0))),
            _ => Ok(Value::Float(0.0)),
        }
    }));
    globals.insert("str".into(), native!("str", |args| {
        Ok(Value::String(Rc::new(args.first().map(|v| v.display_string()).unwrap_or_default())))
    }));
    globals.insert("bool".into(), native!("bool", |args| {
        Ok(Value::Bool(args.first().map(|v| v.is_truthy()).unwrap_or(false)))
    }));
    globals.insert("list".into(), native!("list", |args| {
        match args.first() {
            Some(Value::Range(start, end, inclusive)) => {
                let e = if *inclusive { *end + 1 } else { *end };
                Ok(Value::List(Rc::new(RefCell::new((*start..e).map(Value::Int).collect()))))
            }
            Some(Value::List(l)) => Ok(Value::List(l.clone())),
            _ => Ok(Value::List(Rc::new(RefCell::new(Vec::new())))),
        }
    }));
    globals.insert("type".into(), native!("type", |args| {
        Ok(Value::String(Rc::new(args.first().map(|v| v.type_name().to_string()).unwrap_or("none".into()))))
    }));
    globals.insert("len".into(), native!("len", |args| {
        match args.first() {
            Some(Value::String(s)) => Ok(Value::Int(s.chars().count() as i64)),
            Some(Value::List(l)) => Ok(Value::Int(l.borrow().len() as i64)),
            Some(Value::Map(m)) => Ok(Value::Int(m.borrow().len() as i64)),
            _ => Ok(Value::Int(0)),
        }
    }));
}

// ── String Utilities (global shorthand functions) ──────────────────────
fn register_string_utils(globals: &mut HashMap<String, Value>) {
    globals.insert("upper".into(), native!("upper", |args| {
        Ok(Value::String(Rc::new(args.first().map(|v| v.display_string().to_uppercase()).unwrap_or_default())))
    }));
    globals.insert("lower".into(), native!("lower", |args| {
        Ok(Value::String(Rc::new(args.first().map(|v| v.display_string().to_lowercase()).unwrap_or_default())))
    }));
    globals.insert("trim".into(), native!("trim", |args| {
        Ok(Value::String(Rc::new(args.first().map(|v| v.display_string().trim().to_string()).unwrap_or_default())))
    }));
    globals.insert("split".into(), native!("split", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        let sep = args.get(1).map(|v| v.display_string()).unwrap_or(" ".into());
        let parts: Vec<Value> = s.split(&sep as &str).map(|p| Value::String(Rc::new(p.to_string()))).collect();
        Ok(Value::List(Rc::new(RefCell::new(parts))))
    }));
    globals.insert("join".into(), native!("join", |args| {
        let sep = args.first().map(|v| v.display_string()).unwrap_or_default();
        if let Some(Value::List(l)) = args.get(1) {
            let parts: Vec<String> = l.borrow().iter().map(|v| v.display_string()).collect();
            Ok(Value::String(Rc::new(parts.join(&sep))))
        } else { Ok(Value::String(Rc::new(String::new()))) }
    }));
    globals.insert("replace".into(), native!("replace", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        let from = args.get(1).map(|v| v.display_string()).unwrap_or_default();
        let to = args.get(2).map(|v| v.display_string()).unwrap_or_default();
        Ok(Value::String(Rc::new(s.replacen(&from, &to, 1))))
    }));
    globals.insert("replace_all".into(), native!("replace_all", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        let from = args.get(1).map(|v| v.display_string()).unwrap_or_default();
        let to = args.get(2).map(|v| v.display_string()).unwrap_or_default();
        Ok(Value::String(Rc::new(s.replace(&from as &str, &to))))
    }));
    globals.insert("contains".into(), native!("contains", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        let sub = args.get(1).map(|v| v.display_string()).unwrap_or_default();
        Ok(Value::Bool(s.contains(&sub as &str)))
    }));
    globals.insert("starts_with".into(), native!("starts_with", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        let p = args.get(1).map(|v| v.display_string()).unwrap_or_default();
        Ok(Value::Bool(s.starts_with(&p as &str)))
    }));
    globals.insert("ends_with".into(), native!("ends_with", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        let sf = args.get(1).map(|v| v.display_string()).unwrap_or_default();
        Ok(Value::Bool(s.ends_with(&sf as &str)))
    }));
    globals.insert("find".into(), native!("find", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        let sub = args.get(1).map(|v| v.display_string()).unwrap_or_default();
        Ok(Value::Int(s.find(&sub as &str).map(|i| i as i64).unwrap_or(-1)))
    }));
    globals.insert("repeat".into(), native!("repeat", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        let n = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i as usize) } else { None }).unwrap_or(0);
        Ok(Value::String(Rc::new(s.repeat(n))))
    }));
    globals.insert("reverse".into(), native!("reverse", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        Ok(Value::String(Rc::new(s.chars().rev().collect())))
    }));
    globals.insert("chars".into(), native!("chars", |args| {
        let s = args.first().map(|v| v.display_string()).unwrap_or_default();
        let items: Vec<Value> = s.chars().map(|c| Value::String(Rc::new(c.to_string()))).collect();
        Ok(Value::List(Rc::new(RefCell::new(items))))
    }));
    // List utility aliases
    globals.insert("range".into(), native!("range", |args| {
        let start = if args.len() > 1 { args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0) } else { 0 };
        let end = if args.len() > 1 { args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0) } else { args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0) };
        Ok(Value::List(Rc::new(RefCell::new((start..end).map(Value::Int).collect()))))
    }));
    globals.insert("enumerate".into(), native!("enumerate", |args| {
        if let Some(Value::List(l)) = args.first() {
            let items: Vec<Value> = l.borrow().iter().enumerate().map(|(i, v)| {
                Value::List(Rc::new(RefCell::new(vec![Value::Int(i as i64), v.clone()])))
            }).collect();
            Ok(Value::List(Rc::new(RefCell::new(items))))
        } else { Ok(Value::List(Rc::new(RefCell::new(Vec::new())))) }
    }));
    globals.insert("zip".into(), native!("zip", |args| {
        if let (Some(Value::List(a)), Some(Value::List(b))) = (args.first(), args.get(1)) {
            let items: Vec<Value> = a.borrow().iter().zip(b.borrow().iter()).map(|(x, y)| {
                Value::List(Rc::new(RefCell::new(vec![x.clone(), y.clone()])))
            }).collect();
            Ok(Value::List(Rc::new(RefCell::new(items))))
        } else { Ok(Value::List(Rc::new(RefCell::new(Vec::new())))) }
    }));
    globals.insert("sorted".into(), native!("sorted", |args| {
        if let Some(Value::List(l)) = args.first() {
            let mut items = l.borrow().clone();
            items.sort_by(|a, b| a.compare(b).unwrap_or(std::cmp::Ordering::Equal));
            Ok(Value::List(Rc::new(RefCell::new(items))))
        } else { Ok(Value::List(Rc::new(RefCell::new(Vec::new())))) }
    }));
    globals.insert("reversed".into(), native!("reversed", |args| {
        if let Some(Value::List(l)) = args.first() {
            let mut items = l.borrow().clone();
            items.reverse();
            Ok(Value::List(Rc::new(RefCell::new(items))))
        } else { Ok(Value::List(Rc::new(RefCell::new(Vec::new())))) }
    }));
    globals.insert("read_file".into(), native!("read_file", |args| {
        let path = args.first().map(|v| v.display_string()).unwrap_or_default();
        std::fs::read_to_string(&path).map(|c| Value::String(Rc::new(c))).map_err(|e| format!("Cannot read '{}': {}", path, e))
    }));
    globals.insert("write_file".into(), native!("write_file", |args| {
        let path = args.first().map(|v| v.display_string()).unwrap_or_default();
        let data = args.get(1).map(|v| v.display_string()).unwrap_or_default();
        std::fs::write(&path, &data).map_err(|e| format!("Cannot write '{}': {}", path, e))?;
        Ok(Value::None)
    }));
    globals.insert("append_file".into(), native!("append_file", |args| {
        use std::io::Write;
        let path = args.first().map(|v| v.display_string()).unwrap_or_default();
        let data = args.get(1).map(|v| v.display_string()).unwrap_or_default();
        let mut f = std::fs::OpenOptions::new().append(true).create(true).open(&path)
            .map_err(|e| format!("Cannot open '{}': {}", path, e))?;
        f.write_all(data.as_bytes()).map_err(|e| format!("Cannot write: {}", e))?;
        Ok(Value::None)
    }));
    // Math shortcuts (global)
    globals.insert("abs".into(), native!("abs", |args| {
        match args.first() {
            Some(Value::Int(i)) => Ok(Value::Int(i.abs())),
            Some(Value::Float(f)) => Ok(Value::Float(f.abs())),
            _ => Err("abs() requires a number".into()),
        }
    }));
    globals.insert("round".into(), native!("round", |args| {
        let val = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0);
        let places = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0);
        let factor = 10f64.powi(places as i32);
        Ok(Value::Float((val * factor).round() / factor))
    }));
    globals.insert("floor".into(), native!("floor", |args| {
        Ok(Value::Int(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).floor() as i64))
    }));
    globals.insert("ceil".into(), native!("ceil", |args| {
        Ok(Value::Int(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).ceil() as i64))
    }));
    globals.insert("sqrt".into(), native!("sqrt", |args| {
        Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).sqrt()))
    }));
    globals.insert("pow".into(), native!("pow", |args| {
        let base = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0);
        let exp = args.get(1).and_then(|v| v.as_f64()).unwrap_or(1.0);
        Ok(Value::Float(base.powf(exp)))
    }));
    globals.insert("min".into(), native!("min", |args| {
        match (args.first(), args.get(1)) {
            (Some(Value::Int(a)), Some(Value::Int(b))) => Ok(Value::Int(*a.min(b))),
            (Some(a), Some(b)) => Ok(Value::Float(a.as_f64().unwrap_or(0.0).min(b.as_f64().unwrap_or(0.0)))),
            _ => Err("min() requires two numbers".into()),
        }
    }));
    globals.insert("max".into(), native!("max", |args| {
        match (args.first(), args.get(1)) {
            (Some(Value::Int(a)), Some(Value::Int(b))) => Ok(Value::Int(*a.max(b))),
            (Some(a), Some(b)) => Ok(Value::Float(a.as_f64().unwrap_or(0.0).max(b.as_f64().unwrap_or(0.0)))),
            _ => Err("max() requires two numbers".into()),
        }
    }));
    globals.insert("clamp".into(), native!("clamp", |args| {
        let x = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0);
        let lo = args.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
        let hi = args.get(2).and_then(|v| v.as_f64()).unwrap_or(1.0);
        Ok(Value::Float(x.max(lo).min(hi)))
    }));
    globals.insert("sum".into(), native!("sum", |args| {
        if let Some(Value::List(l)) = args.first() {
            let mut total = 0.0f64;
            let mut is_int = true;
            for v in l.borrow().iter() {
                match v {
                    Value::Int(i) => total += *i as f64,
                    Value::Float(f) => { total += f; is_int = false; }
                    _ => {}
                }
            }
            if is_int { Ok(Value::Int(total as i64)) } else { Ok(Value::Float(total)) }
        } else { Ok(Value::Int(0)) }
    }));
    globals.insert("sign".into(), native!("sign", |args| {
        match args.first() {
            Some(Value::Int(i)) => Ok(Value::Int(i.signum())),
            Some(Value::Float(f)) => Ok(Value::Float(if *f > 0.0 { 1.0 } else if *f < 0.0 { -1.0 } else { 0.0 })),
            _ => Ok(Value::Int(0)),
        }
    }));
    globals.insert("is_even".into(), native!("is_even", |args| {
        match args.first() { Some(Value::Int(i)) => Ok(Value::Bool(i % 2 == 0)), _ => Ok(Value::Bool(false)) }
    }));
    globals.insert("is_odd".into(), native!("is_odd", |args| {
        match args.first() { Some(Value::Int(i)) => Ok(Value::Bool(i % 2 != 0)), _ => Ok(Value::Bool(false)) }
    }));
    globals.insert("PI".into(), Value::Float(std::f64::consts::PI));
    globals.insert("E".into(), Value::Float(std::f64::consts::E));
}

// ── List Utilities ─────────────────────────────────────────────────────
fn register_list_utils(_globals: &mut HashMap<String, Value>) {
    // List methods are accessed as .method() via vm.rs list_property
}

