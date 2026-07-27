use crate::{async_runtime, StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue, RuntimeContext};

impl StdlibRegistry {
    pub fn register_async(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "spawn_async".to_string(),
            Rc::new(StdFunction {
                name: "spawn_async".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let callback = args[0].clone();
                    let mut fut_map = IndexMap::new();
                    fut_map.insert(
                        "state".to_string(),
                        RuntimeValue::Str("pending".to_string()),
                    );
                    fut_map.insert("value".to_string(), RuntimeValue::Null);
                    let future = RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(fut_map)),
                        is_const: false,
                    };

                    let fut_clone = future.clone();
                    if let RuntimeValue::Function(func) = callback {
                        let func_ptr = Box::into_raw(Box::new(func)) as usize;
                        async_runtime::spawn_task(fut_clone, move || {
                            let func = unsafe {
                                Box::from_raw(
                                    func_ptr as *mut Rc<dyn techscript_runtime::function::Callable>,
                                )
                            };
                            let mut ctx =
                                RuntimeContext::new(techscript_runtime::RuntimeConfig::default());
                            func.call(&mut ctx, vec![]).map_err(|e| format!("{:?}", e))
                        });
                    }
                    Ok(future)
                },
            }),
        );

        self.register_module(
            "std.async",
            StdlibModule {
                name: "std.async".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    pub fn register_future(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "make_future".to_string(),
            Rc::new(StdFunction {
                name: "make_future".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let mut fut_map = IndexMap::new();
                    fut_map.insert(
                        "state".to_string(),
                        RuntimeValue::Str("pending".to_string()),
                    );
                    fut_map.insert("value".to_string(), RuntimeValue::Null);
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(fut_map)),
                        is_const: false,
                    })
                },
            }),
        );

        self.register_module(
            "std.future",
            StdlibModule {
                name: "std.future".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    pub fn register_channel(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "make_channel".to_string(),
            Rc::new(StdFunction {
                name: "make_channel".to_string(),
                arity: 0,
                callback: |ctx, _args| {
                    let (tx, rx) = std::sync::mpsc::channel::<RuntimeValue>();
                    let tx_id = ctx.resources.borrow_mut().insert(tx);
                    let rx_id = ctx.resources.borrow_mut().insert(rx);
                    let mut map = IndexMap::new();
                    map.insert("_tx_handle".to_string(), RuntimeValue::Int(tx_id as i64));
                    map.insert("_rx_handle".to_string(), RuntimeValue::Int(rx_id as i64));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "send_channel".to_string(),
            Rc::new(StdFunction {
                name: "send_channel".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let handle_id = entries
                            .borrow()
                            .get("_tx_handle")
                            .cloned()
                            .unwrap_or(RuntimeValue::Null)
                            .try_into_int()? as u32;
                        let resources = ctx.resources.borrow();
                        if let Some(tx) =
                            resources.get::<std::sync::mpsc::Sender<RuntimeValue>>(handle_id)
                        {
                            tx.send(args[1].clone()).ok();
                        }
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "recv_channel".to_string(),
            Rc::new(StdFunction {
                name: "recv_channel".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let handle_id = entries
                            .borrow()
                            .get("_rx_handle")
                            .cloned()
                            .unwrap_or(RuntimeValue::Null)
                            .try_into_int()? as u32;
                        let resources = ctx.resources.borrow();
                        if let Some(rx) =
                            resources.get::<std::sync::mpsc::Receiver<RuntimeValue>>(handle_id)
                        {
                            if let Ok(val) = rx.recv() {
                                return Ok(val);
                            }
                        }
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.channel",
            StdlibModule {
                name: "std.channel".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
