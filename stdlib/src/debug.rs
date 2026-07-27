use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_debug(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "inspect".to_string(),
            Rc::new(StdFunction {
                name: "inspect".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    eprintln!("[debug] {:?}", args[0]);
                    Ok(args[0].clone())
                },
            }),
        );

        exports.insert(
            "trace".to_string(),
            Rc::new(StdFunction {
                name: "trace".to_string(),
                arity: 0,
                callback: |_ctx, _args| Ok(RuntimeValue::Null),
            }),
        );

        self.register_module(
            "std.debug",
            StdlibModule {
                name: "std.debug".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
