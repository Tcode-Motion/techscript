use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue};

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
                    let ptr = Box::into_raw(Box::new(conn)) as i64;
                    Ok(RuntimeValue::Int(ptr))
                },
            }),
        );

        exports.insert(
            "execute".to_string(),
            Rc::new(StdFunction {
                name: "execute".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let ptr = args[0].try_into_int().map_err(|_| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation("expected int handle".to_string()),
                            None,
                            None,
                        )
                    })?;
                    let sql = args[1].to_string();
                    let conn = unsafe { &mut *(ptr as *mut rusqlite::Connection) };
                    conn.execute(&sql, []).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(e.to_string()),
                            None,
                            None,
                        )
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
                    let ptr = args[0].try_into_int().map_err(|_| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation("expected int handle".to_string()),
                            None,
                            None,
                        )
                    })?;
                    let sql = args[1].to_string();
                    let conn = unsafe { &mut *(ptr as *mut rusqlite::Connection) };
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
                    let ptr = args[0].try_into_int().map_err(|_| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation("expected int handle".to_string()),
                            None,
                            None,
                        )
                    })?;
                    unsafe {
                        drop(Box::from_raw(ptr as *mut rusqlite::Connection));
                    }
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
