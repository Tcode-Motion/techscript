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

impl StdlibRegistry {
    pub fn register_sqlite(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "open".to_string(),
            Rc::new(StdFunction {
                name: "open".to_string(),
                arity: 1,
                callback: |_ctx, args| {
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
                },
            }),
        );

        exports.insert(
            "execute".to_string(),
            Rc::new(StdFunction {
                name: "execute".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let id = args[0].try_into_int().map_err(|_| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation("expected int handle".to_string()),
                            None,
                            None,
                        )
                    })?;
                    let sql = args[1].to_string();

                    CONNECTIONS.with(|m| {
                        let mut map = m.borrow_mut();
                        if let Some(conn) = map.get_mut(&id) {
                            conn.execute(&sql, []).map_err(|e| {
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
                },
            }),
        );

        exports.insert(
            "query".to_string(),
            Rc::new(StdFunction {
                name: "query".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let id = args[0].try_into_int().map_err(|_| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation("expected int handle".to_string()),
                            None,
                            None,
                        )
                    })?;
                    let sql = args[1].to_string();

                    let rows = CONNECTIONS.with(|m| {
                        let mut map = m.borrow_mut();
                        if let Some(conn) = map.get_mut(&id) {
                            let mut stmt = conn.prepare(&sql).map_err(|e| {
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
                            let row_iter = stmt
                                .query_map([], |row| {
                                    let mut map = IndexMap::new();
                                    for i in 0..col_count {
                                        let name = col_names[i].clone();
                                        let val: String = row.get::<_, String>(i).unwrap_or_default();
                                        map.insert(name, RuntimeValue::Str(val));
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
                },
            }),
        );

        exports.insert(
            "close".to_string(),
            Rc::new(StdFunction {
                name: "close".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let id = args[0].try_into_int().map_err(|_| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation("expected int handle".to_string()),
                            None,
                            None,
                        )
                    })?;
                    CONNECTIONS.with(|m| m.borrow_mut().remove(&id));
                    Ok(RuntimeValue::Null)
                },
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
