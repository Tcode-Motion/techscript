use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use indexmap::IndexMap;
use techscript_runtime::{
    context::Capability,
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_net(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
            "tcp_listen".to_string(),
            Rc::new(StdFunction {
                name: "tcp_listen".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    let port = args[0].try_into_int()?;
                    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("TCP bind error: {}", e)), None, None)
                    })?;
                    let handle_id = ctx.resources.borrow_mut().insert(listener);
                    let mut listener_map = IndexMap::new();
                    listener_map.insert("port".to_string(), RuntimeValue::Int(port));
                    listener_map.insert("_handle".to_string(), RuntimeValue::Int(handle_id as i64));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(listener_map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "tcp_connect".to_string(),
            Rc::new(StdFunction {
                name: "tcp_connect".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    let ip = args[0].try_into_string()?;
                    let port = args[1].try_into_int()?;
                    let stream = std::net::TcpStream::connect(format!("{}:{}", ip, port)).map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("TCP connect error: {}", e)), None, None)
                    })?;
                    let handle_id = ctx.resources.borrow_mut().insert(stream);
                    let mut stream_map = IndexMap::new();
                    stream_map.insert("ip".to_string(), RuntimeValue::Str(ip));
                    stream_map.insert("port".to_string(), RuntimeValue::Int(port));
                    stream_map.insert("_handle".to_string(), RuntimeValue::Int(handle_id as i64));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(stream_map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "tcp_send".to_string(),
            Rc::new(StdFunction {
                name: "tcp_send".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let handle_id = entries.borrow().get("_handle").cloned().unwrap_or(RuntimeValue::Null).try_into_int()? as u32;
                        let mut resources = ctx.resources.borrow_mut();
                        if let Some(stream) = resources.get_mut::<std::net::TcpStream>(handle_id) {
                            use std::io::Write;
                            let msg = args[1].try_into_string()?;
                            stream.write_all(msg.as_bytes()).ok();
                        }
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "tcp_recv".to_string(),
            Rc::new(StdFunction {
                name: "tcp_recv".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let handle_id = entries.borrow().get("_handle").cloned().unwrap_or(RuntimeValue::Null).try_into_int()? as u32;
                        let mut resources = ctx.resources.borrow_mut();
                        if let Some(stream) = resources.get_mut::<std::net::TcpStream>(handle_id) {
                            use std::io::Read;
                            let mut buf = [0; 512];
                            if let Ok(n) = stream.read(&mut buf) {
                                return Ok(RuntimeValue::Str(String::from_utf8_lossy(&buf[..n]).to_string()));
                            }
                        }
                    }
                    Ok(RuntimeValue::Str(String::new()))
                },
            }),
        );

        self.register_module(
            "std.net",
            StdlibModule {
                name: "std.net".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Network],
            },
        );
    }
}
