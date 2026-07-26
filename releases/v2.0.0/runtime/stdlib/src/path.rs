use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::RuntimeError,
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_path(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
            "join".to_string(),
            Rc::new(StdFunction {
                name: "join".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let p1 = args[0].try_into_string()?;
                    let p2 = args[1].try_into_string()?;
                    let path = std::path::Path::new(&p1).join(&p2).to_string_lossy().to_string();
                    Ok(RuntimeValue::Str(path))
                },
            }),
        );

        exports.insert(
            "basename".to_string(),
            Rc::new(StdFunction {
                name: "basename".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let p = args[0].try_into_string()?;
                    let base = std::path::Path::new(&p).file_name().unwrap_or_default().to_string_lossy().to_string();
                    Ok(RuntimeValue::Str(base))
                },
            }),
        );

        exports.insert(
            "extname".to_string(),
            Rc::new(StdFunction {
                name: "extname".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let p = args[0].try_into_string()?;
                    let ext = std::path::Path::new(&p).extension().unwrap_or_default().to_string_lossy().to_string();
                    Ok(RuntimeValue::Str(ext))
                },
            }),
        );

        self.register_module(
            "std.path",
            StdlibModule {
                name: "std.path".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
