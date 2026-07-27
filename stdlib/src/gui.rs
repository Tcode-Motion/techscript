use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_gui(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "alert".to_string(),
            Rc::new(StdFunction {
                name: "alert".to_string(),
                arity: 1,
                callback: |_ctx, _args| Ok(RuntimeValue::Str("GUI not yet available".to_string())),
            }),
        );

        self.register_module(
            "std.gui",
            StdlibModule {
                name: "std.gui".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
