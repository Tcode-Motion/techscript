// ── TechScript Bytecode Chunk ────────────────────────────────────────

use crate::opcode::OpCode;
use crate::value::Value;

/// A chunk of bytecode: instructions + constants + line info.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    /// Write a single byte (opcode or operand).
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Write an opcode.
    pub fn write_op(&mut self, op: OpCode, line: usize) {
        self.write(op as u8, line);
    }

    /// Add a constant and return its index.
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    /// Write a constant instruction (opcode + 2-byte index).
    pub fn write_constant(&mut self, value: Value, line: usize) -> usize {
        let idx = self.add_constant(value);
        self.write_op(OpCode::Constant, line);
        self.write((idx >> 8) as u8, line);
        self.write((idx & 0xFF) as u8, line);
        idx
    }

    /// Emit a jump instruction, returning the offset to patch later.
    pub fn emit_jump(&mut self, op: OpCode, line: usize) -> usize {
        self.write_op(op, line);
        self.write(0xFF, line);
        self.write(0xFF, line);
        self.code.len() - 2
    }

    /// Patch a previously emitted jump.
    pub fn patch_jump(&mut self, offset: usize) {
        let jump = self.code.len() - offset - 2;
        self.code[offset] = (jump >> 8) as u8;
        self.code[offset + 1] = (jump & 0xFF) as u8;
    }

    /// Emit a loop instruction (jumps backward).
    pub fn emit_loop(&mut self, loop_start: usize, line: usize) {
        self.write_op(OpCode::Loop, line);
        let offset = self.code.len() - loop_start + 2;
        self.write((offset >> 8) as u8, line);
        self.write((offset & 0xFF) as u8, line);
    }

    /// Current code length.
    pub fn len(&self) -> usize {
        self.code.len()
    }

    /// Read a 2-byte big-endian u16 from offset.
    pub fn read_u16(&self, offset: usize) -> u16 {
        ((self.code[offset] as u16) << 8) | (self.code[offset + 1] as u16)
    }
}
