use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_barcode(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        self.register_module(
            "std.barcode",
            StdlibModule {
                name: "std.barcode".to_string(),
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
    use crate::StdlibRegistry;

    #[test]
    fn test_register_barcode() {
        let mut registry = StdlibRegistry::new();
        // Since `StdlibRegistry::new()` calls `register_defaults()`, which calls `register_barcode()`,
        // the module should already be registered.

        let module = registry.get_module("std.barcode").expect("Module should be registered");
        assert_eq!(module.name, "std.barcode");
        assert_eq!(module.version, "1.0.0");
        assert_eq!(module.required_capabilities.len(), 0);
    }
}
