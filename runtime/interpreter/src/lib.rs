//! # TechScript Interpreter Crate
//!
//! Tree-walking interpreter backend for AST evaluation.
//! Maintains local environment states and manages execution control signals.

#![allow(warnings, clippy::all)]

pub mod control_flow;
pub mod expressions;
pub mod functions;
pub mod interpreter;
pub mod operations;
pub mod statements;
pub mod visitor;

pub use control_flow::{CallFrame, EvalResult, ExecResult, FlowSignal};
pub use interpreter::Interpreter;
pub use visitor::AstVisitor;
pub type Value = techscript_runtime::RuntimeValue;
pub type RuntimeError = techscript_runtime::RuntimeError;
pub use techscript_runtime::RuntimeErrorKind;

use techscript_semantic::CheckedProgram;

/// Helper function to evaluate checked programs.
pub fn interpret(checked: CheckedProgram) -> Result<Value, RuntimeError> {
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program)
}
