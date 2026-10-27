use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Condvar, Mutex};
use techscript_runtime::{error::RuntimeError, value::RuntimeValue, RuntimeContext};

pub struct ScriptMutex {
    locked: Mutex<bool>,
    condvar: Condvar,
}

impl ScriptMutex {
    pub fn new() -> Self {
        Self {
            locked: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }

    pub fn lock(&self) {
        let mut guard = self.locked.lock().unwrap();
        while *guard {
            guard = self.condvar.wait(guard).unwrap();
        }
        *guard = true;
    }

    pub fn unlock(&self) {
        let mut guard = self.locked.lock().unwrap();
        *guard = false;
        self.condvar.notify_one();
    }
}

fn sync_make_mutex(
    ctx: &mut RuntimeContext,
    _args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let mutex = ScriptMutex::new();
    let handle_id = ctx.resources.borrow_mut().insert(mutex);
    let mut map = IndexMap::new();
    map.insert("_handle".to_string(), RuntimeValue::Int(handle_id as i64));
    Ok(RuntimeValue::Map {
        entries: Rc::new(RefCell::new(map)),
        is_const: false,
    })
}

fn sync_mutex_lock(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if let RuntimeValue::Map { entries, .. } = &args[0] {
        let handle_id = entries
            .borrow()
            .get("_handle")
            .cloned()
            .unwrap_or(RuntimeValue::Null)
            .try_into_int()? as u32;
        let resources = ctx.resources.borrow();
        if let Some(mutex) = resources.get::<ScriptMutex>(handle_id) {
            mutex.lock();
        }
    }
    Ok(RuntimeValue::Null)
}

fn sync_mutex_unlock(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if let RuntimeValue::Map { entries, .. } = &args[0] {
        let handle_id = entries
            .borrow()
            .get("_handle")
            .cloned()
            .unwrap_or(RuntimeValue::Null)
            .try_into_int()? as u32;
        let resources = ctx.resources.borrow();
        if let Some(mutex) = resources.get::<ScriptMutex>(handle_id) {
            mutex.unlock();
        }
    }
    Ok(RuntimeValue::Null)
}

impl StdlibRegistry {
    pub fn register_sync(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "make_mutex".to_string(),
            Rc::new(StdFunction {
                name: "make_mutex".to_string(),
                arity: 0,
                callback: sync_make_mutex,
            }),
        );

        exports.insert(
            "mutex_lock".to_string(),
            Rc::new(StdFunction {
                name: "mutex_lock".to_string(),
                arity: 1,
                callback: sync_mutex_lock,
            }),
        );

        exports.insert(
            "mutex_unlock".to_string(),
            Rc::new(StdFunction {
                name: "mutex_unlock".to_string(),
                arity: 1,
                callback: sync_mutex_unlock,
            }),
        );

        self.register_module(
            "std.sync",
            StdlibModule {
                name: "std.sync".to_string(),
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
    use techscript_runtime::context::RuntimeConfig;

    #[test]
    fn test_register_sync() {
        let mut registry = StdlibRegistry::new();
        registry.register_sync();

        let module = registry.get_module("std.sync").unwrap();

        let make_mutex = module.exports.get("make_mutex").unwrap();
        assert_eq!(make_mutex.name(), "make_mutex");
        assert_eq!(make_mutex.arity(), 0);

        let mutex_lock = module.exports.get("mutex_lock").unwrap();
        assert_eq!(mutex_lock.name(), "mutex_lock");
        assert_eq!(mutex_lock.arity(), 1);

        let mutex_unlock = module.exports.get("mutex_unlock").unwrap();
        assert_eq!(mutex_unlock.name(), "mutex_unlock");
        assert_eq!(mutex_unlock.arity(), 1);

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let mutex_val = make_mutex.call(&mut ctx, vec![]).unwrap();

        let lock_res = mutex_lock.call(&mut ctx, vec![mutex_val.clone()]);
        assert!(lock_res.is_ok());

        let unlock_res = mutex_unlock.call(&mut ctx, vec![mutex_val]);
        assert!(unlock_res.is_ok());
    }

    #[test]
    fn test_script_mutex_new() {
        let mutex = ScriptMutex::new();
        let guard = mutex.locked.lock().unwrap();
        assert_eq!(*guard, false);
    }

    #[test]
    fn test_script_mutex_lock_unlock() {
        use std::sync::Arc;
        use std::thread;
        use std::time::Duration;

        let mutex = Arc::new(ScriptMutex::new());

        // Lock it in the main thread
        mutex.lock();
        assert_eq!(*mutex.locked.lock().unwrap(), true);

        let mutex_clone = Arc::clone(&mutex);

        // Spawn a thread that will block on lock()
        let handle = thread::spawn(move || {
            // This will block until the main thread calls unlock()
            mutex_clone.lock();
            assert_eq!(*mutex_clone.locked.lock().unwrap(), true);
            mutex_clone.unlock();
        });

        // Give the spawned thread some time to block
        thread::sleep(Duration::from_millis(50));

        // Unlock it in the main thread, this should wake up the spawned thread
        mutex.unlock();

        // Wait for the spawned thread to finish
        handle.join().unwrap();

        // Finally, it should be unlocked
        assert_eq!(*mutex.locked.lock().unwrap(), false);
    }
}
