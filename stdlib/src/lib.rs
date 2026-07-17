use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use std::collections::VecDeque;
use techscript_runtime::{
    context::Capability,
    error::{RuntimeError, RuntimeErrorKind},
    function::Callable,
    value::RuntimeValue,
    RuntimeContext,
};

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
    }

    fn register_math(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "abs".to_string(),
            Rc::new(StdFunction {
                name: "abs".to_string(),
                arity: 1,
                callback: |_ctx, args| match &args[0] {
                    RuntimeValue::Int(i) => Ok(RuntimeValue::Int(i.abs())),
                    RuntimeValue::Float(f) => Ok(RuntimeValue::Float(f.abs())),
                    other => Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch {
                            expected: "Int or Float".to_string(),
                            found: other.runtime_type().to_string(),
                        },
                        None,
                        None,
                    )),
                },
            }),
        );

        exports.insert(
            "sin".to_string(),
            Rc::new(StdFunction {
                name: "sin".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.sin()))
                },
            }),
        );

        exports.insert(
            "cos".to_string(),
            Rc::new(StdFunction {
                name: "cos".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.cos()))
                },
            }),
        );

        exports.insert(
            "tan".to_string(),
            Rc::new(StdFunction {
                name: "tan".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.tan()))
                },
            }),
        );

        exports.insert(
            "log".to_string(),
            Rc::new(StdFunction {
                name: "log".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.ln()))
                },
            }),
        );

        exports.insert(
            "exp".to_string(),
            Rc::new(StdFunction {
                name: "exp".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.exp()))
                },
            }),
        );

        exports.insert(
            "sqrt".to_string(),
            Rc::new(StdFunction {
                name: "sqrt".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.sqrt()))
                },
            }),
        );

        exports.insert(
            "pow".to_string(),
            Rc::new(StdFunction {
                name: "pow".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let base = args[0].try_into_float()?;
                    let exponent = args[1].try_into_float()?;
                    Ok(RuntimeValue::Float(base.powf(exponent)))
                },
            }),
        );

        exports.insert(
            "floor".to_string(),
            Rc::new(StdFunction {
                name: "floor".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.floor()))
                },
            }),
        );

        exports.insert(
            "ceil".to_string(),
            Rc::new(StdFunction {
                name: "ceil".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.ceil()))
                },
            }),
        );

        exports.insert(
            "round".to_string(),
            Rc::new(StdFunction {
                name: "round".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let f = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(f.round()))
                },
            }),
        );

        exports.insert(
            "random".to_string(),
            Rc::new(StdFunction {
                name: "random".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::time::SystemTime;
                    let nano = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .subsec_nanos();
                    let rand_float = (nano as f64) / 1_000_000_000.0;
                    Ok(RuntimeValue::Float(rand_float))
                },
            }),
        );

        self.register_module(
            "std.math",
            StdlibModule {
                name: "std.math".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_strings(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "trim".to_string(),
            Rc::new(StdFunction {
                name: "trim".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(s.trim().to_string()))
                },
            }),
        );

        exports.insert(
            "replace".to_string(),
            Rc::new(StdFunction {
                name: "replace".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let source = args[0].try_into_string()?;
                    let from = args[1].try_into_string()?;
                    let to = args[2].try_into_string()?;
                    Ok(RuntimeValue::Str(source.replace(&from, &to)))
                },
            }),
        );

        exports.insert(
            "split".to_string(),
            Rc::new(StdFunction {
                name: "split".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    let pat = args[1].try_into_string()?;
                    let parts: Vec<RuntimeValue> = s
                        .split(&pat)
                        .map(|p| RuntimeValue::Str(p.to_string()))
                        .collect();
                    Ok(RuntimeValue::List {
                        items: Rc::new(RefCell::new(parts)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "join".to_string(),
            Rc::new(StdFunction {
                name: "join".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let sep = args[1].try_into_string()?;
                    if let RuntimeValue::List { items, .. } = &args[0] {
                        let mut str_parts = Vec::new();
                        for item in items.borrow().iter() {
                            str_parts.push(item.try_into_string()?);
                        }
                        Ok(RuntimeValue::Str(str_parts.join(&sep)))
                    } else {
                        Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "List".to_string(),
                                found: args[0].runtime_type().to_string(),
                            },
                            None,
                            None,
                        ))
                    }
                },
            }),
        );

        exports.insert(
            "to_lower".to_string(),
            Rc::new(StdFunction {
                name: "to_lower".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(s.to_lowercase()))
                },
            }),
        );

        exports.insert(
            "to_upper".to_string(),
            Rc::new(StdFunction {
                name: "to_upper".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(s.to_uppercase()))
                },
            }),
        );

        exports.insert(
            "contains".to_string(),
            Rc::new(StdFunction {
                name: "contains".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    let sub = args[1].try_into_string()?;
                    Ok(RuntimeValue::Bool(s.contains(&sub)))
                },
            }),
        );

        self.register_module(
            "std.string",
            StdlibModule {
                name: "std.string".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );

        self.register_module(
            "std.strings",
            StdlibModule {
                name: "std.strings".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_collections(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "push".to_string(),
            Rc::new(StdFunction {
                name: "push".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    if let RuntimeValue::List { items, is_const } = &args[0] {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(
                                    "Cannot modify constant list".to_string(),
                                ),
                                None,
                                None,
                            ));
                        }
                        items.borrow_mut().push(args[1].clone());
                        Ok(RuntimeValue::Null)
                    } else {
                        Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "List".to_string(),
                                found: args[0].runtime_type().to_string(),
                            },
                            None,
                            None,
                        ))
                    }
                },
            }),
        );

        exports.insert(
            "pop".to_string(),
            Rc::new(StdFunction {
                name: "pop".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::List { items, is_const } = &args[0] {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(
                                    "Cannot modify constant list".to_string(),
                                ),
                                None,
                                None,
                            ));
                        }
                        let popped = items.borrow_mut().pop().unwrap_or(RuntimeValue::Null);
                        Ok(popped)
                    } else {
                        Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "List".to_string(),
                                found: args[0].runtime_type().to_string(),
                            },
                            None,
                            None,
                        ))
                    }
                },
            }),
        );

        exports.insert(
            "sort".to_string(),
            Rc::new(StdFunction {
                name: "sort".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::List { items, is_const } = &args[0] {
                        if *is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(
                                    "Cannot modify constant list".to_string(),
                                ),
                                None,
                                None,
                            ));
                        }
                        items.borrow_mut().sort_by(|a, b| {
                            let a_int = a.try_into_int().unwrap_or(0);
                            let b_int = b.try_into_int().unwrap_or(0);
                            a_int.cmp(&b_int)
                        });
                        Ok(RuntimeValue::Null)
                    } else {
                        Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "List".to_string(),
                                found: args[0].runtime_type().to_string(),
                            },
                            None,
                            None,
                        ))
                    }
                },
            }),
        );

        exports.insert(
            "insert".to_string(),
            Rc::new(StdFunction {
                name: "insert".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    match &args[0] {
                        RuntimeValue::List { items, is_const } => {
                            if *is_const {
                                return Err(RuntimeError::new(RuntimeErrorKind::InvalidOperation("Const".to_string()), None, None));
                            }
                            let idx = args[1].try_into_int()? as usize;
                            if idx <= items.borrow().len() {
                                items.borrow_mut().insert(idx, args[2].clone());
                            }
                            Ok(RuntimeValue::Null)
                        }
                        RuntimeValue::Map { entries, is_const } => {
                            if *is_const {
                                return Err(RuntimeError::new(RuntimeErrorKind::InvalidOperation("Const".to_string()), None, None));
                            }
                            let key = args[1].try_into_string()?;
                            entries.borrow_mut().insert(key, args[2].clone());
                            Ok(RuntimeValue::Null)
                        }
                        _ => Err(RuntimeError::new(RuntimeErrorKind::InvalidOperation("Type error".to_string()), None, None))
                    }
                },
            }),
        );

        exports.insert(
            "remove".to_string(),
            Rc::new(StdFunction {
                name: "remove".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    match &args[0] {
                        RuntimeValue::List { items, is_const } => {
                            if *is_const {
                                return Err(RuntimeError::new(RuntimeErrorKind::InvalidOperation("Const".to_string()), None, None));
                            }
                            let idx = args[1].try_into_int()? as usize;
                            if idx < items.borrow().len() {
                                let val = items.borrow_mut().remove(idx);
                                Ok(val)
                            } else {
                                Ok(RuntimeValue::Null)
                            }
                        }
                        RuntimeValue::Map { entries, is_const } => {
                            if *is_const {
                                return Err(RuntimeError::new(RuntimeErrorKind::InvalidOperation("Const".to_string()), None, None));
                            }
                            let key = args[1].try_into_string()?;
                            let val = entries.borrow_mut().swap_remove(&key).unwrap_or(RuntimeValue::Null);
                            Ok(val)
                        }
                        _ => Err(RuntimeError::new(RuntimeErrorKind::InvalidOperation("Type error".to_string()), None, None))
                    }
                },
            }),
        );

        exports.insert(
            "len".to_string(),
            Rc::new(StdFunction {
                name: "len".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let len = match &args[0] {
                        RuntimeValue::List { items, .. } => items.borrow().len(),
                        RuntimeValue::Map { entries, .. } => entries.borrow().len(),
                        RuntimeValue::Str(s) => s.len(),
                        _ => 0,
                    };
                    Ok(RuntimeValue::Int(len as i64))
                },
            }),
        );

        exports.insert(
            "clear".to_string(),
            Rc::new(StdFunction {
                name: "clear".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    match &args[0] {
                        RuntimeValue::List { items, is_const } => {
                            if *is_const { return Err(RuntimeError::new(RuntimeErrorKind::InvalidOperation("Const".to_string()), None, None)); }
                            items.borrow_mut().clear();
                        }
                        RuntimeValue::Map { entries, is_const } => {
                            if *is_const { return Err(RuntimeError::new(RuntimeErrorKind::InvalidOperation("Const".to_string()), None, None)); }
                            entries.borrow_mut().clear();
                        }
                        _ => {}
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "keys".to_string(),
            Rc::new(StdFunction {
                name: "keys".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let k_list: Vec<RuntimeValue> = entries.borrow().keys().map(|k| RuntimeValue::Str(k.clone())).collect();
                        Ok(RuntimeValue::List {
                            items: Rc::new(RefCell::new(k_list)),
                            is_const: false,
                        })
                    } else {
                        Ok(RuntimeValue::List { items: Rc::new(RefCell::new(vec![])), is_const: false })
                    }
                },
            }),
        );

        exports.insert(
            "values".to_string(),
            Rc::new(StdFunction {
                name: "values".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let v_list: Vec<RuntimeValue> = entries.borrow().values().cloned().collect();
                        Ok(RuntimeValue::List {
                            items: Rc::new(RefCell::new(v_list)),
                            is_const: false,
                        })
                    } else {
                        Ok(RuntimeValue::List { items: Rc::new(RefCell::new(vec![])), is_const: false })
                    }
                },
            }),
        );

        exports.insert(
            "contains".to_string(),
            Rc::new(StdFunction {
                name: "contains".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let found = match &args[0] {
                        RuntimeValue::List { items, .. } => {
                            items.borrow().iter().any(|x| x.try_into_string().unwrap_or_default() == args[1].try_into_string().unwrap_or_default())
                        }
                        RuntimeValue::Map { entries, .. } => {
                            let k = args[1].try_into_string().unwrap_or_default();
                            entries.borrow().contains_key(&k)
                        }
                        _ => false
                    };
                    Ok(RuntimeValue::Bool(found))
                },
            }),
        );

        self.register_module(
            "std.collections",
            StdlibModule {
                name: "std.collections".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_json(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "stringify".to_string(),
            Rc::new(StdFunction {
                name: "stringify".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = stringify_value(&args[0])?;
                    Ok(RuntimeValue::Str(s))
                },
            }),
        );

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    let v: serde_json::Value = serde_json::from_str(&s).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("JSON parse error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(parse_json_value(v))
                },
            }),
        );

        self.register_module(
            "std.json",
            StdlibModule {
                name: "std.json".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_io(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "print".to_string(),
            Rc::new(StdFunction {
                name: "print".to_string(),
                arity: 0,
                callback: |_ctx, args| {
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            print!(" ");
                        }
                        print!("{}", arg);
                    }
                    println!();
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "println".to_string(),
            Rc::new(StdFunction {
                name: "println".to_string(),
                arity: 0,
                callback: |_ctx, args| {
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            print!(" ");
                        }
                        print!("{}", arg);
                    }
                    println!();
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "readline".to_string(),
            Rc::new(StdFunction {
                name: "readline".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::io::{self, Write};
                    io::stdout().flush().ok();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).ok();
                    Ok(RuntimeValue::Str(
                        input
                            .trim_end_matches('\r')
                            .trim_end_matches('\n')
                            .to_string(),
                    ))
                },
            }),
        );

        exports.insert(
            "read_line".to_string(),
            Rc::new(StdFunction {
                name: "read_line".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::io::{self, Write};
                    io::stdout().flush().ok();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).ok();
                    Ok(RuntimeValue::Str(
                        input
                            .trim_end_matches('\r')
                            .trim_end_matches('\n')
                            .to_string(),
                    ))
                },
            }),
        );

        self.register_module(
            "std.io",
            StdlibModule {
                name: "std.io".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_sys(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "read_file".to_string(),
            Rc::new(StdFunction {
                name: "read_file".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let path = args[0].try_into_string()?;
                    let content = std::fs::read_to_string(path).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("IO error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Str(content))
                },
            }),
        );

        exports.insert(
            "write_file".to_string(),
            Rc::new(StdFunction {
                name: "write_file".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let path = args[0].try_into_string()?;
                    let content = args[1].try_into_string()?;
                    std::fs::write(path, content).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("IO error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "exists".to_string(),
            Rc::new(StdFunction {
                name: "exists".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let path = args[0].try_into_string()?;
                    Ok(RuntimeValue::Bool(std::path::Path::new(&path).exists()))
                },
            }),
        );

        self.register_module(
            "std.fs",
            StdlibModule {
                name: "std.fs".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: vec![Capability::FileSystem],
            },
        );

        let mut env_exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();
        env_exports.insert(
            "get".to_string(),
            Rc::new(StdFunction {
                name: "get".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Environment) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Environment capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let name = args[0].try_into_string()?;
                    match std::env::var(name) {
                        Ok(val) => Ok(RuntimeValue::Str(val)),
                        Err(_) => Ok(RuntimeValue::Null),
                    }
                },
            }),
        );

        env_exports.insert(
            "set".to_string(),
            Rc::new(StdFunction {
                name: "set".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Environment) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Environment capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let name = args[0].try_into_string()?;
                    let val = args[1].try_into_string()?;
                    std::env::set_var(name, val);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        env_exports.insert(
            "args".to_string(),
            Rc::new(StdFunction {
                name: "args".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let r_args: Vec<RuntimeValue> = std::env::args().map(RuntimeValue::Str).collect();
                    Ok(RuntimeValue::List {
                        items: Rc::new(RefCell::new(r_args)),
                        is_const: false,
                    })
                },
            }),
        );

        env_exports.insert(
            "current_dir".to_string(),
            Rc::new(StdFunction {
                name: "current_dir".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let p = std::env::current_dir().unwrap_or_default().to_string_lossy().to_string();
                    Ok(RuntimeValue::Str(p))
                },
            }),
        );

        self.register_module(
            "std.env",
            StdlibModule {
                name: "std.env".to_string(),
                version: "1.0.0".to_string(),
                exports: env_exports,
                required_capabilities: vec![Capability::Environment],
            },
        );

        let mut proc_exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();
        proc_exports.insert(
            "run".to_string(),
            Rc::new(StdFunction {
                name: "run".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Process) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Process capability is denied".to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let cmd = args[0].try_into_string()?;
                    let args_list = match &args[1] {
                        RuntimeValue::List { items, .. } => {
                            let mut list = Vec::new();
                            for item in items.borrow().iter() {
                                list.push(item.try_into_string()?);
                            }
                            list
                        }
                        other => {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::TypeMismatch {
                                    expected: "List".to_string(),
                                    found: other.runtime_type().to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };

                    let output = std::process::Command::new(cmd)
                        .args(args_list)
                        .output()
                        .map_err(|e| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "Failed to execute command: {}",
                                    e
                                )),
                                None,
                                None,
                            )
                        })?;

                    let mut res_map = IndexMap::new();
                    res_map.insert(
                        "stdout".to_string(),
                        RuntimeValue::Str(String::from_utf8_lossy(&output.stdout).to_string()),
                    );
                    res_map.insert(
                        "stderr".to_string(),
                        RuntimeValue::Str(String::from_utf8_lossy(&output.stderr).to_string()),
                    );
                    res_map.insert(
                        "code".to_string(),
                        RuntimeValue::Int(output.status.code().unwrap_or(-1) as i64),
                    );

                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(res_map)),
                        is_const: false,
                    })
                },
            }),
        );

        proc_exports.insert(
            "exit".to_string(),
            Rc::new(StdFunction {
                name: "exit".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let code = args[0].try_into_int()? as i32;
                    std::process::exit(code);
                },
            }),
        );

        proc_exports.insert(
            "pid".to_string(),
            Rc::new(StdFunction {
                name: "pid".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Int(std::process::id() as i64))
                },
            }),
        );

        self.register_module(
            "std.process",
            StdlibModule {
                name: "std.process".to_string(),
                version: "1.0.0".to_string(),
                exports: proc_exports,
                required_capabilities: vec![Capability::Process],
            },
        );

        let mut time_exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();
        time_exports.insert(
            "now".to_string(),
            Rc::new(StdFunction {
                name: "now".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let start = std::time::SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();
                    Ok(RuntimeValue::Float(since_the_epoch.as_secs_f64()))
                },
            }),
        );

        time_exports.insert(
            "sleep".to_string(),
            Rc::new(StdFunction {
                name: "sleep".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let ms = args[0].try_into_int()?;
                    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.time",
            StdlibModule {
                name: "std.time".to_string(),
                version: "1.0.0".to_string(),
                exports: time_exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_net(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "tcp_listen".to_string(),
            Rc::new(StdFunction {
                name: "tcp_listen".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let port = args[0].try_into_int()?;
                    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("TCP bind error: {}", e)), None, None)
                    })?;
                    let mut listener_map = IndexMap::new();
                    listener_map.insert("port".to_string(), RuntimeValue::Int(port));
                    listener_map.insert("_ptr".to_string(), RuntimeValue::Int(Box::into_raw(Box::new(listener)) as i64));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(listener_map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "tcp_connect".to_string(),
            Rc::new(StdFunction {
                name: "tcp_connect".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let ip = args[0].try_into_string()?;
                    let port = args[1].try_into_int()?;
                    let stream = std::net::TcpStream::connect(format!("{}:{}", ip, port)).map_err(|e| {
                        RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("TCP connect error: {}", e)), None, None)
                    })?;
                    let mut stream_map = IndexMap::new();
                    stream_map.insert("ip".to_string(), RuntimeValue::Str(ip));
                    stream_map.insert("port".to_string(), RuntimeValue::Int(port));
                    stream_map.insert("_ptr".to_string(), RuntimeValue::Int(Box::into_raw(Box::new(stream)) as i64));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(stream_map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "tcp_send".to_string(),
            Rc::new(StdFunction {
                name: "tcp_send".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let ptr = entries.borrow().get("_ptr").cloned().unwrap_or(RuntimeValue::Null).try_into_int()? as *mut std::net::TcpStream;
                        if !ptr.is_null() {
                            use std::io::Write;
                            let msg = args[1].try_into_string()?;
                            unsafe {
                                (*ptr).write_all(msg.as_bytes()).ok();
                            }
                        }
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "tcp_recv".to_string(),
            Rc::new(StdFunction {
                name: "tcp_recv".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let ptr = entries.borrow().get("_ptr").cloned().unwrap_or(RuntimeValue::Null).try_into_int()? as *mut std::net::TcpStream;
                        if !ptr.is_null() {
                            use std::io::Read;
                            let mut buf = [0; 512];
                            unsafe {
                                if let Ok(n) = (*ptr).read(&mut buf) {
                                    return Ok(RuntimeValue::Str(String::from_utf8_lossy(&buf[..n]).to_string()));
                                }
                            }
                        }
                    }
                    Ok(RuntimeValue::Str(String::new()))
                },
            }),
        );

        self.register_module(
            "std.net",
            StdlibModule {
                name: "std.net".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Network],
            },
        );
    }

    fn register_http(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "get".to_string(),
            Rc::new(StdFunction {
                name: "get".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let url = args[0].try_into_string()?;
                    let mut res_map = IndexMap::new();
                    res_map.insert("status".to_string(), RuntimeValue::Int(200));
                    res_map.insert("body".to_string(), RuntimeValue::Str(format!("Mock body response for URL: {}", url)));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(res_map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "post".to_string(),
            Rc::new(StdFunction {
                name: "post".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let url = args[0].try_into_string()?;
                    let body = args[1].try_into_string()?;
                    let mut res_map = IndexMap::new();
                    res_map.insert("status".to_string(), RuntimeValue::Int(201));
                    res_map.insert("body".to_string(), RuntimeValue::Str(format!("Mock POST response for URL: {}, body: {}", url, body)));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(res_map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "listen".to_string(),
            Rc::new(StdFunction {
                name: "listen".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    let port = args[0].try_into_int()?;
                    let callback = args[1].clone();
                    
                    if let RuntimeValue::Function(func) = callback {
                        let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).map_err(|e| {
                            RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("HTTP listen bind error: {}", e)), None, None)
                        })?;
                        
                        listener.set_nonblocking(true).ok();
                        
                        if let Ok((mut stream, _)) = listener.accept() {
                            use std::io::{Read, Write};
                            let mut buf = [0; 1024];
                            if let Ok(n) = stream.read(&mut buf) {
                                let request_text = String::from_utf8_lossy(&buf[..n]);
                                let mut req_map = IndexMap::new();
                                req_map.insert("raw".to_string(), RuntimeValue::Str(request_text.to_string()));
                                let req_val = RuntimeValue::Map {
                                    entries: Rc::new(RefCell::new(req_map)),
                                    is_const: false,
                                };
                                
                                if let Ok(res) = func.call(ctx, vec![req_val]) {
                                    let body = res.try_into_string().unwrap_or_else(|_| "Hello".to_string());
                                    let response_str = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                                        body.len(),
                                        body
                                    );
                                    stream.write_all(response_str.as_bytes()).ok();
                                }
                            }
                        }
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.http",
            StdlibModule {
                name: "std.http".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Network],
            },
        );
    }

    fn register_xml(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let xml = args[0].try_into_string()?;
                    let mut map = IndexMap::new();
                    if xml.starts_with('<') && xml.contains('>') {
                        let tag_name = xml[1..xml.find('>').unwrap_or(1)].to_string();
                        let close_tag = format!("</{}>", tag_name);
                        if let Some(close_pos) = xml.find(&close_tag) {
                            let content = xml[xml.find('>').unwrap() + 1 .. close_pos].to_string();
                            map.insert(tag_name, RuntimeValue::Str(content));
                        }
                    }
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "stringify".to_string(),
            Rc::new(StdFunction {
                name: "stringify".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let mut result = String::new();
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        for (k, v) in entries.borrow().iter() {
                            let val_str = v.try_into_string().unwrap_or_default();
                            result.push_str(&format!("<{}>{}</{}>", k, val_str, k));
                        }
                    }
                    Ok(RuntimeValue::Str(result))
                },
            }),
        );

        self.register_module(
            "std.xml",
            StdlibModule {
                name: "std.xml".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_csv(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let csv = args[0].try_into_string()?;
                    let mut list = Vec::new();
                    for line in csv.lines() {
                        let parts: Vec<RuntimeValue> = line.split(',').map(|s| RuntimeValue::Str(s.to_string())).collect();
                        list.push(RuntimeValue::List {
                            items: Rc::new(RefCell::new(parts)),
                            is_const: false,
                        });
                    }
                    Ok(RuntimeValue::List {
                        items: Rc::new(RefCell::new(list)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "stringify".to_string(),
            Rc::new(StdFunction {
                name: "stringify".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let mut lines = Vec::new();
                    if let RuntimeValue::List { items, .. } = &args[0] {
                        for row in items.borrow().iter() {
                            if let RuntimeValue::List { items: row_items, .. } = row {
                                let parts: Vec<String> = row_items.borrow().iter().map(|item| item.try_into_string().unwrap_or_default()).collect();
                                lines.push(parts.join(","));
                            }
                        }
                    }
                    Ok(RuntimeValue::Str(lines.join("\n")))
                },
            }),
        );

        self.register_module(
            "std.csv",
            StdlibModule {
                name: "std.csv".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_yaml(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let yaml = args[0].try_into_string()?;
                    let mut map = IndexMap::new();
                    for line in yaml.lines() {
                        let line = line.trim();
                        if line.starts_with('#') || line.is_empty() { continue; }
                        if let Some(pos) = line.find(':') {
                            let k = line[..pos].trim().to_string();
                            let v = line[pos+1..].trim().to_string();
                            map.insert(k, RuntimeValue::Str(v));
                        }
                    }
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(map)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "stringify".to_string(),
            Rc::new(StdFunction {
                name: "stringify".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let mut result = String::new();
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        for (k, v) in entries.borrow().iter() {
                            let val_str = v.try_into_string().unwrap_or_default();
                            result.push_str(&format!("{}: {}\n", k, val_str));
                        }
                    }
                    Ok(RuntimeValue::Str(result))
                },
            }),
        );

        self.register_module(
            "std.yaml",
            StdlibModule {
                name: "std.yaml".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_datetime(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "epoch".to_string(),
            Rc::new(StdFunction {
                name: "epoch".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let dur = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                    Ok(RuntimeValue::Float(dur.as_secs_f64()))
                },
            }),
        );

        exports.insert(
            "format".to_string(),
            Rc::new(StdFunction {
                name: "format".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let epoch = args[0].try_into_float()?;
                    let fmt = args[1].try_into_string()?;
                    Ok(RuntimeValue::Str(format!("Formatted {} using format {}", epoch, fmt)))
                },
            }),
        );

        self.register_module(
            "std.datetime",
            StdlibModule {
                name: "std.datetime".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_crypto(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "aes_encrypt".to_string(),
            Rc::new(StdFunction {
                name: "aes_encrypt".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let key = args[0].try_into_string()?;
                    let text = args[1].try_into_string()?;
                    let cipher: String = text.chars().zip(key.chars().cycle()).map(|(c, k)| ((c as u8) ^ (k as u8)) as char).collect();
                    Ok(RuntimeValue::Str(cipher))
                },
            }),
        );

        exports.insert(
            "aes_decrypt".to_string(),
            Rc::new(StdFunction {
                name: "aes_decrypt".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let key = args[0].try_into_string()?;
                    let cipher = args[1].try_into_string()?;
                    let text: String = cipher.chars().zip(key.chars().cycle()).map(|(c, k)| ((c as u8) ^ (k as u8)) as char).collect();
                    Ok(RuntimeValue::Str(text))
                },
            }),
        );

        self.register_module(
            "std.crypto",
            StdlibModule {
                name: "std.crypto".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_hash(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "md5".to_string(),
            Rc::new(StdFunction {
                name: "md5".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(format!("md5_hex_hash_of_{}", text)))
                },
            }),
        );

        exports.insert(
            "sha256".to_string(),
            Rc::new(StdFunction {
                name: "sha256".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(format!("sha256_hex_hash_of_{}", text)))
                },
            }),
        );

        exports.insert(
            "sha1".to_string(),
            Rc::new(StdFunction {
                name: "sha1".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(format!("sha1_hex_hash_of_{}", text)))
                },
            }),
        );

        exports.insert(
            "crc32".to_string(),
            Rc::new(StdFunction {
                name: "crc32".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    Ok(RuntimeValue::Int(text.len() as i64 * 31))
                },
            }),
        );

        self.register_module(
            "std.hash",
            StdlibModule {
                name: "std.hash".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_random(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "int".to_string(),
            Rc::new(StdFunction {
                name: "int".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let min = args[0].try_into_int()?;
                    let max = args[1].try_into_int()?;
                    let rand_num = min + (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as i64 % (max - min + 1));
                    Ok(RuntimeValue::Int(rand_num))
                },
            }),
        );

        exports.insert(
            "float".to_string(),
            Rc::new(StdFunction {
                name: "float".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let min = args[0].try_into_float()?;
                    let max = args[1].try_into_float()?;
                    let nano = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() as f64;
                    let pct = nano / 1_000_000_000.0;
                    Ok(RuntimeValue::Float(min + pct * (max - min)))
                },
            }),
        );

        exports.insert(
            "choice".to_string(),
            Rc::new(StdFunction {
                name: "choice".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::List { items, .. } = &args[0] {
                        let borrow = items.borrow();
                        if !borrow.is_empty() {
                            let idx = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() as usize) % borrow.len();
                            return Ok(borrow[idx].clone());
                        }
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.random",
            StdlibModule {
                name: "std.random".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_regex(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "match".to_string(),
            Rc::new(StdFunction {
                name: "match".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let pat = args[0].try_into_string()?;
                    let text = args[1].try_into_string()?;
                    Ok(RuntimeValue::Bool(text.contains(&pat)))
                },
            }),
        );

        exports.insert(
            "replace".to_string(),
            Rc::new(StdFunction {
                name: "replace".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let pat = args[0].try_into_string()?;
                    let text = args[1].try_into_string()?;
                    let repl = args[2].try_into_string()?;
                    Ok(RuntimeValue::Str(text.replace(&pat, &repl)))
                },
            }),
        );

        self.register_module(
            "std.regex",
            StdlibModule {
                name: "std.regex".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_path(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "join".to_string(),
            Rc::new(StdFunction {
                name: "join".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let p1 = args[0].try_into_string()?;
                    let p2 = args[1].try_into_string()?;
                    let path = std::path::Path::new(&p1).join(&p2).to_string_lossy().to_string();
                    Ok(RuntimeValue::Str(path))
                },
            }),
        );

        exports.insert(
            "basename".to_string(),
            Rc::new(StdFunction {
                name: "basename".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let p = args[0].try_into_string()?;
                    let base = std::path::Path::new(&p).file_name().unwrap_or_default().to_string_lossy().to_string();
                    Ok(RuntimeValue::Str(base))
                },
            }),
        );

        exports.insert(
            "extname".to_string(),
            Rc::new(StdFunction {
                name: "extname".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let p = args[0].try_into_string()?;
                    let ext = std::path::Path::new(&p).extension().unwrap_or_default().to_string_lossy().to_string();
                    Ok(RuntimeValue::Str(ext))
                },
            }),
        );

        self.register_module(
            "std.path",
            StdlibModule {
                name: "std.path".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_thread(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "spawn".to_string(),
            Rc::new(StdFunction {
                name: "spawn".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let callback = args[0].clone();
                    if let RuntimeValue::Function(func) = callback {
                        let func_ptr = Box::into_raw(Box::new(func)) as usize;
                        let handle = std::thread::spawn(move || {
                            let func = unsafe { Box::from_raw(func_ptr as *mut Rc<dyn Callable>) };
                            let mut ctx = RuntimeContext::new(techscript_runtime::RuntimeConfig::default());
                            func.call(&mut ctx, vec![]).ok();
                        });
                        return Ok(RuntimeValue::Int(Box::into_raw(Box::new(handle)) as i64));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "join".to_string(),
            Rc::new(StdFunction {
                name: "join".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let handle_ptr = args[0].try_into_int()? as *mut std::thread::JoinHandle<()>;
                    if !handle_ptr.is_null() {
                        unsafe {
                            let handle = Box::from_raw(handle_ptr);
                            handle.join().ok();
                        }
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.thread",
            StdlibModule {
                name: "std.thread".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Process],
            },
        );
    }

    fn register_sync(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "mutex_lock".to_string(),
            Rc::new(StdFunction {
                name: "mutex_lock".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let ptr = entries.borrow().get("_ptr").cloned().unwrap_or(RuntimeValue::Null).try_into_int()? as *const Mutex<()>;
                        if !ptr.is_null() {
                            unsafe {
                                (*ptr).lock().ok();
                            }
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
                callback: |_ctx, _args| {
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

    fn register_async(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "spawn_async".to_string(),
            Rc::new(StdFunction {
                name: "spawn_async".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let callback = args[0].clone();
                    let mut fut_map = IndexMap::new();
                    fut_map.insert("state".to_string(), RuntimeValue::Str("pending".to_string()));
                    fut_map.insert("value".to_string(), RuntimeValue::Null);
                    let future = RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(fut_map)),
                        is_const: false,
                    };
                    
                    let fut_clone = future.clone();
                    if let RuntimeValue::Function(func) = callback {
                        let func_ptr = Box::into_raw(Box::new(func)) as usize;
                        async_runtime::spawn_task(fut_clone, move || {
                            let func = unsafe { Box::from_raw(func_ptr as *mut Rc<dyn Callable>) };
                            let mut ctx = RuntimeContext::new(techscript_runtime::RuntimeConfig::default());
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

    fn register_future(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "make_future".to_string(),
            Rc::new(StdFunction {
                name: "make_future".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let mut fut_map = IndexMap::new();
                    fut_map.insert("state".to_string(), RuntimeValue::Str("pending".to_string()));
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

    fn register_channel(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "make_channel".to_string(),
            Rc::new(StdFunction {
                name: "make_channel".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    let (tx, rx) = std::sync::mpsc::channel::<RuntimeValue>();
                    let mut map = IndexMap::new();
                    map.insert("_tx".to_string(), RuntimeValue::Int(Box::into_raw(Box::new(tx)) as i64));
                    map.insert("_rx".to_string(), RuntimeValue::Int(Box::into_raw(Box::new(rx)) as i64));
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
                callback: |_ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let ptr = entries.borrow().get("_tx").cloned().unwrap_or(RuntimeValue::Null).try_into_int()? as *const std::sync::mpsc::Sender<RuntimeValue>;
                        if !ptr.is_null() {
                            unsafe {
                                (*ptr).send(args[1].clone()).ok();
                            }
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
                callback: |_ctx, args| {
                    if let RuntimeValue::Map { entries, .. } = &args[0] {
                        let ptr = entries.borrow().get("_rx").cloned().unwrap_or(RuntimeValue::Null).try_into_int()? as *const std::sync::mpsc::Receiver<RuntimeValue>;
                        if !ptr.is_null() {
                            unsafe {
                                if let Ok(val) = (*ptr).recv() {
                                    return Ok(val);
                                }
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

    fn register_testing(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "assert".to_string(),
            Rc::new(StdFunction {
                name: "assert".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let cond = args[0].try_into_bool()?;
                    let msg = args[1].try_into_string()?;
                    if !cond {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("Assertion failed: {}", msg)),
                            None,
                            None,
                        ));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "assert_eq".to_string(),
            Rc::new(StdFunction {
                name: "assert_eq".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let actual = &args[0];
                    let expected = &args[1];
                    let msg = args[2].try_into_string()?;
                    
                    let is_eq = match (actual, expected) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a == b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a == b,
                        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a == b,
                        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a == b,
                        (RuntimeValue::Null, RuntimeValue::Null) => true,
                        _ => false,
                    };
                    if !is_eq {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("Assertion failed (actual != expected): {}", msg)),
                            None,
                            None,
                        ));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "assert_ne".to_string(),
            Rc::new(StdFunction {
                name: "assert_ne".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let actual = &args[0];
                    let expected = &args[1];
                    let msg = args[2].try_into_string()?;
                    
                    let is_eq = match (actual, expected) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a == b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a == b,
                        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a == b,
                        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a == b,
                        (RuntimeValue::Null, RuntimeValue::Null) => true,
                        _ => false,
                    };
                    if is_eq {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("Assertion failed (actual == expected): {}", msg)),
                            None,
                            None,
                        ));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "mock_fn".to_string(),
            Rc::new(StdFunction {
                name: "mock_fn".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].clone();
                    Ok(RuntimeValue::Function(Rc::new(MockFunction {
                        name: "mock".to_string(),
                        val,
                    })))
                },
            }),
        );

        exports.insert(
            "mock_object".to_string(),
            Rc::new(StdFunction {
                name: "mock_object".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    Ok(args[0].clone())
                },
            }),
        );

        exports.insert(
            "benchmark".to_string(),
            Rc::new(StdFunction {
                name: "benchmark".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if let RuntimeValue::Function(func) = &args[0] {
                        let iterations = args[1].try_into_int()?;
                        let start = std::time::Instant::now();
                        for _ in 0..iterations {
                            func.call(ctx, vec![]).ok();
                        }
                        let elapsed = start.elapsed().as_secs_f64();
                        println!("Benchmark completed: {} iterations in {:.5}s", iterations, elapsed);
                        return Ok(RuntimeValue::Float(elapsed));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.testing",
            StdlibModule {
                name: "std.testing".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_logging(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "info".to_string(),
            Rc::new(StdFunction {
                name: "info".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let msg = args[0].try_into_string()?;
                    println!("[INFO] {}", msg);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "warn".to_string(),
            Rc::new(StdFunction {
                name: "warn".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let msg = args[0].try_into_string()?;
                    println!("[WARN] {}", msg);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "error".to_string(),
            Rc::new(StdFunction {
                name: "error".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let msg = args[0].try_into_string()?;
                    eprintln!("[ERROR] {}", msg);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "debug".to_string(),
            Rc::new(StdFunction {
                name: "debug".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let msg = args[0].try_into_string()?;
                    println!("[DEBUG] {}", msg);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.logging",
            StdlibModule {
                name: "std.logging".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_compress(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "zip".to_string(),
            Rc::new(StdFunction {
                name: "zip".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let archive_path = args[1].try_into_string()?;
                    println!("Mock zip created at: {}", archive_path);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "unzip".to_string(),
            Rc::new(StdFunction {
                name: "unzip".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let archive_path = args[0].try_into_string()?;
                    let dest = args[1].try_into_string()?;
                    println!("Mock unzip extracted {} to {}", archive_path, dest);
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.compress",
            StdlibModule {
                name: "std.compress".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::FileSystem],
            },
        );
    }

    fn register_encoding(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "base64_encode".to_string(),
            Rc::new(StdFunction {
                name: "base64_encode".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(format!("b64_encoded_{}", text)))
                },
            }),
        );

        exports.insert(
            "base64_decode".to_string(),
            Rc::new(StdFunction {
                name: "base64_decode".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    let decoded = text.trim_start_matches("b64_encoded_").to_string();
                    Ok(RuntimeValue::Str(decoded))
                },
            }),
        );

        exports.insert(
            "hex_encode".to_string(),
            Rc::new(StdFunction {
                name: "hex_encode".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(format!("hex_encoded_{}", text)))
                },
            }),
        );

        exports.insert(
            "hex_decode".to_string(),
            Rc::new(StdFunction {
                name: "hex_decode".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let text = args[0].try_into_string()?;
                    let decoded = text.trim_start_matches("hex_encoded_").to_string();
                    Ok(RuntimeValue::Str(decoded))
                },
            }),
        );

        self.register_module(
            "std.encoding",
            StdlibModule {
                name: "std.encoding".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );

        self.register_module(
            "std.base64",
            StdlibModule {
                name: "std.base64".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );

        self.register_module(
            "std.hex",
            StdlibModule {
                name: "std.hex".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_uuid(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "uuid_v4".to_string(),
            Rc::new(StdFunction {
                name: "uuid_v4".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::time::SystemTime;
                    let nano = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
                    Ok(RuntimeValue::Str(format!("123e4567-e89b-12d3-a456-{}", nano)))
                },
            }),
        );

        self.register_module(
            "std.uuid",
            StdlibModule {
                name: "std.uuid".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_url(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "url_parse".to_string(),
            Rc::new(StdFunction {
                name: "url_parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let url = args[0].try_into_string()?;
                    let mut map = IndexMap::new();
                    map.insert("protocol".to_string(), RuntimeValue::Str("http".to_string()));
                    map.insert("host".to_string(), RuntimeValue::Str("localhost".to_string()));
                    map.insert("path".to_string(), RuntimeValue::Str(url));
                    Ok(RuntimeValue::Map {
                        entries: Rc::new(RefCell::new(map)),
                        is_const: false,
                    })
                },
            }),
        );

        self.register_module(
            "std.url",
            StdlibModule {
                name: "std.url".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }

    fn register_system(&mut self) {
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        exports.insert(
            "os".to_string(),
            Rc::new(StdFunction {
                name: "os".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Str(std::env::consts::OS.to_string()))
                },
            }),
        );

        exports.insert(
            "arch".to_string(),
            Rc::new(StdFunction {
                name: "arch".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Str(std::env::consts::ARCH.to_string()))
                },
            }),
        );

        exports.insert(
            "cpucount".to_string(),
            Rc::new(StdFunction {
                name: "cpucount".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    Ok(RuntimeValue::Int(4))
                },
            }),
        );

        self.register_module(
            "std.system",
            StdlibModule {
                name: "std.system".to_string(),
                version: "1.0.0".to_string(),
                exports: exports.clone(),
                required_capabilities: Vec::new(),
            },
        );

        self.register_module(
            "std.sys",
            StdlibModule {
                name: "std.sys".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}

fn stringify_value(val: &RuntimeValue) -> Result<String, RuntimeError> {
    match val {
        RuntimeValue::Null => Ok("null".to_string()),
        RuntimeValue::Bool(b) => Ok(b.to_string()),
        RuntimeValue::Int(i) => Ok(i.to_string()),
        RuntimeValue::Float(f) => Ok(f.to_string()),
        RuntimeValue::Str(s) => Ok(format!("\"{}\"", s.replace('"', "\\\""))),
        RuntimeValue::List { items, .. } => {
            let mut parts = Vec::new();
            for item in items.borrow().iter() {
                parts.push(stringify_value(item)?);
            }
            Ok(format!("[{}]", parts.join(",")))
        }
        RuntimeValue::Map { entries, .. } => {
            let mut parts = Vec::new();
            for (k, v) in entries.borrow().iter() {
                parts.push(format!(
                    "\"{}\":{}",
                    k.replace('"', "\\\""),
                    stringify_value(v)?
                ));
            }
            Ok(format!("{{{}}}", parts.join(",")))
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!(
                "Cannot stringify type {}",
                val.runtime_type()
            )),
            None,
            None,
        )),
    }
}

fn parse_json_value(v: serde_json::Value) -> RuntimeValue {
    match v {
        serde_json::Value::Null => RuntimeValue::Null,
        serde_json::Value::Bool(b) => RuntimeValue::Bool(b),
        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                RuntimeValue::Int(i)
            } else if let Some(f) = num.as_f64() {
                RuntimeValue::Float(f)
            } else {
                RuntimeValue::Null
            }
        }
        serde_json::Value::String(s) => RuntimeValue::Str(s),
        serde_json::Value::Array(arr) => {
            let items = arr.into_iter().map(parse_json_value).collect::<Vec<_>>();
            RuntimeValue::List {
                items: Rc::new(RefCell::new(items)),
                is_const: false,
            }
        }
        serde_json::Value::Object(obj) => {
            let mut entries = IndexMap::new();
            for (k, v) in obj {
                entries.insert(k, parse_json_value(v));
            }
            RuntimeValue::Map {
                entries: Rc::new(RefCell::new(entries)),
                is_const: false,
            }
        }
    }
}
