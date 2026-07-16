use crate::opcode::Opcode;
use crate::operand::Operand;
use serde::{Deserialize, Serialize};
use techscript_common::Span;
use techscript_ir::types::InstructionId;

/// A self-describing VM instruction linked to debug mappings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BytecodeInstruction {
    pub id: InstructionId,
    pub op: Opcode,
    pub operands: Vec<Operand>,
    pub span: Span,
}

impl BytecodeInstruction {
    /// Creates a new BytecodeInstruction.
    pub fn new(id: InstructionId, op: Opcode, operands: Vec<Operand>, span: Span) -> Self {
        Self {
            id,
            op,
            operands,
            span,
        }
    }
}
