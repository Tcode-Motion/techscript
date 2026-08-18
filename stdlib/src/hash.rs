use crate::{StdFunction, StdlibModule, StdlibRegistry};
use crc::{Crc, CRC_32_ISO_HDLC};
use md5;
use sha1::{Digest as _, Sha1};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_hash(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "md5".to_string(),
            Rc::new(StdFunction {
                name: "md5".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    let digest = md5::compute(text.as_bytes());
                    Ok(RuntimeValue::Str(format!("{:x}", digest)))
                },
            }),
        );

        exports.insert(
            "sha256".to_string(),
            Rc::new(StdFunction {
                name: "sha256".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    let mut hasher = Sha256::new();
                    hasher.update(text.as_bytes());
                    let result = hasher.finalize();
                    Ok(RuntimeValue::Str(hex::encode(result)))
                },
            }),
        );

        exports.insert(
            "sha1".to_string(),
            Rc::new(StdFunction {
                name: "sha1".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    let mut hasher = Sha1::new();
                    hasher.update(text.as_bytes());
                    let result = hasher.finalize();
                    Ok(RuntimeValue::Str(hex::encode(result)))
                },
            }),
        );

        exports.insert(
            "crc32".to_string(),
            Rc::new(StdFunction {
                name: "crc32".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
                    let checksum = crc.checksum(text.as_bytes());
                    Ok(RuntimeValue::Int(checksum as i64))
                },
            }),
        );

        self.register_module(
            "std.hash",
            StdlibModule {
                name: "std.hash".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
