use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_jwt(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("encode".to_string(), Rc::new(StdFunction {
            name: "encode".to_string(),
            arity: 2,
            callback: |_ctx, _args| {
                Ok(RuntimeValue::Str("JWT encode not yet available".to_string()))
            },
        }));

        exports.insert("decode".to_string(), Rc::new(StdFunction {
            name: "decode".to_string(),
            arity: 2,
            callback: |_ctx, _args| {
                Ok(RuntimeValue::Null)
            },
        }));

        self.register_module("std.jwt", StdlibModule {
            name: "std.jwt".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
