use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;

pub fn register_math_module(globals: &mut HashMap<String, Value>) {
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
