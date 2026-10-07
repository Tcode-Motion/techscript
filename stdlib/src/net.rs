use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::context::Capability;

impl StdlibRegistry {
    pub fn register_net(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "tcp_listen".to_string(),
            Rc::new(StdFunction {
                name: "tcp_listen".to_string(),
                arity: 1,
                callback: crate::net_handler::tcp_listen,
            }),
        );

        exports.insert(
            "tcp_connect".to_string(),
            Rc::new(StdFunction {
                name: "tcp_connect".to_string(),
                arity: 2,
                callback: crate::net_handler::tcp_connect,
            }),
        );

        exports.insert(
            "tcp_send".to_string(),
            Rc::new(StdFunction {
                name: "tcp_send".to_string(),
                arity: 2,
                callback: crate::net_handler::tcp_send,
            }),
        );

        exports.insert(
            "tcp_recv".to_string(),
            Rc::new(StdFunction {
                name: "tcp_recv".to_string(),
                arity: 1,
                callback: crate::net_handler::tcp_recv,
            }),
        );

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
