use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
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

/// Metadata-rich standard library module.
#[derive(Clone)]
pub struct StdlibModule {
    pub name: String,
    pub version: String,
    pub exports: HashMap<String, Rc<dyn Callable>>,
    pub required_capabilities: Vec<Capability>,
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

    /// Registers a custom library module.
    pub fn register_module(&mut self, name: &str, module: StdlibModule) {
        self.modules.insert(name.to_string(), module);
    }

    /// Checks if a module is registered.
    pub fn has_module(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// Retrieves a module by name.
    pub fn get_module(&self, name: &str) -> Option<&StdlibModule> {
        self.modules.get(name)
    }

    /// Construct the unified "std" namespace map.
    pub fn construct_std_namespace(&self) -> RuntimeValue {
        let mut std_map = IndexMap::new();
        for (name, module) in &self.modules {
            // e.g. "std.math" -> namespace key is "math"
            if let Some(sub_name) = name.strip_prefix("std.") {
                let mut module_map = IndexMap::new();
                for (func_name, func) in &module.exports {
                    module_map.insert(
                        func_name.clone(),
                        RuntimeValue::Function(Rc::clone(func)),
                    );
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
            "sqrt".to_string(),
            Rc::new(StdFunction {
                name: "sqrt".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(val.sqrt()))
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
                    let exp = args[1].try_into_float()?;
                    Ok(RuntimeValue::Float(base.powf(exp)))
                },
            }),
        );

        exports.insert(
            "floor".to_string(),
            Rc::new(StdFunction {
                name: "floor".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(val.floor()))
                },
            }),
        );

        exports.insert(
            "ceil".to_string(),
            Rc::new(StdFunction {
                name: "ceil".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(val.ceil()))
                },
            }),
        );

        exports.insert(
            "round".to_string(),
            Rc::new(StdFunction {
                name: "round".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(val.round()))
                },
            }),
        );

        // Simple, fast LCG (Linear Congruential Generator) pseudo-random number generator
        exports.insert(
            "random".to_string(),
            Rc::new(StdFunction {
                name: "random".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::sync::atomic::{AtomicU64, Ordering};
                    static SEED: AtomicU64 = AtomicU64::new(123456789);
                    let old = SEED.load(Ordering::Relaxed);
                    let next = old.wrapping_mul(6364136223846793005).wrapping_add(1);
                    SEED.store(next, Ordering::Relaxed);
                    let val = (next >> 11) as f64 / (1u64 << 53) as f64;
                    Ok(RuntimeValue::Float(val))
                },
            }),
        );

        exports.insert(
            "sin".to_string(),
            Rc::new(StdFunction {
                name: "sin".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(val.sin()))
                },
            }),
        );

        exports.insert(
            "cos".to_string(),
            Rc::new(StdFunction {
                name: "cos".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(val.cos()))
                },
            }),
        );

        exports.insert(
            "tan".to_string(),
            Rc::new(StdFunction {
                name: "tan".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let val = args[0].try_into_float()?;
                    Ok(RuntimeValue::Float(val.tan()))
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
                    let val = args[0].try_into_string()?;
                    Ok(RuntimeValue::Str(val.trim().to_string()))
                },
            }),
        );

        exports.insert(
            "replace".to_string(),
            Rc::new(StdFunction {
                name: "replace".to_string(),
                arity: 3,
                callback: |_ctx, args| {
                    let s = args[0].try_into_string()?;
                    let old = args[1].try_into_string()?;
                    let new = args[2].try_into_string()?;
                    Ok(RuntimeValue::Str(s.replace(&old, &new)))
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
                    let sep = args[1].try_into_string()?;
                    let items = s
                        .split(&sep)
                        .map(|sub| RuntimeValue::Str(sub.to_string()))
                        .collect::<Vec<_>>();
                    Ok(RuntimeValue::List {
                        items: Rc::new(RefCell::new(items)),
                        is_const: false,
                    })
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
                            match (a, b) {
                                (RuntimeValue::Int(ai), RuntimeValue::Int(bi)) => ai.cmp(bi),
                                (RuntimeValue::Float(af), RuntimeValue::Float(bf)) => {
                                    af.partial_cmp(bf).unwrap_or(std::cmp::Ordering::Equal)
                                }
                                (RuntimeValue::Str(as_val), RuntimeValue::Str(bs_val)) => {
                                    as_val.cmp(bs_val)
                                }
                                _ => std::cmp::Ordering::Equal, // stable fallback
                            }
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
            "map".to_string(),
            Rc::new(StdFunction {
                name: "map".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    let list = match &args[0] {
                        RuntimeValue::List { items, .. } => items.borrow().clone(),
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

                    let callback = match &args[1] {
                        RuntimeValue::Function(f) => Rc::clone(f),
                        other => {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::TypeMismatch {
                                    expected: "Function".to_string(),
                                    found: other.runtime_type().to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };

                    let mut mapped = Vec::with_capacity(list.len());
                    for item in list {
                        let res = callback.call(ctx, vec![item])?;
                        mapped.push(res);
                    }

                    Ok(RuntimeValue::List {
                        items: Rc::new(RefCell::new(mapped)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "filter".to_string(),
            Rc::new(StdFunction {
                name: "filter".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    let list = match &args[0] {
                        RuntimeValue::List { items, .. } => items.borrow().clone(),
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

                    let callback = match &args[1] {
                        RuntimeValue::Function(f) => Rc::clone(f),
                        other => {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::TypeMismatch {
                                    expected: "Function".to_string(),
                                    found: other.runtime_type().to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };

                    let mut filtered = Vec::new();
                    for item in list {
                        let res = callback.call(ctx, vec![item.clone()])?;
                        if res.is_truthy() {
                            filtered.push(item);
                        }
                    }

                    Ok(RuntimeValue::List {
                        items: Rc::new(RefCell::new(filtered)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "reduce".to_string(),
            Rc::new(StdFunction {
                name: "reduce".to_string(),
                arity: 3,
                callback: |ctx, args| {
                    let list = match &args[0] {
                        RuntimeValue::List { items, .. } => items.borrow().clone(),
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

                    let initial = args[1].clone();

                    let callback = match &args[2] {
                        RuntimeValue::Function(f) => Rc::clone(f),
                        other => {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::TypeMismatch {
                                    expected: "Function".to_string(),
                                    found: other.runtime_type().to_string(),
                                },
                                None,
                                None,
                            ))
                        }
                    };

                    let mut acc = initial;
                    for item in list {
                        acc = callback.call(ctx, vec![acc, item])?;
                    }

                    Ok(acc)
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
                    let val = &args[0];
                    let s = stringify_value(val)?;
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
                    let v: serde_json::Value =
                        serde_json::from_str(&s).map_err(|e| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "JSON parse error: {}",
                                    e
                                )),
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
        // Guarded by capability models!
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        // 1. Filesystem APIs
        exports.insert(
            "read_file".to_string(),
            Rc::new(StdFunction {
                name: "read_file".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
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
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
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
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
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

        // 2. Environment Variable APIs
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
                                "Security policy violation: Environment capability is denied"
                                    .to_string(),
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
                                "Security policy violation: Environment capability is denied"
                                    .to_string(),
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

        self.register_module(
            "std.env",
            StdlibModule {
                name: "std.env".to_string(),
                version: "1.0.0".to_string(),
                exports: env_exports,
                required_capabilities: vec![Capability::Environment],
            },
        );

        // 3. Process Execution APIs
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
                                "Security policy violation: Process capability is denied"
                                    .to_string(),
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

        self.register_module(
            "std.process",
            StdlibModule {
                name: "std.process".to_string(),
                version: "1.0.0".to_string(),
                exports: proc_exports,
                required_capabilities: vec![Capability::Process],
            },
        );

        // 4. Timing APIs
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
                parts.push(format!("\"{}\":{}", k.replace('"', "\\\""), stringify_value(v)?));
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
