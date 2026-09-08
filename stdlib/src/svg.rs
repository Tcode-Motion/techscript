use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_svg(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        self.register_module(
            "std.svg",
            StdlibModule {
                name: "std.svg".to_string(),
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
    fn test_register_svg() {
        let mut registry = StdlibRegistry::default();
        registry.register_svg();

        let module = registry.get_module("std.svg").unwrap();
        assert_eq!(module.name, "std.svg");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.is_empty());
        assert!(module.required_capabilities.is_empty());
    }
}
