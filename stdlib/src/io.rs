use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_io(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "print".to_string(),
            Rc::new(StdFunction {
                name: "print".to_string(),
                arity: 0,
                callback: |_ctx, args| {
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            print!(" ");
                        }
                        print!("{}", arg);
                    }
                    println!();
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "println".to_string(),
            Rc::new(StdFunction {
                name: "println".to_string(),
                arity: 0,
                callback: |_ctx, args| {
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            print!(" ");
                        }
                        print!("{}", arg);
                    }
                    println!();
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "readline".to_string(),
            Rc::new(StdFunction {
                name: "readline".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::io::{self, Write};
                    io::stdout().flush().ok();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).ok();
                    Ok(RuntimeValue::Str(
                        input
                            .trim_end_matches('\r')
                            .trim_end_matches('\n')
                            .to_string(),
                    ))
                },
            }),
        );

        exports.insert(
            "read_line".to_string(),
            Rc::new(StdFunction {
                name: "read_line".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::io::{self, Write};
                    io::stdout().flush().ok();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).ok();
                    Ok(RuntimeValue::Str(
                        input
                            .trim_end_matches('\r')
                            .trim_end_matches('\n')
                            .to_string(),
                    ))
                },
            }),
        );

        self.register_module(
            "std.io",
            StdlibModule {
                name: "std.io".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
