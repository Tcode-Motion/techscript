use crate::analysis::AnalysisManager;
use crate::pass::OptimizationPass;
use crate::result::OptimizationResult;
use crate::statistics::PassStatistics;
use techscript_ir::instruction::Op;
use techscript_ir::Module;

/// Removes unused temporary register definitions and unreachable operations.
pub struct DeadCode;

impl OptimizationPass for DeadCode {
    fn name(&self) -> &'static str {
        "dead_code"
    }

    fn run(&mut self, module: &mut Module, analyses: &mut AnalysisManager) -> OptimizationResult {
        let mut stats = PassStatistics::new(self.name());
        let mut changed = false;

        for func in &mut module.functions {
            // Retrieve Use-Def analysis
            let use_def = analyses.get_use_def(func).clone();

            for block in &mut func.blocks {
                block.instructions.retain(|inst| {
                    if let Some(res) = inst.result {
                        if !use_def.uses.contains_key(&res) {
                            // Only remove if it has no side-effects
                            if !Self::has_side_effects(&inst.op) {
                                stats.instructions_removed += 1;
                                stats.changed = true;
                                changed = true;
                                return false; // discard
                            }
                        }
                    }
                    true
                });
            }
        }

        if changed {
            OptimizationResult::changed(stats)
        } else {
            OptimizationResult::unchanged(self.name())
        }
    }
}

impl DeadCode {
    fn has_side_effects(op: &Op) -> bool {
        matches!(
            op,
            Op::Call { .. } | Op::Store { .. } | Op::FieldStore { .. } | Op::IndexStore { .. }
        )
    }
}
