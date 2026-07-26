use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use indexmap::IndexMap;
use techscript_runtime::{
    error::RuntimeError,
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_xml(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let xml = args[0].try_into_string()?;
                    let mut map = IndexMap::new();
                    if xml.starts_with('<') && xml.contains('>') {
                        let tag_name = xml[1..xml.find('>').unwrap_or(1)].to_string();
                        let close_tag = format!("</{}>", tag_name);
                        if let Some(close_pos) = xml.find(&close_tag) {
                            let content = xml[xml.find('>').unwrap() + 1 .. close_pos].to_string();
                            map.insert(tag_name, RuntimeValue::Str(content));
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
                            result.push_str(&format!("<{}>{}</{}>", k, val_str, k));
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
