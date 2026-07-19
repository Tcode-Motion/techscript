use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_security(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("random_bytes".to_string(), Rc::new(StdFunction {
            name: "random_bytes".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let len = args[0].try_into_int().unwrap_or(16) as usize;
                use rand::Rng;
                let bytes: Vec<u8> = rand::thread_rng().gen::<[u8; 32]>()[..len.min(32)].to_vec();
                let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
                Ok(RuntimeValue::Str(hex))
            },
        }));

        exports.insert("random_int".to_string(), Rc::new(StdFunction {
            name: "random_int".to_string(),
            arity: 2,
            callback: |_ctx, args| {
                let min = args[0].try_into_int().unwrap_or(0);
                let max = args[1].try_into_int().unwrap_or(100);
                use rand::Rng;
                let val = rand::thread_rng().gen_range(min..=max);
                Ok(RuntimeValue::Int(val))
            },
        }));

        exports.insert("hash".to_string(), Rc::new(StdFunction {
            name: "hash".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let input = args[0].to_string();
                use sha2::Digest;
                let digest = sha2::Sha256::digest(input.as_bytes());
                let hex: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
                Ok(RuntimeValue::Str(hex))
            },
        }));

        self.register_module("std.security", StdlibModule {
            name: "std.security".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
