use crate::analysis::AnalysisManager;
use crate::pass::OptimizationPass;
use crate::result::OptimizationResult;
use crate::statistics::PassStatistics;
use std::collections::HashMap;
use techscript_ast::LiteralVal;
use techscript_ir::instruction::Op;
use techscript_ir::types::LocalId;
use techscript_ir::value::Value;
use techscript_ir::Module;

/// Propagates constant variables from stores to loads.
pub struct ConstantPropagation;

impl OptimizationPass for ConstantPropagation {
    fn name(&self) -> &'static str {
        "constant_propagation"
    }

    fn run(&mut self, module: &mut Module, _analyses: &mut AnalysisManager) -> OptimizationResult {
        let mut stats = PassStatistics::new(self.name());
        let mut changed = false;

        for func in &mut module.functions {
            let mut local_constants: HashMap<LocalId, LiteralVal> = HashMap::new();

            for block in &mut func.blocks {
                for inst in &mut block.instructions {
                    // Update loaded variables if they are known constants
                    if let Op::Load(Value::Local(local_id)) = &inst.op {
                        if let Some(lit) = local_constants.get(local_id) {
                            inst.op = Op::Constant(lit.clone());
                            stats.changed = true;
                            stats.instructions_removed += 1;
                            changed = true;
                        }
                    }

                    // Replace temporary operands inside ops if they reference folded constants
                    Self::propagate_in_op(&mut inst.op, &local_constants);

                    // Track store values into local registers
                    if let Op::Store {
                        target: Value::Local(local_id),
                        value: Value::Const(lit),
                    } = &inst.op
                    {
                        local_constants.insert(*local_id, lit.clone());
                    } else if let Op::Store {
                        target: Value::Local(local_id),
                        ..
                    } = &inst.op
                    {
                        // If stored value is not constant, invalidate the constant state of local
                        local_constants.remove(local_id);
                    }
                }
            }
        }

        if changed {
            OptimizationResult::changed(stats)
        } else {
            OptimizationResult::unchanged(self.name())
        }
    }
}

impl ConstantPropagation {
    fn propagate_in_op(_op: &mut Op, _constants: &HashMap<LocalId, LiteralVal>) {
        // Implement recursive operand propagation if needed
    }
}
