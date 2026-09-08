use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_mongodb(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "connect".to_string(),
            Rc::new(StdFunction {
                name: "connect".to_string(),
                arity: 1,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Str("MongoDB not yet available".to_string()))
                },
            }),
        );

        self.register_module(
            "std.mongodb",
            StdlibModule {
                name: "std.mongodb".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StdlibRegistry;

    #[test]
    fn test_register_mongodb() {
        let mut registry = StdlibRegistry {
            modules: std::collections::HashMap::new(),
        };
        registry.register_mongodb();

        assert!(registry.modules.contains_key("std.mongodb"));
        let module = registry.modules.get("std.mongodb").unwrap();
        assert_eq!(module.name, "std.mongodb");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("connect"));

        let connect_func = module.exports.get("connect").unwrap();
        assert_eq!(connect_func.name(), "connect");
        assert_eq!(connect_func.arity(), 1);
    }
}
