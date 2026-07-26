use crate::block::BasicBlock;
use crate::function::Function;
use crate::instruction::{Instruction, InstructionMetadata, Op, Terminator, TerminatorKind};
use crate::module::{DslBlockIR, Module};
use crate::types::{BlockId, DslBlockId, FunctionId, GlobalId, IRType, InstructionId, LocalId, ValueId};

use std::collections::HashMap;
use techscript_ast::LiteralVal;
use techscript_common::Span;

/// Helper builder to construct an intermediate representation (IR) program module.
pub struct IRBuilder {
    value_counter: u32,
    local_counter: u32,
    global_counter: u32,
    block_counter: u32,
    func_counter: u32,
    inst_counter: u32,
    dsl_block_counter: u32,

    functions: Vec<Function>,
    globals: Vec<(GlobalId, String, IRType)>,
    dsl_blocks: Vec<DslBlockIR>,
    constants: Vec<(ValueId, LiteralVal)>,
    imports: Vec<String>,
    exports: Vec<String>,

    pub current_function: Option<Function>,
    current_block: Option<BasicBlock>,
    blocks_in_progress: Vec<BasicBlock>,
}

impl Default for IRBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IRBuilder {
    /// Creates a new IR builder instance.
    pub fn new() -> Self {
        Self {
            value_counter: 0,
            local_counter: 0,
            global_counter: 0,
            block_counter: 0,
            func_counter: 0,
            inst_counter: 0,
            dsl_block_counter: 0,
            functions: Vec::new(),
            globals: Vec::new(),
            dsl_blocks: Vec::new(),
            constants: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            current_function: None,
            current_block: None,
            blocks_in_progress: Vec::new(),
        }
    }

    /// Generates a new unique temporary value identifier.
    pub fn next_value_id(&mut self) -> ValueId {
        let id = ValueId(self.value_counter);
        self.value_counter += 1;
        id
    }

    /// Generates a new unique local symbol identifier.
    pub fn next_local_id(&mut self) -> LocalId {
        let id = LocalId(self.local_counter);
        self.local_counter += 1;
        id
    }

    /// Generates a new unique global symbol identifier.
    pub fn next_global_id(&mut self) -> GlobalId {
        let id = GlobalId(self.global_counter);
        self.global_counter += 1;
        id
    }

    /// Generates a new unique DSL block identifier.
    pub fn next_dsl_block_id(&mut self) -> DslBlockId {
        let id = DslBlockId(self.dsl_block_counter);
        self.dsl_block_counter += 1;
        id
    }

    /// Declares a DSL block in the module.
    pub fn declare_dsl_block(&mut self, block: DslBlockIR) -> DslBlockId {
        let id = block.id;
        self.dsl_blocks.push(block);
        id
    }

    /// Generates a new unique instruction identifier.
    pub fn next_instruction_id(&mut self) -> InstructionId {
        let id = InstructionId(self.inst_counter);
        self.inst_counter += 1;
        id
    }

    /// Declares a global variable.
    pub fn declare_global(&mut self, name: String, ty: IRType) -> GlobalId {
        let id = self.next_global_id();
        self.globals.push((id, name, ty));
        id
    }

    /// Declares an import.
    pub fn declare_import(&mut self, name: String) {
        self.imports.push(name);
    }

    /// Declares an export.
    pub fn declare_export(&mut self, name: String) {
        self.exports.push(name);
    }

    /// Starts building a new function.
    pub fn start_function(&mut self, name: String, return_type: IRType) -> FunctionId {
        let id = FunctionId(self.func_counter);
        self.func_counter += 1;

        // Seal previous function if exists
        self.seal_function();

        self.current_function = Some(Function::new(id, name, return_type));
        self.blocks_in_progress = Vec::new();
        id
    }

    /// Adds a parameter to the current function.
    pub fn add_parameter(&mut self, name: String, ty: IRType) -> LocalId {
        let local_id = self.next_local_id();
        if let Some(ref mut func) = self.current_function {
            func.params.push((local_id, name, ty.clone()));
            func.local_types.insert(local_id, ty);
        }
        local_id
    }

    /// Allocates local variable storage in the current function.
    pub fn allocate_local(&mut self, ty: IRType) -> LocalId {
        let local_id = self.next_local_id();
        if let Some(ref mut func) = self.current_function {
            func.local_types.insert(local_id, ty);
        }
        local_id
    }

    /// Creates a new basic block with a label.
    pub fn new_block(&mut self, _label: String) -> BlockId {
        let id = BlockId(self.block_counter);
        self.block_counter += 1;
        id
    }

    /// Enters a basic block to start emitting instructions into it.
    pub fn enter_block(&mut self, block_id: BlockId, label: String) {
        // Save current block if exists
        if let Some(block) = self.current_block.take() {
            self.blocks_in_progress.push(block);
        }
        self.current_block = Some(BasicBlock::new(block_id, label));
    }

    /// Emits a non-terminator instruction into the current active block.
    pub fn emit_instruction(&mut self, op: Op, ty: IRType, span: Span) -> ValueId {
        let result_id = self.next_value_id();
        let inst_id = self.next_instruction_id();

        let inst = Instruction {
            id: inst_id,
            op,
            result: Some(result_id),
            ty,
            span,
            metadata: InstructionMetadata::default(),
        };

        if let Some(ref mut block) = self.current_block {
            block.instructions.push(inst);
        }

        result_id
    }

    /// Emits a store / load side-effect instruction with no return temporary register.
    pub fn emit_effect(&mut self, op: Op, span: Span) {
        let inst_id = self.next_instruction_id();

        let inst = Instruction {
            id: inst_id,
            op,
            result: None,
            ty: IRType::Void,
            span,
            metadata: InstructionMetadata::default(),
        };

        if let Some(ref mut block) = self.current_block {
            block.instructions.push(inst);
        }
    }

    /// Returns true if the current block already has a terminator.
    pub fn has_terminator(&self) -> bool {
        self.current_block.as_ref()
            .and_then(|b| b.terminator.as_ref())
            .is_some()
    }

    /// Emits a block terminator ending code emission for the current block.
    pub fn emit_terminator(&mut self, kind: TerminatorKind, span: Span) {
        if let Some(ref mut block) = self.current_block {
            if block.terminator.is_none() {
                block.terminator = Some(Terminator { kind, span });
            }
        }
    }

    /// Seals the current function and pushes it to functions list.
    pub fn seal_function(&mut self) {
        if let Some(block) = self.current_block.take() {
            self.blocks_in_progress.push(block);
        }

        if let Some(mut func) = self.current_function.take() {
            func.blocks = std::mem::take(&mut self.blocks_in_progress);

            // Reconstruct block links (CFG predecessors/successors)
            let mut label_to_id = HashMap::new();
            for block in &func.blocks {
                label_to_id.insert(block.label.clone(), block.id);
            }

            // Map successors/predecessors based on terminators
            let num_blocks = func.blocks.len();
            for i in 0..num_blocks {
                let mut succs = Vec::new();
                if let Some(ref term) = func.blocks[i].terminator {
                    match &term.kind {
                        TerminatorKind::Jump(target) => {
                            succs.push(*target);
                        }
                        TerminatorKind::ConditionalJump {
                            then_block,
                            else_block,
                            ..
                        } => {
                            succs.push(*then_block);
                            succs.push(*else_block);
                        }
                        _ => {}
                    }
                }
                func.blocks[i].successors = succs;
            }

            // Map predecessors
            for i in 0..num_blocks {
                let block_id = func.blocks[i].id;
                let mut preds = Vec::new();
                for other in &func.blocks {
                    if other.successors.contains(&block_id) {
                        preds.push(other.id);
                    }
                }
                func.blocks[i].predecessors = preds;
            }

            self.functions.push(func);
        }
    }

    /// Finalizes module emission, returning the final immutable Module.
    pub fn build(mut self, module_name: String) -> Module {
        self.seal_function();

        let mut module = Module::new(module_name);
        module.functions = std::mem::take(&mut self.functions);
        module.globals = std::mem::take(&mut self.globals);
        module.dsl_blocks = std::mem::take(&mut self.dsl_blocks);
        module.constants = std::mem::take(&mut self.constants);
        module.imports = std::mem::take(&mut self.imports);
        module.exports = std::mem::take(&mut self.exports);
        module
    }
}
