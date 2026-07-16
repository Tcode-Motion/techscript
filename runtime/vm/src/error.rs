use thiserror::Error;

/// Executable runtime VM errors carrying tracing descriptions.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum VMError {
    #[error("Stack underflow: popped empty stack context")]
    StackUnderflow,

    #[error("Stack overflow: exceeded maximum limit of 1024 elements")]
    StackOverflow,

    #[error("Invalid instruction opcode")]
    InvalidOpcode,

    #[error("Invalid constant pool reference at index {0}")]
    InvalidConstant(u32),

    #[error("Invalid function reference at index {0}")]
    InvalidFunction(u32),

    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeError { expected: String, found: String },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Uncaught runtime exception: {0}")]
    RuntimeException(String),

    #[error("Stack overflow (Call Stack): exceeded limit of 512 frames")]
    CallStackOverflow,
}
