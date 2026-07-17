use std::collections::{HashMap, HashSet};
use techscript_ir::{BlockId, Function, Value, ValueId};

/// Liveness analysis result tracking live variables across block boundaries.
#[derive(Debug, Clone)]
pub struct LivenessAnalysis {
    pub live_in: HashMap<BlockId, HashSet<ValueId>>,
    pub live_out: HashMap<BlockId, HashSet<ValueId>>,
}

impl LivenessAnalysis {
    /// Computes live variables at entry and exit of each basic block.
    pub fn analyze(func: &Function) -> Self {
        let mut live_in = HashMap::new();
        let mut live_out = HashMap::new();
        let mut defs = HashMap::new();
        let mut uses = HashMap::new();

        // Calculate Def and Use sets for each block
        for block in &func.blocks {
            let mut block_def = HashSet::new();
            let mut block_use = HashSet::new();

            for inst in &block.instructions {
                // Collect operands used by the instruction
                let operands = Self::collect_operands(&inst.op);
                for val_id in operands {
                    if !block_def.contains(&val_id) {
                        block_use.insert(val_id);
                    }
                }
                // Collect register defined by the instruction
                if let Some(res) = inst.result {
                    block_def.insert(res);
                }
            }

            // Also check terminator condition operands
            if let Some(ref term) = block.terminator {
                match &term.kind {
                    techscript_ir::TerminatorKind::ConditionalJump {
                        cond: Value::Temp(val_id),
                        ..
                    } if !block_def.contains(val_id) => {
                        block_use.insert(*val_id);
                    }
                    techscript_ir::TerminatorKind::Return(Some(Value::Temp(val_id)))
                        if !block_def.contains(val_id) =>
                    {
                        block_use.insert(*val_id);
                    }
                    _ => {}
                }
            }

            defs.insert(block.id, block_def);
            uses.insert(block.id, block_use);
            live_in.insert(block.id, HashSet::new());
            live_out.insert(block.id, HashSet::new());
        }

        // Iterative dataflow solver
        let mut changed = true;
        while changed {
            changed = false;
            for block in &func.blocks {
                let block_id = block.id;

                // LiveOut = union of LiveIn(succs)
                let mut new_out = HashSet::new();
                for &succ in &block.successors {
                    if let Some(succ_in) = live_in.get(&succ) {
                        new_out.extend(succ_in);
                    }
                }

                // LiveIn = Use union (LiveOut diff Def)
                let mut new_in = uses[&block_id].clone();
                let def = &defs[&block_id];
                for &val in &new_out {
                    if !def.contains(&val) {
                        new_in.insert(val);
                    }
                }

                if let Some(old_in) = live_in.get_mut(&block_id) {
                    if *old_in != new_in {
                        *old_in = new_in;
                        changed = true;
                    }
                }

                if let Some(old_out) = live_out.get_mut(&block_id) {
                    if *old_out != new_out {
                        *old_out = new_out;
                        changed = true;
                    }
                }
            }
        }

        Self { live_in, live_out }
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
            techscript_ir::Op::Try { .. } | techscript_ir::Op::EndTry => {}
            techscript_ir::Op::NoOp => {}
        }
        result
    }
}
