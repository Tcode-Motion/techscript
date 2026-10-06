use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::process::Command;
use std::rc::Rc;
use techscript_runtime::{
    context::{Capability, RuntimeContext},
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};

fn process_run(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
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
    if cmd.contains("..") || cmd.contains('\0') {
        return Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(
                "Security policy violation: Path traversal or null bytes in command are denied"
                    .to_string(),
            ),
            None,
            None,
        ));
    }
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

    let output = Command::new(cmd).args(args_list).output().map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("Failed to execute command: {}", e)),
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
}

fn process_spawn(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
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
        _ => {
            return Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "string".to_string(),
                    found: "other".to_string(),
                },
                None,
                None,
            ))
        }
    };

    if cmd.contains("..") || cmd.contains('\0') {
        return Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(
                "Security policy violation: Path traversal or null bytes in command are denied"
                    .to_string(),
            ),
            None,
            None,
        ));
    }

    let parsed = shlex::split(&cmd).ok_or_else(|| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation("Failed to parse command string".to_string()),
            None,
            None,
        )
    })?;

    if parsed.is_empty() {
        return Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation("Empty command string".to_string()),
            None,
            None,
        ));
    }

    Command::new(&parsed[0])
        .args(&parsed[1..])
        .spawn()
        .map_err(|e| {
            RuntimeError::new(
                RuntimeErrorKind::InvalidOperation(e.to_string()),
                None,
                None,
            )
        })?;
    Ok(RuntimeValue::Null)
}

fn process_exit(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let code = args[0].try_into_int()? as i32;
    std::process::exit(code);
}

fn process_pid(
    _ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    Ok(RuntimeValue::Int(std::process::id() as i64))
}

impl StdlibRegistry {
    pub fn register_process(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "run".to_string(),
            Rc::new(StdFunction {
                name: "run".to_string(),
                arity: 2,
                callback: process_run,
            }),
        );

        exports.insert(
            "spawn".to_string(),
            Rc::new(StdFunction {
                name: "spawn".to_string(),
                arity: 1,
                callback: process_spawn,
            }),
        );

        exports.insert(
            "exit".to_string(),
            Rc::new(StdFunction {
                name: "exit".to_string(),
                arity: 1,
                callback: process_exit,
            }),
        );

        exports.insert(
            "pid".to_string(),
            Rc::new(StdFunction {
                name: "pid".to_string(),
                arity: 0,
                callback: process_pid,
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
