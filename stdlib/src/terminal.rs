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

#[cfg(test)]
mod tests {
    use super::*;
    use techscript_runtime::context::{RuntimeConfig, RuntimeContext};

    #[test]
    fn test_register_terminal() {
        let mut registry = StdlibRegistry::new();
        let module = registry
            .get_module("std.terminal")
            .expect("Module std.terminal should be registered");

        assert_eq!(module.name, "std.terminal");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("clear"));
        assert!(module.exports.contains_key("color"));
        assert!(module.exports.contains_key("read"));
    }

    #[test]
    fn test_terminal_clear() {
        let mut registry = StdlibRegistry::new();
        let module = registry.get_module("std.terminal").unwrap();
        let clear_fn = module.exports.get("clear").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let res = clear_fn.call(&mut ctx, vec![]).unwrap();
        assert_eq!(res, RuntimeValue::Null);
    }

    #[test]
    fn test_terminal_color() {
        let mut registry = StdlibRegistry::new();
        let module = registry.get_module("std.terminal").unwrap();
        let color_fn = module.exports.get("color").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        // Test valid colors
        let res_red = color_fn
            .call(&mut ctx, vec![RuntimeValue::Str("red".to_string())])
            .unwrap();
        assert_eq!(res_red, RuntimeValue::Str("\x1b[31m".to_string()));

        let res_reset = color_fn
            .call(&mut ctx, vec![RuntimeValue::Str("reset".to_string())])
            .unwrap();
        assert_eq!(res_reset, RuntimeValue::Str("\x1b[0m".to_string()));

        let res_unknown = color_fn
            .call(&mut ctx, vec![RuntimeValue::Str("unknown".to_string())])
            .unwrap();
        assert_eq!(res_unknown, RuntimeValue::Str("".to_string()));

        // Test type mismatch error
        let err_res = color_fn.call(&mut ctx, vec![RuntimeValue::Int(123)]);
        assert!(err_res.is_err());
    }

    #[test]
    fn test_terminal_read() {
        let mut registry = StdlibRegistry::new();
        let module = registry.get_module("std.terminal").unwrap();
        let read_fn = module.exports.get("read").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        // read reads from stdin, which in unit test execution will return Ok("") (EOF)
        let res = read_fn.call(&mut ctx, vec![]).unwrap();
        assert_eq!(res, RuntimeValue::Str("".to_string()));
    }
}
