use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;

/// Simple pseudo-random using system time (LCG).
pub fn pseudo_random() -> f64 {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
    ((t.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) >> 33) as f64) / u32::MAX as f64
}

pub fn register_random_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("random",   native!("random",   |_| { Ok(Value::Float(pseudo_random())) })),
        ("randint",  native!("randint",  |args| { let lo = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0); let hi = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(100); Ok(Value::Int(lo + (pseudo_random() * (hi - lo + 1) as f64) as i64)) })),
        ("randfloat",native!("randfloat",|args| { let lo = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0); let hi = args.get(1).and_then(|v| v.as_f64()).unwrap_or(1.0); Ok(Value::Float(lo + pseudo_random() * (hi - lo))) })),
        ("choice",   native!("choice",   |args| { if let Some(Value::List(l)) = args.first() { let b = l.borrow(); if b.is_empty() { return Ok(Value::None); } let idx = (pseudo_random() * b.len() as f64) as usize; Ok(b[idx.min(b.len()-1)].clone()) } else { Ok(Value::None) } })),
        ("boolean",  native!("boolean",  |_| { Ok(Value::Bool(pseudo_random() >= 0.5)) })),
        ("uuid",     native!("uuid",     |_| { let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos(); Ok(Value::String(Rc::new(format!("{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}", t & 0xFFFFFFFF, (t >> 32) & 0xFFFF, (t >> 48) & 0xFFF, 0x8000 | ((t >> 60) & 0x3FFF), t & 0xFFFFFFFFFFFF)))) })),
        ("sample",   native!("sample",   |args| { if let Some(Value::List(l)) = args.first() { let n = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i as usize) } else { None }).unwrap_or(1); let mut pool = l.borrow().clone(); let mut result = Vec::new(); for _ in 0..n.min(pool.len()) { let idx = (pseudo_random() * pool.len() as f64) as usize; result.push(pool.remove(idx.min(pool.len()-1))); } Ok(Value::List(Rc::new(RefCell::new(result)))) } else { Ok(Value::List(Rc::new(RefCell::new(Vec::new())))) } })),
    ]);
    globals.insert("random".into(), m);
    // Global shorthand aliases
    globals.insert("random_int".into(), native!("random_int", |args| {
        let lo = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(0);
        let hi = args.get(1).and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(100);
        Ok(Value::Int(lo + (pseudo_random() * (hi - lo + 1) as f64) as i64))
    }));
}
