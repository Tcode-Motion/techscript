use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_docs(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        self.register_module(
            "std.docs",
            StdlibModule {
                name: "std.docs".to_string(),
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
    fn test_register_docs() {
        let registry = StdlibRegistry::new();
        let module = registry
            .get_module("std.docs")
            .expect("std.docs module should be registered");

        assert_eq!(module.name, "std.docs");
        assert_eq!(module.version, "1.0.0");
        assert!(module.required_capabilities.is_empty());
    }
}
