use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    context::{Capability, RuntimeContext},
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};

fn sys_read_file(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
        return Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(
                "Security policy violation: FileSystem capability is denied".to_string(),
            ),
            None,
            None,
        ));
    }
    let path = args[0].try_into_string()?;
    let content = std::fs::read_to_string(path).map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("IO error: {}", e)),
            None,
            None,
        )
    })?;
    Ok(RuntimeValue::Str(content))
}

fn sys_write_file(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
        return Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(
                "Security policy violation: FileSystem capability is denied".to_string(),
            ),
            None,
            None,
        ));
    }
    let path = args[0].try_into_string()?;
    let content = args[1].try_into_string()?;
    std::fs::write(path, content).map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("IO error: {}", e)),
            None,
            None,
        )
    })?;
    Ok(RuntimeValue::Null)
}

fn sys_exists(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
        return Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(
                "Security policy violation: FileSystem capability is denied".to_string(),
            ),
            None,
            None,
        ));
    }
    let path = args[0].try_into_string()?;
    Ok(RuntimeValue::Bool(std::path::Path::new(&path).exists()))
}

fn sys_now(
    _ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let start = std::time::SystemTime::now();
    let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    Ok(RuntimeValue::Float(since_the_epoch.as_secs_f64()))
}

fn sys_sleep(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let ms = args[0].try_into_int()?;
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    Ok(RuntimeValue::Null)
}

impl StdlibRegistry {
    pub fn register_sys(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "read_file".to_string(),
            Rc::new(StdFunction {
                name: "read_file".to_string(),
                arity: 1,
                callback: sys_read_file,
            }),
        );

        exports.insert(
            "write_file".to_string(),
            Rc::new(StdFunction {
                name: "write_file".to_string(),
                arity: 2,
                callback: sys_write_file,
            }),
        );

        exports.insert(
            "exists".to_string(),
            Rc::new(StdFunction {
                name: "exists".to_string(),
                arity: 1,
                callback: sys_exists,
            }),
        );

        self.register_module(
            "std.fs",
            StdlibModule {
                name: "std.fs".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: vec![Capability::FileSystem],
            },
        );

        let mut time_exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();
        time_exports.insert(
            "now".to_string(),
            Rc::new(StdFunction {
                name: "now".to_string(),
                arity: 0,
                callback: sys_now,
            }),
        );

        time_exports.insert(
            "sleep".to_string(),
            Rc::new(StdFunction {
                name: "sleep".to_string(),
                arity: 1,
                callback: sys_sleep,
            }),
        );

        self.register_module(
            "std.time",
            StdlibModule {
                name: "std.time".to_string(),
                version: "1.0.0".to_string(),
                exports: time_exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
