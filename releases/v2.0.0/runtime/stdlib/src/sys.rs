use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use indexmap::IndexMap;
use techscript_runtime::{
    context::Capability,
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_sys(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
            "read_file".to_string(),
            Rc::new(StdFunction {
                name: "read_file".to_string(),
                arity: 1,
                callback: |ctx, args| {
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
                },
            }),
        );

        exports.insert(
            "write_file".to_string(),
            Rc::new(StdFunction {
                name: "write_file".to_string(),
                arity: 2,
                callback: |ctx, args| {
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
                },
            }),
        );

        exports.insert(
            "exists".to_string(),
            Rc::new(StdFunction {
                name: "exists".to_string(),
                arity: 1,
                callback: |ctx, args| {
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
                },
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


        let mut time_exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();
        time_exports.insert(
            "now".to_string(),
            Rc::new(StdFunction {
                name: "now".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let start = std::time::SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();
                    Ok(RuntimeValue::Float(since_the_epoch.as_secs_f64()))
                },
            }),
        );

        time_exports.insert(
            "sleep".to_string(),
            Rc::new(StdFunction {
                name: "sleep".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let ms = args[0].try_into_int()?;
                    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
                    Ok(RuntimeValue::Null)
                },
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
