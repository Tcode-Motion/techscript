use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
    function::Callable,
    RuntimeContext,
};
use crate::{StdFunction, MockFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_testing(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "assert".to_string(),
            Rc::new(StdFunction {
                name: "assert".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let cond = args[0].try_into_bool()?;
                    let msg = args[1].try_into_string()?;
                    if !cond {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("Assertion failed: {}", msg)),
                            None,
                            None,
                        ));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "assert_eq".to_string(),
            Rc::new(StdFunction {
                name: "assert_eq".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let actual = &args[0];
                    let expected = &args[1];
                    let msg = args[2].try_into_string()?;
                    
                    let is_eq = match (actual, expected) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a == b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a == b,
                        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a == b,
                        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a == b,
                        (RuntimeValue::Null, RuntimeValue::Null) => true,
                        _ => false,
                    };
                    if !is_eq {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("Assertion failed (actual != expected): {}", msg)),
                            None,
                            None,
                        ));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "assert_ne".to_string(),
            Rc::new(StdFunction {
                name: "assert_ne".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let actual = &args[0];
                    let expected = &args[1];
                    let msg = args[2].try_into_string()?;
                    
                    let is_eq = match (actual, expected) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a == b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a == b,
                        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a == b,
                        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a == b,
                        (RuntimeValue::Null, RuntimeValue::Null) => true,
                        _ => false,
                    };
                    if is_eq {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("Assertion failed (actual == expected): {}", msg)),
                            None,
                            None,
                        ));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "mock_fn".to_string(),
            Rc::new(StdFunction {
                name: "mock_fn".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].clone();
                    Ok(RuntimeValue::Function(Rc::new(MockFunction {
                        name: "mock".to_string(),
                        val,
                    })))
                },
            }),
        );

        exports.insert(
            "mock_object".to_string(),
            Rc::new(StdFunction {
                name: "mock_object".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    Ok(args[0].clone())
                },
            }),
        );

        exports.insert(
            "benchmark".to_string(),
            Rc::new(StdFunction {
                name: "benchmark".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if let RuntimeValue::Function(func) = &args[0] {
                        let iterations = args[1].try_into_int()?;
                        let start = std::time::Instant::now();
                        for _ in 0..iterations {
                            func.call(ctx, vec![]).ok();
                        }
                        let elapsed = start.elapsed().as_secs_f64();
                        println!("Benchmark completed: {} iterations in {:.5}s", iterations, elapsed);
                        return Ok(RuntimeValue::Float(elapsed));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.testing",
            StdlibModule {
                name: "std.testing".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
