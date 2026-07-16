use crate::builder::{BytecodeBuilder, Label};
use crate::function::BytecodeFunction;
use crate::module::BytecodeModule;
use crate::opcode::Opcode;
use crate::operand::Operand;
use std::collections::HashMap;
use techscript_ir::instruction::{Instruction, Op, TerminatorKind};
use techscript_ir::types::{BlockId, LocalId, ValueId};
use techscript_ir::value::Value;

/// Lowers flat SSA IR modules into stack-based VM bytecode.
pub struct BytecodeLowerer {
    builder: BytecodeBuilder,
    block_labels: HashMap<BlockId, Label>,
    local_map: HashMap<LocalId, u32>,
    temp_map: HashMap<ValueId, u32>,
}

impl BytecodeLowerer {
    /// Creates a new lowerer.
    pub fn new(name: String, param_count: u32) -> Self {
        Self {
            builder: BytecodeBuilder::new(name, param_count),
            block_labels: HashMap::new(),
            local_map: HashMap::new(),
            temp_map: HashMap::new(),
        }
    }

    /// Lowers an entire IR module.
    pub fn lower(module: &techscript_ir::Module) -> BytecodeModule {
        let mut bc_module = BytecodeModule::new(module.name.clone());

        for global in &module.globals {
            bc_module.globals.push((global.1.clone(), global.2.clone()));
        }
        bc_module.imports = module.imports.clone();
        bc_module.exports = module.exports.clone();

        for func in &module.functions {
            let lowerer = BytecodeLowerer::new(func.name.clone(), func.params.len() as u32);
            let bc_func = lowerer.lower_function(func);
            bc_module.functions.push(bc_func);
        }

        bc_module
    }

    fn lower_function(mut self, func: &techscript_ir::Function) -> BytecodeFunction {
        // Map parameters to local slots
        for param in &func.params {
            let slot = self.builder.allocate_local(param.1.clone());
            self.local_map.insert(param.0, slot);
        }

        // Pre-create labels for all basic blocks
        for block in &func.blocks {
            let label = self.builder.make_label();
            self.block_labels.insert(block.id, label);
        }

        // Lower blocks sequentially
        for block in &func.blocks {
            let label = self.block_labels[&block.id];
            self.builder.mark_label(label);

            for inst in &block.instructions {
                self.lower_instruction(inst);
            }

            if let Some(ref term) = block.terminator {
                match &term.kind {
                    TerminatorKind::Jump(target) => {
                        let lbl = self.block_labels[target];
                        self.builder.emit_jump(
                            Opcode::Jump,
                            lbl,
                            term.span,
                            techscript_ir::types::InstructionId(9999),
                        );
                    }
                    TerminatorKind::ConditionalJump {
                        cond,
                        then_block,
                        else_block,
                    } => {
                        self.emit_load(cond, term.span);
                        let then_lbl = self.block_labels[then_block];
                        let else_lbl = self.block_labels[else_block];

                        self.builder.emit_jump(
                            Opcode::JumpIfTrue,
                            then_lbl,
                            term.span,
                            techscript_ir::types::InstructionId(9999),
                        );
                        self.builder.emit_jump(
                            Opcode::Jump,
                            else_lbl,
                            term.span,
                            techscript_ir::types::InstructionId(9999),
                        );
                    }
                    TerminatorKind::Return(val) => {
                        if let Some(ref v) = val {
                            self.emit_load(v, term.span);
                        }
                        self.builder.emit(
                            Opcode::Return,
                            Vec::new(),
                            term.span,
                            techscript_ir::types::InstructionId(9999),
                        );
                    }
                    TerminatorKind::Unreachable => {
                        self.builder.emit(
                            Opcode::Throw,
                            Vec::new(),
                            term.span,
                            techscript_ir::types::InstructionId(9999),
                        );
                    }
                }
            }
        }

        self.builder.finish()
    }

    fn lower_instruction(&mut self, inst: &Instruction) {
        match &inst.op {
            Op::Constant(lit) => {
                let const_idx = self.builder.constants.add(lit.clone());
                self.builder.emit(
                    Opcode::LoadConst,
                    vec![Operand::ConstantIndex(const_idx)],
                    inst.span,
                    inst.id,
                );
                self.store_result(inst);
            }
            Op::Load(val) => {
                self.emit_load(val, inst.span);
                self.store_result(inst);
            }
            Op::Store { target, value } => {
                self.emit_load(value, inst.span);
                self.emit_store(target, inst.span);
            }
            Op::Move { target, value } => {
                self.emit_load(value, inst.span);
                let slot = self.get_or_allocate_temp(*target);
                self.builder.emit(
                    Opcode::StoreLocal,
                    vec![Operand::LocalIndex(slot)],
                    inst.span,
                    inst.id,
                );
            }
            Op::BinaryOp { op, left, right } => {
                self.emit_load(left, inst.span);
                self.emit_load(right, inst.span);
                let opcode = match op {
                    techscript_syntax::TokenKind::Plus => Opcode::Add,
                    techscript_syntax::TokenKind::Minus => Opcode::Sub,
                    techscript_syntax::TokenKind::Star => Opcode::Mul,
                    techscript_syntax::TokenKind::Slash => Opcode::Div,
                    _ => Opcode::Add,
                };
                self.builder.emit(opcode, Vec::new(), inst.span, inst.id);
                self.store_result(inst);
            }
            Op::UnaryOp { op, right } => {
                self.emit_load(right, inst.span);
                let opcode = match op {
                    techscript_syntax::TokenKind::Minus => Opcode::Neg,
                    techscript_syntax::TokenKind::Not => Opcode::Not,
                    _ => Opcode::Not,
                };
                self.builder.emit(opcode, Vec::new(), inst.span, inst.id);
                self.store_result(inst);
            }
            Op::Compare { op, left, right } => {
                self.emit_load(left, inst.span);
                self.emit_load(right, inst.span);
                let opcode = match op {
                    techscript_syntax::TokenKind::EqualEqual => Opcode::Equal,
                    techscript_syntax::TokenKind::TripleEqual => Opcode::StrictEqual,
                    techscript_syntax::TokenKind::BangEqual => Opcode::NotEqual,
                    techscript_syntax::TokenKind::Less => Opcode::Less,
                    techscript_syntax::TokenKind::LessEqual => Opcode::LessEqual,
                    techscript_syntax::TokenKind::Greater => Opcode::Greater,
                    techscript_syntax::TokenKind::GreaterEqual => Opcode::GreaterEqual,
                    _ => Opcode::Equal,
                };
                self.builder.emit(opcode, Vec::new(), inst.span, inst.id);
                self.store_result(inst);
            }
            Op::Call { callee, args } => {
                self.emit_load(callee, inst.span);
                for arg in args {
                    self.emit_load(arg, inst.span);
                }
                self.builder.emit(
                    Opcode::Call,
                    vec![Operand::Count(args.len() as u32)],
                    inst.span,
                    inst.id,
                );
                self.store_result(inst);
            }
            Op::IndexLoad { base, index } => {
                self.emit_load(base, inst.span);
                self.emit_load(index, inst.span);
                self.builder
                    .emit(Opcode::IndexLoad, Vec::new(), inst.span, inst.id);
                self.store_result(inst);
            }
            Op::IndexStore { base, index, value } => {
                self.emit_load(base, inst.span);
                self.emit_load(index, inst.span);
                self.emit_load(value, inst.span);
                self.builder
                    .emit(Opcode::IndexStore, Vec::new(), inst.span, inst.id);
            }
            Op::FieldLoad { base, field } => {
                self.emit_load(base, inst.span);
                let const_idx = self
                    .builder
                    .constants
                    .add(techscript_ast::LiteralVal::Str(field.clone()));
                self.builder.emit(
                    Opcode::FieldLoad,
                    vec![Operand::ConstantIndex(const_idx)],
                    inst.span,
                    inst.id,
                );
                self.store_result(inst);
            }
            Op::FieldStore { base, field, value } => {
                self.emit_load(base, inst.span);
                self.emit_load(value, inst.span);
                let const_idx = self
                    .builder
                    .constants
                    .add(techscript_ast::LiteralVal::Str(field.clone()));
                self.builder.emit(
                    Opcode::FieldStore,
                    vec![Operand::ConstantIndex(const_idx)],
                    inst.span,
                    inst.id,
                );
            }
            Op::MakeList(elems) => {
                for elem in elems {
                    self.emit_load(elem, inst.span);
                }
                self.builder.emit(
                    Opcode::MakeList,
                    vec![Operand::Count(elems.len() as u32)],
                    inst.span,
                    inst.id,
                );
                self.store_result(inst);
            }
            Op::MakeMap(entries) => {
                for (k, v) in entries {
                    self.emit_load(k, inst.span);
                    self.emit_load(v, inst.span);
                }
                self.builder.emit(
                    Opcode::MakeMap,
                    vec![Operand::Count(entries.len() as u32)],
                    inst.span,
                    inst.id,
                );
                self.store_result(inst);
            }
            _ => {
                // Default NoOp for unimplemented placeholders
                self.builder
                    .emit(Opcode::NoOp, Vec::new(), inst.span, inst.id);
                self.store_result(inst);
            }
        }
    }

    fn emit_load(&mut self, val: &Value, span: techscript_common::Span) {
        match val {
            Value::Const(lit) => {
                let const_idx = self.builder.constants.add(lit.clone());
                self.builder.emit(
                    Opcode::LoadConst,
                    vec![Operand::ConstantIndex(const_idx)],
                    span,
                    techscript_ir::types::InstructionId(9999),
                );
            }
            Value::Local(lid) => {
                let slot = self.get_or_allocate_local(*lid);
                self.builder.emit(
                    Opcode::LoadLocal,
                    vec![Operand::LocalIndex(slot)],
                    span,
                    techscript_ir::types::InstructionId(9999),
                );
            }
            Value::Temp(vid) => {
                let slot = self.get_or_allocate_temp(*vid);
                self.builder.emit(
                    Opcode::LoadLocal,
                    vec![Operand::LocalIndex(slot)],
                    span,
                    techscript_ir::types::InstructionId(9999),
                );
            }
            Value::Global(gid) => {
                self.builder.emit(
                    Opcode::LoadGlobal,
                    vec![Operand::GlobalIndex(gid.0)],
                    span,
                    techscript_ir::types::InstructionId(9999),
                );
            }
            Value::Null => {
                let const_idx = self.builder.constants.add(techscript_ast::LiteralVal::None);
                self.builder.emit(
                    Opcode::LoadConst,
                    vec![Operand::ConstantIndex(const_idx)],
                    span,
                    techscript_ir::types::InstructionId(9999),
                );
            }
        }
    }

    fn emit_store(&mut self, target: &Value, span: techscript_common::Span) {
        match target {
            Value::Local(lid) => {
                let slot = self.get_or_allocate_local(*lid);
                self.builder.emit(
                    Opcode::StoreLocal,
                    vec![Operand::LocalIndex(slot)],
                    span,
                    techscript_ir::types::InstructionId(9999),
                );
            }
            Value::Temp(vid) => {
                let slot = self.get_or_allocate_temp(*vid);
                self.builder.emit(
                    Opcode::StoreLocal,
                    vec![Operand::LocalIndex(slot)],
                    span,
                    techscript_ir::types::InstructionId(9999),
                );
            }
            Value::Global(gid) => {
                self.builder.emit(
                    Opcode::StoreGlobal,
                    vec![Operand::GlobalIndex(gid.0)],
                    span,
                    techscript_ir::types::InstructionId(9999),
                );
            }
            _ => {}
        }
    }

    fn store_result(&mut self, inst: &Instruction) {
        if let Some(res) = inst.result {
            let slot = self.get_or_allocate_temp(res);
            self.builder.emit(
                Opcode::StoreLocal,
                vec![Operand::LocalIndex(slot)],
                inst.span,
                inst.id,
            );
        }
    }

    fn get_or_allocate_local(&mut self, lid: LocalId) -> u32 {
        *self
            .local_map
            .entry(lid)
            .or_insert_with(|| self.builder.allocate_local(format!("local_{}", lid.0)))
    }

    fn get_or_allocate_temp(&mut self, vid: ValueId) -> u32 {
        *self
            .temp_map
            .entry(vid)
            .or_insert_with(|| self.builder.allocate_local(format!("temp_{}", vid.0)))
    }
}
