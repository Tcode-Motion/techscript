//! # TechScript Builtins Crate
//!
//! Registers core runtime built-in functions (e.g. say, ask, len, exit).
//! Interfaces native Rust operations with interpreter scopes.

use std::collections::HashMap;
use techscript_interpreter::{RuntimeError, Value};

/// Type definition for native built-in functions.
pub type NativeCallback = fn(args: &[Value]) -> Result<Value, RuntimeError>;

/// Built-in function registry.
#[derive(Default)]
pub struct BuiltinRegistry {
    functions: HashMap<String, NativeCallback>,
}

impl BuiltinRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    /// Register a custom native function.
    pub fn register(&mut self, name: &str, callback: NativeCallback) {
        self.functions.insert(name.to_string(), callback);
    }

    /// Checks if function name is registered.
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Invokes a native function by name.
    pub fn call(&self, name: &str, args: &[Value]) -> Result<Value, RuntimeError> {
        match self.functions.get(name) {
            Some(callback) => callback(args),
            None => Err(RuntimeError::new(
                techscript_interpreter::RuntimeErrorKind::MemberNotFound(name.to_string()),
                None,
                None,
            )),
        }
    }

    fn register_defaults(&mut self) {
        self.register("say", |args| {
            let mut first = true;
            for arg in args {
                if !first {
                    print!(" ");
                }
                print!("{}", arg);
                first = false;
            }
            println!();
            Ok(Value::Null)
        });
        self.register("fstring_concat", |args| {
            let mut res = String::new();
            for arg in args {
                res.push_str(&format!("{}", arg));
            }
            Ok(Value::Str(res))
        });
        self.register("ask", |args| {
            use std::io::{self, Write};
            if let Some(prompt) = args.first() {
                print!("{}", prompt);
                let _ = io::stdout().flush();
            }
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_ok() {
                Ok(Value::Str(input.trim_end().to_string()))
            } else {
                Ok(Value::Null)
            }
        });
        self.register("range", |args| {
            if args.len() != 2 {
                return Err(RuntimeError::new(
                    techscript_interpreter::RuntimeErrorKind::InvalidOperation(
                        "range requires 2 arguments (start, end)".to_string(),
                    ),
                    None,
                    None,
                ));
            }
            let start = match args[0] {
                Value::Int(n) => n,
                _ => {
                    return Err(RuntimeError::new(
                        techscript_interpreter::RuntimeErrorKind::TypeMismatch {
                            expected: "Int".to_string(),
                            found: args[0].runtime_type().to_string(),
                        },
                        None,
                        None,
                    ))
                }
            };
            let end = match args[1] {
                Value::Int(n) => n,
                _ => {
                    return Err(RuntimeError::new(
                        techscript_interpreter::RuntimeErrorKind::TypeMismatch {
                            expected: "Int".to_string(),
                            found: args[1].runtime_type().to_string(),
                        },
                        None,
                        None,
                    ))
                }
            };
            let items: Vec<Value> = (start..end).map(Value::Int).collect();
            Ok(Value::List {
                items: std::rc::Rc::new(std::cell::RefCell::new(items)),
                is_const: false,
            })
        });
        self.register("type_of", |args| {
            if args.is_empty() {
                return Err(RuntimeError::new(
                    techscript_interpreter::RuntimeErrorKind::InvalidOperation(
                        "type_of requires 1 argument".to_string(),
                    ),
                    None,
                    None,
                ));
            }
            let type_name = match &args[0] {
                Value::Str(_) => "str".to_string(),
                Value::Map { .. } => "dict".to_string(),
                other => other.runtime_type().to_string(),
            };
            Ok(Value::Str(type_name))
        });
        self.register("len", |args| {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    techscript_interpreter::RuntimeErrorKind::InvalidOperation(
                        "len requires 1 argument".to_string(),
                    ),
                    None,
                    None,
                ));
            }
            match &args[0] {
                Value::Str(s) => Ok(Value::Int(s.len() as i64)),
                Value::List { items, .. } => Ok(Value::Int(items.borrow().len() as i64)),
                other => Err(RuntimeError::new(
                    techscript_interpreter::RuntimeErrorKind::TypeMismatch {
                        expected: "Str or List".to_string(),
                        found: other.runtime_type().to_string(),
                    },
                    None,
                    None,
                )),
            }
        });
    }
}
