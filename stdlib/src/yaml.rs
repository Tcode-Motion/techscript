use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;
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
                            let _ = write!(result, "{}: {}\n", k, val_str);
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
    use techscript_runtime::context::{RuntimeConfig, RuntimeContext};
    use techscript_runtime::value::RuntimeValue;

    #[test]
    fn test_register_yaml() {
        let mut registry = StdlibRegistry::default();
        registry.register_yaml();

        let module = registry
            .modules
            .get("std.yaml")
            .expect("Module std.yaml should be registered");
        assert_eq!(module.name, "std.yaml");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("parse"));
        assert!(module.exports.contains_key("stringify"));
    }

    #[test]
    fn test_yaml_parse() {
        let mut registry = StdlibRegistry::default();
        registry.register_yaml();

        let module = registry.modules.get("std.yaml").unwrap();
        let parse_fn = module.exports.get("parse").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let yaml_input = "# Header comment\n\nname: Tanmoy\nage: 25\n# Footer comment\n";
        let args = vec![RuntimeValue::Str(yaml_input.to_string())];

        let result = parse_fn.call(&mut ctx, args).unwrap();

        if let RuntimeValue::Map { entries, .. } = result {
            let map = entries.borrow();
            assert_eq!(
                map.get("name").unwrap().try_into_string().unwrap(),
                "Tanmoy"
            );
            assert_eq!(map.get("age").unwrap().try_into_string().unwrap(), "25");
        } else {
            panic!("Result is not a Map");
        }
    }

    #[test]
    fn test_yaml_stringify() {
        let mut registry = StdlibRegistry::default();
        registry.register_yaml();

        let module = registry.modules.get("std.yaml").unwrap();
        let stringify_fn = module.exports.get("stringify").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        let mut map = indexmap::IndexMap::new();
        map.insert("name".to_string(), RuntimeValue::Str("Tanmoy".to_string()));
        map.insert("age".to_string(), RuntimeValue::Int(25));

        let input_map = RuntimeValue::Map {
            entries: Rc::new(RefCell::new(map)),
            is_const: false,
        };

        let result = stringify_fn.call(&mut ctx, vec![input_map]).unwrap();

        let res_str = result.try_into_string().unwrap();
        assert!(res_str.contains("name: Tanmoy\n"));
        assert!(res_str.contains("age: 25\n"));
    }
}
