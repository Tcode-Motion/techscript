use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_dns(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "lookup".to_string(),
            Rc::new(StdFunction {
                name: "lookup".to_string(),
                arity: 1,
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
                    let addrs = format!("{}:0", host).to_socket_addrs().map_err(|e| {
                        RuntimeError::new(
                            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                                e.to_string(),
                            ),
                            None,
                            None,
                        )
                    })?;
                    let ip = addrs.map(|a| a.ip().to_string()).next().unwrap_or_default();
                    Ok(RuntimeValue::Str(ip))
                },
            }),
        );

        self.register_module(
            "std.dns",
            StdlibModule {
                name: "std.dns".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
