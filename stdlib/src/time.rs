use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_time(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("now".to_string(), Rc::new(StdFunction {
            name: "now".to_string(),
            arity: 0,
            callback: |_ctx, _args| {
                let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                Ok(RuntimeValue::Str(ts.to_string()))
            },
        }));

        exports.insert("sleep".to_string(), Rc::new(StdFunction {
            name: "sleep".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let ms = match &args[0] {
                    RuntimeValue::Int(n) => *n as u64,
                    RuntimeValue::Float(f) => *f as u64,
                    _ => return Err(RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::TypeMismatch { expected: "number".to_string(), found: "other".to_string() },
                        None, None,
                    )),
                };
                thread::sleep(std::time::Duration::from_millis(ms));
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("format".to_string(), Rc::new(StdFunction {
            name: "format".to_string(),
            arity: 1,
            callback: |_ctx, _args| {
                Ok(RuntimeValue::Null)
            },
        }));

        self.register_module("std.time", StdlibModule {
            name: "std.time".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
