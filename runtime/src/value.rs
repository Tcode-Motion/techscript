// ── TechScript Runtime Values ────────────────────────────────────────

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::chunk::Chunk;

/// A native (built-in) function type.
pub type NativeFn = dyn Fn(&[Value]) -> Result<Value, String>;

/// Runtime value for the TechScript VM.
#[derive(Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    None,
    String(Rc<String>),
    List(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<HashMap<String, Value>>>),
    Function(Rc<Function>),
    NativeFunction(Rc<NativeFnObj>),
    Class(Rc<RefCell<ClassObj>>),
    Instance(Rc<RefCell<InstanceObj>>),
    Closure(Rc<ClosureObj>),
    BoundMethod(Rc<RefCell<InstanceObj>>, Rc<ClosureObj>),
    Iterator(Rc<RefCell<IterState>>),
    Range(i64, i64, bool), // start, end, inclusive
}

/// A compiled TechScript function.
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub chunk: Chunk,
    pub upvalue_count: usize,
}

impl Function {
    pub fn new(name: impl Into<String>, arity: usize) -> Self {
        Function {
            name: name.into(),
            arity,
            chunk: Chunk::new(),
            upvalue_count: 0,
        }
    }
}

/// A runtime closure with captured upvalues.
#[derive(Clone)]
pub struct ClosureObj {
    pub function: Rc<Function>,
    pub upvalues: Vec<Rc<RefCell<Value>>>,
}

/// A native function wrapper.
pub struct NativeFnObj {
    pub name: String,
    pub func: Box<NativeFn>,
}

impl Clone for NativeFnObj {
    fn clone(&self) -> Self {
        // NativeFnObj isn't truly cloneable, but we wrap in Rc so this is fine
        panic!("NativeFnObj should not be cloned directly; use Rc<NativeFnObj>")
    }
}

impl fmt::Debug for NativeFnObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

/// A TechScript class.
#[derive(Debug, Clone)]
pub struct ClassObj {
    pub name: String,
    pub methods: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<ClassObj>>>,
}

/// An instance of a TechScript class.
#[derive(Debug, Clone)]
pub struct InstanceObj {
    pub class: Rc<RefCell<ClassObj>>,
    pub fields: HashMap<String, Value>,
}

/// Internal iterator state.
#[derive(Debug, Clone)]
pub struct IterState {
    pub items: Vec<Value>,
    pub index: usize,
}

impl IterState {
    pub fn new(items: Vec<Value>) -> Self {
        IterState { items, index: 0 }
    }

    pub fn next(&mut self) -> Option<Value> {
        if self.index < self.items.len() {
            let val = self.items[self.index].clone();
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::None => false,
            Value::Int(0) => false,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.borrow().is_empty(),
            _ => true,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::None => "none",
            Value::String(_) => "string",
            Value::List(_) => "list",
            Value::Map(_) => "map",
            Value::Function(_) => "function",
            Value::Closure(_) => "function",
            Value::NativeFunction(_) => "function",
            Value::Class(_) => "class",
            Value::Instance(_) => "instance",
            Value::BoundMethod(_, _) => "method",
            Value::Iterator(_) => "iterator",
            Value::Range(_, _, _) => "range",
        }
    }

    /// Convert to a display string (for `say`).
    pub fn display_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => {
                if *f == (*f as i64) as f64 && f.is_finite() {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            Value::Bool(true) => "true".to_string(),
            Value::Bool(false) => "false".to_string(),
            Value::None => "none".to_string(),
            Value::String(s) => s.as_ref().clone(),
            Value::List(l) => {
                let items: Vec<String> = l.borrow().iter().map(|v| v.repr_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Map(m) => {
                let entries: Vec<String> = m.borrow().iter()
                    .map(|(k, v)| format!("{}: {}", k, v.repr_string()))
                    .collect();
                format!("{{{}}}", entries.join(", "))
            }
            Value::Function(f) => format!("<fn {}>", f.name),
            Value::Closure(c) => format!("<fn {}>", c.function.name),
            Value::NativeFunction(f) => format!("<native fn {}>", f.name),
            Value::Class(c) => format!("<class {}>", c.borrow().name),
            Value::Instance(inst) => format!("<{} instance>", inst.borrow().class.borrow().name),
            Value::BoundMethod(_, c) => format!("<method {}>", c.function.name),
            Value::Iterator(_) => "<iterator>".to_string(),
            Value::Range(s, e, inc) => {
                if *inc {
                    format!("{}..={}", s, e)
                } else {
                    format!("{}..{}", s, e)
                }
            }
        }
    }

    /// String representation for embedding in containers (shows quotes around strings).
    pub fn repr_string(&self) -> String {
        match self {
            Value::String(s) => format!("\"{}\"", s),
            _ => self.display_string(),
        }
    }

    /// Attempt to compare for equality.
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::None, Value::None) => true,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }

    /// Numeric ordering comparison.
    pub fn compare(&self, other: &Value) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            _ => Option::None,
        }
    }

    /// Extract as f64 for arithmetic.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => Option::None,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_string())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_string())
    }
}
