use std::collections::HashMap;
use std::rc::Rc;
use std::process::Command;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_process(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("run".to_string(), Rc::new(StdFunction {
            name: "run".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let cmd = match &args[0] {
                    RuntimeValue::Str(s) => s.clone(),
                    _ => return Err(RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::TypeMismatch { expected: "string".to_string(), found: "other".to_string() },
                        None, None,
                    )),
                };
                let output = Command::new("cmd")
                    .args(["/C", &cmd])
                    .output()
                    .map_err(|e| RuntimeError::new(techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(RuntimeValue::Str(stdout))
            },
        }));

        exports.insert("spawn".to_string(), Rc::new(StdFunction {
            name: "spawn".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let cmd = match &args[0] {
                    RuntimeValue::Str(s) => s.clone(),
                    _ => return Err(RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::TypeMismatch { expected: "string".to_string(), found: "other".to_string() },
                        None, None,
                    )),
                };
                Command::new("cmd")
                    .args(["/C", &cmd])
                    .spawn()
                    .map_err(|e| RuntimeError::new(techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                Ok(RuntimeValue::Null)
            },
        }));

        self.register_module("std.process", StdlibModule {
            name: "std.process".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
