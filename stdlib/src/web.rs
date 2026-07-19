use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue, function::Callable, context::RuntimeContext};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

static SERVER_RUNNING: AtomicBool = AtomicBool::new(false);
static PAGE_CONTENT: Mutex<String> = Mutex::new(String::new());

impl StdlibRegistry {
    pub fn register_web(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert("start".to_string(), Rc::new(StdFunction {
            name: "start".to_string(),
            arity: 2,
            callback: |_ctx, args| {
                let port = args[0].try_into_int().map_err(|e| RuntimeError::new(
                    techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))? as u16;
                let content = args[1].to_string();
                *PAGE_CONTENT.lock().unwrap() = content;
                if SERVER_RUNNING.load(Ordering::SeqCst) {
                    return Ok(RuntimeValue::Str("Server already running".to_string()));
                }
                SERVER_RUNNING.store(true, Ordering::SeqCst);
                let server = Mutex::new(tiny_http::Server::http(format!("0.0.0.0:{}", port)).unwrap());
                thread::spawn(move || {
                    while SERVER_RUNNING.load(Ordering::SeqCst) {
                        let page = PAGE_CONTENT.lock().unwrap().clone();
                        if let Ok(mut req) = server.lock().unwrap().recv() {
                            let r = tiny_http::Response::from_string(&page)
                                .with_header(
                                    tiny_http::Header::from_bytes(
                                        &b"Content-Type"[..], &b"text/html; charset=utf-8"[..],
                                    ).unwrap()
                                );
                            let _ = req.respond(r);
                        }
                    }
                });
                Ok(RuntimeValue::Str(format!("Server started on port {}", port)))
            },
        }));

        exports.insert("page".to_string(), Rc::new(StdFunction {
            name: "page".to_string(),
            arity: 2,
            callback: |_ctx, args| {
                let _path = args[0].to_string();
                let content = args[1].to_string();
                *PAGE_CONTENT.lock().unwrap() = content;
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("serve".to_string(), Rc::new(StdFunction {
            name: "serve".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let port = args[0].try_into_int().map_err(|e| RuntimeError::new(
                    techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))? as u16;
                if SERVER_RUNNING.load(Ordering::SeqCst) {
                    return Ok(RuntimeValue::Str("Server already running".to_string()));
                }
                SERVER_RUNNING.store(true, Ordering::SeqCst);
                let server = Mutex::new(tiny_http::Server::http(format!("0.0.0.0:{}", port)).unwrap());
                thread::spawn(move || {
                    while SERVER_RUNNING.load(Ordering::SeqCst) {
                        let page = PAGE_CONTENT.lock().unwrap().clone();
                        if let Ok(mut req) = server.lock().unwrap().recv() {
                            let r = tiny_http::Response::from_string(&page)
                                .with_header(
                                    tiny_http::Header::from_bytes(
                                        &b"Content-Type"[..], &b"text/html; charset=utf-8"[..],
                                    ).unwrap()
                                );
                            let _ = req.respond(r);
                        }
                    }
                });
                Ok(RuntimeValue::Str(format!("Server started on port {}", port)))
            },
        }));

        exports.insert("stop".to_string(), Rc::new(StdFunction {
            name: "stop".to_string(),
            arity: 0,
            callback: |_ctx, _args| {
                SERVER_RUNNING.store(false, Ordering::SeqCst);
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("set_content".to_string(), Rc::new(StdFunction {
            name: "set_content".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let new_content = args[0].to_string();
                *PAGE_CONTENT.lock().unwrap() = new_content;
                Ok(RuntimeValue::Str("Content updated".to_string()))
            },
        }));

        exports.insert("fetch".to_string(), Rc::new(StdFunction {
            name: "fetch".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let url = args[0].to_string();
                let body = ureq::get(&url)
                    .call()
                    .map_err(|e| RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?
                    .into_string()
                    .map_err(|e| RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                Ok(RuntimeValue::Str(body))
            },
        }));

        self.register_module("std.web", StdlibModule {
            name: "std.web".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
