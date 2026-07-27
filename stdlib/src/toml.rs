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
    pub fn register_toml(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let toml_str = args[0].try_into_string()?.replace("\\\"", "\"");
                    let v: toml::Value = toml::from_str(&toml_str).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("TOML parse error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(parse_toml_value(v))
                },
            }),
        );

        exports.insert(
            "stringify".to_string(),
            Rc::new(StdFunction {
                name: "stringify".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let toml_val = to_toml_value(&args[0])?;
                    let toml_str = toml::to_string(&toml_val).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "TOML stringify error: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Str(toml_str))
                },
            }),
        );

        self.register_module(
            "std.toml",
            StdlibModule {
                name: "std.toml".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}

pub fn parse_toml_value(v: toml::Value) -> RuntimeValue {
    match v {
        toml::Value::Boolean(b) => RuntimeValue::Bool(b),
        toml::Value::Integer(i) => RuntimeValue::Int(i),
        toml::Value::Float(f) => RuntimeValue::Float(f),
        toml::Value::String(s) => RuntimeValue::Str(s),
        toml::Value::Array(arr) => {
            let items = arr.into_iter().map(parse_toml_value).collect::<Vec<_>>();
            RuntimeValue::List {
                items: Rc::new(RefCell::new(items)),
                is_const: false,
            }
        }
        toml::Value::Table(tab) => {
            let mut entries = IndexMap::new();
            for (k, v) in tab {
                entries.insert(k, parse_toml_value(v));
            }
            RuntimeValue::Map {
                entries: Rc::new(RefCell::new(entries)),
                is_const: false,
            }
        }
        toml::Value::Datetime(dt) => RuntimeValue::Str(dt.to_string()),
    }
}

pub fn to_toml_value(val: &RuntimeValue) -> Result<toml::Value, RuntimeError> {
    match val {
        RuntimeValue::Null => Ok(toml::Value::String("none".to_string())),
        RuntimeValue::Bool(b) => Ok(toml::Value::Boolean(*b)),
        RuntimeValue::Int(i) => Ok(toml::Value::Integer(*i)),
        RuntimeValue::Float(f) => Ok(toml::Value::Float(*f)),
        RuntimeValue::Str(s) => Ok(toml::Value::String(s.clone())),
        RuntimeValue::List { items, .. } => {
            let mut arr = Vec::new();
            for item in items.borrow().iter() {
                arr.push(to_toml_value(item)?);
            }
            Ok(toml::Value::Array(arr))
        }
        RuntimeValue::Map { entries, .. } => {
            let mut table = toml::map::Map::new();
            for (k, v) in entries.borrow().iter() {
                table.insert(k.clone(), to_toml_value(v)?);
            }
            Ok(toml::Value::Table(table))
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!(
                "Cannot convert type {} to TOML",
                val.runtime_type()
            )),
            None,
            None,
        )),
    }
}
