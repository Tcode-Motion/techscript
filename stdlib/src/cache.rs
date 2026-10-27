use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

thread_local! {
    static CACHE: RefCell<IndexMap<String, (RuntimeValue, Instant, u64)>> = RefCell::new(IndexMap::new());
}

impl StdlibRegistry {
    pub fn register_cache(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "set".to_string(),
            Rc::new(StdFunction {
                name: "set".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let key = args[0].to_string();
                    let val = args[1].clone();
                    CACHE.with(|c| c.borrow_mut().insert(key, (val, Instant::now(), 0)));
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "set_ttl".to_string(),
            Rc::new(StdFunction {
                name: "set_ttl".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let key = args[0].to_string();
                    let val = args[1].clone();
                    let ttl = args[2].try_into_int().unwrap_or(0) as u64;
                    CACHE.with(|c| c.borrow_mut().insert(key, (val, Instant::now(), ttl)));
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "get".to_string(),
            Rc::new(StdFunction {
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
            }),
        );

        exports.insert(
            "remove".to_string(),
            Rc::new(StdFunction {
                name: "remove".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let key = args[0].to_string();
                    CACHE.with(|c| {
                        c.borrow_mut().shift_remove(&key);
                    });
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "clear".to_string(),
            Rc::new(StdFunction {
                name: "clear".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    CACHE.with(|c| c.borrow_mut().clear());
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.cache",
            StdlibModule {
                name: "std.cache".to_string(),
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
    fn test_register_cache() {
        let mut registry = StdlibRegistry::default();
        registry.register_cache();

        let module = registry
            .modules
            .get("std.cache")
            .expect("Module std.cache should be registered");
        assert_eq!(module.name, "std.cache");
        assert_eq!(module.version, "1.0.0");
        assert!(module.exports.contains_key("set"));
        assert!(module.exports.contains_key("set_ttl"));
        assert!(module.exports.contains_key("get"));
        assert!(module.exports.contains_key("remove"));
        assert!(module.exports.contains_key("clear"));
    }

    #[test]
    fn test_cache_set_and_get() {
        let mut registry = StdlibRegistry::default();
        registry.register_cache();

        let module = registry.modules.get("std.cache").unwrap();
        let set_fn = module.exports.get("set").unwrap();
        let get_fn = module.exports.get("get").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        // Test non-existent key
        let res = get_fn
            .call(&mut ctx, vec![RuntimeValue::Str("missing".to_string())])
            .unwrap();
        assert_eq!(res, RuntimeValue::Null);

        // Test setting a value
        let set_res = set_fn
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Str("k1".to_string()),
                    RuntimeValue::Str("v1".to_string()),
                ],
            )
            .unwrap();
        assert_eq!(set_res, RuntimeValue::Null);

        // Test getting the set value
        let get_res = get_fn
            .call(&mut ctx, vec![RuntimeValue::Str("k1".to_string())])
            .unwrap();
        assert_eq!(get_res, RuntimeValue::Str("v1".to_string()));
    }

    #[test]
    fn test_cache_set_ttl() {
        let mut registry = StdlibRegistry::default();
        registry.register_cache();

        let module = registry.modules.get("std.cache").unwrap();
        let set_ttl_fn = module.exports.get("set_ttl").unwrap();
        let get_fn = module.exports.get("get").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        // Set key with TTL of 10 seconds
        set_ttl_fn
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Str("ttl_key".to_string()),
                    RuntimeValue::Int(100),
                    RuntimeValue::Int(10),
                ],
            )
            .unwrap();

        let val = get_fn
            .call(&mut ctx, vec![RuntimeValue::Str("ttl_key".to_string())])
            .unwrap();
        assert_eq!(val, RuntimeValue::Int(100));

        // Set key with TTL of 1 second and sleep 2.1 seconds to test expiration
        set_ttl_fn
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Str("exp_key".to_string()),
                    RuntimeValue::Str("exp_val".to_string()),
                    RuntimeValue::Int(1),
                ],
            )
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(2100));

        let expired_val = get_fn
            .call(&mut ctx, vec![RuntimeValue::Str("exp_key".to_string())])
            .unwrap();
        assert_eq!(expired_val, RuntimeValue::Null);
    }

    #[test]
    fn test_cache_remove_and_clear() {
        let mut registry = StdlibRegistry::default();
        registry.register_cache();

        let module = registry.modules.get("std.cache").unwrap();
        let set_fn = module.exports.get("set").unwrap();
        let get_fn = module.exports.get("get").unwrap();
        let remove_fn = module.exports.get("remove").unwrap();
        let clear_fn = module.exports.get("clear").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        set_fn
            .call(
                &mut ctx,
                vec![RuntimeValue::Str("a".to_string()), RuntimeValue::Int(1)],
            )
            .unwrap();
        set_fn
            .call(
                &mut ctx,
                vec![RuntimeValue::Str("b".to_string()), RuntimeValue::Int(2)],
            )
            .unwrap();

        // Remove key "a"
        remove_fn
            .call(&mut ctx, vec![RuntimeValue::Str("a".to_string())])
            .unwrap();

        assert_eq!(
            get_fn
                .call(&mut ctx, vec![RuntimeValue::Str("a".to_string())])
                .unwrap(),
            RuntimeValue::Null
        );
        assert_eq!(
            get_fn
                .call(&mut ctx, vec![RuntimeValue::Str("b".to_string())])
                .unwrap(),
            RuntimeValue::Int(2)
        );

        // Clear all keys
        clear_fn.call(&mut ctx, vec![]).unwrap();

        assert_eq!(
            get_fn
                .call(&mut ctx, vec![RuntimeValue::Str("b".to_string())])
                .unwrap(),
            RuntimeValue::Null
        );
    }
}
