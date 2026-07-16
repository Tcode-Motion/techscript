use crate::analysis::AnalysisManager;
use crate::result::OptimizationResult;
use techscript_ir::Module;

/// The base trait that every IR optimization pass must implement.
pub trait OptimizationPass {
    /// Returns the static name of the optimization pass.
    fn name(&self) -> &'static str;

    /// Runs the optimization pass over the IR module.
    fn run(&mut self, module: &mut Module, analyses: &mut AnalysisManager) -> OptimizationResult;
}
