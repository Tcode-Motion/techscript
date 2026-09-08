use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_ftp(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "connect".to_string(),
            Rc::new(StdFunction {
                name: "connect".to_string(),
                arity: 2,
                callback: |_ctx, _args| Ok(RuntimeValue::Str("FTP not yet available".to_string())),
            }),
        );

        self.register_module(
            "std.ftp",
            StdlibModule {
                name: "std.ftp".to_string(),
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
    fn test_register_ftp() {
        let mut registry = StdlibRegistry {
            modules: HashMap::new(),
        };
        registry.register_ftp();

        assert!(registry.has_module("std.ftp"));
        let module = registry.get_module("std.ftp").unwrap();
        assert_eq!(module.name, "std.ftp");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("connect"));
    }
}
