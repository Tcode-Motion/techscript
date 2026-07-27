use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};

impl StdlibRegistry {
    pub fn register_json(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "stringify".to_string(),
            Rc::new(StdFunction {
                name: "stringify".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = stringify_value(&args[0])?;
                    Ok(RuntimeValue::Str(s))
                },
            }),
        );

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?.replace("\\\"", "\"");
                    let v: serde_json::Value = serde_json::from_str(&s).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("JSON parse error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(parse_json_value(v))
                },
            }),
        );

        self.register_module(
            "std.json",
            StdlibModule {
                name: "std.json".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}

pub fn stringify_value(val: &RuntimeValue) -> Result<String, RuntimeError> {
    match val {
        RuntimeValue::Null => Ok("null".to_string()),
        RuntimeValue::Bool(b) => Ok(b.to_string()),
        RuntimeValue::Int(i) => Ok(i.to_string()),
        RuntimeValue::Float(f) => Ok(f.to_string()),
        RuntimeValue::Str(s) => Ok(format!("\"{}\"", s.replace('"', "\\\""))),
        RuntimeValue::List { items, .. } => {
            let mut parts = Vec::new();
            for item in items.borrow().iter() {
                parts.push(stringify_value(item)?);
            }
            Ok(format!("[{}]", parts.join(",")))
        }
        RuntimeValue::Map { entries, .. } => {
            let mut parts = Vec::new();
            for (k, v) in entries.borrow().iter() {
                parts.push(format!(
                    "\"{}\":{}",
                    k.replace('"', "\\\""),
                    stringify_value(v)?
                ));
            }
            Ok(format!("{{{}}}", parts.join(",")))
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!(
                "Cannot stringify type {}",
                val.runtime_type()
            )),
            None,
            None,
        )),
    }
}

pub fn parse_json_value(v: serde_json::Value) -> RuntimeValue {
    match v {
        serde_json::Value::Null => RuntimeValue::Null,
        serde_json::Value::Bool(b) => RuntimeValue::Bool(b),
        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                RuntimeValue::Int(i)
            } else if let Some(f) = num.as_f64() {
                RuntimeValue::Float(f)
            } else {
                RuntimeValue::Null
            }
        }
        serde_json::Value::String(s) => RuntimeValue::Str(s),
        serde_json::Value::Array(arr) => {
            let items = arr.into_iter().map(parse_json_value).collect::<Vec<_>>();
            RuntimeValue::List {
                items: Rc::new(RefCell::new(items)),
                is_const: false,
            }
        }
        serde_json::Value::Object(obj) => {
            let mut entries = IndexMap::new();
            for (k, v) in obj {
                entries.insert(k, parse_json_value(v));
            }
            RuntimeValue::Map {
                entries: Rc::new(RefCell::new(entries)),
                is_const: false,
            }
        }
    }
}
