use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Condvar, Mutex};
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

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

impl StdlibRegistry {
    pub fn register_sync(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "make_mutex".to_string(),
            Rc::new(StdFunction {
                name: "make_mutex".to_string(),
                arity: 0,
                callback: |ctx, _args| {
                    let mutex = ScriptMutex::new();
                    let handle_id = ctx.resources.borrow_mut().insert(mutex);
                    let mut map = IndexMap::new();
                    map.insert("_handle".to_string(), RuntimeValue::Int(handle_id as i64));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "mutex_lock".to_string(),
            Rc::new(StdFunction {
                name: "mutex_lock".to_string(),
                arity: 1,
                callback: |ctx, args| {
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
                },
            }),
        );

        exports.insert(
            "mutex_unlock".to_string(),
            Rc::new(StdFunction {
                name: "mutex_unlock".to_string(),
                arity: 1,
                callback: |ctx, args| {
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
                },
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

    #[test]
    fn test_script_mutex_new() {
        let mutex = ScriptMutex::new();
        let guard = mutex.locked.lock().unwrap();
        assert_eq!(*guard, false);
    }

    #[test]
    fn test_script_mutex_unlock() {
        use std::sync::Arc;
        use std::thread;
        use std::time::Duration;

        let mutex = Arc::new(ScriptMutex::new());
        mutex.lock();

        let mutex_clone = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            // This will block until the main thread unlocks it.
            mutex_clone.lock();
            // Now unlock it so the test can finish cleanly
            mutex_clone.unlock();
        });

        // Give the spawned thread a moment to block on the mutex.
        thread::sleep(Duration::from_millis(50));

        // Unlock, which should wake the sleeping thread.
        mutex.unlock();

        // If unlock() doesn't work, this join will hang indefinitely.
        handle.join().unwrap();
    }
}
