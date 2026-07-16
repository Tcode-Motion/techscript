use techscript_common::Span;
use techscript_runtime::{RuntimeError, RuntimeValue};

/// Type aliases for visitor execution and evaluation outputs.
pub type EvalResult = Result<RuntimeValue, RuntimeError>;
pub type ExecResult = Result<FlowSignal, RuntimeError>;

/// Control signals directed internally to navigate statement loops and calls.
#[derive(Debug, Clone)]
pub enum FlowSignal {
    Normal,
    Return(RuntimeValue),
    Break,
    Continue,
    Throw(RuntimeError),
}

/// Represents a stack frame inside the interpreter's call stack.
#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function_name: String,
    pub span: Option<Span>,
}

impl CallFrame {
    pub fn new(function_name: String, span: Option<Span>) -> Self {
        Self {
            function_name,
            span,
        }
    }
}
