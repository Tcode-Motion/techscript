use crate::{MockFunction, StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::{RuntimeError, RuntimeErrorKind},
    function::Callable,
    value::RuntimeValue,
    RuntimeContext,
};

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
                            RuntimeErrorKind::InvalidOperation(format!(
                                "Assertion failed: {}",
                                msg
                            )),
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
                            RuntimeErrorKind::InvalidOperation(format!(
                                "Assertion failed (actual != expected): {}",
                                msg
                            )),
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
                            RuntimeErrorKind::InvalidOperation(format!(
                                "Assertion failed (actual == expected): {}",
                                msg
                            )),
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
                callback: |_ctx, args| Ok(args[0].clone()),
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
                        println!(
                            "Benchmark completed: {} iterations in {:.5}s",
                            iterations, elapsed
                        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use techscript_runtime::RuntimeConfig;

    #[test]
    fn test_register_testing() {
        let mut registry = StdlibRegistry {
            modules: HashMap::new(),
        };
        registry.register_testing();

        assert!(registry.has_module("std.testing"));
        let module = registry.get_module("std.testing").unwrap();
        assert_eq!(module.name, "std.testing");
        assert_eq!(module.version, "1.0.0");
        assert_eq!(module.exports.len(), 6);
        assert!(module.exports.contains_key("assert"));
        assert!(module.exports.contains_key("assert_eq"));
        assert!(module.exports.contains_key("assert_ne"));
        assert!(module.exports.contains_key("mock_fn"));
        assert!(module.exports.contains_key("mock_object"));
        assert!(module.exports.contains_key("benchmark"));
    }

    #[test]
    fn test_testing_functions() {
        let mut registry = StdlibRegistry {
            modules: HashMap::new(),
        };
        registry.register_testing();
        let module = registry.get_module("std.testing").unwrap();
        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        // assert
        let assert_fn = module.exports.get("assert").unwrap();
        assert_eq!(assert_fn.arity(), 2);
        assert!(assert_fn
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Bool(true),
                    RuntimeValue::Str("ok".to_string())
                ]
            )
            .is_ok());
        assert!(assert_fn
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Bool(false),
                    RuntimeValue::Str("failed".to_string())
                ]
            )
            .is_err());

        // assert_eq
        let assert_eq_fn = module.exports.get("assert_eq").unwrap();
        assert_eq!(assert_eq_fn.arity(), 3);
        assert!(assert_eq_fn
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Int(42),
                    RuntimeValue::Int(42),
                    RuntimeValue::Str("numbers match".to_string())
                ]
            )
            .is_ok());
        assert!(assert_eq_fn
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Int(42),
                    RuntimeValue::Int(100),
                    RuntimeValue::Str("numbers differ".to_string())
                ]
            )
            .is_err());

        // assert_ne
        let assert_ne_fn = module.exports.get("assert_ne").unwrap();
        assert_eq!(assert_ne_fn.arity(), 3);
        assert!(assert_ne_fn
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Int(42),
                    RuntimeValue::Int(100),
                    RuntimeValue::Str("numbers differ".to_string())
                ]
            )
            .is_ok());
        assert!(assert_ne_fn
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Int(42),
                    RuntimeValue::Int(42),
                    RuntimeValue::Str("numbers match".to_string())
                ]
            )
            .is_err());

        // mock_fn
        let mock_fn = module.exports.get("mock_fn").unwrap();
        assert_eq!(mock_fn.arity(), 1);
        let mock_res = mock_fn
            .call(&mut ctx, vec![RuntimeValue::Int(123)])
            .unwrap();
        if let RuntimeValue::Function(func) = mock_res {
            let res = func.call(&mut ctx, vec![]).unwrap();
            assert_eq!(res, RuntimeValue::Int(123));
        } else {
            panic!("Expected RuntimeValue::Function");
        }

        // mock_object
        let mock_obj_fn = module.exports.get("mock_object").unwrap();
        assert_eq!(mock_obj_fn.arity(), 1);
        let obj_res = mock_obj_fn
            .call(&mut ctx, vec![RuntimeValue::Str("test_obj".to_string())])
            .unwrap();
        assert_eq!(obj_res, RuntimeValue::Str("test_obj".to_string()));

        // benchmark
        let bench_fn = module.exports.get("benchmark").unwrap();
        assert_eq!(bench_fn.arity(), 2);
        let mock_res = mock_fn.call(&mut ctx, vec![RuntimeValue::Null]).unwrap();
        let bench_res = bench_fn
            .call(&mut ctx, vec![mock_res, RuntimeValue::Int(5)])
            .unwrap();
        if let RuntimeValue::Float(elapsed) = bench_res {
            assert!(elapsed >= 0.0);
        } else {
            panic!("Expected RuntimeValue::Float");
        }
    }
}
