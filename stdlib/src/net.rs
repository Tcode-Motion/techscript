use crate::net_handler::{tcp_connect_fn, tcp_listen_fn, tcp_recv_fn, tcp_send_fn};
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
                callback: tcp_listen_fn,
            }),
        );

        exports.insert(
            "tcp_connect".to_string(),
            Rc::new(StdFunction {
                name: "tcp_connect".to_string(),
                arity: 2,
                callback: tcp_connect_fn,
            }),
        );

        exports.insert(
            "tcp_send".to_string(),
            Rc::new(StdFunction {
                name: "tcp_send".to_string(),
                arity: 2,
                callback: tcp_send_fn,
            }),
        );

        exports.insert(
            "tcp_recv".to_string(),
            Rc::new(StdFunction {
                name: "tcp_recv".to_string(),
                arity: 1,
                callback: tcp_recv_fn,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_net() {
        let mut registry = StdlibRegistry::new();
        registry.register_net();

        assert!(registry.has_module("std.net"));
        let module = registry.get_module("std.net").unwrap();
        assert_eq!(module.name, "std.net");
        assert_eq!(module.version, "1.0.0");
        assert_eq!(module.required_capabilities, vec![Capability::Network]);

        assert_eq!(module.exports.len(), 4);

        let tcp_listen = module.exports.get("tcp_listen").unwrap();
        assert_eq!(tcp_listen.name(), "tcp_listen");
        assert_eq!(tcp_listen.arity(), 1);

        let tcp_connect = module.exports.get("tcp_connect").unwrap();
        assert_eq!(tcp_connect.name(), "tcp_connect");
        assert_eq!(tcp_connect.arity(), 2);

        let tcp_send = module.exports.get("tcp_send").unwrap();
        assert_eq!(tcp_send.name(), "tcp_send");
        assert_eq!(tcp_send.arity(), 2);

        let tcp_recv = module.exports.get("tcp_recv").unwrap();
        assert_eq!(tcp_recv.name(), "tcp_recv");
        assert_eq!(tcp_recv.arity(), 1);
    }
}
