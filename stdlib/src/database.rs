use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    context::{Capability, RuntimeContext},
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};

fn get_params_list(args: &[RuntimeValue]) -> Vec<RuntimeValue> {
    if args.len() > 2 {
        if let RuntimeValue::List { items, .. } = &args[2] {
            items.borrow().clone()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn params_list_to_sqlite_params(params_list: &[RuntimeValue]) -> Vec<rusqlite::types::Value> {
    params_list
        .iter()
        .map(|p| match p {
            RuntimeValue::Null => rusqlite::types::Value::Null,
            RuntimeValue::Bool(b) => rusqlite::types::Value::Integer(if *b { 1 } else { 0 }),
            RuntimeValue::Int(i) => rusqlite::types::Value::Integer(*i),
            RuntimeValue::Float(f) => rusqlite::types::Value::Real(*f),
            RuntimeValue::Str(s) => rusqlite::types::Value::Text(s.clone()),
            _ => rusqlite::types::Value::Null,
        })
        .collect()
}

fn std_database_connect(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let url = args[0].try_into_string()?;

    // Whitelist sandbox checks: require FileSystem capability for local files
    if url != ":memory:" && !ctx.config.capabilities.contains(&Capability::FileSystem) {
        return Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(
                "Security policy violation: FileSystem capability is denied for local database file"
                    .to_string(),
            ),
            None,
            None,
        ));
    }

    let conn = if url == ":memory:" {
        rusqlite::Connection::open_in_memory()
    } else {
        rusqlite::Connection::open(&url)
    }
    .map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("Database connect error: {}", e)),
            None,
            None,
        )
    })?;

    let handle = ctx.resources.borrow_mut().insert(conn);
    Ok(RuntimeValue::Int(handle as i64))
}

fn std_database_query(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let handle = args[0].try_into_int()? as u32;
    let sql = args[1].try_into_string()?;
    let params_list = get_params_list(&args);

    let resources = ctx.resources.clone();
    let resources_borrow = resources.borrow();
    let conn = resources_borrow
        .get::<rusqlite::Connection>(handle)
        .ok_or_else(|| {
            RuntimeError::new(
                RuntimeErrorKind::InvalidOperation(format!(
                    "Invalid database connection handle: {}",
                    handle
                )),
                None,
                None,
            )
        })?;

    let mut stmt = conn.prepare(&sql).map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("Database prepare error: {}", e)),
            None,
            None,
        )
    })?;

    let params_converted = params_list_to_sqlite_params(&params_list);
    let column_names: Vec<String> = stmt
        .column_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = params_converted
        .iter()
        .map(|p| p as &dyn rusqlite::types::ToSql)
        .collect();

    let mut rows = stmt.query(params_refs.as_slice()).map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("Database query error: {}", e)),
            None,
            None,
        )
    })?;

    let mut result_rows = Vec::new();

    while let Some(row) = rows.next().map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("Database row fetch error: {}", e)),
            None,
            None,
        )
    })? {
        let mut row_map = IndexMap::new();
        for (idx, name) in column_names.iter().enumerate() {
            let value = match row.get_ref(idx).map_err(|e| {
                RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation(format!("Database column get error: {}", e)),
                    None,
                    None,
                )
            })? {
                rusqlite::types::ValueRef::Null => RuntimeValue::Null,
                rusqlite::types::ValueRef::Integer(i) => RuntimeValue::Int(i),
                rusqlite::types::ValueRef::Real(f) => RuntimeValue::Float(f),
                rusqlite::types::ValueRef::Text(bytes) => {
                    let s = String::from_utf8_lossy(bytes).into_owned();
                    RuntimeValue::Str(s)
                }
                rusqlite::types::ValueRef::Blob(bytes) => {
                    let s = String::from_utf8_lossy(bytes).into_owned();
                    RuntimeValue::Str(s)
                }
            };
            row_map.insert(name.clone(), value);
        }
        result_rows.push(RuntimeValue::Map {
            entries: Rc::new(RefCell::new(row_map)),
            is_const: false,
        });
    }

    Ok(RuntimeValue::List {
        items: Rc::new(RefCell::new(result_rows)),
        is_const: false,
    })
}

fn std_database_execute(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let handle = args[0].try_into_int()? as u32;
    let sql = args[1].try_into_string()?;
    let params_list = get_params_list(&args);

    let resources = ctx.resources.clone();
    let resources_borrow = resources.borrow();
    let conn = resources_borrow
        .get::<rusqlite::Connection>(handle)
        .ok_or_else(|| {
            RuntimeError::new(
                RuntimeErrorKind::InvalidOperation(format!(
                    "Invalid database connection handle: {}",
                    handle
                )),
                None,
                None,
            )
        })?;

    let params_converted = params_list_to_sqlite_params(&params_list);
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = params_converted
        .iter()
        .map(|p| p as &dyn rusqlite::types::ToSql)
        .collect();

    let rows_affected = conn.execute(&sql, params_refs.as_slice()).map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("Database execute error: {}", e)),
            None,
            None,
        )
    })?;

    Ok(RuntimeValue::Int(rows_affected as i64))
}

fn std_database_close(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let handle = args[0].try_into_int()? as u32;
    let removed = ctx
        .resources
        .borrow_mut()
        .remove::<rusqlite::Connection>(handle);
    if removed.is_some() {
        Ok(RuntimeValue::Bool(true))
    } else {
        Ok(RuntimeValue::Bool(false))
    }
}

impl StdlibRegistry {
    pub fn register_database(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "connect".to_string(),
            Rc::new(StdFunction {
                name: "connect".to_string(),
                arity: 1,
                callback: std_database_connect,
            }),
        );

        exports.insert(
            "query".to_string(),
            Rc::new(StdFunction {
                name: "query".to_string(),
                arity: 2,
                callback: std_database_query,
            }),
        );

        exports.insert(
            "execute".to_string(),
            Rc::new(StdFunction {
                name: "execute".to_string(),
                arity: 2,
                callback: std_database_execute,
            }),
        );

        exports.insert(
            "close".to_string(),
            Rc::new(StdFunction {
                name: "close".to_string(),
                arity: 1,
                callback: std_database_close,
            }),
        );

        self.register_module(
            "std.database",
            StdlibModule {
                name: "std.database".to_string(),
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
    use std::collections::HashMap;

    #[test]
    fn test_register_database() {
        let mut registry = StdlibRegistry {
            modules: HashMap::new(),
        };
        registry.register_database();

        assert!(registry.has_module("std.database"));
        let module = registry.get_module("std.database").unwrap();
        assert_eq!(module.name, "std.database");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("connect"));
        assert!(module.exports.contains_key("query"));
        assert!(module.exports.contains_key("execute"));
        assert!(module.exports.contains_key("close"));
        assert_eq!(module.exports.len(), 4);
    }
}
