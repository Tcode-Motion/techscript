use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
    RuntimeContext,
};

impl StdlibRegistry {
    pub fn register_math(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "abs".to_string(),
            Rc::new(StdFunction {
                name: "abs".to_string(),
                arity: 1,
                callback: |_ctx, args| match &args[0] {
                    RuntimeValue::Int(i) => Ok(RuntimeValue::Int(i.abs())),
                    RuntimeValue::Float(f) => Ok(RuntimeValue::Float(f.abs())),
                    other => Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch {
                            expected: "Int or Float".to_string(),
                            found: other.runtime_type().to_string(),
                        },
                        None,
                        None,
                    )),
                },
            }),
        );

        exports.insert(
            "sin".to_string(),
            Rc::new(StdFunction {
                name: "sin".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.sin()))
                },
            }),
        );

        exports.insert(
            "cos".to_string(),
            Rc::new(StdFunction {
                name: "cos".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.cos()))
                },
            }),
        );

        exports.insert(
            "tan".to_string(),
            Rc::new(StdFunction {
                name: "tan".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.tan()))
                },
            }),
        );

        exports.insert(
            "log".to_string(),
            Rc::new(StdFunction {
                name: "log".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.ln()))
                },
            }),
        );

        exports.insert(
            "exp".to_string(),
            Rc::new(StdFunction {
                name: "exp".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.exp()))
                },
            }),
        );

        exports.insert(
            "sqrt".to_string(),
            Rc::new(StdFunction {
                name: "sqrt".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.sqrt()))
                },
            }),
        );

        exports.insert(
            "pow".to_string(),
            Rc::new(StdFunction {
                name: "pow".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let base = args[0].try_into_float()?;
                    let exponent = args[1].try_into_float()?;
                    Ok(RuntimeValue::Float(base.powf(exponent)))
                },
            }),
        );

        exports.insert(
            "floor".to_string(),
            Rc::new(StdFunction {
                name: "floor".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.floor()))
                },
            }),
        );

        exports.insert(
            "ceil".to_string(),
            Rc::new(StdFunction {
                name: "ceil".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.ceil()))
                },
            }),
        );

        exports.insert(
            "round".to_string(),
            Rc::new(StdFunction {
                name: "round".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.round()))
                },
            }),
        );

        exports.insert(
            "random".to_string(),
            Rc::new(StdFunction {
                name: "random".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::time::SystemTime;
                    let nano = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .subsec_nanos();
                    let rand_float = (nano as f64) / 1_000_000_000.0;
                    Ok(RuntimeValue::Float(rand_float))
                },
            }),
        );

        exports.insert(
            "to_float".to_string(),
            Rc::new(StdFunction {
                name: "to_float".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f))
                },
            }),
        );

        self.register_module(
            "std.math",
            StdlibModule {
                name: "std.math".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
