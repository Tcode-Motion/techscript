use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};

impl StdlibRegistry {
    pub fn register_collections(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "push".to_string(),
            Rc::new(StdFunction {
                name: "push".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    if let RuntimeValue::List { items, is_const } = &args[0] {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(
                                    "Cannot modify constant list".to_string(),
                                ),
                                None,
                                None,
                            ));
                        }
                        items.borrow_mut().push(args[1].clone());
                        Ok(RuntimeValue::Null)
                    } else {
                        Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "List".to_string(),
                                found: args[0].runtime_type().to_string(),
                            },
                            None,
                            None,
                        ))
                    }
                },
            }),
        );

        exports.insert(
            "pop".to_string(),
            Rc::new(StdFunction {
                name: "pop".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::List { items, is_const } = &args[0] {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(
                                    "Cannot modify constant list".to_string(),
                                ),
                                None,
                                None,
                            ));
                        }
                        let popped = items.borrow_mut().pop().unwrap_or(RuntimeValue::Null);
                        Ok(popped)
                    } else {
                        Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "List".to_string(),
                                found: args[0].runtime_type().to_string(),
                            },
                            None,
                            None,
                        ))
                    }
                },
            }),
        );

        exports.insert(
            "sort".to_string(),
            Rc::new(StdFunction {
                name: "sort".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::List { items, is_const } = &args[0] {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(
                                    "Cannot modify constant list".to_string(),
                                ),
                                None,
                                None,
                            ));
                        }
                        items.borrow_mut().sort_by(|a, b| {
                            let a_int = a.try_into_int().unwrap_or(0);
                            let b_int = b.try_into_int().unwrap_or(0);
                            a_int.cmp(&b_int)
                        });
                        Ok(RuntimeValue::Null)
                    } else {
                        Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "List".to_string(),
                                found: args[0].runtime_type().to_string(),
                            },
                            None,
                            None,
                        ))
                    }
                },
            }),
        );

        exports.insert(
            "insert".to_string(),
            Rc::new(StdFunction {
                name: "insert".to_string(),
                arity: 3,
                callback: |_ctx, args| match &args[0] {
                    RuntimeValue::List { items, is_const } => {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation("Const".to_string()),
                                None,
                                None,
                            ));
                        }
                        let idx = args[1].try_into_int()? as usize;
                        if idx <= items.borrow().len() {
                            items.borrow_mut().insert(idx, args[2].clone());
                        }
                        Ok(RuntimeValue::Null)
                    }
                    RuntimeValue::Map { entries, is_const } => {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation("Const".to_string()),
                                None,
                                None,
                            ));
                        }
                        let key = args[1].try_into_string()?;
                        entries.borrow_mut().insert(key, args[2].clone());
                        Ok(RuntimeValue::Null)
                    }
                    _ => Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation("Type error".to_string()),
                        None,
                        None,
                    )),
                },
            }),
        );

        exports.insert(
            "remove".to_string(),
            Rc::new(StdFunction {
                name: "remove".to_string(),
                arity: 2,
                callback: |_ctx, args| match &args[0] {
                    RuntimeValue::List { items, is_const } => {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation("Const".to_string()),
                                None,
                                None,
                            ));
                        }
                        let idx = args[1].try_into_int()? as usize;
                        if idx < items.borrow().len() {
                            let val = items.borrow_mut().remove(idx);
                            Ok(val)
                        } else {
                            Ok(RuntimeValue::Null)
                        }
                    }
                    RuntimeValue::Map { entries, is_const } => {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation("Const".to_string()),
                                None,
                                None,
                            ));
                        }
                        let key = args[1].try_into_string()?;
                        let val = entries
                            .borrow_mut()
                            .swap_remove(&key)
                            .unwrap_or(RuntimeValue::Null);
                        Ok(val)
                    }
                    _ => Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation("Type error".to_string()),
                        None,
                        None,
                    )),
                },
            }),
        );

        exports.insert(
            "len".to_string(),
            Rc::new(StdFunction {
                name: "len".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let len = match &args[0] {
                        RuntimeValue::List { items, .. } => items.borrow().len(),
                        RuntimeValue::Map { entries, .. } => entries.borrow().len(),
                        RuntimeValue::Str(s) => s.len(),
                        _ => 0,
                    };
                    Ok(RuntimeValue::Int(len as i64))
                },
            }),
        );

        exports.insert(
            "clear".to_string(),
            Rc::new(StdFunction {
                name: "clear".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    match &args[0] {
                        RuntimeValue::List { items, is_const } => {
                            if *is_const {
                                return Err(RuntimeError::new(
                                    RuntimeErrorKind::InvalidOperation("Const".to_string()),
                                    None,
                                    None,
                                ));
                            }
                            items.borrow_mut().clear();
                        }
                        RuntimeValue::Map { entries, is_const } => {
                            if *is_const {
                                return Err(RuntimeError::new(
                                    RuntimeErrorKind::InvalidOperation("Const".to_string()),
                                    None,
                                    None,
                                ));
                            }
                            entries.borrow_mut().clear();
                        }
                        _ => {}
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "keys".to_string(),
            Rc::new(StdFunction {
                name: "keys".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let k_list: Vec<RuntimeValue> = entries
                            .borrow()
                            .keys()
                            .map(|k| RuntimeValue::Str(k.clone()))
                            .collect();
                        Ok(RuntimeValue::List {
                            items: Rc::new(std::cell::RefCell::new(k_list)),
                            is_const: false,
                        })
                    } else {
                        Ok(RuntimeValue::List {
                            items: Rc::new(std::cell::RefCell::new(vec![])),
                            is_const: false,
                        })
                    }
                },
            }),
        );

        exports.insert(
            "values".to_string(),
            Rc::new(StdFunction {
                name: "values".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let v_list: Vec<RuntimeValue> =
                            entries.borrow().values().cloned().collect();
                        Ok(RuntimeValue::List {
                            items: Rc::new(std::cell::RefCell::new(v_list)),
                            is_const: false,
                        })
                    } else {
                        Ok(RuntimeValue::List {
                            items: Rc::new(std::cell::RefCell::new(vec![])),
                            is_const: false,
                        })
                    }
                },
            }),
        );

        exports.insert(
            "contains".to_string(),
            Rc::new(StdFunction {
                name: "contains".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let found = match &args[0] {
                        RuntimeValue::List { items, .. } => items.borrow().iter().any(|x| {
                            x.try_into_string().unwrap_or_default()
                                == args[1].try_into_string().unwrap_or_default()
                        }),
                        RuntimeValue::Map { entries, .. } => {
                            let k = args[1].try_into_string().unwrap_or_default();
                            entries.borrow().contains_key(&k)
                        }
                        _ => false,
                    };
                    Ok(RuntimeValue::Bool(found))
                },
            }),
        );

        self.register_module(
            "std.collections",
            StdlibModule {
                name: "std.collections".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
