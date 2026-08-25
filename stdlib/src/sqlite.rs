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

fn runtime_to_sql_value(v: &RuntimeValue) -> rusqlite::types::Value {
    match v {
        RuntimeValue::Null => rusqlite::types::Value::Null,
        RuntimeValue::Bool(b) => rusqlite::types::Value::Integer(if *b { 1 } else { 0 }),
        RuntimeValue::Int(i) => rusqlite::types::Value::Integer(*i),
        RuntimeValue::Float(f) => rusqlite::types::Value::Real(*f),
        RuntimeValue::Str(s) => rusqlite::types::Value::Text(s.clone()),
        _ => rusqlite::types::Value::Text(v.to_string()),
    }
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
                    let params_list = if args.len() > 2 {
                        if let RuntimeValue::List { items, .. } = &args[2] {
                            items.borrow().clone()
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    };

                    let params: Vec<rusqlite::types::Value> = if let Some(arg) = args.get(2) {
                        if let RuntimeValue::List { items, .. } = arg {
                            items.borrow().iter().map(runtime_to_sql_value).collect()
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

                            Ok(())
                        } else {
                            Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(
                                    "Invalid connection handle".to_string(),
                                ),
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
                    let params_list = if args.len() > 2 {
                        if let RuntimeValue::List { items, .. } = &args[2] {
                            items.borrow().clone()
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    };

                    let params: Vec<rusqlite::types::Value> = if let Some(arg) = args.get(2) {
                        if let RuntimeValue::List { items, .. } = arg {
                            items.borrow().iter().map(runtime_to_sql_value).collect()
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


                                    for i in 0..col_count {
                                        let val: String =
                                            row.get::<_, String>(i).unwrap_or_default();
                                        if let Some((_, v)) = map.get_index_mut(i) {
                                            *v = RuntimeValue::Str(val);
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
                                RuntimeErrorKind::InvalidOperation(
                                    "Invalid connection handle".to_string(),
                                ),
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
