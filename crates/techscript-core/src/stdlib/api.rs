use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::builtins::make_module;
use crate::native;
use crate::stdlib::json::value_to_json;
use crate::value::Value;

fn parse_request_line(req: &str) -> (String, String) {
    if let Some(line) = req.lines().next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            return (parts[0].to_string(), parts[1].to_string());
        }
    }
    ("GET".into(), "/".into())
}

pub fn register_api_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("listen", native!("api.listen", |args| {
            let port = args
                .first()
                .and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None })
                .unwrap_or(3000);
            let once = args.get(1).map(|v| v.is_truthy()).unwrap_or(false);

            let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port))
                .map_err(|e| format!("api.listen: {}", e))?;

            println!("🚀 TechScript API listening on http://localhost:{}", port);
            println!("Press Ctrl+C to stop the server.");

            let mut served = 0usize;
            for stream in listener.incoming() {
                let Ok(mut s) = stream else { continue };
                use std::io::{Read, Write};
                let mut buf = [0u8; 8192];
                let n = s.read(&mut buf).unwrap_or(0);
                let req = String::from_utf8_lossy(&buf[..n]).into_owned();
                let (method, path) = parse_request_line(&req);

                // Default “introspection” response (matches docs/examples expectations).
                let mut map: HashMap<String, Value> = HashMap::new();
                map.insert("ok".into(), Value::Bool(true));
                map.insert("method".into(), Value::String(Rc::new(method)));
                map.insert("path".into(), Value::String(Rc::new(path)));
                map.insert("engine".into(), Value::String(Rc::new("techscript-rust".into())));

                let body = value_to_json(&Value::Map(Rc::new(RefCell::new(map))));
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );

                let _ = s.write_all(resp.as_bytes());
                let _ = s.flush();

                served += 1;
                if once && served >= 1 {
                    break;
                }
            }

            Ok(Value::None)
        })),

        // Response helpers (currently simple pass-through building blocks).
        ("json", native!("api.json", |args| {
            let v = args.first().cloned().unwrap_or(Value::None);
            Ok(Value::String(Rc::new(value_to_json(&v))))
        })),
        ("text", native!("api.text", |args| {
            Ok(Value::String(Rc::new(args.first().map(|v| v.display_string()).unwrap_or_default())))
        })),
        ("html", native!("api.html", |args| {
            Ok(Value::String(Rc::new(args.first().map(|v| v.display_string()).unwrap_or_default())))
        })),
        ("status", native!("api.status", |args| {
            let code = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(200);
            Ok(Value::Int(code))
        })),
    ]);

    globals.insert("api".into(), m);
}

