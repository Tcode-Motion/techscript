//! # TechScript Bytecode Crate
//!
//! Handles VM instructions representation, IR-to-Bytecode lowering, binary
//! serialization formats, and human-readable disassemblers.

pub mod builder;
pub mod chunk;
pub mod constant_pool;
pub mod debug;
pub mod disassembler;
pub mod function;
pub mod instruction;
pub mod lowering;
pub mod module;
pub mod opcode;
pub mod operand;
pub mod serializer;
pub mod source_map;
pub mod validator;

pub use builder::{BytecodeBuilder, Label};
pub use chunk::Chunk;
pub use constant_pool::ConstantPool;
pub use debug::DebugSymbols;
pub use disassembler::BytecodeDisassembler;
pub use function::BytecodeFunction;
pub use instruction::BytecodeInstruction;
pub use lowering::BytecodeLowerer;
pub use module::BytecodeModule;
pub use opcode::Opcode;
pub use operand::Operand;
pub use serializer::{BytecodeSerializer, BytecodeVersion};
pub use source_map::SourceMap;
pub use validator::BytecodeValidator;

/// Compiles an optimized IR module to VM bytecode.
pub fn compile(module: &techscript_ir::Module) -> BytecodeModule {
    BytecodeLowerer::lower(module)
}
