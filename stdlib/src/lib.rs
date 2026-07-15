//! # TechScript Standard Library Crate
//!
//! Extended core modules for TechScript 2.0 (`io`, `math`, `string`, `file`, `web`, etc.).
//! Handles module loading registry mappings and relative imports.

use std::collections::HashMap;
use techscript_interpreter::{RuntimeError, Value};

/// Type definition for module function calls.
pub type LibraryFunction = fn(args: &[Value]) -> Result<Value, RuntimeError>;

/// Module container for library functions.
#[derive(Default, Clone)]
pub struct StdModule {
    pub functions: HashMap<String, LibraryFunction>,
}

/// Standard Library Registry controller.
#[derive(Default)]
pub struct StdlibRegistry {
    modules: HashMap<String, StdModule>,
}

impl StdlibRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            modules: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    /// Registers a custom library module.
    pub fn register_module(&mut self, name: &str, module: StdModule) {
        self.modules.insert(name.to_string(), module);
    }

    /// Checks if a module is loaded.
    pub fn has_module(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// Retrieve a module by name.
    pub fn get_module(&self, name: &str) -> Option<&StdModule> {
        self.modules.get(name)
    }

    fn register_defaults(&mut self) {
        // io module
        let mut io = StdModule::default();
        io.functions.insert("print".to_string(), |args| {
            for arg in args {
                print!("{:?}", arg);
            }
            Ok(Value::None)
        });
        self.register_module("io", io);

        // math module
        let mut math = StdModule::default();
        math.functions.insert("abs".to_string(), |args| {
            if args.len() != 1 {
                return Err(RuntimeError::MemberNotFound(
                    "abs requires 1 argument".to_string(),
                ));
            }
            match &args[0] {
                Value::Int(i) => Ok(Value::Int(i.abs())),
                Value::Float(f) => Ok(Value::Float(f.abs())),
                _ => Err(RuntimeError::TypeMismatch(
                    "abs expects Int or Float".to_string(),
                )),
            }
        });
        self.register_module("math", math);
    }
}
