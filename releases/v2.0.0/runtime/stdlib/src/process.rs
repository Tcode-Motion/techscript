use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::process::Command;
use indexmap::IndexMap;
use techscript_runtime::{
    context::Capability,
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_process(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
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

                    let output = Command::new(cmd)
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

        exports.insert(
            "spawn".to_string(),
            Rc::new(StdFunction {
                name: "spawn".to_string(),
                arity: 1,
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
                    let cmd = match &args[0] {
                        RuntimeValue::Str(s) => s.clone(),
                        _ => return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch { expected: "string".to_string(), found: "other".to_string() },
                            None, None,
                        )),
                    };
                    Command::new("cmd")
                        .args(["/C", &cmd])
                        .spawn()
                        .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
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

        exports.insert(
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
                exports,
                required_capabilities: vec![Capability::Process],
            },
        );
    }
}
