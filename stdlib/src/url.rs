use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_url(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "url_parse".to_string(),
            Rc::new(StdFunction {
                name: "url_parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let url = args[0].try_into_string()?;
                    let mut map = IndexMap::new();
                    map.insert(
                        "protocol".to_string(),
                        RuntimeValue::Str("http".to_string()),
                    );
                    map.insert(
                        "host".to_string(),
                        RuntimeValue::Str("localhost".to_string()),
                    );
                    map.insert("path".to_string(), RuntimeValue::Str(url));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(map)),
                        is_const: false,
                    })
                },
            }),
        );

        self.register_module(
            "std.url",
            StdlibModule {
                name: "std.url".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
