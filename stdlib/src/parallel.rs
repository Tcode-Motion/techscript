use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_parallel(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "run".to_string(),
            Rc::new(StdFunction {
                name: "run".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let _task = args[0].to_string();
                    thread::spawn(move || {});
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "sleep".to_string(),
            Rc::new(StdFunction {
                name: "sleep".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let ms = args[0].try_into_int().unwrap_or(0) as u64;
                    thread::sleep(std::time::Duration::from_millis(ms));
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "num_cpus".to_string(),
            Rc::new(StdFunction {
                name: "num_cpus".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Int(
                        std::thread::available_parallelism()
                            .map(|n| n.get() as i64)
                            .unwrap_or(1),
                    ))
                },
            }),
        );

        self.register_module(
            "std.parallel",
            StdlibModule {
                name: "std.parallel".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
