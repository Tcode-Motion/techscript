use crate::constant_pool::ConstantPool;
use crate::instruction::BytecodeInstruction;
use serde::{Deserialize, Serialize};
use techscript_ast::LiteralVal;

/// Executable instructions segment carrying its own local constant pool.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Chunk {
    pub instructions: Vec<BytecodeInstruction>,
    pub constants: ConstantPool,
}

impl Chunk {
    /// Creates an empty Chunk.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends an instruction to the chunk.
    pub fn write(&mut self, inst: BytecodeInstruction) {
        self.instructions.push(inst);
    }

    /// Registers a literal constant, returning its deduplicated index.
    pub fn add_const(&mut self, val: LiteralVal) -> u32 {
        self.constants.add(val)
    }
}
