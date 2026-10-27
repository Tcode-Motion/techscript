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
    use techscript_runtime::context::{RuntimeConfig, RuntimeContext};

    #[test]
    fn test_register_redis() {
        let mut registry = StdlibRegistry::new();
        registry.register_redis();

        assert!(registry.has_module("std.redis"));
        let module = registry.get_module("std.redis").unwrap();
        assert_eq!(module.name, "std.redis");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("connect"));

        let connect_fn = module.exports.get("connect").unwrap();
        assert_eq!(connect_fn.name(), "connect");
        assert_eq!(connect_fn.arity(), 1);

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let res = connect_fn
            .call(
                &mut ctx,
                vec![RuntimeValue::Str("redis://127.0.0.1:6379".to_string())],
            )
            .unwrap();
        assert_eq!(
            res,
            RuntimeValue::Str("Redis not yet available".to_string())
        );
    }
}
