use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_image(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "open".to_string(),
            Rc::new(StdFunction {
                name: "open".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let path = args[0].to_string();
                    let img = image::open(&path).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(e.to_string()),
                            None,
                            None,
                        )
                    })?;
                    let (w, h) = (img.width(), img.height());
                    Ok(RuntimeValue::Str(format!("image {}x{}", w, h)))
                },
            }),
        );

        exports.insert(
            "resize".to_string(),
            Rc::new(StdFunction {
                name: "resize".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let path = args[0].to_string();
                    let w = args[1].try_into_int().unwrap_or(0) as u32;
                    let h = args[2].try_into_int().unwrap_or(0) as u32;
                    let img = image::open(&path).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(e.to_string()),
                            None,
                            None,
                        )
                    })?;
                    let resized = img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);
                    let out_path = format!(
                        "{}_resized.png",
                        path.trim_end_matches(".png").trim_end_matches(".jpg")
                    );
                    resized.save(&out_path).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(e.to_string()),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Str(out_path))
                },
            }),
        );

        exports.insert(
            "grayscale".to_string(),
            Rc::new(StdFunction {
                name: "grayscale".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let path = args[0].to_string();
                    let img = image::open(&path).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(e.to_string()),
                            None,
                            None,
                        )
                    })?;
                    let gray = img.grayscale();
                    let out_path = format!(
                        "{}_gray.png",
                        path.trim_end_matches(".png").trim_end_matches(".jpg")
                    );
                    gray.save(&out_path).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(e.to_string()),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Str(out_path))
                },
            }),
        );

        self.register_module(
            "std.image",
            StdlibModule {
                name: "std.image".to_string(),
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
    use techscript_runtime::context::{RuntimeConfig, RuntimeContext};

    #[test]
    fn test_register_image() {
        let mut registry = StdlibRegistry::default();
        registry.register_image();

        assert!(registry.has_module("std.image"));
        let module = registry.get_module("std.image").unwrap();
        assert_eq!(module.name, "std.image");
        assert_eq!(module.version, "1.0.0");
        assert!(module.required_capabilities.is_empty());

        assert_eq!(module.exports.len(), 3);
        assert_eq!(module.exports.get("open").unwrap().arity(), 1);
        assert_eq!(module.exports.get("resize").unwrap().arity(), 3);
        assert_eq!(module.exports.get("grayscale").unwrap().arity(), 1);
    }

    #[test]
    fn test_image_open_and_errors() {
        let mut registry = StdlibRegistry::default();
        registry.register_image();
        let module = registry.get_module("std.image").unwrap();
        let open_fn = module.exports.get("open").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        // Error path: file does not exist
        let err_res = open_fn.call(
            &mut ctx,
            vec![RuntimeValue::Str("non_existent_file_9999.png".to_string())],
        );
        assert!(err_res.is_err());

        // Success path: create a temporary 10x20 image
        let temp_dir = std::env::temp_dir();
        let test_img_path = temp_dir.join("techscript_test_open.png");
        let img = image::RgbImage::new(10, 20);
        img.save(&test_img_path).expect("failed to save temp image");

        let res = open_fn.call(
            &mut ctx,
            vec![RuntimeValue::Str(
                test_img_path.to_str().unwrap().to_string(),
            )],
        );
        assert!(res.is_ok());
        if let Ok(RuntimeValue::Str(val)) = res {
            assert_eq!(val, "image 10x20");
        } else {
            panic!("expected RuntimeValue::Str");
        }

        let _ = std::fs::remove_file(test_img_path);
    }

    #[test]
    fn test_image_resize() {
        let mut registry = StdlibRegistry::default();
        registry.register_image();
        let module = registry.get_module("std.image").unwrap();
        let resize_fn = module.exports.get("resize").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        let temp_dir = std::env::temp_dir();
        let test_img_path = temp_dir.join("techscript_test_resize.png");
        let img = image::RgbImage::new(20, 20);
        img.save(&test_img_path).expect("failed to save temp image");

        let res = resize_fn.call(
            &mut ctx,
            vec![
                RuntimeValue::Str(test_img_path.to_str().unwrap().to_string()),
                RuntimeValue::Int(10),
                RuntimeValue::Int(15),
            ],
        );
        assert!(res.is_ok());
        if let Ok(RuntimeValue::Str(out_path)) = res {
            assert!(std::path::Path::new(&out_path).exists());
            let _ = std::fs::remove_file(&out_path);
        } else {
            panic!("expected RuntimeValue::Str");
        }

        let _ = std::fs::remove_file(test_img_path);
    }

    #[test]
    fn test_image_grayscale() {
        let mut registry = StdlibRegistry::default();
        registry.register_image();
        let module = registry.get_module("std.image").unwrap();
        let grayscale_fn = module.exports.get("grayscale").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        let temp_dir = std::env::temp_dir();
        let test_img_path = temp_dir.join("techscript_test_gray.png");
        let img = image::RgbImage::new(20, 20);
        img.save(&test_img_path).expect("failed to save temp image");

        let res = grayscale_fn.call(
            &mut ctx,
            vec![RuntimeValue::Str(
                test_img_path.to_str().unwrap().to_string(),
            )],
        );
        assert!(res.is_ok());
        if let Ok(RuntimeValue::Str(out_path)) = res {
            assert!(std::path::Path::new(&out_path).exists());
            let _ = std::fs::remove_file(&out_path);
        } else {
            panic!("expected RuntimeValue::Str");
        }

        let _ = std::fs::remove_file(test_img_path);
    }
}
