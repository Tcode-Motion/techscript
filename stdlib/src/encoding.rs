use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_encoding(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "base64_encode".to_string(),
            Rc::new(StdFunction {
                name: "base64_encode".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(format!("b64_encoded_{}", text)))
                },
            }),
        );

        exports.insert(
            "base64_decode".to_string(),
            Rc::new(StdFunction {
                name: "base64_decode".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    let decoded = text.trim_start_matches("b64_encoded_").to_string();
                    Ok(RuntimeValue::Str(decoded))
                },
            }),
        );

        exports.insert(
            "hex_encode".to_string(),
            Rc::new(StdFunction {
                name: "hex_encode".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(format!("hex_encoded_{}", text)))
                },
            }),
        );

        exports.insert(
            "hex_decode".to_string(),
            Rc::new(StdFunction {
                name: "hex_decode".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    let decoded = text.trim_start_matches("hex_encoded_").to_string();
                    Ok(RuntimeValue::Str(decoded))
                },
            }),
        );

        self.register_module(
            "std.encoding",
            StdlibModule {
                name: "std.encoding".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );

        self.register_module(
            "std.base64",
            StdlibModule {
                name: "std.base64".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );

        self.register_module(
            "std.hex",
            StdlibModule {
                name: "std.hex".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );
    }
}
