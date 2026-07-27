use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_postgres(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "connect".to_string(),
            Rc::new(StdFunction {
                name: "connect".to_string(),
                arity: 1,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Str(
                        "PostgreSQL not yet available".to_string(),
                    ))
                },
            }),
        );

        self.register_module(
            "std.postgres",
            StdlibModule {
                name: "std.postgres".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
