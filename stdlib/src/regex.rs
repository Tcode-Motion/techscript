use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_regex(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "match".to_string(),
            Rc::new(StdFunction {
                name: "match".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let pat = args[0].try_into_string()?;
                    let text = args[1].try_into_string()?;
                    Ok(RuntimeValue::Bool(text.contains(&pat)))
                },
            }),
        );

        exports.insert(
            "replace".to_string(),
            Rc::new(StdFunction {
                name: "replace".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let pat = args[0].try_into_string()?;
                    let text = args[1].try_into_string()?;
                    let repl = args[2].try_into_string()?;
                    Ok(RuntimeValue::Str(text.replace(&pat, &repl)))
                },
            }),
        );

        self.register_module(
            "std.regex",
            StdlibModule {
                name: "std.regex".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
