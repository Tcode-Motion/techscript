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
    use techscript_runtime::RuntimeContext;

    #[test]
    fn test_register_redis() {
        let mut registry = StdlibRegistry::default();
        registry.register_redis();

        let module = registry
            .modules
            .get("std.redis")
            .expect("Module std.redis should be registered");
        assert_eq!(module.name, "std.redis");
        assert_eq!(module.version, "1.0.0");

        let connect_fn = module
            .exports
            .get("connect")
            .expect("connect function should be exported");
        assert_eq!(connect_fn.arity(), 1);

        let mut ctx = RuntimeContext::new(techscript_runtime::RuntimeConfig::default());
        let args = vec![RuntimeValue::Str("localhost:6379".to_string())];
        let result = connect_fn
            .call(&mut ctx, args)
            .expect("connect function should succeed");

        assert_eq!(
            result,
            RuntimeValue::Str("Redis not yet available".to_string())
        );
    }
}
