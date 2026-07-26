use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::RuntimeError,
    error::RuntimeErrorKind,
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_strings(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
            "trim".to_string(),
            Rc::new(StdFunction {
                name: "trim".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(s.trim().to_string()))
                },
            }),
        );

        exports.insert(
            "replace".to_string(),
            Rc::new(StdFunction {
                name: "replace".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let source = args[0].try_into_string()?;
                    let from = args[1].try_into_string()?;
                    let to = args[2].try_into_string()?;
                    Ok(RuntimeValue::Str(source.replace(&from, &to)))
                },
            }),
        );

        exports.insert(
            "split".to_string(),
            Rc::new(StdFunction {
                name: "split".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    let pat = args[1].try_into_string()?;
                    let parts: Vec<RuntimeValue> = s
                        .split(&pat)
                        .map(|p| RuntimeValue::Str(p.to_string()))
                        .collect();
                    Ok(RuntimeValue::List {
                        items: Rc::new(std::cell::RefCell::new(parts)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "join".to_string(),
            Rc::new(StdFunction {
                name: "join".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let sep = args[1].try_into_string()?;
                    if let RuntimeValue::List { items, .. } = &args[0] {
                        let mut str_parts = Vec::new();
                        for item in items.borrow().iter() {
                            str_parts.push(item.try_into_string()?);
                        }
                        Ok(RuntimeValue::Str(str_parts.join(&sep)))
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
            "to_lower".to_string(),
            Rc::new(StdFunction {
                name: "to_lower".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(s.to_lowercase()))
                },
            }),
        );

        exports.insert(
            "to_upper".to_string(),
            Rc::new(StdFunction {
                name: "to_upper".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(s.to_uppercase()))
                },
            }),
        );

        exports.insert(
            "contains".to_string(),
            Rc::new(StdFunction {
                name: "contains".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    let sub = args[1].try_into_string()?;
                    Ok(RuntimeValue::Bool(s.contains(&sub)))
                },
            }),
        );

        exports.insert(
            "from_int".to_string(),
            Rc::new(StdFunction {
                name: "from_int".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_int()?;
                    Ok(RuntimeValue::Str(val.to_string()))
                },
            }),
        );

        exports.insert(
            "from_float".to_string(),
            Rc::new(StdFunction {
                name: "from_float".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_float()?;
                    Ok(RuntimeValue::Str(val.to_string()))
                },
            }),
        );

        exports.insert(
            "from_bool".to_string(),
            Rc::new(StdFunction {
                name: "from_bool".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_bool()?;
                    Ok(RuntimeValue::Str(val.to_string()))
                },
            }),
        );

        self.register_module(
            "std.string",
            StdlibModule {
                name: "std.string".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );

        self.register_module(
            "std.strings",
            StdlibModule {
                name: "std.strings".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
