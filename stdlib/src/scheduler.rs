use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_scheduler(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "sleep".to_string(),
            Rc::new(StdFunction {
                name: "sleep".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let ms = args[0].try_into_int().unwrap_or(0) as u64;
                    thread::sleep(Duration::from_millis(ms));
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "delay".to_string(),
            Rc::new(StdFunction {
                name: "delay".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let ms = args[0].try_into_int().unwrap_or(0) as u64;
                    let msg = args[1].to_string();
                    thread::spawn(move || {
                        thread::sleep(Duration::from_millis(ms));
                    });
                    Ok(RuntimeValue::Str(format!("Delayed task: {}ms", ms)))
                },
            }),
        );

        exports.insert(
            "interval".to_string(),
            Rc::new(StdFunction {
                name: "interval".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let ms = args[0].try_into_int().unwrap_or(1000) as u64;
                    let _msg = args[1].to_string();
                    thread::spawn(move || loop {
                        thread::sleep(Duration::from_millis(ms));
                    });
                    Ok(RuntimeValue::Int(1))
                },
            }),
        );

        self.register_module(
            "std.scheduler",
            StdlibModule {
                name: "std.scheduler".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
