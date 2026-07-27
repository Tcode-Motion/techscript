use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    context::Capability,
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};

impl StdlibRegistry {
    pub fn register_http(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "get".to_string(),
            Rc::new(StdFunction {
                name: "get".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Network) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Network capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let url = args[0].try_into_string()?;
                    let response = ureq::get(&url).call().map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "HTTP GET request failed: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;
                    let status = response.status();
                    let body = response.into_string().map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "Failed to read HTTP response body: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;

                    let mut res_map = IndexMap::new();
                    res_map.insert("status".to_string(), RuntimeValue::Int(status as i64));
                    res_map.insert("body".to_string(), RuntimeValue::Str(body));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(res_map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "post".to_string(),
            Rc::new(StdFunction {
                name: "post".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Network) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Network capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let url = args[0].try_into_string()?;
                    let body = args[1].try_into_string()?;
                    let response = ureq::post(&url).send_string(&body).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "HTTP POST request failed: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;
                    let status = response.status();
                    let res_body = response.into_string().map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "Failed to read HTTP response body: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;

                    let mut res_map = IndexMap::new();
                    res_map.insert("status".to_string(), RuntimeValue::Int(status as i64));
                    res_map.insert("body".to_string(), RuntimeValue::Str(res_body));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(res_map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "listen".to_string(),
            Rc::new(StdFunction {
                name: "listen".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Network) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Network capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let port = args[0].try_into_int()?;
                    let callback = args[1].clone();

                    if let RuntimeValue::Function(func) = callback {
                        let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).map_err(|e| {
                            RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("HTTP listen bind error: {}", e)), None, None)
                        })?;

                        listener.set_nonblocking(true).ok();

                        if let Ok((mut stream, _)) = listener.accept() {
                            use std::io::{Read, Write};
                            let mut buf = [0; 1024];
                            if let Ok(n) = stream.read(&mut buf) {
                                let request_text = String::from_utf8_lossy(&buf[..n]);
                                let mut req_map = IndexMap::new();
                                req_map.insert("raw".to_string(), RuntimeValue::Str(request_text.to_string()));
                                let req_val = RuntimeValue::Map {
                                    entries: Rc::new(RefCell::new(req_map)),
                                    is_const: false,
                                };

                                if let Ok(res) = func.call(ctx, vec![req_val]) {
                                    let body = res.try_into_string().unwrap_or_else(|_| "Hello".to_string());
                                    let response_str = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                                        body.len(),
                                        body
                                    );
                                    stream.write_all(response_str.as_bytes()).ok();
                                }
                            }
                        }
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.http",
            StdlibModule {
                name: "std.http".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Network],
            },
        );
    }
}
