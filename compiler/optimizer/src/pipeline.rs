use crate::context::{OptimizationContext, OptimizationLevel};
use crate::pass_manager::PassManager;
use crate::passes::*;
use crate::result::OptimizationResult;
use techscript_ir::Module;

/// Executes Registered passes up to a fixed point with safety thresholds.
pub struct OptimizationPipeline {
    pub manager: PassManager,
    pub max_iterations: usize,
}

impl OptimizationPipeline {
    /// Creates a pipeline configured by the given context.
    pub fn new(ctx: &OptimizationContext) -> Self {
        let mut manager = PassManager::new();
        let mut max_iterations = 1;

        match ctx.level {
            OptimizationLevel::O0 => {
                // Verification checks only
            }
            OptimizationLevel::O1 => {
                manager.add_pass(Box::new(Canonicalize));
                manager.add_pass(Box::new(CopyPropagation));
                manager.add_pass(Box::new(CFGCleanup));
            }
            OptimizationLevel::O2 => {
                manager.add_pass(Box::new(Canonicalize));
                manager.add_pass(Box::new(ConstantFolding));
                manager.add_pass(Box::new(ConstantPropagation));
                manager.add_pass(Box::new(CopyPropagation));
                manager.add_pass(Box::new(Peephole));
                manager.add_pass(Box::new(CFGCleanup));
            }
            OptimizationLevel::O3 => {
                manager.add_pass(Box::new(Canonicalize));
                manager.add_pass(Box::new(ConstantFolding));
                manager.add_pass(Box::new(ConstantPropagation));
                manager.add_pass(Box::new(CopyPropagation));
                manager.add_pass(Box::new(AlgebraicSimplification));
                manager.add_pass(Box::new(BranchSimplification));
                manager.add_pass(Box::new(DeadCode));
                manager.add_pass(Box::new(DeadStore));
                manager.add_pass(Box::new(Peephole));
                manager.add_pass(Box::new(UnreachableBlockElimination));
                manager.add_pass(Box::new(CFGCleanup));
                max_iterations = 5; // Run fixed point loop up to 5 times
            }
            OptimizationLevel::Os => {
                manager.add_pass(Box::new(Canonicalize));
                manager.add_pass(Box::new(DeadStore));
                manager.add_pass(Box::new(Peephole));
                manager.add_pass(Box::new(CFGCleanup));
            }
        }

        Self {
            manager,
            max_iterations,
        }
    }

    /// Optimizes the given module by running passes up to fixed point.
    pub fn optimize(&mut self, module: &mut Module) -> OptimizationResult {
        let mut overall_changed = false;

        for _ in 0..self.max_iterations {
            let res = self.manager.run(module);
            if !res.changed {
                break;
            }
            overall_changed = true;
        }

        if overall_changed {
            OptimizationResult::changed(self.manager.stats)
        } else {
            OptimizationResult::unchanged("pipeline")
        }
    }
}
