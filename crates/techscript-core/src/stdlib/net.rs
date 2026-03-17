use std::collections::HashMap;
use std::rc::Rc;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;

fn headers_from_map(v: Option<&Value>) -> Vec<(String, String)> {
    let mut headers = Vec::new();
    if let Some(Value::Map(m)) = v {
        for (k, val) in m.borrow().iter() {
            headers.push((k.clone(), val.display_string()));
        }
    }
    headers
}

pub fn register_net_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("get", native!("net.get", |args| {
            let url = args.first().map(|v| v.display_string()).unwrap_or_default();
            let headers = headers_from_map(args.get(1));

            let agent = ureq::agent();
            let mut req = agent.get(&url);
            for (k, v) in headers {
                req = req.header(&k, &v);
            }

            let mut resp = req.call().map_err(|e| format!("net.get: {}", e))?;
            let body = resp.body_mut().read_to_string().map_err(|e| format!("net.get: {}", e))?;
            Ok(Value::String(Rc::new(body)))
        })),
        ("post", native!("net.post", |args| {
            let url = args.first().map(|v| v.display_string()).unwrap_or_default();
            let body = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            let headers = headers_from_map(args.get(2));

            let agent = ureq::agent();
            let mut req = agent.post(&url);
            for (k, v) in headers {
                req = req.header(&k, &v);
            }

            // Default to text/plain unless caller set Content-Type explicitly.
            let has_ct = args.get(2).and_then(|v| if let Value::Map(m) = v { Some(m.borrow().contains_key("Content-Type")) } else { None }).unwrap_or(false);
            if !has_ct {
                req = req.header("Content-Type", "text/plain; charset=utf-8");
            }

            let mut resp = req.send(body).map_err(|e| format!("net.post: {}", e))?;
            let text = resp.body_mut().read_to_string().map_err(|e| format!("net.post: {}", e))?;
            Ok(Value::String(Rc::new(text)))
        })),
    ]);
    globals.insert("net".into(), m);
}
