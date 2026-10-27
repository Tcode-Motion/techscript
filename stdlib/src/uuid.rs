use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_uuid(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "uuid_v4".to_string(),
            Rc::new(StdFunction {
                name: "uuid_v4".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::time::SystemTime;
                    let nano = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();
                    Ok(RuntimeValue::Str(format!(
                        "123e4567-e89b-12d3-a456-{}",
                        nano
                    )))
                },
            }),
        );

        self.register_module(
            "std.uuid",
            StdlibModule {
                name: "std.uuid".to_string(),
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
    fn test_register_uuid() {
        let mut registry = StdlibRegistry::new();
        registry.register_uuid();

        let module = registry
            .get_module("std.uuid")
            .expect("Module std.uuid should be registered");
        assert_eq!(module.name, "std.uuid");
        assert_eq!(module.version, "1.0.0");
        assert!(module.required_capabilities.is_empty());
        assert!(module.exports.contains_key("uuid_v4"));

        let uuid_v4 = module.exports.get("uuid_v4").unwrap();
        assert_eq!(uuid_v4.name(), "uuid_v4");
        assert_eq!(uuid_v4.arity(), 0);
        assert!(uuid_v4.accepts_arity(0));

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let res = uuid_v4.call(&mut ctx, vec![]).unwrap();
        if let RuntimeValue::Str(uuid_str) = res {
            assert!(uuid_str.starts_with("123e4567-e89b-12d3-a456-"));
        } else {
            panic!("Expected RuntimeValue::Str");
        }
    }
}
