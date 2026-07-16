use techscript_runtime::{RuntimeError, RuntimeErrorKind, RuntimeValue};

/// Evaluates a unary operator on a value.
pub fn eval_unary(op: &str, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    match op {
        "-" => match right {
            RuntimeValue::Int(i) => Ok(RuntimeValue::Int(-i)),
            RuntimeValue::Float(f) => Ok(RuntimeValue::Float(-f)),
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "number".to_string(),
                    found: right.runtime_type().to_string(),
                },
                None,
                None,
            )),
        },
        "+" => match right {
            RuntimeValue::Int(_) | RuntimeValue::Float(_) => Ok(right),
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "number".to_string(),
                    found: right.runtime_type().to_string(),
                },
                None,
                None,
            )),
        },
        "not" | "!" => Ok(RuntimeValue::Bool(!right.is_truthy())),
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("Unknown unary operator '{}'", op)),
            None,
            None,
        )),
    }
}

/// Evaluates a binary operator between two values.
pub fn eval_binary(
    op: &str,
    left: RuntimeValue,
    right: RuntimeValue,
) -> Result<RuntimeValue, RuntimeError> {
    match op {
        "+" => match (left, right) {
            (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => Ok(RuntimeValue::Int(i1 + i2)),
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => Ok(RuntimeValue::Float(f1 + f2)),
            (RuntimeValue::Int(i), RuntimeValue::Float(f)) => Ok(RuntimeValue::Float(i as f64 + f)),
            (RuntimeValue::Float(f), RuntimeValue::Int(i)) => Ok(RuntimeValue::Float(f + i as f64)),
            (RuntimeValue::Str(s1), RuntimeValue::Str(s2)) => Ok(RuntimeValue::Str(s1 + &s2)),
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "compatible numbers or strings".to_string(),
                    found: "mismatched types".to_string(),
                },
                None,
                None,
            )),
        },
        "-" => match (left, right) {
            (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => Ok(RuntimeValue::Int(i1 - i2)),
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => Ok(RuntimeValue::Float(f1 - f2)),
            (RuntimeValue::Int(i), RuntimeValue::Float(f)) => Ok(RuntimeValue::Float(i as f64 - f)),
            (RuntimeValue::Float(f), RuntimeValue::Int(i)) => Ok(RuntimeValue::Float(f - i as f64)),
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "numbers".to_string(),
                    found: "mismatched types".to_string(),
                },
                None,
                None,
            )),
        },
        "*" => match (left, right) {
            (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => Ok(RuntimeValue::Int(i1 * i2)),
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => Ok(RuntimeValue::Float(f1 * f2)),
            (RuntimeValue::Int(i), RuntimeValue::Float(f)) => Ok(RuntimeValue::Float(i as f64 * f)),
            (RuntimeValue::Float(f), RuntimeValue::Int(i)) => Ok(RuntimeValue::Float(f * i as f64)),
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "numbers".to_string(),
                    found: "mismatched types".to_string(),
                },
                None,
                None,
            )),
        },
        "/" => {
            let (f1, f2) = match (left, right) {
                (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => (i1 as f64, i2 as f64),
                (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => (f1, f2),
                (RuntimeValue::Int(i), RuntimeValue::Float(f)) => (i as f64, f),
                (RuntimeValue::Float(f), RuntimeValue::Int(i)) => (f, i as f64),
                _ => {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch {
                            expected: "numbers".to_string(),
                            found: "mismatched types".to_string(),
                        },
                        None,
                        None,
                    ))
                }
            };
            if f2 == 0.0 {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::DivisionByZero,
                    None,
                    None,
                ));
            }
            Ok(RuntimeValue::Float(f1 / f2))
        }
        "//" => {
            let (i1, i2) = match (left, right) {
                (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => (i1, i2),
                _ => {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch {
                            expected: "integers".to_string(),
                            found: "mismatched types".to_string(),
                        },
                        None,
                        None,
                    ))
                }
            };
            if i2 == 0 {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::DivisionByZero,
                    None,
                    None,
                ));
            }
            Ok(RuntimeValue::Int(i1 / i2))
        }
        "**" => {
            let (f1, f2) = match (left, right) {
                (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => (i1 as f64, i2 as f64),
                (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => (f1, f2),
                (RuntimeValue::Int(i), RuntimeValue::Float(f)) => (i as f64, f),
                (RuntimeValue::Float(f), RuntimeValue::Int(i)) => (f, i as f64),
                _ => {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch {
                            expected: "numbers".to_string(),
                            found: "mismatched types".to_string(),
                        },
                        None,
                        None,
                    ))
                }
            };
            Ok(RuntimeValue::Float(f1.powf(f2)))
        }
        "<" => match (left, right) {
            (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => Ok(RuntimeValue::Bool(i1 < i2)),
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => Ok(RuntimeValue::Bool(f1 < f2)),
            (RuntimeValue::Int(i), RuntimeValue::Float(f)) => {
                Ok(RuntimeValue::Bool((i as f64) < f))
            }
            (RuntimeValue::Float(f), RuntimeValue::Int(i)) => {
                Ok(RuntimeValue::Bool(f < (i as f64)))
            }
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "numbers".to_string(),
                    found: "mismatched types".to_string(),
                },
                None,
                None,
            )),
        },
        "<=" => match (left, right) {
            (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => Ok(RuntimeValue::Bool(i1 <= i2)),
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => Ok(RuntimeValue::Bool(f1 <= f2)),
            (RuntimeValue::Int(i), RuntimeValue::Float(f)) => {
                Ok(RuntimeValue::Bool((i as f64) <= f))
            }
            (RuntimeValue::Float(f), RuntimeValue::Int(i)) => {
                Ok(RuntimeValue::Bool(f <= (i as f64)))
            }
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "numbers".to_string(),
                    found: "mismatched types".to_string(),
                },
                None,
                None,
            )),
        },
        ">" => match (left, right) {
            (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => Ok(RuntimeValue::Bool(i1 > i2)),
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => Ok(RuntimeValue::Bool(f1 > f2)),
            (RuntimeValue::Int(i), RuntimeValue::Float(f)) => {
                Ok(RuntimeValue::Bool((i as f64) > f))
            }
            (RuntimeValue::Float(f), RuntimeValue::Int(i)) => {
                Ok(RuntimeValue::Bool(f > (i as f64)))
            }
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "numbers".to_string(),
                    found: "mismatched types".to_string(),
                },
                None,
                None,
            )),
        },
        ">=" => match (left, right) {
            (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => Ok(RuntimeValue::Bool(i1 >= i2)),
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => Ok(RuntimeValue::Bool(f1 >= f2)),
            (RuntimeValue::Int(i), RuntimeValue::Float(f)) => {
                Ok(RuntimeValue::Bool((i as f64) >= f))
            }
            (RuntimeValue::Float(f), RuntimeValue::Int(i)) => {
                Ok(RuntimeValue::Bool(f >= (i as f64)))
            }
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "numbers".to_string(),
                    found: "mismatched types".to_string(),
                },
                None,
                None,
            )),
        },
        "==" => Ok(RuntimeValue::Bool(left == right)),
        "!=" => Ok(RuntimeValue::Bool(left != right)),
        "===" => Ok(RuntimeValue::Bool(left.physical_eq(&right))),
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("Unknown binary operator '{}'", op)),
            None,
            None,
        )),
    }
}
