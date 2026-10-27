use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_audio(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        self.register_module(
            "std.audio",
            StdlibModule {
                name: "std.audio".to_string(),
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
    use std::collections::HashMap;

    #[test]
    fn test_register_audio() {
        let mut registry = StdlibRegistry {
            modules: HashMap::new(),
        };
        registry.register_audio();

        assert!(registry.has_module("std.audio"));
        let module = registry.get_module("std.audio").unwrap();
        assert_eq!(module.name, "std.audio");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.is_empty());
        assert!(module.required_capabilities.is_empty());
    }

    #[test]
    fn test_register_audio_in_default_registry() {
        let registry = StdlibRegistry::new();
        assert!(registry.has_module("std.audio"));
        let module = registry.get_module("std.audio").unwrap();
        assert_eq!(module.name, "std.audio");
        assert_eq!(module.version, "1.0.0");
    }
}
