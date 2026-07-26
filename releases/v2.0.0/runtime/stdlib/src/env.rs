use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use techscript_runtime::{
    context::Capability,
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_env(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("get".to_string(), Rc::new(StdFunction {
            name: "get".to_string(),
            arity: 1,
            callback: |ctx, args| {
                if !ctx.config.capabilities.contains(&Capability::Environment) {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation(
                            "Security policy violation: Environment capability is denied".to_string(),
                        ),
                        None, None,
                    ));
                }
                let key = match &args[0] {
                    RuntimeValue::Str(s) => s.clone(),
                    _ => return Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch { expected: "string".to_string(), found: "other".to_string() },
                        None, None,
                    )),
                };
                let val = std::env::var(&key).unwrap_or_default();
                Ok(RuntimeValue::Str(val))
            },
        }));

        exports.insert("set".to_string(), Rc::new(StdFunction {
            name: "set".to_string(),
            arity: 2,
            callback: |ctx, args| {
                if !ctx.config.capabilities.contains(&Capability::Environment) {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation(
                            "Security policy violation: Environment capability is denied".to_string(),
                        ),
                        None, None,
                    ));
                }
                let key = match &args[0] {
                    RuntimeValue::Str(s) => s.clone(),
                    _ => return Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch { expected: "string".to_string(), found: "other".to_string() },
                        None, None,
                    )),
                };
                let val = match &args[1] {
                    RuntimeValue::Str(s) => s.clone(),
                    _ => return Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch { expected: "string".to_string(), found: "other".to_string() },
                        None, None,
                    )),
                };
                std::env::set_var(&key, &val);
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("all".to_string(), Rc::new(StdFunction {
            name: "all".to_string(),
            arity: 0,
            callback: |ctx, _args| {
                if !ctx.config.capabilities.contains(&Capability::Environment) {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation(
                            "Security policy violation: Environment capability is denied".to_string(),
                        ),
                        None, None,
                    ));
                }
                let mut entries = indexmap::IndexMap::new();
                for (k, v) in std::env::vars() {
                    entries.insert(k, RuntimeValue::Str(v));
                }
                Ok(RuntimeValue::Map {
                    entries: Rc::new(RefCell::new(entries)),
                    is_const: true,
                })
            },
        }));

        exports.insert("args".to_string(), Rc::new(StdFunction {
            name: "args".to_string(),
            arity: 0,
            callback: |ctx, _args| {
                if !ctx.config.capabilities.contains(&Capability::Environment) {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation(
                            "Security policy violation: Environment capability is denied".to_string(),
                        ),
                        None, None,
                    ));
                }
                let r_args: Vec<RuntimeValue> = std::env::args().map(RuntimeValue::Str).collect();
                Ok(RuntimeValue::List {
                    items: Rc::new(RefCell::new(r_args)),
                    is_const: false,
                })
            },
        }));

        exports.insert("current_dir".to_string(), Rc::new(StdFunction {
            name: "current_dir".to_string(),
            arity: 0,
            callback: |ctx, _args| {
                if !ctx.config.capabilities.contains(&Capability::Environment) {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation(
                            "Security policy violation: Environment capability is denied".to_string(),
                        ),
                        None, None,
                    ));
                }
                let p = std::env::current_dir().unwrap_or_default().to_string_lossy().to_string();
                Ok(RuntimeValue::Str(p))
            },
        }));

        self.register_module("std.env", StdlibModule {
            name: "std.env".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: vec![Capability::Environment],
        });
    }
}
