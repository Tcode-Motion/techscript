//! # TechScript Optimizer Crate
//!
//! Production-quality modular optimization pipeline for TechScript 2.0 IR.

pub mod analysis;
pub mod context;
pub mod pass;
pub mod pass_manager;
pub mod passes;
pub mod pipeline;
pub mod result;
pub mod statistics;
pub mod verifier;

pub use context::{OptimizationContext, OptimizationLevel};
pub use pass::OptimizationPass;
pub use pass_manager::PassManager;
pub use pipeline::OptimizationPipeline;
pub use result::OptimizationResult;
pub use statistics::PassStatistics;
pub use verifier::IRVerifier;

/// Convenience function to optimize an IR module with the specified context.
pub fn optimize(
    module: &mut techscript_ir::Module,
    ctx: &OptimizationContext,
) -> OptimizationResult {
    let mut pipeline = OptimizationPipeline::new(ctx);
    pipeline.optimize(module)
}
