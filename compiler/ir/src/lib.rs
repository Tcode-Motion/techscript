//! # TechScript IR Crate
//!
//! Typed intermediate representation and AST lowering pipeline.

#![allow(warnings, clippy::all)]

pub mod block;
pub mod builder;
pub mod function;
pub mod instruction;
pub mod lowering;
pub mod module;
pub mod types;
pub mod value;

pub use block::BasicBlock;
pub use builder::IRBuilder;
pub use function::Function;
pub use instruction::{Instruction, Op, Terminator, TerminatorKind};
pub use lowering::{LoweringContext, LoweringResult, SymbolBinding};
pub use module::{DslBlockIR, Module};
pub use types::{
    BlockId, DslBlockId, FunctionId, GlobalId, IRType, InstructionId, LocalId, ValueId,
};
pub use value::Value;

/// Lower AST program into intermediate representation (IR) module.
pub fn lower(program: &techscript_ast::Program, name: &str) -> LoweringResult {
    let context = LoweringContext::new();
    context.lower(program, name)
}
