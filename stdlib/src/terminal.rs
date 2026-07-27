use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_terminal(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "clear".to_string(),
            Rc::new(StdFunction {
                name: "clear".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    print!("\x1B[2J\x1B[1;1H");
                    io::stdout().flush().ok();
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "color".to_string(),
            Rc::new(StdFunction {
                name: "color".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let name = match &args[0] {
                        RuntimeValue::Str(s) => s.as_str(),
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "string".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    let code = match name {
                        "red" => "\x1b[31m",
                        "green" => "\x1b[32m",
                        "yellow" => "\x1b[33m",
                        "blue" => "\x1b[34m",
                        "magenta" => "\x1b[35m",
                        "cyan" => "\x1b[36m",
                        "white" => "\x1b[37m",
                        "reset" => "\x1b[0m",
                        _ => "",
                    };
                    Ok(RuntimeValue::Str(code.to_string()))
                },
            }),
        );

        exports.insert(
            "read".to_string(),
            Rc::new(StdFunction {
                name: "read".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let stdin = io::stdin();
                    let mut line = String::new();
                    stdin.lock().read_line(&mut line).map_err(|e| {
                        RuntimeError::new(
                            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                                e.to_string(),
                            ),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Str(line.trim_end().to_string()))
                },
            }),
        );

        self.register_module(
            "std.terminal",
            StdlibModule {
                name: "std.terminal".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
