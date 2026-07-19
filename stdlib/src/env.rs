use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_env(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("get".to_string(), Rc::new(StdFunction {
            name: "get".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let key = match &args[0] {
                    RuntimeValue::Str(s) => s.clone(),
                    _ => return Err(RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::TypeMismatch { expected: "string".to_string(), found: "other".to_string() },
                        None, None,
                    )),
                };
                let val = std::env::var(&key).unwrap_or_default();
                Ok(RuntimeValue::Str(val))
            },
        }));

        exports.insert("set".to_string(), Rc::new(StdFunction {
            name: "set".to_string(),
            arity: 2,
            callback: |_ctx, args| {
                let key = match &args[0] {
                    RuntimeValue::Str(s) => s.clone(),
                    _ => return Err(RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::TypeMismatch { expected: "string".to_string(), found: "other".to_string() },
                        None, None,
                    )),
                };
                let val = match &args[1] {
                    RuntimeValue::Str(s) => s.clone(),
                    _ => return Err(RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::TypeMismatch { expected: "string".to_string(), found: "other".to_string() },
                        None, None,
                    )),
                };
                std::env::set_var(&key, &val);
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("all".to_string(), Rc::new(StdFunction {
            name: "all".to_string(),
            arity: 0,
            callback: |_ctx, _args| {
                let mut entries = indexmap::IndexMap::new();
                for (k, v) in std::env::vars() {
                    entries.insert(k, RuntimeValue::Str(v));
                }
                Ok(RuntimeValue::Map {
                    entries: std::rc::Rc::new(std::cell::RefCell::new(entries)),
                    is_const: true,
                })
            },
        }));

        self.register_module("std.env", StdlibModule {
            name: "std.env".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
