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
        crate::run::exit(1);
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
        crate::run::exit(code as i32);
    }));
    globals.insert("version".into(), native!("version", |_| {
        Ok(Value::String(Rc::new("1.0.6".into())))
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
    globals.insert("to_int".into(), globals.get("int").unwrap().clone());
    
    globals.insert("float".into(), native!("float", |args| {
        match args.first() {
            Some(Value::Int(i)) => Ok(Value::Float(*i as f64)),
            Some(Value::Float(f)) => Ok(Value::Float(*f)),
            Some(Value::String(s)) => Ok(Value::Float(s.parse::<f64>().unwrap_or(0.0))),
            _ => Ok(Value::Float(0.0)),
        }
    }));
    globals.insert("to_float".into(), globals.get("float").unwrap().clone());
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
        ("randint",  native!("randint",  |args| { let lo = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0); let hi = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(100); let span = (hi as i128).saturating_sub(lo as i128).saturating_add(1).max(1); let offset = ((pseudo_random() * span as f64).floor() as i128).min(span - 1); let v = (lo as i128).saturating_add(offset); Ok(Value::Int(v.clamp(i64::MIN as i128, i64::MAX as i128) as i64)) })),
        ("randfloat",native!("randfloat",|args| { let lo = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0); let hi = args.get(1).and_then(|v| v.as_f64()).unwrap_or(1.0); Ok(Value::Float(lo + pseudo_random() * (hi - lo))) })),
        ("choice",   native!("choice",   |args| { if let Some(Value::List(l)) = args.first() { let b = l.borrow(); if b.is_empty() { return Ok(Value::None); } let idx = (pseudo_random() * b.len() as f64) as usize; Ok(b[idx.min(b.len()-1)].clone()) } else { Ok(Value::None) } })),
        ("boolean",  native!("boolean",  |_| { Ok(Value::Bool(pseudo_random() >= 0.5)) })),
        ("uuid",     native!("uuid",     |_| { let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos(); Ok(Value::String(Rc::new(format!("{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}", t & 0xFFFFFFFF, (t >> 32) & 0xFFFF, (t >> 48) & 0xFFF, 0x8000 | ((t >> 60) & 0x3FFF), t & 0xFFFFFFFFFFFF)))) })),
        ("sample",   native!("sample",   |args| { if let Some(Value::List(l)) = args.first() { let n = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i as usize) } else { None }).unwrap_or(1); let mut pool = l.borrow().clone(); let mut result = Vec::new(); for _ in 0..n.min(pool.len()) { let idx = (pseudo_random() * pool.len() as f64) as usize; result.push(pool.remove(idx.min(pool.len()-1))); } Ok(Value::List(Rc::new(RefCell::new(result)))) } else { Ok(Value::List(Rc::new(RefCell::new(Vec::new())))) } })),
        // Also keep global random/random_int aliases
    ]);
    globals.insert("random_mod".into(), m);
    globals.insert("random".into(), native!("random", |_| { Ok(Value::Float(pseudo_random())) }));
    // Global shorthand aliases
    globals.insert("random_int".into(), native!("random_int", |args| {
        let lo = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0);
        let hi = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(100);
        let span = (hi as i128).saturating_sub(lo as i128).saturating_add(1).max(1);
        let offset = ((pseudo_random() * span as f64).floor() as i128).min(span - 1);
        let v = (lo as i128).saturating_add(offset);
        Ok(Value::Int(v.clamp(i64::MIN as i128, i64::MAX as i128) as i64))
    }));
    globals.insert("randint".into(), native!("randint", |args| {
        let lo = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0);
        let hi = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(100);
        let span = (hi as i128).saturating_sub(lo as i128).saturating_add(1).max(1);
        let offset = ((pseudo_random() * span as f64).floor() as i128).min(span - 1);
        let v = (lo as i128).saturating_add(offset);
        Ok(Value::Int(v.clamp(i64::MIN as i128, i64::MAX as i128) as i64))
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

/// Simple pseudo-random in [0, 1) using system time (LCG on low bits).
fn pseudo_random() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let x = t.wrapping_mul(1_103_515_245).wrapping_add(12_345);
    (((x >> 16) & 0x7fff) as f64) / 32768.0
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
