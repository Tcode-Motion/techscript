use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_jwt(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "encode".to_string(),
            Rc::new(StdFunction {
                name: "encode".to_string(),
                arity: 2,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Str(
                        "JWT encode not yet available".to_string(),
                    ))
                },
            }),
        );

        exports.insert(
            "decode".to_string(),
            Rc::new(StdFunction {
                name: "decode".to_string(),
                arity: 2,
                callback: |_ctx, _args| Ok(RuntimeValue::Null),
            }),
        );

        self.register_module(
            "std.jwt",
            StdlibModule {
                name: "std.jwt".to_string(),
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
    fn test_register_jwt() {
        let mut registry = StdlibRegistry::new();
        registry.register_jwt();

        let module = registry.get_module("std.jwt").unwrap();

        let encode_fn = module.exports.get("encode").unwrap();
        assert_eq!(encode_fn.name(), "encode");
        assert_eq!(encode_fn.arity(), 2);

        let decode_fn = module.exports.get("decode").unwrap();
        assert_eq!(decode_fn.name(), "decode");
        assert_eq!(decode_fn.arity(), 2);

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        let encode_result = encode_fn
            .call(&mut ctx, vec![RuntimeValue::Null, RuntimeValue::Null])
            .unwrap();
        assert_eq!(
            encode_result,
            RuntimeValue::Str("JWT encode not yet available".to_string())
        );

        let decode_result = decode_fn
            .call(&mut ctx, vec![RuntimeValue::Null, RuntimeValue::Null])
            .unwrap();
        assert_eq!(decode_result, RuntimeValue::Null);
    }
}
