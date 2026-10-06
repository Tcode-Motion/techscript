use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue, RuntimeContext};

fn sys_os(
    _ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Str(std::env::consts::OS.to_string()))
}

fn sys_arch(
    _ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Str(std::env::consts::ARCH.to_string()))
}

fn sys_cpucount(
    _ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let count = std::thread::available_parallelism()
        .map(|n| n.get() as i64)
        .unwrap_or(4);
    Ok(RuntimeValue::Int(count))
}

fn sys_memory(
    _ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    // ⚡ Bolt Performance Optimization:
    // Pre-allocating the IndexMap capacity matching the expected number of keys
    // avoids intermediate allocations and re-hashing during insertion.
    let mut mem_map = IndexMap::with_capacity(2);
    // Provide a cross-platform system memory lookup
    let total = 16 * 1024 * 1024 * 1024; // 16 GB simulated
    let free = 8 * 1024 * 1024 * 1024; // 8 GB simulated

    mem_map.insert("total".to_string(), RuntimeValue::Int(total));
    mem_map.insert("free".to_string(), RuntimeValue::Int(free));
    Ok(RuntimeValue::Map {
        entries: Rc::new(std::cell::RefCell::new(mem_map)),
        is_const: false,
    })
}

fn sys_disk(
    _ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    // ⚡ Bolt Performance Optimization:
    // Pre-allocating the IndexMap capacity matching the expected number of keys
    // avoids intermediate allocations and re-hashing during insertion.
    let mut disk_map = IndexMap::with_capacity(2);
    let total = 512 * 1024 * 1024 * 1024; // 512 GB simulated
    let free = 256 * 1024 * 1024 * 1024; // 256 GB simulated

    disk_map.insert("total".to_string(), RuntimeValue::Int(total));
    disk_map.insert("free".to_string(), RuntimeValue::Int(free));
    Ok(RuntimeValue::Map {
        entries: Rc::new(std::cell::RefCell::new(disk_map)),
        is_const: false,
    })
}

impl StdlibRegistry {
    pub fn register_system(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "os".to_string(),
            Rc::new(StdFunction {
                name: "os".to_string(),
                arity: 0,
                callback: sys_os,
            }),
        );

        exports.insert(
            "arch".to_string(),
            Rc::new(StdFunction {
                name: "arch".to_string(),
                arity: 0,
                callback: sys_arch,
            }),
        );

        exports.insert(
            "cpucount".to_string(),
            Rc::new(StdFunction {
                name: "cpucount".to_string(),
                arity: 0,
                callback: sys_cpucount,
            }),
        );

        exports.insert(
            "memory".to_string(),
            Rc::new(StdFunction {
                name: "memory".to_string(),
                arity: 0,
                callback: sys_memory,
            }),
        );

        exports.insert(
            "disk".to_string(),
            Rc::new(StdFunction {
                name: "disk".to_string(),
                arity: 0,
                callback: sys_disk,
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
