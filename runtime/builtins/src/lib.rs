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
            None => Err(RuntimeError::MemberNotFound(format!(
                "Built-in function '{}' not found",
                name
            ))),
        }
    }

    fn register_defaults(&mut self) {
        self.register("say", |args| {
            for arg in args {
                print!("{:?} ", arg);
            }
            println!();
            Ok(Value::None)
        });
        self.register("len", |args| {
            if args.len() != 1 {
                return Err(RuntimeError::MemberNotFound(
                    "len requires 1 argument".to_string(),
                ));
            }
            match &args[0] {
                Value::Str(s) => Ok(Value::Int(s.len() as i64)),
                Value::List(l) => Ok(Value::Int(l.len() as i64)),
                _ => Err(RuntimeError::TypeMismatch(
                    "len expects Str or List".to_string(),
                )),
            }
        });
    }
}
