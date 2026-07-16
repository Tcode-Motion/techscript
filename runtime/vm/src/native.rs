use crate::error::VMError;
use techscript_builtins::BuiltinRegistry;
use techscript_runtime::RuntimeValue;

/// Native bridge mapping VM calls to core built-in functions.
pub struct NativeBridge {
    registry: BuiltinRegistry,
}

impl Default for NativeBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeBridge {
    /// Creates a native bridge.
    pub fn new() -> Self {
        Self {
            registry: BuiltinRegistry::new(),
        }
    }

    /// Checks if a native function is registered.
    pub fn has_function(&self, name: &str) -> bool {
        self.registry.has_function(name)
    }

    /// Calls a native function.
    pub fn call(&self, name: &str, args: &[RuntimeValue]) -> Result<RuntimeValue, VMError> {
        self.registry
            .call(name, args)
            .map_err(|e| VMError::RuntimeException(e.to_string()))
    }
}
