use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::RuntimeError,
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_logging(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
            "info".to_string(),
            Rc::new(StdFunction {
                name: "info".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let msg = args[0].try_into_string()?;
                    println!("[INFO] {}", msg);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "warn".to_string(),
            Rc::new(StdFunction {
                name: "warn".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let msg = args[0].try_into_string()?;
                    println!("[WARN] {}", msg);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "error".to_string(),
            Rc::new(StdFunction {
                name: "error".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let msg = args[0].try_into_string()?;
                    eprintln!("[ERROR] {}", msg);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "debug".to_string(),
            Rc::new(StdFunction {
                name: "debug".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let msg = args[0].try_into_string()?;
                    println!("[DEBUG] {}", msg);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.logging",
            StdlibModule {
                name: "std.logging".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
