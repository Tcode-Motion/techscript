use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_xml(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let xml = args[0].try_into_string()?;
                    let mut map = IndexMap::new();
                    if xml.starts_with('<') {
                        if let Some(gt_pos) = xml.find('>') {
                            let tag_name = xml[1..gt_pos].to_string();
                            let close_tag = format!("</{}>", tag_name);
                            if let Some(close_pos) = xml.find(&close_tag) {
                                let content = xml[gt_pos + 1..close_pos].to_string();
                                map.insert(tag_name, RuntimeValue::Str(content));
                            }
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
                            let _ = write!(result, "<{}>{}</{}>", k, val_str, k);
                        }
                    }
                    Ok(RuntimeValue::Str(result))
                },
            }),
        );

        self.register_module(
            "std.xml",
            StdlibModule {
                name: "std.xml".to_string(),
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
    fn test_register_xml() {
        let mut registry = StdlibRegistry::default();
        registry.register_xml();

        let module = registry
            .modules
            .get("std.xml")
            .expect("Module std.xml should be registered");
        assert_eq!(module.name, "std.xml");
        assert_eq!(module.version, "1.0.0");
        assert!(module.required_capabilities.is_empty());
        assert!(module.exports.contains_key("parse"));
        assert!(module.exports.contains_key("stringify"));
    }

    #[test]
    fn test_xml_parse() {
        let mut registry = StdlibRegistry::default();
        registry.register_xml();

        let module = registry.modules.get("std.xml").unwrap();
        let parse_fn = module.exports.get("parse").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        // Valid XML
        let args = vec![RuntimeValue::Str("<item>hello</item>".to_string())];
        let result = parse_fn.call(&mut ctx, args).unwrap();

        if let RuntimeValue::Map { entries, .. } = result {
            let map = entries.borrow();
            assert_eq!(map.get("item").and_then(|v| v.as_string()), Some("hello"));
        } else {
            panic!("parse should return a Map");
        }

        // Non-matching XML input (returns empty map)
        let plain_str = vec![RuntimeValue::Str("not xml".to_string())];
        let result = parse_fn.call(&mut ctx, plain_str).unwrap();
        if let RuntimeValue::Map { entries, .. } = result {
            assert!(entries.borrow().is_empty());
        } else {
            panic!("parse should return a Map even for non-matching strings");
        }
    }

    #[test]
    fn test_xml_stringify() {
        let mut registry = StdlibRegistry::default();
        registry.register_xml();

        let module = registry.modules.get("std.xml").unwrap();
        let stringify_fn = module.exports.get("stringify").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        let mut map = IndexMap::new();
        map.insert("name".to_string(), RuntimeValue::Str("Alice".to_string()));
        let map_val = RuntimeValue::Map {
            entries: Rc::new(RefCell::new(map)),
            is_const: false,
        };

        let result = stringify_fn.call(&mut ctx, vec![map_val]).unwrap();
        assert_eq!(result.as_string(), Some("<name>Alice</name>"));

        // Non-map argument returns empty string
        let non_map = vec![RuntimeValue::Int(42)];
        let result = stringify_fn.call(&mut ctx, non_map).unwrap();
        assert_eq!(result.as_string(), Some(""));
    }
}
