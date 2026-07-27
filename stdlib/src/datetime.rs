use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_datetime(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "epoch".to_string(),
            Rc::new(StdFunction {
                name: "epoch".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let dur = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();
                    Ok(RuntimeValue::Float(dur.as_secs_f64()))
                },
            }),
        );

        exports.insert(
            "format".to_string(),
            Rc::new(StdFunction {
                name: "format".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let epoch = args[0].try_into_float()?;
                    let fmt = args[1].try_into_string()?;
                    Ok(RuntimeValue::Str(format!(
                        "Formatted {} using format {}",
                        epoch, fmt
                    )))
                },
            }),
        );

        self.register_module(
            "std.datetime",
            StdlibModule {
                name: "std.datetime".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
