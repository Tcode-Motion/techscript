use crate::chunk::Chunk;
use crate::constant_pool::ConstantPool;
use crate::debug::DebugSymbols;
use crate::function::BytecodeFunction;
use crate::instruction::BytecodeInstruction;
use crate::opcode::Opcode;
use crate::operand::Operand;
use crate::source_map::SourceMap;
use std::collections::HashMap;
use techscript_common::Span;
use techscript_ir::types::InstructionId;

/// Reference placeholder for relative branch offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label(pub u32);

/// Assembler emitting instructions with lazy label references and auto stack estimation.
pub struct BytecodeBuilder {
    pub name: String,
    pub param_count: u32,
    pub local_count: u32,
    pub instructions: Vec<BytecodeInstruction>,
    pub constants: ConstantPool,
    pub source_map: SourceMap,
    pub debug_symbols: DebugSymbols,

    label_counter: u32,
    labels: HashMap<u32, Option<u32>>,
    label_references: Vec<(usize, u32)>, // Instruction index -> Label ID
}

impl BytecodeBuilder {
    /// Creates a new assembler for a function.
    pub fn new(name: String, param_count: u32) -> Self {
        Self {
            name,
            param_count,
            local_count: 0,
            instructions: Vec::new(),
            constants: ConstantPool::new(),
            source_map: SourceMap::new(),
            debug_symbols: DebugSymbols::new(),
            label_counter: 0,
            labels: HashMap::new(),
            label_references: Vec::new(),
        }
    }

    /// Allocates a new local variable slot.
    pub fn allocate_local(&mut self, name: String) -> u32 {
        let idx = self.local_count;
        self.local_count += 1;
        self.debug_symbols.local_names.insert(idx, name);
        idx
    }

    /// Creates a new unresolved label.
    pub fn make_label(&mut self) -> Label {
        let id = self.label_counter;
        self.label_counter += 1;
        self.labels.insert(id, None);
        Label(id)
    }

    /// Binds the label to the current instruction offset point.
    pub fn mark_label(&mut self, label: Label) {
        let offset = self.instructions.len() as u32;
        self.labels.insert(label.0, Some(offset));
    }

    /// Emits a single stack instruction.
    pub fn emit(&mut self, op: Opcode, operands: Vec<Operand>, span: Span, inst_id: InstructionId) {
        let offset = self.instructions.len() as u32;
        self.source_map.add(offset, span);

        let inst = BytecodeInstruction::new(inst_id, op, operands, span);
        self.instructions.push(inst);
    }

    /// Emits a branch instruction targeting a lazy label.
    pub fn emit_jump(&mut self, op: Opcode, label: Label, span: Span, inst_id: InstructionId) {
        let offset = self.instructions.len();
        self.label_references.push((offset, label.0));

        // Placeholder offset operand
        self.emit(op, vec![Operand::JumpOffset(9999)], span, inst_id);
    }

    /// Finalizes compilation, patching label offsets and calculating stack limits.
    pub fn finish(mut self) -> BytecodeFunction {
        // Patch jump offsets
        for &(inst_idx, label_id) in &self.label_references {
            if let Some(Some(target_offset)) = self.labels.get(&label_id) {
                let jump_offset = (*target_offset as i32) - (inst_idx as i32);
                if let Some(ref mut inst) = self.instructions.get_mut(inst_idx) {
                    inst.operands = vec![Operand::JumpOffset(jump_offset)];
                }
            }
        }

        // Calculate max stack size via static traversal
        let mut max_stack_size = 0;
        let mut curr_stack = 0i32;

        for inst in &self.instructions {
            let effect = inst.op.stack_effect();
            curr_stack += effect;
            if curr_stack < 0 {
                // Stack underflows are normalized
                curr_stack = 0;
            }
            if curr_stack > max_stack_size {
                max_stack_size = curr_stack;
            }
        }

        let mut chunk = Chunk::new();
        chunk.instructions = std::mem::take(&mut self.instructions);
        chunk.constants = std::mem::take(&mut self.constants);

        BytecodeFunction {
            name: self.name,
            param_count: self.param_count,
            local_count: self.local_count,
            max_stack_size: max_stack_size as u32,
            chunk,
            source_map: self.source_map,
            debug_symbols: self.debug_symbols,
        }
    }
}
