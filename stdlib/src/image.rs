use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_image(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("open".to_string(), Rc::new(StdFunction {
            name: "open".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let path = args[0].to_string();
                let img = image::open(&path)
                    .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                let (w, h) = (img.width(), img.height());
                Ok(RuntimeValue::Str(format!("image {}x{}", w, h)))
            },
        }));

        exports.insert("resize".to_string(), Rc::new(StdFunction {
            name: "resize".to_string(),
            arity: 3,
            callback: |_ctx, args| {
                let path = args[0].to_string();
                let w = args[1].try_into_int().unwrap_or(0) as u32;
                let h = args[2].try_into_int().unwrap_or(0) as u32;
                let img = image::open(&path)
                    .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                let resized = img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);
                let out_path = format!("{}_resized.png", path.trim_end_matches(".png").trim_end_matches(".jpg"));
                resized.save(&out_path)
                    .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                Ok(RuntimeValue::Str(out_path))
            },
        }));

        exports.insert("grayscale".to_string(), Rc::new(StdFunction {
            name: "grayscale".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let path = args[0].to_string();
                let img = image::open(&path)
                    .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                let gray = img.grayscale();
                let out_path = format!("{}_gray.png", path.trim_end_matches(".png").trim_end_matches(".jpg"));
                gray.save(&out_path)
                    .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                Ok(RuntimeValue::Str(out_path))
            },
        }));

        self.register_module("std.image", StdlibModule {
            name: "std.image".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
