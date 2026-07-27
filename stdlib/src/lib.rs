#![allow(warnings, clippy::all)]

use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use techscript_runtime::{
    context::Capability,
    error::{RuntimeError, RuntimeErrorKind},
    function::Callable,
    value::RuntimeValue,
    RuntimeContext,
};

pub mod ai;
pub mod async_mod;
pub mod audio;
pub mod barcode;
pub mod benchmark;
pub mod binary;
pub mod cache;
pub mod canvas;
pub mod charts;
pub mod collections;
pub mod compress;
pub mod config;
pub mod crypto;
pub mod csv;
pub mod database;
pub mod datetime;
pub mod debug;
pub mod dns;
pub mod docs;
pub mod email;
pub mod encoding;
pub mod env;
pub mod excel;
pub mod file;
pub mod ftp;
pub mod graphics;
pub mod gui;
pub mod hash;
pub mod http;
pub mod image;
pub mod ini;
pub mod io;
pub mod json;
pub mod jwt;
pub mod localization;
pub mod logging;
pub mod math;
pub mod mock;
pub mod mongodb;
pub mod mysql;
pub mod net;
pub mod notification;
pub mod oauth;
pub mod os;
pub mod parallel;
pub mod path;
pub mod pdf;
pub mod postgres;
pub mod powerpoint;
pub mod process;
pub mod profiler;
pub mod qrcode;
pub mod random;
pub mod redis;
pub mod regex;
pub mod report;
pub mod scheduler;
pub mod security;
pub mod settings;
pub mod socket;
pub mod sqlite;
pub mod strings;
pub mod svg;
pub mod sync;
pub mod sys;
pub mod system;
pub mod task;
pub mod terminal;
pub mod testing;
pub mod theme;
pub mod thread;
pub mod time;
pub mod toml;
pub mod url;
pub mod uuid;
pub mod video;
pub mod web;
pub mod websocket;
pub mod word;
pub mod xml;
pub mod yaml;

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
        let task_opt = SCHEDULER.with(|sched| sched.borrow_mut().tasks.pop_front());
        if let Some(task) = task_opt {
            let res = (task.callback)();
            if let RuntimeValue::Map { entries, .. } = &task.future {
                match res {
                    Ok(val) => {
                        let mut borrow = entries.borrow_mut();
                        borrow.insert(
                            "state".to_string(),
                            RuntimeValue::Str("resolved".to_string()),
                        );
                        borrow.insert("value".to_string(), val);
                    }
                    Err(err) => {
                        let mut borrow = entries.borrow_mut();
                        borrow.insert(
                            "state".to_string(),
                            RuntimeValue::Str("rejected".to_string()),
                        );
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
        self.register_os();
        self.register_env();
        self.register_process();
        self.register_file();
        self.register_time();
        self.register_terminal();
        self.register_socket();
        self.register_websocket();
        self.register_dns();
        self.register_email();
        self.register_ftp();
        self.register_sqlite();
        self.register_mysql();
        self.register_postgres();
        self.register_mongodb();
        self.register_redis();
        self.register_jwt();
        self.register_oauth();
        self.register_image();
        self.register_gui();
        self.register_web();
        self.register_audio();
        self.register_video();
        self.register_scheduler();
        self.register_cache();
        self.register_binary();
        self.register_ini();
        self.register_docs();
        self.register_task();
        self.register_config();
        self.register_parallel();
        self.register_benchmark();
        self.register_profiler();
        self.register_svg();
        self.register_pdf();
        self.register_barcode();
        self.register_qrcode();
        self.register_notification();
        self.register_security();
        self.register_settings();
        self.register_localization();
        self.register_theme();
        self.register_charts();
        self.register_report();
        self.register_excel();
        self.register_word();
        self.register_powerpoint();
        self.register_canvas();
        self.register_debug();
        self.register_mock();
        self.register_v1_compatibility();
    }

    /// Add v1.0.8 public module spellings without changing the 2.0 `std.*`
    /// API. Aliases share the existing callable implementations.
    fn register_v1_compatibility(&mut self) {
        if let Some(hash) = self.modules.get("std.hash").cloned() {
            if let Some(crypto) = self.modules.get_mut("std.crypto") {
                for (name, function) in hash.exports {
                    crypto.exports.entry(name).or_insert(function);
                }
            }
        }
        if let Some(json) = self.modules.get_mut("std.json") {
            if let Some(value) = json.exports.get("stringify").cloned() {
                json.exports.insert("encode".to_string(), value);
            }
            if let Some(value) = json.exports.get("parse").cloned() {
                json.exports.insert("decode".to_string(), value);
            }
        }
        if let Some(fs) = self.modules.get_mut("std.fs") {
            if let Some(value) = fs.exports.get("write_file").cloned() {
                fs.exports.insert("write".to_string(), value);
            }
            if let Some(value) = fs.exports.get("read_file").cloned() {
                fs.exports.insert("read".to_string(), value);
            }
        }
        if let Some(random) = self.modules.get_mut("std.random") {
            if let Some(value) = random.exports.get("int").cloned() {
                random.exports.insert("randint".to_string(), value);
            }
            if let Some(value) = random.exports.get("float").cloned() {
                random.exports.insert("random".to_string(), value);
            }
        }
        if let Some(system) = self.modules.get_mut("std.system") {
            if let Some(value) = system.exports.get("os").cloned() {
                system.exports.insert("name".to_string(), value);
            }
        }
        if let Some(datetime) = self.modules.get_mut("std.datetime") {
            if let Some(value) = datetime.exports.get("epoch").cloned() {
                datetime.exports.insert("unix".to_string(), value);
            }
        }
    }
}
