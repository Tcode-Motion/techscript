use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use techscript_runtime::{error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue};

thread_local! {
    static CONNECTIONS: RefCell<HashMap<i64, rusqlite::Connection>> = RefCell::new(HashMap::new());
}

static NEXT_ID: AtomicI64 = AtomicI64::new(1);

struct SqlParam<'a>(&'a RuntimeValue);

impl<'a> rusqlite::types::ToSql for SqlParam<'a> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self.0 {
            RuntimeValue::Null => Ok(rusqlite::types::ToSqlOutput::Borrowed(
                rusqlite::types::ValueRef::Null,
            )),
            RuntimeValue::Bool(b) => Ok(rusqlite::types::ToSqlOutput::Borrowed(
                rusqlite::types::ValueRef::Integer(if *b { 1 } else { 0 }),
            )),
            RuntimeValue::Int(i) => Ok(rusqlite::types::ToSqlOutput::Borrowed(
                rusqlite::types::ValueRef::Integer(*i),
            )),
            RuntimeValue::Float(f) => Ok(rusqlite::types::ToSqlOutput::Borrowed(
                rusqlite::types::ValueRef::Real(*f),
            )),
            RuntimeValue::Str(s) => Ok(rusqlite::types::ToSqlOutput::Borrowed(
                rusqlite::types::ValueRef::Text(s.as_bytes()),
            )),
            _ => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Text(self.0.to_string()),
            )),
        }
    }
}

fn sqlite_open(
    _ctx: &mut techscript_runtime::RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let path = args[0].to_string();
    let conn = rusqlite::Connection::open(&path).map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(e.to_string()),
            None,
            None,
        )
    })?;
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    CONNECTIONS.with(|m| m.borrow_mut().insert(id, conn));
    Ok(RuntimeValue::Int(id))
}

fn sqlite_execute(
    _ctx: &mut techscript_runtime::RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let id = args[0].try_into_int().map_err(|_| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation("expected int handle".to_string()),
            None,
            None,
        )
    })?;
    let sql = args[1].to_string();
    let _items_borrow;
    let params: Vec<SqlParam> = if let Some(arg) = args.get(2) {
        if let RuntimeValue::List { items, .. } = arg {
            _items_borrow = items.borrow();
            _items_borrow.iter().map(SqlParam).collect()
        } else {
            return Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "list".to_string(),
                    found: arg.runtime_type().to_string(),
                },
                None,
                None,
            ));
        }
    } else {
        Vec::new()
    };

    CONNECTIONS.with(|m| {
        let mut map = m.borrow_mut();
        if let Some(conn) = map.get_mut(&id) {
            let mut stmt = conn.prepare_cached(&sql).map_err(|e| {
                RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation(e.to_string()),
                    None,
                    None,
                )
            })?;
            stmt.execute(rusqlite::params_from_iter(params))
                .map_err(|e| {
                    RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation(e.to_string()),
                        None,
                        None,
                    )
                })?;
            Ok(())
        } else {
            Err(RuntimeError::new(
                RuntimeErrorKind::InvalidOperation("Invalid connection handle".to_string()),
                None,
                None,
            ))
        }
    })?;
    Ok(RuntimeValue::Null)
}

fn sqlite_query(
    _ctx: &mut techscript_runtime::RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let id = args[0].try_into_int().map_err(|_| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation("expected int handle".to_string()),
            None,
            None,
        )
    })?;
    let sql = args[1].to_string();
    let _items_borrow;
    let params: Vec<SqlParam> = if let Some(arg) = args.get(2) {
        if let RuntimeValue::List { items, .. } = arg {
            _items_borrow = items.borrow();
            _items_borrow.iter().map(SqlParam).collect()
        } else {
            return Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "list".to_string(),
                    found: arg.runtime_type().to_string(),
                },
                None,
                None,
            ));
        }
    } else {
        Vec::new()
    };

    let rows = CONNECTIONS.with(|m| {
        let mut map = m.borrow_mut();
        if let Some(conn) = map.get_mut(&id) {
            let mut stmt = conn.prepare_cached(&sql).map_err(|e| {
                RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation(e.to_string()),
                    None,
                    None,
                )
            })?;
            let col_count = stmt.column_count();
            let col_names: Vec<String> = (0..col_count)
                .map(|i| stmt.column_name(i).unwrap_or("?").to_string())
                .collect();

            let mut rows = Vec::new();

            let mut template = IndexMap::with_capacity(col_count);
            for name in &col_names {
                template.insert(name.clone(), RuntimeValue::Null);
            }

            let row_iter = stmt
                .query_map(rusqlite::params_from_iter(params), |row| {
                    let mut map = template.clone();
                    for i in 0..col_count {
                        let val = row.get_ref_unwrap(i);
                        let rt_val = match val {
                            rusqlite::types::ValueRef::Null => RuntimeValue::Null,
                            rusqlite::types::ValueRef::Integer(v) => RuntimeValue::Int(v),
                            rusqlite::types::ValueRef::Real(v) => RuntimeValue::Float(v),
                            rusqlite::types::ValueRef::Text(v) => RuntimeValue::Str(
                                std::str::from_utf8(v).unwrap_or_default().to_string(),
                            ),
                            rusqlite::types::ValueRef::Blob(v) => {
                                RuntimeValue::Str(String::from_utf8_lossy(v).into_owned())
                            }
                        };
                        if let Some((_, v)) = map.get_index_mut(i) {
                            *v = rt_val;
                        }
                    }
                    Ok(map)
                })
                .map_err(|e| {
                    RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation(e.to_string()),
                        None,
                        None,
                    )
                })?;
            for row in row_iter {
                if let Ok(map) = row {
                    rows.push(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(map)),
                        is_const: false,
                    });
                }
            }
            Ok(rows)
        } else {
            Err(RuntimeError::new(
                RuntimeErrorKind::InvalidOperation("Invalid connection handle".to_string()),
                None,
                None,
            ))
        }
    })?;

    Ok(RuntimeValue::List {
        items: Rc::new(RefCell::new(rows)),
        is_const: false,
    })
}

fn sqlite_close(
    _ctx: &mut techscript_runtime::RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let id = args[0].try_into_int().map_err(|_| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation("expected int handle".to_string()),
            None,
            None,
        )
    })?;
    CONNECTIONS.with(|m| m.borrow_mut().remove(&id));
    Ok(RuntimeValue::Null)
}

impl StdlibRegistry {
    pub fn register_sqlite(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "open".to_string(),
            Rc::new(StdFunction {
                name: "open".to_string(),
                arity: 1,
                callback: sqlite_open,
            }),
        );

        exports.insert(
            "execute".to_string(),
            Rc::new(StdFunction {
                name: "execute".to_string(),
                arity: 2,
                callback: sqlite_execute,
            }),
        );

        exports.insert(
            "query".to_string(),
            Rc::new(StdFunction {
                name: "query".to_string(),
                arity: 2,
                callback: sqlite_query,
            }),
        );

        exports.insert(
            "close".to_string(),
            Rc::new(StdFunction {
                name: "close".to_string(),
                arity: 1,
                callback: sqlite_close,
            }),
        );

        self.register_module(
            "std.sqlite",
            StdlibModule {
                name: "std.sqlite".to_string(),
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
    use techscript_runtime::context::{RuntimeConfig, RuntimeContext};

    #[test]
    fn test_register_sqlite() {
        let mut registry = StdlibRegistry::default();
        registry.register_sqlite();

        assert!(registry.has_module("std.sqlite"));
        let module = registry.get_module("std.sqlite").unwrap();
        assert_eq!(module.name, "std.sqlite");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("open"));
        assert!(module.exports.contains_key("execute"));
        assert!(module.exports.contains_key("query"));
        assert!(module.exports.contains_key("close"));
        assert_eq!(module.exports.len(), 4);
    }

    #[test]
    fn test_sqlite_flow() {
        let mut registry = StdlibRegistry::default();
        registry.register_sqlite();

        let module = registry.get_module("std.sqlite").unwrap();
        let open_fn = module.exports.get("open").unwrap();
        let execute_fn = module.exports.get("execute").unwrap();
        let query_fn = module.exports.get("query").unwrap();
        let close_fn = module.exports.get("close").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        // Open in-memory sqlite connection
        let handle = open_fn
            .call(&mut ctx, vec![RuntimeValue::Str(":memory:".to_string())])
            .unwrap();
        assert!(handle.as_int().is_some());

        // Create table
        let res = execute_fn.call(
            &mut ctx,
            vec![
                handle.clone(),
                RuntimeValue::Str(
                    "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)".to_string(),
                ),
            ],
        );
        assert!(res.is_ok());

        // Insert row with parameters
        let res = execute_fn.call(
            &mut ctx,
            vec![
                handle.clone(),
                RuntimeValue::Str("INSERT INTO users (id, name) VALUES (?, ?)".to_string()),
                RuntimeValue::List {
                    items: Rc::new(RefCell::new(vec![
                        RuntimeValue::Int(1),
                        RuntimeValue::Str("Alice".to_string()),
                    ])),
                    is_const: false,
                },
            ],
        );
        assert!(res.is_ok());

        // Query row
        let query_res = query_fn
            .call(
                &mut ctx,
                vec![
                    handle.clone(),
                    RuntimeValue::Str("SELECT id, name FROM users WHERE id = ?".to_string()),
                    RuntimeValue::List {
                        items: Rc::new(RefCell::new(vec![RuntimeValue::Int(1)])),
                        is_const: false,
                    },
                ],
            )
            .unwrap();

        if let RuntimeValue::List { items, .. } = query_res {
            let list = items.borrow();
            assert_eq!(list.len(), 1);
            if let RuntimeValue::Map { entries, .. } = &list[0] {
                let map = entries.borrow();
                assert_eq!(map.get("id"), Some(&RuntimeValue::Int(1)));
                assert_eq!(
                    map.get("name"),
                    Some(&RuntimeValue::Str("Alice".to_string()))
                );
            } else {
                panic!("Expected row map");
            }
        } else {
            panic!("Expected list result");
        }

        // Close connection
        let close_res = close_fn.call(&mut ctx, vec![handle]);
        assert!(close_res.is_ok());
    }
}
