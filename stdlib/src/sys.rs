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

        let mut env_exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();
        env_exports.insert(
            "get".to_string(),
            Rc::new(StdFunction {
                name: "get".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Environment) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Environment capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let name = args[0].try_into_string()?;
                    match std::env::var(name) {
                        Ok(val) => Ok(RuntimeValue::Str(val)),
                        Err(_) => Ok(RuntimeValue::Null),
                    }
                },
            }),
        );

        env_exports.insert(
            "set".to_string(),
            Rc::new(StdFunction {
                name: "set".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Environment) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Environment capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let name = args[0].try_into_string()?;
                    let val = args[1].try_into_string()?;
                    std::env::set_var(name, val);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        env_exports.insert(
            "args".to_string(),
            Rc::new(StdFunction {
                name: "args".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let r_args: Vec<RuntimeValue> = std::env::args().map(RuntimeValue::Str).collect();
                    Ok(RuntimeValue::List {
                        items: Rc::new(RefCell::new(r_args)),
                        is_const: false,
                    })
                },
            }),
        );

        env_exports.insert(
            "current_dir".to_string(),
            Rc::new(StdFunction {
                name: "current_dir".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let p = std::env::current_dir().unwrap_or_default().to_string_lossy().to_string();
                    Ok(RuntimeValue::Str(p))
                },
            }),
        );

        self.register_module(
            "std.env",
            StdlibModule {
                name: "std.env".to_string(),
                version: "1.0.0".to_string(),
                exports: env_exports,
                required_capabilities: vec![Capability::Environment],
            },
        );

        let mut proc_exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();
        proc_exports.insert(
            "run".to_string(),
            Rc::new(StdFunction {
                name: "run".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Process) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Process capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let cmd = args[0].try_into_string()?;
                    let args_list = match &args[1] {
                        RuntimeValue::List { items, .. } => {
                            let mut list = Vec::new();
                            for item in items.borrow().iter() {
                                list.push(item.try_into_string()?);
                            }
                            list
                        }
                        other => {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::TypeMismatch {
                                    expected: "List".to_string(),
                                    found: other.runtime_type().to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };

                    let output = std::process::Command::new(cmd)
                        .args(args_list)
                        .output()
                        .map_err(|e| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "Failed to execute command: {}",
                                    e
                                )),
                                None,
                                None,
                            )
                        })?;

                    let mut res_map = IndexMap::new();
                    res_map.insert(
                        "stdout".to_string(),
                        RuntimeValue::Str(String::from_utf8_lossy(&output.stdout).to_string()),
                    );
                    res_map.insert(
                        "stderr".to_string(),
                        RuntimeValue::Str(String::from_utf8_lossy(&output.stderr).to_string()),
                    );
                    res_map.insert(
                        "code".to_string(),
                        RuntimeValue::Int(output.status.code().unwrap_or(-1) as i64),
                    );

                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(res_map)),
                        is_const: false,
                    })
                },
            }),
        );

        proc_exports.insert(
            "exit".to_string(),
            Rc::new(StdFunction {
                name: "exit".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let code = args[0].try_into_int()? as i32;
                    std::process::exit(code);
                },
            }),
        );

        proc_exports.insert(
            "pid".to_string(),
            Rc::new(StdFunction {
                name: "pid".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Int(std::process::id() as i64))
                },
            }),
        );

        self.register_module(
            "std.process",
            StdlibModule {
                name: "std.process".to_string(),
                version: "1.0.0".to_string(),
                exports: proc_exports,
                required_capabilities: vec![Capability::Process],
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
