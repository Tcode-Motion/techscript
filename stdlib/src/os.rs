use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_os(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("name".to_string(), Rc::new(StdFunction {
            name: "name".to_string(),
            arity: 0,
            callback: |_ctx, _args| {
                Ok(RuntimeValue::Str(std::env::consts::OS.to_string()))
            },
        }));

        exports.insert("hostname".to_string(), Rc::new(StdFunction {
            name: "hostname".to_string(),
            arity: 0,
            callback: |_ctx, _args| {
                let host = std::env::var("COMPUTERNAME").unwrap_or_default();
                Ok(RuntimeValue::Str(host))
            },
        }));

        exports.insert("version".to_string(), Rc::new(StdFunction {
            name: "version".to_string(),
            arity: 0,
            callback: |_ctx, _args| {
                Ok(RuntimeValue::Str(std::env::consts::ARCH.to_string()))
            },
        }));

        self.register_module("std.os", StdlibModule {
            name: "std.os".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
