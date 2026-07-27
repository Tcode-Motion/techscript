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
