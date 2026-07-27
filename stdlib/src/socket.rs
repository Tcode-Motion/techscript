use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::net::TcpStream;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_socket(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "connect".to_string(),
            Rc::new(StdFunction {
                name: "connect".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let host = match &args[0] {
                        RuntimeValue::Str(s) => s.clone(),
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "string".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    let port = match &args[1] {
                        RuntimeValue::Int(n) => *n as u16,
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "int".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    let addr = format!("{}:{}", host, port);
                    TcpStream::connect(&addr).map_err(|e| {
                        RuntimeError::new(
                            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                                e.to_string(),
                            ),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Str(format!("Connected to {}", addr)))
                },
            }),
        );

        exports.insert(
            "listen".to_string(),
            Rc::new(StdFunction {
                name: "listen".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let port = match &args[0] {
                        RuntimeValue::Int(n) => *n as u16,
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "int".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    let addr = format!("0.0.0.0:{}", port);
                    let _listener = std::net::TcpListener::bind(&addr).map_err(|e| {
                        RuntimeError::new(
                            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                                e.to_string(),
                            ),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Str(format!("Listening on {}", addr)))
                },
            }),
        );

        self.register_module(
            "std.socket",
            StdlibModule {
                name: "std.socket".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
