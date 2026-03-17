use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;

pub fn value_to_json(v: &Value) -> String {
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

pub fn value_to_json_pretty(v: &Value, indent: usize) -> String {
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

pub fn json_to_value(s: &str) -> Result<Value, String> {
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

pub fn register_json_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("encode",        native!("encode",        |args| { let v = args.first().cloned().unwrap_or(Value::None); let s = value_to_json(&v); Ok(Value::String(Rc::new(s))) })),
        ("encode_pretty", native!("encode_pretty", |args| { let v = args.first().cloned().unwrap_or(Value::None); let s = value_to_json_pretty(&v, 0); Ok(Value::String(Rc::new(s))) })),
        ("decode",        native!("decode",        |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); json_to_value(&s).map_err(|e| format!("json.decode: {}", e)) })),
    ]);
    globals.insert("json".into(), m);
}
