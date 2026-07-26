use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_oauth(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("authorize".to_string(), Rc::new(StdFunction {
            name: "authorize".to_string(),
            arity: 1,
            callback: |_ctx, _args| {
                Ok(RuntimeValue::Str("OAuth not yet available".to_string()))
            },
        }));

        self.register_module("std.oauth", StdlibModule {
            name: "std.oauth".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
