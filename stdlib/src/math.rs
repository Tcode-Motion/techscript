use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
    RuntimeContext,
};

fn abs(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    match &args[0] {
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
    }
}

fn sin(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f.sin()))
}

fn cos(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f.cos()))
}

fn tan(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f.tan()))
}

fn log(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f.ln()))
}

fn exp(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f.exp()))
}

fn sqrt(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f.sqrt()))
}

fn pow(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let base = args[0].try_into_float()?;
    let exponent = args[1].try_into_float()?;
    Ok(RuntimeValue::Float(base.powf(exponent)))
}

fn floor(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f.floor()))
}

fn ceil(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f.ceil()))
}

fn round(_ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f.round()))
}

fn random(
    _ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    use std::cell::Cell;
    use std::time::SystemTime;
    thread_local! {
        static SEED: Cell<u64> = Cell::new({
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        });
    }
    let next_seed = SEED.with(|cell| {
        let current = cell.get();
        let next = current
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        cell.set(next);
        next
    });
    let rand_float = (next_seed as f64) / (u64::MAX as f64);
    Ok(RuntimeValue::Float(rand_float))
}

fn to_float(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let f = args[0].try_into_float()?;
    Ok(RuntimeValue::Float(f))
}

impl StdlibRegistry {
    pub fn register_math(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "abs".to_string(),
            Rc::new(StdFunction {
                name: "abs".to_string(),
                arity: 1,
                callback: abs,
            }),
        );

        exports.insert(
            "sin".to_string(),
            Rc::new(StdFunction {
                name: "sin".to_string(),
                arity: 1,
                callback: sin,
            }),
        );

        exports.insert(
            "cos".to_string(),
            Rc::new(StdFunction {
                name: "cos".to_string(),
                arity: 1,
                callback: cos,
            }),
        );

        exports.insert(
            "tan".to_string(),
            Rc::new(StdFunction {
                name: "tan".to_string(),
                arity: 1,
                callback: tan,
            }),
        );

        exports.insert(
            "log".to_string(),
            Rc::new(StdFunction {
                name: "log".to_string(),
                arity: 1,
                callback: log,
            }),
        );

        exports.insert(
            "exp".to_string(),
            Rc::new(StdFunction {
                name: "exp".to_string(),
                arity: 1,
                callback: exp,
            }),
        );

        exports.insert(
            "sqrt".to_string(),
            Rc::new(StdFunction {
                name: "sqrt".to_string(),
                arity: 1,
                callback: sqrt,
            }),
        );

        exports.insert(
            "pow".to_string(),
            Rc::new(StdFunction {
                name: "pow".to_string(),
                arity: 2,
                callback: pow,
            }),
        );

        exports.insert(
            "floor".to_string(),
            Rc::new(StdFunction {
                name: "floor".to_string(),
                arity: 1,
                callback: floor,
            }),
        );

        exports.insert(
            "ceil".to_string(),
            Rc::new(StdFunction {
                name: "ceil".to_string(),
                arity: 1,
                callback: ceil,
            }),
        );

        exports.insert(
            "round".to_string(),
            Rc::new(StdFunction {
                name: "round".to_string(),
                arity: 1,
                callback: round,
            }),
        );

        exports.insert(
            "random".to_string(),
            Rc::new(StdFunction {
                name: "random".to_string(),
                arity: 0,
                callback: random,
            }),
        );

        exports.insert(
            "to_float".to_string(),
            Rc::new(StdFunction {
                name: "to_float".to_string(),
                arity: 1,
                callback: to_float,
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
