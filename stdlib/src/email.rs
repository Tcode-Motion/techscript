use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_email(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("send".to_string(), Rc::new(StdFunction {
            name: "send".to_string(),
            arity: 3,
            callback: |_ctx, _args| {
                Ok(RuntimeValue::Str("Email not yet available".to_string()))
            },
        }));

        self.register_module("std.email", StdlibModule {
            name: "std.email".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
