use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::context::Capability;

pub mod tcp;

impl StdlibRegistry {
    pub fn register_net(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "tcp_listen".to_string(),
            Rc::new(StdFunction {
                name: "tcp_listen".to_string(),
                arity: 1,
                callback: tcp::tcp_listen,
            }),
        );

        exports.insert(
            "tcp_connect".to_string(),
            Rc::new(StdFunction {
                name: "tcp_connect".to_string(),
                arity: 2,
                callback: tcp::tcp_connect,
            }),
        );

        exports.insert(
            "tcp_send".to_string(),
            Rc::new(StdFunction {
                name: "tcp_send".to_string(),
                arity: 2,
                callback: tcp::tcp_send,
            }),
        );

        exports.insert(
            "tcp_recv".to_string(),
            Rc::new(StdFunction {
                name: "tcp_recv".to_string(),
                arity: 1,
                callback: tcp::tcp_recv,
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
