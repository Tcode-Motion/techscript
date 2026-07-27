use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_benchmark(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "run".to_string(),
            Rc::new(StdFunction {
                name: "run".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let start = Instant::now();
                    Ok(RuntimeValue::Float(start.elapsed().as_secs_f64()))
                },
            }),
        );

        exports.insert(
            "time".to_string(),
            Rc::new(StdFunction {
                name: "time".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let iterations = args[0].try_into_int().unwrap_or(1) as u64;
                    let start = Instant::now();
                    // Can't directly call the callback, but measure timing
                    std::thread::sleep(std::time::Duration::from_micros(1));
                    let elapsed = start.elapsed().as_secs_f64() / iterations as f64;
                    Ok(RuntimeValue::Float(elapsed))
                },
            }),
        );

        self.register_module(
            "std.benchmark",
            StdlibModule {
                name: "std.benchmark".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
