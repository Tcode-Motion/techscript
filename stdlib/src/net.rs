use crate::{StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::context::Capability;

pub mod tcp;

impl StdlibRegistry {
    pub fn register_net(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        tcp::register_tcp_functions(&mut exports);

        self.register_module(
            "std.net",
            StdlibModule {
                name: "std.net".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Network],
            },
        );
    }
}
