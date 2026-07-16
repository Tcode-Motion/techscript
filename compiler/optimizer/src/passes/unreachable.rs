use crate::analysis::AnalysisManager;
use crate::pass::OptimizationPass;
use crate::result::OptimizationResult;
use crate::statistics::PassStatistics;
use techscript_ir::Module;

/// Deletes CFG blocks that cannot be reached from the function entry point.
pub struct UnreachableBlockElimination;

impl OptimizationPass for UnreachableBlockElimination {
    fn name(&self) -> &'static str {
        "unreachable"
    }

    fn run(&mut self, module: &mut Module, _analyses: &mut AnalysisManager) -> OptimizationResult {
        let mut stats = PassStatistics::new(self.name());
        let mut changed = false;

        for func in &mut module.functions {
            if func.blocks.len() <= 1 {
                continue;
            }

            let initial_count = func.blocks.len();

            // Retain the entry block (index 0) and any block that has at least one predecessor
            let mut idx = 0;
            func.blocks.retain(|block| {
                let keep = idx == 0 || !block.predecessors.is_empty();
                idx += 1;
                keep
            });

            let removed = initial_count - func.blocks.len();
            if removed > 0 {
                stats.blocks_removed += removed;
                stats.changed = true;
                changed = true;
            }
        }

        if changed {
            OptimizationResult::changed(stats)
        } else {
            OptimizationResult::unchanged(self.name())
        }
    }
}
