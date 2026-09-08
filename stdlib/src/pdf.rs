use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_pdf(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        self.register_module(
            "std.pdf",
            StdlibModule {
                name: "std.pdf".to_string(),
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

    #[test]
    fn test_register_pdf() {
        let mut registry = StdlibRegistry::default();
        registry.register_pdf();

        assert!(registry.has_module("std.pdf"));
        let module = registry.get_module("std.pdf").unwrap();
        assert_eq!(module.name, "std.pdf");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.is_empty());
        assert!(module.required_capabilities.is_empty());
    }
}
