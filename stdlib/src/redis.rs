use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_redis(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "connect".to_string(),
            Rc::new(StdFunction {
                name: "connect".to_string(),
                arity: 1,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Str("Redis not yet available".to_string()))
                },
            }),
        );

        self.register_module(
            "std.redis",
            StdlibModule {
                name: "std.redis".to_string(),
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
    use techscript_runtime::RuntimeContext;

    #[test]
    fn test_register_redis() {
        let mut registry = StdlibRegistry {
            modules: std::collections::HashMap::new(),
        };
        registry.register_redis();

        assert!(registry.modules.contains_key("std.redis"));
        let module = registry.modules.get("std.redis").unwrap();
        assert_eq!(module.name, "std.redis");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("connect"));

        let connect_func = module.exports.get("connect").unwrap();
        assert_eq!(connect_func.name(), "connect");
        assert_eq!(connect_func.arity(), 1);

        use techscript_runtime::RuntimeConfig;
        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let res = connect_func.call(
            &mut ctx,
            vec![RuntimeValue::Str("redis://localhost".to_string())],
        );
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            RuntimeValue::Str("Redis not yet available".to_string())
        );
    }
}
