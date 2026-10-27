use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use techscript_runtime::{function::Callable, value::RuntimeValue};

fn fn_run() -> StdFunction {
    StdFunction {
        name: "run".to_string(),
        arity: 1,
        callback: |_ctx, args| {
            let _task = args[0].to_string();
            thread::spawn(move || {});
            Ok(RuntimeValue::Null)
        },
    }
}

fn fn_sleep() -> StdFunction {
    StdFunction {
        name: "sleep".to_string(),
        arity: 1,
        callback: |_ctx, args| {
            let ms = args[0].try_into_int().unwrap_or(0) as u64;
            thread::sleep(std::time::Duration::from_millis(ms));
            Ok(RuntimeValue::Null)
        },
    }
}

fn fn_num_cpus() -> StdFunction {
    StdFunction {
        name: "num_cpus".to_string(),
        arity: 0,
        callback: |_ctx, _args| {
            Ok(RuntimeValue::Int(
                std::thread::available_parallelism()
                    .map(|n| n.get() as i64)
                    .unwrap_or(1),
            ))
        },
    }
}

fn create_parallel_exports() -> HashMap<String, Rc<dyn Callable>> {
    let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();
    exports.insert("run".to_string(), Rc::new(fn_run()));
    exports.insert("sleep".to_string(), Rc::new(fn_sleep()));
    exports.insert("num_cpus".to_string(), Rc::new(fn_num_cpus()));
    exports
}

impl StdlibRegistry {
    pub fn register_parallel(&mut self) {
        self.register_module(
            "std.parallel",
            StdlibModule {
                name: "std.parallel".to_string(),
                version: "1.0.0".to_string(),
                exports: create_parallel_exports(),
                required_capabilities: Vec::new(),
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_parallel() {
        let mut registry = StdlibRegistry::new();
        registry.register_parallel();
        assert!(registry.get_module("std.parallel").is_some());

        let module = registry.get_module("std.parallel").unwrap();
        assert!(module.exports.contains_key("run"));
        assert!(module.exports.contains_key("sleep"));
        assert!(module.exports.contains_key("num_cpus"));
    }
}
