use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue, RuntimeContext,
};

fn string_trim(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let s = args[0].try_into_string()?;
    Ok(RuntimeValue::Str(s.trim().to_string()))
}

fn string_replace(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let source = args[0].try_into_string()?;
    let from = args[1].try_into_string()?;
    let to = args[2].try_into_string()?;
    Ok(RuntimeValue::Str(source.replace(&from, &to)))
}

fn string_split(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
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
}

fn string_join(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
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
}

fn string_to_lower(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let s = args[0].try_into_string()?;
    Ok(RuntimeValue::Str(s.to_lowercase()))
}

fn string_to_upper(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let s = args[0].try_into_string()?;
    Ok(RuntimeValue::Str(s.to_uppercase()))
}

fn string_contains(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let s = args[0].try_into_string()?;
    let sub = args[1].try_into_string()?;
    Ok(RuntimeValue::Bool(s.contains(&sub)))
}

fn string_from_int(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let val = args[0].try_into_int()?;
    Ok(RuntimeValue::Str(val.to_string()))
}

fn string_from_float(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let val = args[0].try_into_float()?;
    Ok(RuntimeValue::Str(val.to_string()))
}

fn string_from_bool(
    _ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let val = args[0].try_into_bool()?;
    Ok(RuntimeValue::Str(val.to_string()))
}

impl StdlibRegistry {
    pub fn register_strings(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "trim".to_string(),
            Rc::new(StdFunction {
                name: "trim".to_string(),
                arity: 1,
                callback: string_trim,
            }),
        );

        exports.insert(
            "replace".to_string(),
            Rc::new(StdFunction {
                name: "replace".to_string(),
                arity: 3,
                callback: string_replace,
            }),
        );

        exports.insert(
            "split".to_string(),
            Rc::new(StdFunction {
                name: "split".to_string(),
                arity: 2,
                callback: string_split,
            }),
        );

        exports.insert(
            "join".to_string(),
            Rc::new(StdFunction {
                name: "join".to_string(),
                arity: 2,
                callback: string_join,
            }),
        );

        exports.insert(
            "to_lower".to_string(),
            Rc::new(StdFunction {
                name: "to_lower".to_string(),
                arity: 1,
                callback: string_to_lower,
            }),
        );

        exports.insert(
            "to_upper".to_string(),
            Rc::new(StdFunction {
                name: "to_upper".to_string(),
                arity: 1,
                callback: string_to_upper,
            }),
        );

        exports.insert(
            "contains".to_string(),
            Rc::new(StdFunction {
                name: "contains".to_string(),
                arity: 2,
                callback: string_contains,
            }),
        );

        exports.insert(
            "from_int".to_string(),
            Rc::new(StdFunction {
                name: "from_int".to_string(),
                arity: 1,
                callback: string_from_int,
            }),
        );

        exports.insert(
            "from_float".to_string(),
            Rc::new(StdFunction {
                name: "from_float".to_string(),
                arity: 1,
                callback: string_from_float,
            }),
        );

        exports.insert(
            "from_bool".to_string(),
            Rc::new(StdFunction {
                name: "from_bool".to_string(),
                arity: 1,
                callback: string_from_bool,
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
