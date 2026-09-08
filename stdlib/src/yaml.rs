use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_yaml(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let yaml = args[0].try_into_string()?;
                    let mut map = IndexMap::new();
                    for line in yaml.lines() {
                        let line = line.trim();
                        if line.starts_with('#') || line.is_empty() {
                            continue;
                        }
                        if let Some(pos) = line.find(':') {
                            let k = line[..pos].trim().to_string();
                            let v = line[pos + 1..].trim().to_string();
                            map.insert(k, RuntimeValue::Str(v));
                        }
                    }
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "stringify".to_string(),
            Rc::new(StdFunction {
                name: "stringify".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let mut result = String::new();
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        for (k, v) in entries.borrow().iter() {
                            let val_str = v.try_into_string().unwrap_or_default();
                            result.push_str(&format!("{}: {}\n", k, val_str));
                        }
                    }
                    Ok(RuntimeValue::Str(result))
                },
            }),
        );

        self.register_module(
            "std.yaml",
            StdlibModule {
                name: "std.yaml".to_string(),
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
    use std::cell::RefCell;
    use std::rc::Rc;
    use techscript_runtime::RuntimeContext;
    use indexmap::IndexMap;

    #[test]
    fn test_register_yaml() {
        let mut registry = StdlibRegistry {
            modules: std::collections::HashMap::new(),
        };
        registry.register_yaml();
        assert!(registry.has_module("std.yaml"));
    }

    #[test]
    fn test_yaml_parse() {
        let mut registry = StdlibRegistry {
            modules: std::collections::HashMap::new(),
        };
        registry.register_yaml();
        let module = registry.get_module("std.yaml").unwrap();
        let parse_func = module.exports.get("parse").unwrap();

        let mut ctx = RuntimeContext::new(techscript_runtime::RuntimeConfig::default());
        let yaml_str = RuntimeValue::Str("key1: value1\n# comment\nkey2: value2".to_string());

        let result = parse_func.call(&mut ctx, vec![yaml_str]).unwrap();

        if let RuntimeValue::Map { entries, .. } = result {
            let entries = entries.borrow();
            assert_eq!(entries.len(), 2);
            assert_eq!(entries.get("key1").unwrap().try_into_string().unwrap(), "value1");
            assert_eq!(entries.get("key2").unwrap().try_into_string().unwrap(), "value2");
        } else {
            panic!("Expected Map");
        }
    }

    #[test]
    fn test_yaml_stringify() {
        let mut registry = StdlibRegistry {
            modules: std::collections::HashMap::new(),
        };
        registry.register_yaml();
        let module = registry.get_module("std.yaml").unwrap();
        let stringify_func = module.exports.get("stringify").unwrap();

        let mut ctx = RuntimeContext::new(techscript_runtime::RuntimeConfig::default());

        let mut map = IndexMap::new();
        map.insert("key1".to_string(), RuntimeValue::Str("value1".to_string()));
        map.insert("key2".to_string(), RuntimeValue::Str("value2".to_string()));

        let map_val = RuntimeValue::Map {
            entries: Rc::new(RefCell::new(map)),
            is_const: false,
        };

        let result = stringify_func.call(&mut ctx, vec![map_val]).unwrap();

        assert_eq!(result.try_into_string().unwrap(), "key1: value1\nkey2: value2\n");
    }
}
