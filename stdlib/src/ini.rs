use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_ini(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("parse".to_string(), Rc::new(StdFunction {
            name: "parse".to_string(),
            arity: 1,
            callback: |_ctx, _args| {
                Ok(RuntimeValue::Null)
            },
        }));

        self.register_module("std.ini", StdlibModule {
            name: "std.ini".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
