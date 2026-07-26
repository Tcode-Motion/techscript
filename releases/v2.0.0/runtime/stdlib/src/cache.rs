use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;
use indexmap::IndexMap;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

thread_local! {
    static CACHE: RefCell<IndexMap<String, (RuntimeValue, Instant, u64)>> = RefCell::new(IndexMap::new());
}

impl StdlibRegistry {
    pub fn register_cache(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("set".to_string(), Rc::new(StdFunction {
            name: "set".to_string(),
            arity: 2,
            callback: |_ctx, args| {
                let key = args[0].to_string();
                let val = args[1].clone();
                CACHE.with(|c| c.borrow_mut().insert(key, (val, Instant::now(), 0)));
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("set_ttl".to_string(), Rc::new(StdFunction {
            name: "set_ttl".to_string(),
            arity: 3,
            callback: |_ctx, args| {
                let key = args[0].to_string();
                let val = args[1].clone();
                let ttl = args[2].try_into_int().unwrap_or(0) as u64;
                CACHE.with(|c| c.borrow_mut().insert(key, (val, Instant::now(), ttl)));
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("get".to_string(), Rc::new(StdFunction {
            name: "get".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let key = args[0].to_string();
                let result = CACHE.with(|c| {
                    let mut cache = c.borrow_mut();
                    if let Some((val, time, ttl)) = cache.get(&key) {
                        if *ttl > 0 && time.elapsed().as_secs() > *ttl {
                            cache.shift_remove(&key);
                            return RuntimeValue::Null;
                        }
                        val.clone()
                    } else {
                        RuntimeValue::Null
                    }
                });
                Ok(result)
            },
        }));

        exports.insert("remove".to_string(), Rc::new(StdFunction {
            name: "remove".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let key = args[0].to_string();
                CACHE.with(|c| { c.borrow_mut().shift_remove(&key); });
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("clear".to_string(), Rc::new(StdFunction {
            name: "clear".to_string(),
            arity: 0,
            callback: |_ctx, _args| {
                CACHE.with(|c| c.borrow_mut().clear());
                Ok(RuntimeValue::Null)
            },
        }));

        self.register_module("std.cache", StdlibModule {
            name: "std.cache".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
