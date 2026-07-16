use crate::error::{RuntimeError, RuntimeErrorKind};
use crate::function::Callable;
use crate::object::{ModelInstance, StructInstance};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// The complete set of language types supported at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeType {
    Int,
    Float,
    Bool,
    String,
    List,
    Map,
    Tuple,
    Struct,
    Enum,
    Model,
    Function,
    Null,
}

impl fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Int => "int",
            Self::Float => "float",
            Self::Bool => "bool",
            Self::String => "string",
            Self::List => "list",
            Self::Map => "map",
            Self::Tuple => "tuple",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Model => "model",
            Self::Function => "function",
            Self::Null => "null",
        };
        write!(f, "{}", name)
    }
}

/// The universal runtime value representing any dynamic object in the language.
#[derive(Clone)]
pub enum RuntimeValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List {
        items: Rc<RefCell<Vec<RuntimeValue>>>,
        is_const: bool,
    },
    Map {
        entries: Rc<RefCell<IndexMap<String, RuntimeValue>>>,
        is_const: bool,
    },
    Tuple(Vec<RuntimeValue>),
    StructInstance(Rc<RefCell<StructInstance>>),
    EnumVariant {
        name: String,
        payload: Option<Vec<RuntimeValue>>,
    },
    ModelInstance(Rc<RefCell<ModelInstance>>),
    Function(Rc<dyn Callable>),
}

impl RuntimeValue {
    /// Determines whether the value resolves to true in conditions.
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Bool(b) => *b,
            _ => true,
        }
    }

    /// Queries the RuntimeType category of this value.
    pub fn runtime_type(&self) -> RuntimeType {
        match self {
            Self::Null => RuntimeType::Null,
            Self::Bool(_) => RuntimeType::Bool,
            Self::Int(_) => RuntimeType::Int,
            Self::Float(_) => RuntimeType::Float,
            Self::Str(_) => RuntimeType::String,
            Self::List { .. } => RuntimeType::List,
            Self::Map { .. } => RuntimeType::Map,
            Self::Tuple(_) => RuntimeType::Tuple,
            Self::StructInstance(_) => RuntimeType::Struct,
            Self::EnumVariant { .. } => RuntimeType::Enum,
            Self::ModelInstance(_) => RuntimeType::Model,
            Self::Function(_) => RuntimeType::Function,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn try_into_int(&self) -> Result<i64, RuntimeError> {
        match self {
            Self::Int(i) => Ok(*i),
            Self::Float(f) => Ok(*f as i64),
            Self::Str(s) => s.parse::<i64>().map_err(|_| {
                RuntimeError::new(
                    RuntimeErrorKind::InvalidCast(format!("Cannot cast string '{}' to int", s)),
                    None,
                    None,
                )
            }),
            Self::Bool(b) => Ok(if *b { 1 } else { 0 }),
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "int".to_string(),
                    found: self.runtime_type().to_string(),
                },
                None,
                None,
            )),
        }
    }

    pub fn try_into_float(&self) -> Result<f64, RuntimeError> {
        match self {
            Self::Float(f) => Ok(*f),
            Self::Int(i) => Ok(*i as f64),
            Self::Str(s) => s.parse::<f64>().map_err(|_| {
                RuntimeError::new(
                    RuntimeErrorKind::InvalidCast(format!("Cannot cast string '{}' to float", s)),
                    None,
                    None,
                )
            }),
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "float".to_string(),
                    found: self.runtime_type().to_string(),
                },
                None,
                None,
            )),
        }
    }

    pub fn try_into_bool(&self) -> Result<bool, RuntimeError> {
        match self {
            Self::Bool(b) => Ok(*b),
            Self::Int(i) => Ok(*i != 0),
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "bool".to_string(),
                    found: self.runtime_type().to_string(),
                },
                None,
                None,
            )),
        }
    }

    pub fn try_into_string(&self) -> Result<String, RuntimeError> {
        match self {
            Self::Str(s) => Ok(s.clone()),
            other => Ok(other.to_string()),
        }
    }

    /// Verifies reference identity check (===).
    pub fn physical_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StructInstance(s1), Self::StructInstance(s2)) => {
                s1.borrow().id == s2.borrow().id
            }
            (Self::ModelInstance(m1), Self::ModelInstance(m2)) => m1.borrow().id == m2.borrow().id,
            (Self::List { items: i1, .. }, Self::List { items: i2, .. }) => Rc::ptr_eq(i1, i2),
            (Self::Map { entries: e1, .. }, Self::Map { entries: e2, .. }) => Rc::ptr_eq(e1, e2),
            (Self::Function(f1), Self::Function(f2)) => Rc::ptr_eq(f1, f2),
            _ => self == other,
        }
    }
}

impl PartialEq for RuntimeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(b1), Self::Bool(b2)) => b1 == b2,
            (Self::Int(i1), Self::Int(i2)) => i1 == i2,
            (Self::Float(f1), Self::Float(f2)) => f1 == f2,
            (Self::Str(s1), Self::Str(s2)) => s1 == s2,
            (Self::Tuple(t1), Self::Tuple(t2)) => t1 == t2,
            (
                Self::EnumVariant {
                    name: n1,
                    payload: p1,
                },
                Self::EnumVariant {
                    name: n2,
                    payload: p2,
                },
            ) => n1 == n2 && p1 == p2,
            (Self::List { items: i1, .. }, Self::List { items: i2, .. }) => {
                *i1.borrow() == *i2.borrow()
            }
            (Self::Map { entries: e1, .. }, Self::Map { entries: e2, .. }) => {
                *e1.borrow() == *e2.borrow()
            }
            (Self::StructInstance(s1), Self::StructInstance(s2)) => {
                s1.borrow().name == s2.borrow().name && s1.borrow().fields == s2.borrow().fields
            }
            (Self::ModelInstance(m1), Self::ModelInstance(m2)) => {
                m1.borrow().name == m2.borrow().name && m1.borrow().fields == m2.borrow().fields
            }
            (Self::Function(f1), Self::Function(f2)) => Rc::ptr_eq(f1, f2),
            _ => false,
        }
    }
}

impl fmt::Debug for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
            Self::Bool(b) => write!(f, "Bool({})", b),
            Self::Int(i) => write!(f, "Int({})", i),
            Self::Float(fl) => write!(f, "Float({})", fl),
            Self::Str(s) => write!(f, "Str({:?})", s),
            Self::List { items, is_const } => {
                write!(f, "List(const={}, {:?})", is_const, items.borrow())
            }
            Self::Map { entries, is_const } => {
                write!(f, "Map(const={}, {:?})", is_const, entries.borrow())
            }
            Self::Tuple(t) => write!(f, "Tuple({:?})", t),
            Self::StructInstance(s) => write!(
                f,
                "StructInstance({}, {:?})",
                s.borrow().name,
                s.borrow().fields
            ),
            Self::EnumVariant { name, payload } => {
                write!(f, "EnumVariant({}, {:?})", name, payload)
            }
            Self::ModelInstance(m) => write!(
                f,
                "ModelInstance({}, {:?})",
                m.borrow().name,
                m.borrow().fields
            ),
            Self::Function(func) => write!(f, "Function({})", func.name()),
        }
    }
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "none"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::Str(s) => write!(f, "{}", s),
            Self::List { items, .. } => {
                write!(f, "[")?;
                for (idx, item) in items.borrow().iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Self::Map { entries, .. } => {
                write!(f, "{{")?;
                for (idx, (k, v)) in entries.borrow().iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Self::Tuple(t) => {
                write!(f, "(")?;
                for (idx, item) in t.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Self::StructInstance(s) => write!(f, "struct {}", s.borrow().name),
            Self::EnumVariant {
                name,
                payload: None,
            } => write!(f, ".{}", name),
            Self::EnumVariant {
                name,
                payload: Some(payload),
            } => {
                write!(f, ".{}(", name)?;
                for (idx, item) in payload.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Self::ModelInstance(m) => write!(f, "instance of model {}", m.borrow().name),
            Self::Function(func) => write!(f, "<function {}>", func.name()),
        }
    }
}
