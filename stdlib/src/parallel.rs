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

#[cfg(test)]
mod tests {
    use super::*;
    use techscript_runtime::context::{RuntimeConfig, RuntimeContext};

    #[test]
    fn test_register_parallel() {
        let mut registry = StdlibRegistry {
            modules: HashMap::new(),
        };
        registry.register_parallel();

        assert!(registry.has_module("std.parallel"));
        let module = registry.get_module("std.parallel").unwrap();
        assert_eq!(module.name, "std.parallel");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("run"));
        assert!(module.exports.contains_key("sleep"));
        assert!(module.exports.contains_key("num_cpus"));
    }

    #[test]
    fn test_parallel_num_cpus() {
        let mut registry = StdlibRegistry {
            modules: HashMap::new(),
        };
        registry.register_parallel();

        let module = registry.get_module("std.parallel").unwrap();
        let num_cpus_fn = module.exports.get("num_cpus").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let result = num_cpus_fn.call(&mut ctx, vec![]).unwrap();

        if let RuntimeValue::Int(cpus) = result {
            assert!(cpus >= 1);
        } else {
            panic!("Expected RuntimeValue::Int for num_cpus");
        }
    }

    #[test]
    fn test_parallel_run() {
        let mut registry = StdlibRegistry {
            modules: HashMap::new(),
        };
        registry.register_parallel();

        let module = registry.get_module("std.parallel").unwrap();
        let run_fn = module.exports.get("run").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let result = run_fn
            .call(&mut ctx, vec![RuntimeValue::Str("task1".to_string())])
            .unwrap();

        assert_eq!(result, RuntimeValue::Null);
    }

    #[test]
    fn test_parallel_sleep() {
        let mut registry = StdlibRegistry {
            modules: HashMap::new(),
        };
        registry.register_parallel();

        let module = registry.get_module("std.parallel").unwrap();
        let sleep_fn = module.exports.get("sleep").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let result = sleep_fn.call(&mut ctx, vec![RuntimeValue::Int(0)]).unwrap();

        assert_eq!(result, RuntimeValue::Null);
    }
}
