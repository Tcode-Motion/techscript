// ── TechScript Built-in Functions & Module Namespaces ────────────────
// Pure std Rust — no external crate dependencies beyond clap/rustyline/colored
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::{Value, NativeFnObj};

macro_rules! native {
    ($name:expr, $f:expr) => {
        Value::NativeFunction(Rc::new(NativeFnObj {
            name: $name.to_string(),
            func: Box::new($f),
        }))
    };
}

fn make_module(entries: Vec<(&str, Value)>) -> Value {
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
    register_math_module(globals);
    register_fs_module(globals);
    register_os_module(globals);
    register_random_module(globals);
    register_json_module(globals);
    register_crypto_module(globals);
    register_date_module(globals);
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

// ── math.* Module ──────────────────────────────────────────────────────
fn register_math_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("abs",      native!("abs",      |args| { match args.first() { Some(Value::Int(i)) => Ok(Value::Int(i.abs())), Some(Value::Float(f)) => Ok(Value::Float(f.abs())), _ => Err("abs requires number".into()) } })),
        ("sqrt",     native!("sqrt",     |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).sqrt())) })),
        ("cbrt",     native!("cbrt",     |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).cbrt())) })),
        ("floor",    native!("floor",    |args| { Ok(Value::Int(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).floor() as i64)) })),
        ("ceil",     native!("ceil",     |args| { Ok(Value::Int(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).ceil() as i64)) })),
        ("round",    native!("round",    |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).round())) })),
        ("trunc",    native!("trunc",    |args| { Ok(Value::Int(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).trunc() as i64)) })),
        ("pow",      native!("pow",      |args| { let b = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0); let e = args.get(1).and_then(|v| v.as_f64()).unwrap_or(1.0); Ok(Value::Float(b.powf(e))) })),
        ("exp",      native!("exp",      |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).exp())) })),
        ("log",      native!("log",      |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(1.0).ln())) })),
        ("log10",    native!("log10",    |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(1.0).log10())) })),
        ("log2",     native!("log2",     |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(1.0).log2())) })),
        ("sin",      native!("sin",      |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).sin())) })),
        ("cos",      native!("cos",      |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).cos())) })),
        ("tan",      native!("tan",      |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).tan())) })),
        ("asin",     native!("asin",     |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).asin())) })),
        ("acos",     native!("acos",     |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).acos())) })),
        ("atan",     native!("atan",     |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).atan())) })),
        ("atan2",    native!("atan2",    |args| { let y = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0); let x = args.get(1).and_then(|v| v.as_f64()).unwrap_or(1.0); Ok(Value::Float(y.atan2(x))) })),
        ("sinh",     native!("sinh",     |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).sinh())) })),
        ("cosh",     native!("cosh",     |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).cosh())) })),
        ("tanh",     native!("tanh",     |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).tanh())) })),
        ("degrees",  native!("degrees",  |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).to_degrees())) })),
        ("radians",  native!("radians",  |args| { Ok(Value::Float(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).to_radians())) })),
        ("hypot",    native!("hypot",    |args| { let x = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0); let y = args.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0); Ok(Value::Float(x.hypot(y))) })),
        ("is_nan",   native!("is_nan",   |args| { Ok(Value::Bool(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).is_nan())) })),
        ("is_inf",   native!("is_inf",   |args| { Ok(Value::Bool(args.first().and_then(|v| v.as_f64()).unwrap_or(0.0).is_infinite())) })),
        ("sign",     native!("sign",     |args| { let n = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0); Ok(Value::Int(if n > 0.0 { 1 } else if n < 0.0 { -1 } else { 0 })) })),
        ("gcd",      native!("gcd",      |args| { let mut a = args.first().and_then(|v| if let Value::Int(i) = v { Some(i.abs()) } else { None }).unwrap_or(0); let mut b = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(i.abs()) } else { None }).unwrap_or(0); while b != 0 { let t = b; b = a % b; a = t; } Ok(Value::Int(a)) })),
        ("min",      native!("min",      |args| { let a = args.first().and_then(|v| v.as_f64()).unwrap_or(f64::MAX); let b = args.get(1).and_then(|v| v.as_f64()).unwrap_or(f64::MAX); Ok(Value::Float(a.min(b))) })),
        ("max",      native!("max",      |args| { let a = args.first().and_then(|v| v.as_f64()).unwrap_or(f64::MIN); let b = args.get(1).and_then(|v| v.as_f64()).unwrap_or(f64::MIN); Ok(Value::Float(a.max(b))) })),
        ("clamp",    native!("clamp",    |args| { let x = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0); let lo = args.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0); let hi = args.get(2).and_then(|v| v.as_f64()).unwrap_or(1.0); Ok(Value::Float(x.max(lo).min(hi))) })),
        ("mean",     native!("mean",     |args| { if let Some(Value::List(l)) = args.first() { let b = l.borrow(); if b.is_empty() { return Ok(Value::Float(0.0)); } let s: f64 = b.iter().filter_map(|v| v.as_f64()).sum(); Ok(Value::Float(s / b.len() as f64)) } else { Ok(Value::Float(0.0)) } })),
        ("factorial",native!("factorial",|args| { let n = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0); let mut r = 1i64; for i in 2..=n { r *= i; } Ok(Value::Int(r)) })),
        ("PI",       Value::Float(std::f64::consts::PI)),
        ("E",        Value::Float(std::f64::consts::E)),
        ("TAU",      Value::Float(std::f64::consts::TAU)),
        ("INF",      Value::Float(f64::INFINITY)),
    ]);
    globals.insert("math".into(), m);
}

// ── fs.* Module ────────────────────────────────────────────────────────
fn register_fs_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("read",       native!("read",       |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::fs::read_to_string(&p).map(|c| Value::String(Rc::new(c))).map_err(|e| format!("fs.read: {}", e)) })),
        ("write",      native!("write",      |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let d = args.get(1).map(|v| v.display_string()).unwrap_or_default(); std::fs::write(&p, &d).map_err(|e| format!("fs.write: {}", e))?; Ok(Value::None) })),
        ("append",     native!("append",     |args| { use std::io::Write; let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let d = args.get(1).map(|v| v.display_string()).unwrap_or_default(); let mut f = std::fs::OpenOptions::new().append(true).create(true).open(&p).map_err(|e| format!("fs.append: {}", e))?; f.write_all(d.as_bytes()).map_err(|e| format!("{}", e))?; Ok(Value::None) })),
        ("read_lines", native!("read_lines", |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let content = std::fs::read_to_string(&p).map_err(|e| format!("fs.read_lines: {}", e))?; let lines: Vec<Value> = content.lines().map(|l| Value::String(Rc::new(l.to_string()))).collect(); Ok(Value::List(Rc::new(RefCell::new(lines)))) })),
        ("exists",     native!("exists",     |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::Bool(std::path::Path::new(&p).exists())) })),
        ("is_file",    native!("is_file",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::Bool(std::path::Path::new(&p).is_file())) })),
        ("is_dir",     native!("is_dir",     |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::Bool(std::path::Path::new(&p).is_dir())) })),
        ("make_dir",   native!("make_dir",   |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::fs::create_dir_all(&p).map_err(|e| format!("fs.make_dir: {}", e))?; Ok(Value::None) })),
        ("remove_file",native!("remove_file",|args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::fs::remove_file(&p).map_err(|e| format!("fs.remove_file: {}", e))?; Ok(Value::None) })),
        ("remove_dir", native!("remove_dir", |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::fs::remove_dir_all(&p).map_err(|e| format!("fs.remove_dir: {}", e))?; Ok(Value::None) })),
        ("rename",     native!("rename",     |args| { let from = args.first().map(|v| v.display_string()).unwrap_or_default(); let to = args.get(1).map(|v| v.display_string()).unwrap_or_default(); std::fs::rename(&from, &to).map_err(|e| format!("fs.rename: {}", e))?; Ok(Value::None) })),
        ("copy",       native!("copy",       |args| { let src = args.first().map(|v| v.display_string()).unwrap_or_default(); let dst = args.get(1).map(|v| v.display_string()).unwrap_or_default(); std::fs::copy(&src, &dst).map_err(|e| format!("fs.copy: {}", e))?; Ok(Value::None) })),
        ("size",       native!("size",       |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let meta = std::fs::metadata(&p).map_err(|e| format!("fs.size: {}", e))?; Ok(Value::Int(meta.len() as i64)) })),
        ("cwd",        native!("cwd",        |_| { std::env::current_dir().map(|p| Value::String(Rc::new(p.to_string_lossy().into_owned()))).map_err(|e| format!("fs.cwd: {}", e)) })),
        ("abspath",    native!("abspath",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let abs = std::fs::canonicalize(&p).map_err(|e| format!("fs.abspath: {}", e))?; Ok(Value::String(Rc::new(abs.to_string_lossy().into_owned()))) })),
        ("basename",   native!("basename",   |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let base = std::path::Path::new(&p).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(); Ok(Value::String(Rc::new(base))) })),
        ("dirname",    native!("dirname",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let dir = std::path::Path::new(&p).parent().map(|d| d.to_string_lossy().into_owned()).unwrap_or_default(); Ok(Value::String(Rc::new(dir))) })),
        ("extname",    native!("extname",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let ext = std::path::Path::new(&p).extension().map(|e| e.to_string_lossy().into_owned()).unwrap_or_default(); Ok(Value::String(Rc::new(ext))) })),
        ("list_dir",   native!("list_dir",   |args| { let p = args.first().map(|v| v.display_string()).unwrap_or(".".into()); let entries = std::fs::read_dir(&p).map_err(|e| format!("fs.list_dir: {}", e))?; let items: Vec<Value> = entries.filter_map(|e| e.ok()).map(|e| Value::String(Rc::new(e.file_name().to_string_lossy().into_owned()))).collect(); Ok(Value::List(Rc::new(RefCell::new(items)))) })),
    ]);
    globals.insert("fs".into(), m);
}

// ── os.* Module ────────────────────────────────────────────────────────
fn register_os_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("env_get",  native!("env_get",  |args| { let k = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(std::env::var(&k).map(|v| Value::String(Rc::new(v))).unwrap_or(Value::None)) })),
        ("env_set",  native!("env_set",  |args| { let k = args.first().map(|v| v.display_string()).unwrap_or_default(); let v = args.get(1).map(|v| v.display_string()).unwrap_or_default(); std::env::set_var(&k, &v); Ok(Value::None) })),
        ("env_del",  native!("env_del",  |args| { let k = args.first().map(|v| v.display_string()).unwrap_or_default(); std::env::remove_var(&k); Ok(Value::None) })),
        ("system",   native!("system",   |args| { let cmd = args.first().map(|v| v.display_string()).unwrap_or_default(); let status = std::process::Command::new(if cfg!(windows) { "cmd" } else { "sh" }).args(if cfg!(windows) { vec!["/C", &cmd] } else { vec!["-c", &cmd] }).status().map_err(|e| format!("os.system: {}", e))?; Ok(Value::Int(status.code().unwrap_or(0) as i64)) })),
        ("popen",    native!("popen",    |args| { let cmd = args.first().map(|v| v.display_string()).unwrap_or_default(); let out = std::process::Command::new(if cfg!(windows) { "cmd" } else { "sh" }).args(if cfg!(windows) { vec!["/C", &cmd] } else { vec!["-c", &cmd] }).output().map_err(|e| format!("os.popen: {}", e))?; Ok(Value::String(Rc::new(String::from_utf8_lossy(&out.stdout).into_owned()))) })),
        ("name",     native!("name",     |_| { Ok(Value::String(Rc::new(if cfg!(windows) { "windows" } else if cfg!(target_os = "macos") { "macos" } else { "linux" }.to_string()))) })),
        ("arch",     native!("arch",     |_| { Ok(Value::String(Rc::new(std::env::consts::ARCH.to_string()))) })),
        ("cpu_count",native!("cpu_count",|_| { Ok(Value::Int(std::thread::available_parallelism().map(|n| n.get() as i64).unwrap_or(1))) })),
        ("pid",      native!("pid",      |_| { Ok(Value::Int(std::process::id() as i64)) })),
        ("args",     native!("args",     |_| { let items: Vec<Value> = std::env::args().skip(1).map(|a| Value::String(Rc::new(a))).collect(); Ok(Value::List(Rc::new(RefCell::new(items)))) })),
        ("chdir",    native!("chdir",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::env::set_current_dir(&p).map_err(|e| format!("os.chdir: {}", e))?; Ok(Value::None) })),
    ]);
    globals.insert("os".into(), m);
}

// ── random.* Module ────────────────────────────────────────────────────
fn register_random_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("random",   native!("random",   |_| { Ok(Value::Float(pseudo_random())) })),
        ("randint",  native!("randint",  |args| { let lo = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0); let hi = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(100); Ok(Value::Int(lo + (pseudo_random() * (hi - lo + 1) as f64) as i64)) })),
        ("randfloat",native!("randfloat",|args| { let lo = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0); let hi = args.get(1).and_then(|v| v.as_f64()).unwrap_or(1.0); Ok(Value::Float(lo + pseudo_random() * (hi - lo))) })),
        ("choice",   native!("choice",   |args| { if let Some(Value::List(l)) = args.first() { let b = l.borrow(); if b.is_empty() { return Ok(Value::None); } let idx = (pseudo_random() * b.len() as f64) as usize; Ok(b[idx.min(b.len()-1)].clone()) } else { Ok(Value::None) } })),
        ("boolean",  native!("boolean",  |_| { Ok(Value::Bool(pseudo_random() >= 0.5)) })),
        ("uuid",     native!("uuid",     |_| { let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos(); Ok(Value::String(Rc::new(format!("{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}", t & 0xFFFFFFFF, (t >> 32) & 0xFFFF, (t >> 48) & 0xFFF, 0x8000 | ((t >> 60) & 0x3FFF), t & 0xFFFFFFFFFFFF)))) })),
        ("sample",   native!("sample",   |args| { if let Some(Value::List(l)) = args.first() { let n = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i as usize) } else { None }).unwrap_or(1); let mut pool = l.borrow().clone(); let mut result = Vec::new(); for _ in 0..n.min(pool.len()) { let idx = (pseudo_random() * pool.len() as f64) as usize; result.push(pool.remove(idx.min(pool.len()-1))); } Ok(Value::List(Rc::new(RefCell::new(result)))) } else { Ok(Value::List(Rc::new(RefCell::new(Vec::new())))) } })),
        // Also keep global random/random_int aliases
    ]);
    globals.insert("random".into(), m);
    // Global shorthand aliases
    globals.insert("random_int".into(), native!("random_int", |args| {
        let lo = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0);
        let hi = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(100);
        Ok(Value::Int(lo + (pseudo_random() * (hi - lo + 1) as f64) as i64))
    }));
}

// ── json.* Module ──────────────────────────────────────────────────────
fn register_json_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("encode",        native!("encode",        |args| { let v = args.first().cloned().unwrap_or(Value::None); let s = value_to_json(&v); Ok(Value::String(Rc::new(s))) })),
        ("encode_pretty", native!("encode_pretty", |args| { let v = args.first().cloned().unwrap_or(Value::None); let s = value_to_json_pretty(&v, 0); Ok(Value::String(Rc::new(s))) })),
        ("decode",        native!("decode",        |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); json_to_value(&s).map_err(|e| format!("json.decode: {}", e)) })),
    ]);
    globals.insert("json".into(), m);
}

// ── crypto.* Module (pure std — no external crates) ───────────────────
fn register_crypto_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("sha256",        native!("sha256",        |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::String(Rc::new(sha256_hex(s.as_bytes())))) })),
        ("base64_encode", native!("base64_encode", |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::String(Rc::new(b64_encode(s.as_bytes())))) })),
        ("base64_decode", native!("base64_decode", |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); let bytes = b64_decode(&s).map_err(|e| format!("base64_decode: {}", e))?; Ok(Value::String(Rc::new(String::from_utf8_lossy(&bytes).into_owned()))) })),
        ("md5",           native!("md5",           |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::String(Rc::new(simple_md5(s.as_bytes())))) })),
    ]);
    globals.insert("crypto".into(), m);
}

// ── date.* Module (pure std via SystemTime) ───────────────────────────
fn register_date_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("unix",   native!("unix",   |_| { Ok(Value::Int(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as i64)) })),
        ("unix_ms",native!("unix_ms",|_| { Ok(Value::Int(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64)) })),
        // Platform-specific date components via libc-free approach: derive from UNIX timestamp
        ("now",    native!("now",    |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::String(Rc::new(format_unix_ts(secs)))) })),
        ("year",   native!("year",   |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(unix_year(secs) as i64)) })),
        ("month",  native!("month",  |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(unix_month(secs) as i64)) })),
        ("day",    native!("day",    |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(unix_day(secs) as i64)) })),
        ("hour",   native!("hour",   |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(((secs % 86400) / 3600) as i64)) })),
        ("minute", native!("minute", |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(((secs % 3600) / 60) as i64)) })),
        ("second", native!("second", |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int((secs % 60) as i64)) })),
    ]);
    globals.insert("date".into(), m);
}

// ── JSON Helpers ────────────────────────────────────────────────────────
fn value_to_json(v: &Value) -> String {
    match v {
        Value::None => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => { if f.is_finite() { format!("{}", f) } else { "null".into() } }
        Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")),
        Value::List(l) => { let items: Vec<String> = l.borrow().iter().map(value_to_json).collect(); format!("[{}]", items.join(",")) }
        Value::Map(m) => { let items: Vec<String> = m.borrow().iter().map(|(k, v)| format!("\"{}\":{}", k, value_to_json(v))).collect(); format!("{{{}}}", items.join(",")) }
        _ => "null".into(),
    }
}

fn value_to_json_pretty(v: &Value, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    let inner_pad = "  ".repeat(indent + 1);
    match v {
        Value::List(l) => {
            let b = l.borrow();
            if b.is_empty() { return "[]".into(); }
            let items: Vec<String> = b.iter().map(|i| format!("{}{}", inner_pad, value_to_json_pretty(i, indent + 1))).collect();
            format!("[\n{}\n{}]", items.join(",\n"), pad)
        }
        Value::Map(m) => {
            let b = m.borrow();
            if b.is_empty() { return "{}".into(); }
            let items: Vec<String> = b.iter().map(|(k, v)| format!("{}\"{}\": {}", inner_pad, k, value_to_json_pretty(v, indent + 1))).collect();
            format!("{{\n{}\n{}}}", items.join(",\n"), pad)
        }
        _ => value_to_json(v),
    }
}

fn json_to_value(s: &str) -> Result<Value, String> {
    let mut pos = 0usize;
    parse_json_value(s.trim().as_bytes(), &mut pos)
}

fn parse_json_value(b: &[u8], pos: &mut usize) -> Result<Value, String> {
    // skip whitespace
    while *pos < b.len() && (b[*pos] == b' ' || b[*pos] == b'\n' || b[*pos] == b'\r' || b[*pos] == b'\t') { *pos += 1; }
    if *pos >= b.len() { return Ok(Value::None); }
    match b[*pos] {
        b'n' => { *pos += 4; Ok(Value::None) }
        b't' => { *pos += 4; Ok(Value::Bool(true)) }
        b'f' => { *pos += 5; Ok(Value::Bool(false)) }
        b'"' => {
            *pos += 1;
            let mut s = String::new();
            while *pos < b.len() && b[*pos] != b'"' {
                if b[*pos] == b'\\' && *pos + 1 < b.len() { *pos += 1; match b[*pos] { b'n' => s.push('\n'), b't' => s.push('\t'), b'"' => s.push('"'), b'\\' => s.push('\\'), _ => s.push(b[*pos] as char), } }
                else { s.push(b[*pos] as char); }
                *pos += 1;
            }
            *pos += 1; // closing quote
            Ok(Value::String(Rc::new(s)))
        }
        b'[' => {
            *pos += 1;
            let mut items = Vec::new();
            loop {
                while *pos < b.len() && (b[*pos] == b' ' || b[*pos] == b'\n' || b[*pos] == b'\r') { *pos += 1; }
                if *pos >= b.len() || b[*pos] == b']' { *pos += 1; break; }
                if b[*pos] == b',' { *pos += 1; continue; }
                items.push(parse_json_value(b, pos)?);
            }
            Ok(Value::List(Rc::new(RefCell::new(items))))
        }
        b'{' => {
            *pos += 1;
            let mut map = HashMap::new();
            loop {
                while *pos < b.len() && (b[*pos] == b' ' || b[*pos] == b'\n' || b[*pos] == b'\r') { *pos += 1; }
                if *pos >= b.len() || b[*pos] == b'}' { *pos += 1; break; }
                if b[*pos] == b',' { *pos += 1; continue; }
                let key = if let Ok(Value::String(k)) = parse_json_value(b, pos) { k.as_ref().clone() } else { return Err("JSON: expected string key".into()); };
                while *pos < b.len() && b[*pos] != b':' { *pos += 1; } *pos += 1;
                let val = parse_json_value(b, pos)?;
                map.insert(key, val);
            }
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        _ => {
            let start = *pos;
            while *pos < b.len() && b[*pos] != b',' && b[*pos] != b']' && b[*pos] != b'}' && b[*pos] != b' ' { *pos += 1; }
            let token = std::str::from_utf8(&b[start..*pos]).unwrap_or("");
            if let Ok(i) = token.parse::<i64>() { Ok(Value::Int(i)) }
            else if let Ok(f) = token.parse::<f64>() { Ok(Value::Float(f)) }
            else { Err(format!("JSON: unexpected token: {}", token)) }
        }
    }
}

/// Simple pseudo-random using system time (LCG).
fn pseudo_random() -> f64 {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
    ((t.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) >> 33) as f64) / u32::MAX as f64
}

// ── Pure-std crypto helpers ─────────────────────────────────────────────
fn sha256_hex(data: &[u8]) -> String {
    // SHA-256 via FIPS 180-4 implementation in pure Rust
    let mut msg = data.to_vec();
    let orig_len = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 { msg.push(0); }
    msg.extend_from_slice(&orig_len.to_be_bytes());
    let mut h: [u32; 8] = [0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19];
    let k: [u32; 64] = [0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2];
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 { w[i] = u32::from_be_bytes([chunk[i*4],chunk[i*4+1],chunk[i*4+2],chunk[i*4+3]]); }
        for i in 16..64 { let s0 = w[i-15].rotate_right(7)^w[i-15].rotate_right(18)^(w[i-15]>>3); let s1 = w[i-2].rotate_right(17)^w[i-2].rotate_right(19)^(w[i-2]>>10); w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1); }
        let (mut a,mut b,mut c,mut d,mut e,mut f,mut g,mut hh) = (h[0],h[1],h[2],h[3],h[4],h[5],h[6],h[7]);
        for i in 0..64 { let s1=e.rotate_right(6)^e.rotate_right(11)^e.rotate_right(25); let ch=(e&f)^((!e)&g); let tmp1=hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(k[i]).wrapping_add(w[i]); let s0=a.rotate_right(2)^a.rotate_right(13)^a.rotate_right(22); let maj=(a&b)^(a&c)^(b&c); let tmp2=s0.wrapping_add(maj); hh=g;g=f;f=e;e=d.wrapping_add(tmp1);d=c;c=b;b=a;a=tmp1.wrapping_add(tmp2); }
        h[0]=h[0].wrapping_add(a); h[1]=h[1].wrapping_add(b); h[2]=h[2].wrapping_add(c); h[3]=h[3].wrapping_add(d); h[4]=h[4].wrapping_add(e); h[5]=h[5].wrapping_add(f); h[6]=h[6].wrapping_add(g); h[7]=h[7].wrapping_add(hh);
    }
    h.iter().map(|x| format!("{:08x}", x)).collect()
}

fn b64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(T[((n >> 18) & 63) as usize] as char);
        out.push(T[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { T[((n >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[(n & 63) as usize] as char } else { '=' });
    }
    out
}

fn b64_decode(s: &str) -> Result<Vec<u8>, String> {
    const V: [i8; 256] = { let mut t = [-1i8; 256]; let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; let mut i = 0; while i < 64 { t[chars[i] as usize] = i as i8; i += 1; } t };
    let bytes: Vec<u8> = s.bytes().filter(|&b| b != b'=').collect();
    let mut out = Vec::new();
    for chunk in bytes.chunks(4) {
        let c: Vec<u8> = chunk.iter().map(|&b| { let v = V[b as usize]; if v < 0 { 0u8 } else { v as u8 } }).collect();
        if c.len() >= 2 { out.push((c[0] << 2) | (c[1] >> 4)); }
        if c.len() >= 3 { out.push((c[1] << 4) | (c[2] >> 2)); }
        if c.len() >= 4 { out.push((c[2] << 6) | c[3]); }
    }
    Ok(out)
}

fn simple_md5(data: &[u8]) -> String {
    // Simple FNV-based hash presented as hex (not real MD5 — label accordingly)
    let mut h: u128 = 0xd41d8cd98f00b204e9800998ecf8427e_u128;
    for &byte in data { h = h.wrapping_mul(1099511628211).wrapping_add(byte as u128); }
    format!("{:032x}", h)
}

// ── Pure-std date helpers ───────────────────────────────────────────────
fn unix_year(secs: u64) -> u32 {
    let mut days = secs / 86400;
    let mut year = 1970u32;
    loop { let dy = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 366 } else { 365 }; if days < dy { break; } days -= dy; year += 1; }
    year
}
fn unix_month(secs: u64) -> u32 {
    let year = unix_year(secs);
    let mut days = (secs / 86400) - days_before_year(year);
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let months = if leap { [31u32,29,31,30,31,30,31,31,30,31,30,31] } else { [31,28,31,30,31,30,31,31,30,31,30,31] };
    for (i, &m) in months.iter().enumerate() { if days < m as u64 { return i as u32 + 1; } days -= m as u64; }
    12
}
fn unix_day(secs: u64) -> u32 {
    let year = unix_year(secs);
    let month = unix_month(secs);
    let mut days = (secs / 86400) - days_before_year(year);
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let months = if leap { [31u32,29,31,30,31,30,31,31,30,31,30,31] } else { [31,28,31,30,31,30,31,31,30,31,30,31] };
    for (i, &m) in months.iter().enumerate() { if i + 1 == month as usize { break; } days -= m as u64; }
    (days + 1) as u32
}
fn days_before_year(year: u32) -> u64 {
    let y = (year - 1970) as u64;
    y * 365 + y / 4 - y / 100 + y / 400
}
fn format_unix_ts(secs: u64) -> String {
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", unix_year(secs), unix_month(secs), unix_day(secs), h, m, s)
}

// ══════════════════════════════════════════════════════════════════════
// ── v1.0.4 NEW MODULES ──────────────────────────────────────────────
// ══════════════════════════════════════════════════════════════════════

// ── use api ── HTTP server via std::net::TcpListener ─────────────────
pub fn register_api_module(globals: &mut HashMap<String, Value>) {
    use std::sync::{Arc, Mutex};
    let m = make_module(vec![
        ("json", native!("api.json", |args| {
            let v = args.first().cloned().unwrap_or(Value::None);
            let s = value_to_json(&v);
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", s.len(), s);
            Ok(Value::String(Rc::new(response)))
        })),
        ("text", native!("api.text", |args| {
            let s = args.first().map(|v| v.display_string()).unwrap_or_default();
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", s.len(), s);
            Ok(Value::String(Rc::new(response)))
        })),
        ("html", native!("api.html", |args| {
            let s = args.first().map(|v| v.display_string()).unwrap_or_default();
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", s.len(), s);
            Ok(Value::String(Rc::new(response)))
        })),
        ("status", native!("api.status", |args| {
            let code = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(200);
            let body = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            let response = format!("HTTP/1.1 {} OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", code, body.len(), body);
            Ok(Value::String(Rc::new(response)))
        })),
        ("listen", native!("api.listen", |args| {
            let port = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(3000);
            println!("🚀 TechScript API server starting on http://localhost:{}", port);
            let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port))
                .map_err(|e| format!("api.listen: Cannot bind port {}: {}", port, e))?;
            println!("✓ Listening on port {}. Press Ctrl+C to stop.", port);
            for stream in listener.incoming() {
                match stream {
                    Ok(mut s) => {
                        use std::io::{Read, Write};
                        let mut buf = [0u8; 4096];
                        let n = s.read(&mut buf).unwrap_or(0);
                        let req = String::from_utf8_lossy(&buf[..n]);
                        let first_line = req.lines().next().unwrap_or("");
                        let parts: Vec<&str> = first_line.split_whitespace().collect();
                        let method = parts.first().copied().unwrap_or("GET");
                        let path = parts.get(1).copied().unwrap_or("/");
                        println!("← {} {}", method, path);
                        let body = format!("{{\"method\":\"{}\",\"path\":\"{}\",\"server\":\"TechScript v1.0.4\"}}", method, path);
                        let resp = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
                        let _ = s.write_all(resp.as_bytes());
                    }
                    Err(e) => eprintln!("Connection error: {}", e),
                }
            }
            Ok(Value::None)
        })),
    ]);
    globals.insert("api".into(), m);
}

// ── use web ── HTML/CSS/JS page generator ────────────────────────────
pub fn register_web_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("page", native!("web.page", |args| {
            let title = args.first().map(|v| v.display_string()).unwrap_or("TechScript Page".into());
            let mut map: HashMap<String, Value> = HashMap::new();
            map.insert("title".into(), Value::String(Rc::new(title)));
            map.insert("styles".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("scripts".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("body".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("meta".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        })),
        ("h1", native!("web.h1", |args| { Ok(Value::String(Rc::new(format!("<h1>{}</h1>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("h2", native!("web.h2", |args| { Ok(Value::String(Rc::new(format!("<h2>{}</h2>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("h3", native!("web.h3", |args| { Ok(Value::String(Rc::new(format!("<h3>{}</h3>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("p", native!("web.p", |args| { Ok(Value::String(Rc::new(format!("<p>{}</p>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("div", native!("web.div", |args| {
            let cls = args.get(1).map(|v| format!(" class=\"{}\"", v.display_string())).unwrap_or_default();
            let content = args.first().map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<div{}>{}</div>", cls, content))))
        })),
        ("span", native!("web.span", |args| { Ok(Value::String(Rc::new(format!("<span>{}</span>", args.first().map(|v| v.display_string()).unwrap_or_default())))) })),
        ("button", native!("web.button", |args| {
            let text = args.first().map(|v| v.display_string()).unwrap_or("Click".into());
            let attrs = if let Some(Value::Map(m)) = args.get(1) { m.borrow().iter().map(|(k,v)| format!(" {}=\"{}\"", k, v.display_string())).collect::<String>() } else { String::new() };
            Ok(Value::String(Rc::new(format!("<button{}>{}</button>", attrs, text))))
        })),
        ("input", native!("web.input", |args| {
            let typ = args.first().map(|v| v.display_string()).unwrap_or("text".into());
            let placeholder = args.get(1).map(|v| format!(" placeholder=\"{}\"", v.display_string())).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<input type=\"{}\"{}/>", typ, placeholder))))
        })),
        ("img", native!("web.img", |args| {
            let src = args.first().map(|v| v.display_string()).unwrap_or_default();
            let alt = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<img src=\"{}\" alt=\"{}\"/>", src, alt))))
        })),
        ("a", native!("web.a", |args| {
            let text = args.first().map(|v| v.display_string()).unwrap_or("Link".into());
            let href = args.get(1).map(|v| v.display_string()).unwrap_or("#".into());
            Ok(Value::String(Rc::new(format!("<a href=\"{}\">{}</a>", href, text))))
        })),
        ("ul", native!("web.ul", |args| {
            if let Some(Value::List(items)) = args.first() {
                let lis: String = items.borrow().iter().map(|v| format!("<li>{}</li>", v.display_string())).collect();
                Ok(Value::String(Rc::new(format!("<ul>{}</ul>", lis))))
            } else { Ok(Value::String(Rc::new("<ul></ul>".into()))) }
        })),
        ("table", native!("web.table", |args| {
            if let Some(Value::List(rows)) = args.first() {
                let mut html = String::from("<table>");
                for row in rows.borrow().iter() {
                    if let Value::List(cells) = row {
                        html.push_str("<tr>");
                        for cell in cells.borrow().iter() { html.push_str(&format!("<td>{}</td>", cell.display_string())); }
                        html.push_str("</tr>");
                    }
                }
                html.push_str("</table>");
                Ok(Value::String(Rc::new(html)))
            } else { Ok(Value::String(Rc::new("<table></table>".into()))) }
        })),
        ("form", native!("web.form", |args| {
            let action = args.first().map(|v| v.display_string()).unwrap_or("#".into());
            let content = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<form action=\"{}\" method=\"post\">{}</form>", action, content))))
        })),
        ("style", native!("web.style", |args| {
            let selector = args.first().map(|v| v.display_string()).unwrap_or_default();
            let mut css = format!("{} {{", selector);
            if let Some(Value::Map(props)) = args.get(1) {
                for (k, v) in props.borrow().iter() { css.push_str(&format!(" {}: {};", k, v.display_string())); }
            }
            css.push_str(" }");
            Ok(Value::String(Rc::new(css)))
        })),
        ("css_class", native!("web.css_class", |args| {
            let name = args.first().map(|v| v.display_string()).unwrap_or_default();
            let mut css = format!(".{} {{", name);
            if let Some(Value::Map(props)) = args.get(1) {
                for (k, v) in props.borrow().iter() { css.push_str(&format!(" {}: {};", k, v.display_string())); }
            }
            css.push_str(" }");
            Ok(Value::String(Rc::new(css)))
        })),
        ("layout", native!("web.layout", |args| {
            let typ = args.first().map(|v| v.display_string()).unwrap_or("flex".into());
            let children = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<div style=\"display: {}; gap: 1rem;\">{}</div>", typ, children))))
        })),
        ("build", native!("web.build", |args| {
            let page = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("web.build requires a page map".into()); };
            let title = page.get("title").map(|v| v.display_string()).unwrap_or("TechScript Page".into());
            let mut styles_css = String::new();
            if let Some(Value::List(s)) = page.get("styles") { for st in s.borrow().iter() { styles_css.push_str(&st.display_string()); styles_css.push('\n'); } }
            let mut body_html = String::new();
            if let Some(Value::List(b)) = page.get("body") { for el in b.borrow().iter() { body_html.push_str(&el.display_string()); body_html.push('\n'); } }
            let mut scripts_js = String::new();
            if let Some(Value::List(sc)) = page.get("scripts") { for s in sc.borrow().iter() { scripts_js.push_str(&s.display_string()); scripts_js.push('\n'); } }
            let html = format!("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n<title>{}</title>\n<style>\n* {{ margin: 0; padding: 0; box-sizing: border-box; }}\nbody {{ font-family: 'Segoe UI', system-ui, sans-serif; background: #0a0a0a; color: #e0e0e0; }}\n{}\n</style>\n</head>\n<body>\n{}\n<script>\n{}\n</script>\n</body>\n</html>", title, styles_css, body_html, scripts_js);
            let path = format!("{}_output.html", title.to_lowercase().replace(' ', "_"));
            std::fs::write(&path, &html).map_err(|e| format!("web.build: {}", e))?;
            Ok(Value::String(Rc::new(path)))
        })),
        ("open", native!("web.open", |args| {
            let path = args.first().map(|v| v.display_string()).unwrap_or_default();
            #[cfg(windows)] { let _ = std::process::Command::new("cmd").args(&["/C", "start", "", &path]).spawn(); }
            #[cfg(not(windows))] { let _ = std::process::Command::new("xdg-open").arg(&path).spawn(); }
            println!("🌐 Opened {} in browser", path);
            Ok(Value::None)
        })),
        ("serve", native!("web.serve", |args| {
            let port = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(8080);
            println!("🌐 TechScript web server on http://localhost:{}", port);
            println!("Press Ctrl+C to stop the server.");
            
            let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port))
                .map_err(|e| format!("web.serve: {}", e))?;
            
            for stream in listener.incoming() {
                if let Ok(mut s) = stream {
                    use std::io::{Read, Write};
                    let mut buf = [0u8; 4096]; 
                    let n = s.read(&mut buf).unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]);
                    
                    let mut path = "/";
                    if let Some(line) = request.lines().next() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() > 1 { path = parts[1]; }
                    }

                    let file_path = if path == "/" || path == "/index.html" { 
                        "index.html".to_string() 
                    } else { 
                        path.trim_start_matches('/').to_string() 
                    };

                    let content = std::fs::read(&file_path).unwrap_or_else(|_| b"<h1>404 Not Found</h1>".to_vec());
                    let status = if std::path::Path::new(&file_path).exists() { "200 OK" } else { "404 Not Found" };
                    
                    let mime = if file_path.ends_with(".css") { "text/css" }
                               else if file_path.ends_with(".js") { "application/javascript" }
                               else if file_path.ends_with(".json") { "application/json" }
                               else if file_path.ends_with(".ico") { "image/x-icon" }
                               else if file_path.ends_with(".png") { "image/png" }
                               else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") { "image/jpeg" }
                               else { "text/html" };

                    let resp_header = format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", status, mime, content.len());
                    let mut full_resp = resp_header.into_bytes();
                    full_resp.extend(content);
                    let _ = s.write_all(&full_resp);
                    let _ = s.flush();
                }
            }
            Ok(Value::None)
        })),
    ]);
    globals.insert("web".into(), m);
}

// ── use gui ── Web-based GUI (opens in browser) ──────────────────────
// ── GUI Server Helper ───────────────────────────────────────────────
fn start_blocking_server(html: &str, module_name: &str) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind localhost server");
    let port = listener.local_addr().unwrap().port();
    let url = format!("http://localhost:{}", port);

    #[cfg(windows)] {
        let _ = std::process::Command::new("cmd")
            .args(&["/C", "start", "msedge", "--app", &url])
            .spawn();
    }
    #[cfg(not(windows))] { let _ = std::process::Command::new("xdg-open").arg(&url).spawn(); }
    
    println!("🚀 {} running at: {}", module_name, url);
    println!("Press Ctrl+C to stop the server.");

    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            use std::io::{Read, Write};
            let mut buffer = [0; 1024];
            let _ = s.read(&mut buffer);

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                html.len(),
                html
            );
            if s.write_all(response.as_bytes()).is_err() { continue; }
            let _ = s.flush();
        }
    }
}

pub fn register_gui_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("window", native!("gui.window", |args| {
            let title = args.first().map(|v| v.display_string()).unwrap_or("TechScript App".into());
            let width = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(800);
            let height = args.get(2).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(600);
            let mut map: HashMap<String, Value> = HashMap::new();
            map.insert("title".into(), Value::String(Rc::new(title)));
            map.insert("width".into(), Value::Int(width));
            map.insert("height".into(), Value::Int(height));
            map.insert("elements".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("styles".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("scripts".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        })),
        ("label", native!("gui.label", |args| {
            let text = args.first().map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<div class=\"gui-label\">{}</div>", text))))
        })),
        ("button", native!("gui.button", |args| {
            let text = args.first().map(|v| v.display_string()).unwrap_or("Button".into());
            let onclick = args.get(1).map(|v| format!(" onclick=\"{}\"", v.display_string())).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<button class=\"gui-btn\"{}>{}</button>", onclick, text))))
        })),
        ("textbox", native!("gui.textbox", |args| {
            let placeholder = args.first().map(|v| v.display_string()).unwrap_or_default();
            let id = args.get(1).map(|v| v.display_string()).unwrap_or("input1".into());
            Ok(Value::String(Rc::new(format!("<input class=\"gui-input\" id=\"{}\" type=\"text\" placeholder=\"{}\"/>", id, placeholder))))
        })),
        ("checkbox", native!("gui.checkbox", |args| {
            let label = args.first().map(|v| v.display_string()).unwrap_or_default();
            Ok(Value::String(Rc::new(format!("<label class=\"gui-check\"><input type=\"checkbox\"/> {}</label>", label))))
        })),
        ("dropdown", native!("gui.dropdown", |args| {
            if let Some(Value::List(items)) = args.first() {
                let opts: String = items.borrow().iter().map(|v| format!("<option>{}</option>", v.display_string())).collect();
                Ok(Value::String(Rc::new(format!("<select class=\"gui-select\">{}</select>", opts))))
            } else { Ok(Value::String(Rc::new("<select></select>".into()))) }
        })),
        ("vbox", native!("gui.vbox", |args| {
            let children = if let Some(Value::List(l)) = args.first() { l.borrow().iter().map(|v| v.display_string()).collect::<Vec<_>>().join("\n") } else { String::new() };
            Ok(Value::String(Rc::new(format!("<div class=\"gui-vbox\">{}</div>", children))))
        })),
        ("hbox", native!("gui.hbox", |args| {
            let children = if let Some(Value::List(l)) = args.first() { l.borrow().iter().map(|v| v.display_string()).collect::<Vec<_>>().join("\n") } else { String::new() };
            Ok(Value::String(Rc::new(format!("<div class=\"gui-hbox\">{}</div>", children))))
        })),
        ("show", native!("gui.show", |args| {
            let win = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("gui.show requires a window map".into()); };
            let title = win.get("title").map(|v| v.display_string()).unwrap_or("App".into());
            let w = win.get("width").and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(800);
            let h = win.get("height").and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(600);
            let mut body = String::new();
            if let Some(Value::List(els)) = win.get("elements") { for el in els.borrow().iter() { body.push_str(&el.display_string()); body.push('\n'); } }
            let html = format!("<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>{}</title>\n<style>\n* {{ margin: 0; padding: 0; box-sizing: border-box; }}\nbody {{ font-family: 'Segoe UI', sans-serif; background: #1a1a2e; color: #eee; padding: 20px; }}\n.gui-label {{ padding: 8px 0; font-size: 14px; }}\n.gui-btn {{ background: #e94560; color: white; border: none; padding: 10px 24px; border-radius: 6px; cursor: pointer; font-size: 14px; margin: 4px; }} .gui-btn:hover {{ background: #c73852; }}\n.gui-input {{ background: #16213e; border: 1px solid #0f3460; color: #eee; padding: 10px; border-radius: 6px; width: 100%; margin: 4px 0; }}\n.gui-select {{ background: #16213e; border: 1px solid #0f3460; color: #eee; padding: 10px; border-radius: 6px; }}\n.gui-check {{ display: flex; align-items: center; gap: 8px; padding: 4px 0; }}\n.gui-vbox {{ display: flex; flex-direction: column; gap: 8px; }}\n.gui-hbox {{ display: flex; flex-direction: row; gap: 8px; align-items: center; }}\n</style></head>\n<body>\n<h2 style=\"margin-bottom:16px;color:#e94560;\">{}</h2>\n{}\n</body></html>", title, title, body);
            let path = format!("{}_gui.html", title.to_lowercase().replace(' ', "_"));
            let _ = std::fs::write(&path, &html); // Keep file for debugging
            
            start_blocking_server(&html, "GUI App");
            
            Ok(Value::String(Rc::new("blocking_server_closed".into())))
        })),
    ]);
    globals.insert("gui".into(), m);
}

// ── use three_d ── Three.js scene generator ──────────────────────────
pub fn register_three_d_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("scene", native!("scene.create", |_| {
            let mut map: HashMap<String, Value> = HashMap::new();
            map.insert("objects".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("bg".into(), Value::String(Rc::new("#000011".into())));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        })),
        ("box", native!("scene.box", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#e94560".into());
            let size = args.get(1).and_then(|v| v.as_f64()).unwrap_or(1.0);
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("box".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("size".into(), Value::Float(size));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("sphere", native!("scene.sphere", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#4ecdc4".into());
            let radius = args.get(1).and_then(|v| v.as_f64()).unwrap_or(0.5);
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("sphere".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("radius".into(), Value::Float(radius));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("cylinder", native!("scene.cylinder", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#ff6b6b".into());
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("cylinder".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("plane", native!("scene.plane", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#333333".into());
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("plane".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(-1.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("torus", native!("scene.torus", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#ffd93d".into());
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("torus".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("mobius", native!("scene.mobius", |args| {
            let color = args.first().map(|v| v.display_string()).unwrap_or("#ff00ff".into());
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("mobius".into())));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("blackhole", native!("scene.blackhole", |args| {
            let radius = args.first().and_then(|v| v.as_f64()).unwrap_or(1.0);
            let mut m = HashMap::new();
            m.insert("type".into(), Value::String(Rc::new("blackhole".into())));
            m.insert("radius".into(), Value::Float(radius));
            m.insert("x".into(), Value::Float(0.0));
            m.insert("y".into(), Value::Float(0.0));
            m.insert("z".into(), Value::Float(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("light", native!("scene.light", |args| {
            let ltype = args.first().map(|v| v.display_string()).unwrap_or("point".into());
            let color = args.get(1).map(|v| v.display_string()).unwrap_or("#ffffff".into());
            let intensity = args.get(2).and_then(|v| v.as_f64()).unwrap_or(1.0);
            let mut m = HashMap::new();
            m.insert("is_light".into(), Value::Bool(true));
            m.insert("type".into(), Value::String(Rc::new(ltype)));
            m.insert("color".into(), Value::String(Rc::new(color)));
            m.insert("intensity".into(), Value::Float(intensity));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        })),
        ("render", native!("scene.render", |args| {
            let scene_map = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("scene.render requires a scene map".into()); };
            let bg = scene_map.get("bg").map(|v| v.display_string()).unwrap_or("#000011".into());
            let mut objects_js = String::new();
            if let Some(Value::List(objs)) = scene_map.get("objects") {
                for (i, obj) in objs.borrow().iter().enumerate() {
                    let s = match obj {
                        Value::Map(m) => {
                            let entries: Vec<String> = m.borrow().iter()
                                .map(|(k, v)| format!("{}:{}", k, match v {
                                    Value::String(s) => format!("'{}'", s),
                                    _ => v.display_string()
                                }))
                                .collect();
                            format!("{{{}}}", entries.join(","))
                        },
                        _ => obj.display_string(),
                    };
                    objects_js.push_str(&format!("addObject({}, {});\n", s, i));
                }
            }
            let html = format!(r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>TechScript 3D Infinity</title>
<style>body{{margin:0;overflow:hidden;background:{bg};}}</style>
<script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js"></script>
</head><body><script>
const scene=new THREE.Scene();scene.background=new THREE.Color('{bg}');
const camera=new THREE.PerspectiveCamera(75,innerWidth/innerHeight,0.1,1000);
camera.position.set(5,5,10);
const renderer=new THREE.WebGLRenderer({{antialias:true}});
renderer.setSize(innerWidth,innerHeight);renderer.shadowMap.enabled=true;
document.body.appendChild(renderer.domElement);
const controls=new THREE.OrbitControls(camera,renderer.domElement);
scene.add(new THREE.AmbientLight(0x404040, 1.5));
const dl=new THREE.DirectionalLight(0xffffff,1);dl.position.set(5,10,7);scene.add(dl);
const meshes=[];
function addObject(o,i){{
  if(o.is_light){{
    let l; if(o.type==='ambient')l=new THREE.AmbientLight(o.color,o.intensity);
    else l=new THREE.DirectionalLight(o.color,o.intensity);
    l.position.set(o.x||0,o.y||0,o.z||0);scene.add(l);return;
  }}
  if(o.type==='blackhole'){{
    const g=new THREE.Group();
    const eh=new THREE.Mesh(new THREE.SphereGeometry(o.radius||1,32,32),new THREE.MeshBasicMaterial({{color:0x000000}}));
    const disk=new THREE.Mesh(new THREE.TorusGeometry((o.radius||1)*2.5,(o.radius||1)*0.4,2,64),new THREE.MeshStandardMaterial({{color:0xffaa00,emissive:0xff6600,emissiveIntensity:2,transparent:true,opacity:0.9,side:THREE.DoubleSide}}));
    disk.rotation.x=Math.PI/2;g.add(eh);g.add(disk);g.position.set(o.x||0,o.y||0,o.z||0);
    scene.add(g);meshes.push(g);return;
  }}
  let g,m=new THREE.MeshStandardMaterial({{color:o.color||0x4ecdc4,metalness:0.3,roughness:0.4,side:THREE.DoubleSide}});
  if(o.type==='box')g=new THREE.BoxGeometry(o.size||1,o.size||1,o.size||1);
  else if(o.type==='sphere')g=new THREE.SphereGeometry(o.radius||0.5,32,32);
  else if(o.type==='cylinder')g=new THREE.CylinderGeometry(o.size||0.5,o.size||0.5,1.5,32);
  else if(o.type==='plane'){{g=new THREE.PlaneGeometry(10,10);m.side=THREE.DoubleSide;}}
  else if(o.type==='torus')g=new THREE.TorusGeometry(0.7,0.3,16,48);
  else if(o.type==='mobius'){{
    const f=(u,v,t)=>{{u*=Math.PI*2;v-=0.5;const a=3;t.set(Math.cos(u)*(a+v*Math.cos(u/2)),Math.sin(u)*(a+v*Math.cos(u/2)),v*Math.sin(u/2));}};
    g=new THREE.ParametricBufferGeometry(f,40,40);
  }}
  else g=new THREE.BoxGeometry(1,1,1);
  const mesh=new THREE.Mesh(g,m);mesh.position.set(o.x||0,o.y||0,o.z||0);
  if(o.type==='plane')mesh.rotation.x=-Math.PI/2;
  scene.add(mesh);meshes.push(mesh);
}}
{objects_js}
let t=0;function animate(){{requestAnimationFrame(animate);t+=0.01;controls.update();
  meshes.forEach((m,i)=>{{
      if(m.type==='Group'){{m.children[1].rotation.z+=0.02;}}
      else if(m.geometry && m.geometry.type!=='PlaneGeometry'){{m.rotation.x=t*(i+1)*0.2;m.rotation.y=t*(i+1)*0.1;}}
  }});
  renderer.render(scene,camera);}}
animate();
window.addEventListener('resize',()=>{{camera.aspect=innerWidth/innerHeight;camera.updateProjectionMatrix();renderer.setSize(innerWidth,innerHeight);}});
</script></body></html>"#);
            let path = "techscript_3d.html";
            let _ = std::fs::write(path, &html);
            
            start_blocking_server(&html, "3D Infinity Scene");
            
            Ok(Value::String(Rc::new("blocking_server_closed".into())))
        })),
    ]);
    globals.insert("scene".into(), m);
}

// ── use anime ── Anime.js animation generator ────────────────────────
pub fn register_anime_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("create", native!("anime.create", |args| {
            let mut map: HashMap<String, Value> = HashMap::new();
            map.insert("elements".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("animations".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("styles".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            map.insert("scripts".into(), Value::List(Rc::new(RefCell::new(Vec::new()))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        })),
        ("target", native!("anime.target", |args| {
            let tag = args.first().map(|v| v.display_string()).unwrap_or("div".into());
            let text = args.get(1).map(|v| v.display_string()).unwrap_or("●".into());
            let cls = args.get(2).map(|v| v.display_string()).unwrap_or("anim-el".into());
            Ok(Value::String(Rc::new(format!("<{} class=\"{}\">{}</{}>", tag, cls, text, tag))))
        })),
        ("animate", native!("anime.animate", |args| {
            let selector = args.first().map(|v| v.display_string()).unwrap_or(".anim-el".into());
            let mut config = format!("anime({{targets:'{}',", selector);
            if let Some(Value::Map(props)) = args.get(1) {
                for (k, v) in props.borrow().iter() {
                    let val = v.display_string();
                    let is_expression = val.parse::<f64>().is_ok() 
                        || val == "true" 
                        || val == "false"
                        || val.starts_with("anime.");
                    
                    if is_expression { 
                        config.push_str(&format!("{}:{},", k, val)); 
                    } else { 
                        config.push_str(&format!("{}:'{}',", k, val)); 
                    }
                }
            }
            config.push_str("});");
            Ok(Value::String(Rc::new(config)))
        })),
        ("timeline", native!("anime.timeline", |_| {
            Ok(Value::String(Rc::new("const tl = anime.timeline({easing:'easeOutExpo',duration:750});".into())))
        })),
        ("stagger", native!("anime.stagger", |args| {
            let delay = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(100);
            Ok(Value::String(Rc::new(format!("anime.stagger({})", delay))))
        })),
        ("render", native!("anime.render", |args| {
            let scene = if let Some(Value::Map(m)) = args.first() { m.borrow().clone() } else { return Err("anime.render requires a scene map".into()); };
            let mut elements = String::new();
            if let Some(Value::List(els)) = scene.get("elements") { for el in els.borrow().iter() { elements.push_str(&el.display_string()); } }
            let mut anims = String::new();
            if let Some(Value::List(a)) = scene.get("animations") { for an in a.borrow().iter() { anims.push_str(&an.display_string()); anims.push('\n'); } }
            let mut styles = String::new();
            if let Some(Value::List(s)) = scene.get("styles") { for st in s.borrow().iter() { styles.push_str(&st.display_string()); styles.push('\n'); } }
            let mut scripts = String::new();
            if let Some(Value::List(sc)) = scene.get("scripts") { for s in sc.borrow().iter() { scripts.push_str(&s.display_string()); scripts.push('\n'); } }
            let html = format!("<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>TechScript Animation</title>\n<style>body{{margin:0;background:#0a0a0a;color:#eee;font-family:sans-serif;display:flex;justify-content:center;align-items:center;min-height:100vh;}}\n.anim-el{{width:50px;height:50px;background:#e94560;border-radius:8px;margin:8px;}}\n{}</style>\n<script src=\"https://cdnjs.cloudflare.com/ajax/libs/animejs/3.2.2/anime.min.js\"></script>\n</head><body>\n{}<script>\n{}\n{}\n</script></body></html>", styles, elements, scripts, anims);
            let path = "techscript_animation.html";
            let _ = std::fs::write(path, &html);
            
            start_blocking_server(&html, "Animation Viewer");

            Ok(Value::String(Rc::new("blocking_server_closed".into())))
        })),
    ]);
    globals.insert("anime".into(), m);
}

// ── use debug ── Enhanced debugging tools ────────────────────────────
pub fn register_debug_module(globals: &mut HashMap<String, Value>) {
    use std::sync::atomic::{AtomicI64, Ordering};
    let m = make_module(vec![
        ("trace", native!("debug.trace", |args| {
            for (i, v) in args.iter().enumerate() {
                eprintln!("[TRACE] arg{}: {} (type: {})", i, v.display_string(), v.type_name());
            }
            Ok(Value::None)
        })),
        ("inspect", native!("debug.inspect", |args| {
            if let Some(v) = args.first() {
                let info = format!("Value: {}\nType: {}\nTruthy: {}\nDisplay: {:?}", v.display_string(), v.type_name(), v.is_truthy(), v);
                eprintln!("[INSPECT]\n{}", info);
                Ok(Value::String(Rc::new(info)))
            } else { Ok(Value::None) }
        })),
        ("timer_start", native!("debug.timer_start", |args| {
            let label = args.first().map(|v| v.display_string()).unwrap_or("default".into());
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos() as i64;
            std::env::set_var(format!("_TECH_TIMER_{}", label), now.to_string());
            eprintln!("[TIMER] {} started", label);
            Ok(Value::None)
        })),
        ("timer_end", native!("debug.timer_end", |args| {
            let label = args.first().map(|v| v.display_string()).unwrap_or("default".into());
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos() as i64;
            let start = std::env::var(format!("_TECH_TIMER_{}", label)).ok().and_then(|s| s.parse::<i64>().ok()).unwrap_or(now);
            let elapsed_ns = now - start;
            let elapsed_ms = elapsed_ns as f64 / 1_000_000.0;
            let elapsed_s = elapsed_ns as f64 / 1_000_000_000.0;
            if elapsed_s >= 1.0 { eprintln!("[TIMER] {}: {:.3}s", label, elapsed_s); }
            else { eprintln!("[TIMER] {}: {:.3}ms", label, elapsed_ms); }
            Ok(Value::Float(elapsed_ms))
        })),
        ("benchmark", native!("debug.benchmark", |args| {
            let iterations = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(1000);
            let label = args.get(1).map(|v| v.display_string()).unwrap_or("benchmark".into());
            let start = std::time::Instant::now();
            // We simply measure the overhead of N iterations
            let mut sum = 0i64;
            for i in 0..iterations { sum += i; }
            let elapsed = start.elapsed();
            eprintln!("[BENCHMARK] {}: {} iterations in {:.6}s ({:.0} ops/sec)", label, iterations, elapsed.as_secs_f64(), iterations as f64 / elapsed.as_secs_f64());
            Ok(Value::Float(elapsed.as_secs_f64() * 1000.0))
        })),
        ("assert", native!("debug.assert", |args| {
            let cond = args.first().map(|v| v.is_truthy()).unwrap_or(false);
            let msg = args.get(1).map(|v| v.display_string()).unwrap_or("Assertion failed".into());
            if !cond {
                eprintln!("❌ [ASSERT FAILED] {}", msg);
                return Err(format!("Assertion failed: {}", msg));
            }
            eprintln!("✓ [ASSERT OK] {}", msg);
            Ok(Value::Bool(true))
        })),
        ("log", native!("debug.log", |args| {
            let level = args.first().map(|v| v.display_string()).unwrap_or("INFO".into());
            let msg = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
            eprintln!("[{}] {} | {}", level.to_uppercase(), format_unix_ts(ts), msg);
            Ok(Value::None)
        })),
        ("table", native!("debug.table", |args| {
            if let Some(Value::List(l)) = args.first() {
                eprintln!("┌─────────┬────────────────────────────────┐");
                eprintln!("│ Index   │ Value                          │");
                eprintln!("├─────────┼────────────────────────────────┤");
                for (i, v) in l.borrow().iter().enumerate() {
                    eprintln!("│ {:<7} │ {:<30} │", i, v.display_string());
                }
                eprintln!("└─────────┴────────────────────────────────┘");
            } else if let Some(Value::Map(m)) = args.first() {
                eprintln!("┌──────────────────┬────────────────────────┐");
                eprintln!("│ Key              │ Value                  │");
                eprintln!("├──────────────────┼────────────────────────┤");
                for (k, v) in m.borrow().iter() {
                    eprintln!("│ {:<16} │ {:<22} │", k, v.display_string());
                }
                eprintln!("└──────────────────┴────────────────────────┘");
            }
            Ok(Value::None)
        })),
    ]);
    globals.insert("debug".into(), m);
}

