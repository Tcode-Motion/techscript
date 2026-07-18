use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::collections::VecDeque;
use techscript_runtime::{
    context::Capability,
    error::{RuntimeError, RuntimeErrorKind},
    function::Callable,
    value::RuntimeValue,
    RuntimeContext,
};

pub mod math;
pub mod strings;
pub mod collections;
pub mod json;
pub mod io;
pub mod sys;
pub mod net;
pub mod http;
pub mod xml;
pub mod csv;
pub mod yaml;
pub mod datetime;
pub mod crypto;
pub mod hash;
pub mod random;
pub mod regex;
pub mod path;
pub mod thread;
pub mod sync;
pub mod async_mod;
pub mod testing;
pub mod logging;
pub mod compress;
pub mod encoding;
pub mod uuid;
pub mod url;
pub mod system;
pub mod toml;
pub mod database;
pub mod graphics;
pub mod ai;

/// Type definition for module function callbacks.
pub type StdFnCallback =
    fn(ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError>;

/// Helper representing a native standard library function.
#[derive(Clone)]
pub struct StdFunction {
    pub name: String,
    pub arity: usize,
    pub callback: StdFnCallback,
}

impl Callable for StdFunction {
    fn name(&self) -> &str {
        &self.name
    }
    fn arity(&self) -> usize {
        self.arity
    }
    fn call(
        &self,
        ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.len() < self.arity {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: self.arity,
                    found: args.len(),
                },
                None,
                None,
            ));
        }
        (self.callback)(ctx, args)
    }
}

/// Custom mock function implementing Callable.
#[derive(Clone)]
pub struct MockFunction {
    pub name: String,
    pub val: RuntimeValue,
}

impl Callable for MockFunction {
    fn name(&self) -> &str {
        &self.name
    }
    fn arity(&self) -> usize {
        0
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        _args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        Ok(self.val.clone())
    }
}

/// Metadata-rich standard library module.
#[derive(Clone)]
pub struct StdlibModule {
    pub name: String,
    pub version: String,
    pub exports: HashMap<String, Rc<dyn Callable>>,
    pub required_capabilities: Vec<Capability>,
}

/// Cooperative async scheduler queue.
pub struct AsyncTask {
    pub id: usize,
    pub future: RuntimeValue,
    pub callback: Box<dyn FnOnce() -> Result<RuntimeValue, String> + 'static>,
}

pub struct Scheduler {
    next_id: usize,
    pub tasks: VecDeque<AsyncTask>,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            next_id: 1,
            tasks: VecDeque::new(),
        }
    }
}

pub mod async_runtime {
    use super::*;

    thread_local! {
        static SCHEDULER: RefCell<Scheduler> = RefCell::new(Scheduler::new());
    }

    pub fn spawn_task<F>(future: RuntimeValue, f: F)
    where
        F: FnOnce() -> Result<RuntimeValue, String> + 'static,
    {
        SCHEDULER.with(|sched| {
            let mut s = sched.borrow_mut();
            let id = s.next_id;
            s.next_id += 1;
            s.tasks.push_back(AsyncTask {
                id,
                future,
                callback: Box::new(f),
            });
        });
    }

    pub fn tick() {
        let task_opt = SCHEDULER.with(|sched| {
            sched.borrow_mut().tasks.pop_front()
        });
        if let Some(task) = task_opt {
            let res = (task.callback)();
            if let RuntimeValue::Map { entries, .. } = &task.future {
                match res {
                    Ok(val) => {
                        let mut borrow = entries.borrow_mut();
                        borrow.insert("state".to_string(), RuntimeValue::Str("resolved".to_string()));
                        borrow.insert("value".to_string(), val);
                    }
                    Err(err) => {
                        let mut borrow = entries.borrow_mut();
                        borrow.insert("state".to_string(), RuntimeValue::Str("rejected".to_string()));
                        borrow.insert("value".to_string(), RuntimeValue::Str(err));
                    }
                }
            }
        }
    }
}

/// Standard Library Registry controller.
pub struct StdlibRegistry {
    pub modules: HashMap<String, StdlibModule>,
}

impl Default for StdlibRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl StdlibRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            modules: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    pub fn register_module(&mut self, name: &str, module: StdlibModule) {
        self.modules.insert(name.to_string(), module);
    }

    pub fn has_module(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    pub fn get_module(&self, name: &str) -> Option<&StdlibModule> {
        self.modules.get(name)
    }

    pub fn construct_std_namespace(&self) -> RuntimeValue {
        let mut std_map = IndexMap::new();
        for (name, module) in &self.modules {
            if let Some(sub_name) = name.strip_prefix("std.") {
                let mut module_map = IndexMap::new();
                for (func_name, func) in &module.exports {
                    module_map.insert(func_name.clone(), RuntimeValue::Function(Rc::clone(func)));
                }
                std_map.insert(
                    sub_name.to_string(),
                    RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(module_map)),
                        is_const: true,
                    },
                );
            }
        }
        RuntimeValue::Map {
            entries: Rc::new(RefCell::new(std_map)),
            is_const: true,
        }
    }

    fn register_defaults(&mut self) {
        self.register_math();
        self.register_strings();
        self.register_collections();
        self.register_json();
        self.register_io();
        self.register_sys();
        self.register_net();
        self.register_http();
        self.register_xml();
        self.register_csv();
        self.register_yaml();
        self.register_datetime();
        self.register_crypto();
        self.register_hash();
        self.register_random();
        self.register_regex();
        self.register_path();
        self.register_thread();
        self.register_sync();
        self.register_async();
        self.register_future();
        self.register_channel();
        self.register_testing();
        self.register_logging();
        self.register_compress();
        self.register_encoding();
        self.register_uuid();
        self.register_url();
        self.register_system();
        self.register_toml();
        self.register_database();
        self.register_graphics();
        self.register_ai();
    }
}
