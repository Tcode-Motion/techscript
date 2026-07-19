use serde::{Deserialize, Serialize};
use std::fmt;
use techscript_common::Span;
use techscript_errors::ErrorCode;

/// Detailed categories for execution failures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeErrorKind {
    UndefinedVariable(String),
    DivisionByZero,
    IndexOutOfBounds,
    InvalidCast(String),
    AssertionFailed(String),
    TypeMismatch { expected: String, found: String },
    InvalidOperation(String),
    StackOverflow,
    MemberNotFound(String),
    ArityMismatch { expected: usize, found: usize },
    UserError(String),
}

impl fmt::Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndefinedVariable(name) => write!(f, "Undefined variable '{}'", name),
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::IndexOutOfBounds => write!(f, "Index out of bounds"),
            Self::InvalidCast(msg) => write!(f, "Invalid cast: {}", msg),
            Self::AssertionFailed(msg) => write!(f, "Assertion failed: {}", msg),
            Self::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "Type mismatch: expected '{}', found '{}'",
                    expected, found
                )
            }
            Self::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            Self::StackOverflow => write!(f, "Stack overflow"),
            Self::MemberNotFound(name) => write!(f, "Member '{}' not found", name),
            Self::ArityMismatch { expected, found } => {
                write!(
                    f,
                    "Arity mismatch: expected {} arguments, found {}",
                    expected, found
                )
            }
            Self::UserError(msg) => write!(f, "{}", msg),
        }
    }
}

/// Standardized RuntimeError context holding code categorizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub span: Option<Span>,
    pub code: Option<ErrorCode>,
    pub message: String,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind, span: Option<Span>, code: Option<ErrorCode>) -> Self {
        let message = kind.to_string();
        Self {
            kind,
            span,
            code,
            message,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime Error: {}", self.message)
    }
}

impl std::error::Error for RuntimeError {}
