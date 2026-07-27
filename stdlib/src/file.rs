use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_file(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "read".to_string(),
            Rc::new(StdFunction {
                name: "read".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let path = match &args[0] {
                        RuntimeValue::Str(s) => s.clone(),
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "string".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    let content = fs::read_to_string(&path).map_err(|e| {
                        RuntimeError::new(
                            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                                e.to_string(),
                            ),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Str(content))
                },
            }),
        );

        exports.insert(
            "write".to_string(),
            Rc::new(StdFunction {
                name: "write".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let path = match &args[0] {
                        RuntimeValue::Str(s) => s.clone(),
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "string".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    let content = match &args[1] {
                        RuntimeValue::Str(s) => s.clone(),
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "string".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    fs::write(&path, &content).map_err(|e| {
                        RuntimeError::new(
                            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                                e.to_string(),
                            ),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "copy".to_string(),
            Rc::new(StdFunction {
                name: "copy".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let src = match &args[0] {
                        RuntimeValue::Str(s) => s.clone(),
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "string".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    let dest = match &args[1] {
                        RuntimeValue::Str(s) => s.clone(),
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "string".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    fs::copy(&src, &dest).map_err(|e| {
                        RuntimeError::new(
                            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                                e.to_string(),
                            ),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "remove".to_string(),
            Rc::new(StdFunction {
                name: "remove".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let path = match &args[0] {
                        RuntimeValue::Str(s) => s.clone(),
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "string".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    fs::remove_file(&path).map_err(|e| {
                        RuntimeError::new(
                            techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                                e.to_string(),
                            ),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "exists".to_string(),
            Rc::new(StdFunction {
                name: "exists".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let path = match &args[0] {
                        RuntimeValue::Str(s) => s.clone(),
                        _ => {
                            return Err(RuntimeError::new(
                                techscript_runtime::error::RuntimeErrorKind::TypeMismatch {
                                    expected: "string".to_string(),
                                    found: "other".to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };
                    Ok(RuntimeValue::Bool(std::path::Path::new(&path).exists()))
                },
            }),
        );

        self.register_module(
            "std.file",
            StdlibModule {
                name: "std.file".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
