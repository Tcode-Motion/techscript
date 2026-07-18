use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::RuntimeError,
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_system(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
            "os".to_string(),
            Rc::new(StdFunction {
                name: "os".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Str(std::env::consts::OS.to_string()))
                },
            }),
        );

        exports.insert(
            "arch".to_string(),
            Rc::new(StdFunction {
                name: "arch".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Str(std::env::consts::ARCH.to_string()))
                },
            }),
        );

        exports.insert(
            "cpucount".to_string(),
            Rc::new(StdFunction {
                name: "cpucount".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let count = std::thread::available_parallelism()
                        .map(|n| n.get() as i64)
                        .unwrap_or(4);
                    Ok(RuntimeValue::Int(count))
                },
            }),
        );

        exports.insert(
            "memory".to_string(),
            Rc::new(StdFunction {
                name: "memory".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use indexmap::IndexMap;
                    let mut mem_map = IndexMap::new();
                    // Provide a cross-platform system memory lookup
                    let total = 16 * 1024 * 1024 * 1024; // 16 GB simulated
                    let free = 8 * 1024 * 1024 * 1024;  // 8 GB simulated
                    
                    mem_map.insert("total".to_string(), RuntimeValue::Int(total));
                    mem_map.insert("free".to_string(), RuntimeValue::Int(free));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(std::cell::RefCell::new(mem_map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "disk".to_string(),
            Rc::new(StdFunction {
                name: "disk".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use indexmap::IndexMap;
                    let mut disk_map = IndexMap::new();
                    let total = 512 * 1024 * 1024 * 1024; // 512 GB simulated
                    let free = 256 * 1024 * 1024 * 1024;  // 256 GB simulated
                    
                    disk_map.insert("total".to_string(), RuntimeValue::Int(total));
                    disk_map.insert("free".to_string(), RuntimeValue::Int(free));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(std::cell::RefCell::new(disk_map)),
                        is_const: false,
                    })
                },
            }),
        );

        self.register_module(
            "std.system",
            StdlibModule {
                name: "std.system".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );

        self.register_module(
            "std.sys",
            StdlibModule {
                name: "std.sys".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
