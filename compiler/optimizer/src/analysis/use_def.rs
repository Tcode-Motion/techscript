use std::collections::{HashMap, HashSet};
use techscript_ir::{Function, InstructionId, Value, ValueId};

/// Use-Def chain tracker mapping value definitions and read usages.
#[derive(Debug, Clone)]
pub struct UseDefAnalysis {
    pub defs: HashMap<ValueId, InstructionId>,
    pub uses: HashMap<ValueId, HashSet<InstructionId>>,
}

impl UseDefAnalysis {
    /// Builds Use-Def chains by traversing function blocks.
    pub fn analyze(func: &Function) -> Self {
        let mut defs = HashMap::new();
        let mut uses = HashMap::new();

        for block in &func.blocks {
            for inst in &block.instructions {
                if let Some(res) = inst.result {
                    defs.insert(res, inst.id);
                }

                // Check operand reads
                let operands = Self::collect_operands(&inst.op);
                for val_id in operands {
                    uses.entry(val_id)
                        .or_insert_with(HashSet::new)
                        .insert(inst.id);
                }
            }
        }

        Self { defs, uses }
    }

    fn collect_operands(op: &techscript_ir::Op) -> Vec<ValueId> {
        let mut result = Vec::new();
        let mut add_val = |v: &Value| {
            if let Value::Temp(vid) = v {
                result.push(*vid);
            }
        };

        match op {
            techscript_ir::Op::Constant(_) => {}
            techscript_ir::Op::Load(v) => add_val(v),
            techscript_ir::Op::Store { target, value } => {
                add_val(target);
                add_val(value);
            }
            techscript_ir::Op::Move { value, .. } => add_val(value),
            techscript_ir::Op::BinaryOp { left, right, .. } => {
                add_val(left);
                add_val(right);
            }
            techscript_ir::Op::UnaryOp { right, .. } => add_val(right),
            techscript_ir::Op::Compare { left, right, .. } => {
                add_val(left);
                add_val(right);
            }
            techscript_ir::Op::Phi(paths) => {
                for (_, v) in paths {
                    add_val(v);
                }
            }
            techscript_ir::Op::Call { callee, args } => {
                add_val(callee);
                for arg in args {
                    add_val(arg);
                }
            }
            techscript_ir::Op::Allocate(_) => {}
            techscript_ir::Op::FieldLoad { base, .. } => add_val(base),
            techscript_ir::Op::FieldStore { base, value, .. } => {
                add_val(base);
                add_val(value);
            }
            techscript_ir::Op::IndexLoad { base, index } => {
                add_val(base);
                add_val(index);
            }
            techscript_ir::Op::IndexStore { base, index, value } => {
                add_val(base);
                add_val(index);
                add_val(value);
            }
            techscript_ir::Op::MakeStruct { fields, .. } => {
                for (_, v) in fields {
                    add_val(v);
                }
            }
            techscript_ir::Op::MakeEnum { value, .. } => {
                if let Some(ref v) = value {
                    add_val(v);
                }
            }
            techscript_ir::Op::MakeModel { fields, .. } => {
                for (_, v) in fields {
                    add_val(v);
                }
            }
            techscript_ir::Op::MakeList(elems) => {
                for elem in elems {
                    add_val(elem);
                }
            }
            techscript_ir::Op::MakeMap(entries) => {
                for (k, v) in entries {
                    add_val(k);
                    add_val(v);
                }
            }
            techscript_ir::Op::Cast { value, .. } => add_val(value),
            techscript_ir::Op::NoOp => {}
        }
        result
    }
}
