use crate::context::RuntimeContext;
use crate::error::{RuntimeError, RuntimeErrorKind};
use crate::function::Callable;
use crate::value::RuntimeValue;
use std::collections::HashMap;
use std::rc::Rc;

/// Stores a map of pre-registered native callables.
pub struct NativeRegistry {
    functions: HashMap<String, Rc<dyn Callable>>,
}

impl Default for NativeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeRegistry {
    /// Creates and pre-populates a NativeRegistry with standard library functions.
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        registry.register("say", Rc::new(SayNative));
        registry.register("ask", Rc::new(AskNative));
        registry.register("len", Rc::new(LenNative));
        registry.register("type_of", Rc::new(TypeOfNative));
        registry.register("range", Rc::new(RangeNative));
        registry.register("to_int", Rc::new(ToIntNative));
        registry.register("to_float", Rc::new(ToFloatNative));
        registry.register("to_str", Rc::new(ToStrNative));
        registry.register("to_bool", Rc::new(ToBoolNative));
        registry.register("assert", Rc::new(AssertNative));
        registry.register("exit", Rc::new(ExitNative));
        registry
    }

    /// Registers a callable under a name identifier.
    pub fn register(&mut self, name: &str, func: Rc<dyn Callable>) {
        self.functions.insert(name.to_string(), func);
    }

    /// Looks up a registered callable by name.
    pub fn lookup(&self, name: &str) -> Option<Rc<dyn Callable>> {
        self.functions.get(name).cloned()
    }

    /// Removes a registered callable.
    pub fn remove(&mut self, name: &str) -> Option<Rc<dyn Callable>> {
        self.functions.remove(name)
    }

    /// Iterates over all (name, callable) pairs in the registry.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Rc<dyn Callable>)> {
        self.functions.iter().map(|(k, v)| (k.as_str(), v))
    }
}

pub struct SayNative;
impl Callable for SayNative {
    fn name(&self) -> &str {
        "say"
    }
    fn arity(&self) -> usize {
        1
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.is_empty() {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 1,
                    found: 0,
                },
                None,
                None,
            ));
        }
        println!("{}", args[0]);
        Ok(RuntimeValue::Null)
    }
}

pub struct AskNative;
impl Callable for AskNative {
    fn name(&self) -> &str {
        "ask"
    }
    fn arity(&self) -> usize {
        1
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.is_empty() {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 1,
                    found: 0,
                },
                None,
                None,
            ));
        }
        use std::io::{self, Write};
        print!("{}", args[0]);
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        Ok(RuntimeValue::Str(
            input
                .trim_end_matches('\r')
                .trim_end_matches('\n')
                .to_string(),
        ))
    }
}

pub struct LenNative;
impl Callable for LenNative {
    fn name(&self) -> &str {
        "len"
    }
    fn arity(&self) -> usize {
        1
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.is_empty() {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 1,
                    found: 0,
                },
                None,
                None,
            ));
        }
        let len = match &args[0] {
            RuntimeValue::Str(s) => s.len() as i64,
            RuntimeValue::List { items, .. } => items.borrow().len() as i64,
            RuntimeValue::Map { entries, .. } => entries.borrow().len() as i64,
            RuntimeValue::Tuple(t) => t.len() as i64,
            _ => {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::TypeMismatch {
                        expected: "collection or string".to_string(),
                        found: args[0].runtime_type().to_string(),
                    },
                    None,
                    None,
                ))
            }
        };
        Ok(RuntimeValue::Int(len))
    }
}

pub struct TypeOfNative;
impl Callable for TypeOfNative {
    fn name(&self) -> &str {
        "type_of"
    }
    fn arity(&self) -> usize {
        1
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.is_empty() {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 1,
                    found: 0,
                },
                None,
                None,
            ));
        }
        // v1.0.8 type names: `str` for strings, `dict` for maps.
        // All other type names (`int`, `float`, `bool`, `list`, `null`, etc.) match.
        let type_name = match &args[0] {
            RuntimeValue::Str(_)        => "str".to_string(),
            RuntimeValue::Map { .. }    => "dict".to_string(),
            other                       => other.runtime_type().to_string(),
        };
        Ok(RuntimeValue::Str(type_name))
    }
}

pub struct RangeNative;
impl Callable for RangeNative {
    fn name(&self) -> &str {
        "range"
    }
    fn arity(&self) -> usize {
        2
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.len() < 2 {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 2,
                    found: args.len(),
                },
                None,
                None,
            ));
        }
        let start = args[0].try_into_int()?;
        let end = args[1].try_into_int()?;
        let list = (start..end).map(RuntimeValue::Int).collect::<Vec<_>>();
        Ok(RuntimeValue::List {
            items: std::rc::Rc::new(std::cell::RefCell::new(list)),
            is_const: false,
        })
    }
}

pub struct ToIntNative;
impl Callable for ToIntNative {
    fn name(&self) -> &str {
        "to_int"
    }
    fn arity(&self) -> usize {
        1
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.is_empty() {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 1,
                    found: 0,
                },
                None,
                None,
            ));
        }
        args[0].try_into_int().map(RuntimeValue::Int)
    }
}

pub struct ToFloatNative;
impl Callable for ToFloatNative {
    fn name(&self) -> &str {
        "to_float"
    }
    fn arity(&self) -> usize {
        1
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.is_empty() {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 1,
                    found: 0,
                },
                None,
                None,
            ));
        }
        args[0].try_into_float().map(RuntimeValue::Float)
    }
}

pub struct ToStrNative;
impl Callable for ToStrNative {
    fn name(&self) -> &str {
        "to_str"
    }
    fn arity(&self) -> usize {
        1
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.is_empty() {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 1,
                    found: 0,
                },
                None,
                None,
            ));
        }
        args[0].try_into_string().map(RuntimeValue::Str)
    }
}

pub struct ToBoolNative;
impl Callable for ToBoolNative {
    fn name(&self) -> &str {
        "to_bool"
    }
    fn arity(&self) -> usize {
        1
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.is_empty() {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 1,
                    found: 0,
                },
                None,
                None,
            ));
        }
        args[0].try_into_bool().map(RuntimeValue::Bool)
    }
}

pub struct AssertNative;
impl Callable for AssertNative {
    fn name(&self) -> &str {
        "assert"
    }
    fn arity(&self) -> usize {
        1
    }
    fn call(
        &self,
        ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        if !ctx.config.enable_assertions {
            return Ok(RuntimeValue::Null);
        }
        if args.is_empty() {
            return Err(RuntimeError::new(
                RuntimeErrorKind::ArityMismatch {
                    expected: 1,
                    found: 0,
                },
                None,
                None,
            ));
        }
        if !args[0].is_truthy() {
            let msg = if args.len() > 1 {
                args[1].try_into_string()?
            } else {
                "Assertion failed".to_string()
            };
            return Err(RuntimeError::new(
                RuntimeErrorKind::AssertionFailed(msg),
                None,
                None,
            ));
        }
        Ok(RuntimeValue::Null)
    }
}

pub struct ExitNative;
impl Callable for ExitNative {
    fn name(&self) -> &str {
        "exit"
    }
    fn arity(&self) -> usize {
        0
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        let code = if !args.is_empty() {
            args[0].try_into_int()? as i32
        } else {
            0
        };
        std::process::exit(code);
    }
}
