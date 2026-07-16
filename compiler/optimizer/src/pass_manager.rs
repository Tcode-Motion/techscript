use crate::analysis::AnalysisManager;
use crate::pass::OptimizationPass;
use crate::result::OptimizationResult;
use crate::statistics::PassStatistics;
use crate::verifier::IRVerifier;
use techscript_ir::Module;

/// Orchestrates optimization passes, caching analyses, and verifying CFG state.
pub struct PassManager {
    pub analyses: AnalysisManager,
    passes: Vec<Box<dyn OptimizationPass>>,
    pub stats: PassStatistics,
}

impl Default for PassManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PassManager {
    /// Creates a new PassManager.
    pub fn new() -> Self {
        Self {
            analyses: AnalysisManager::new(),
            passes: Vec::new(),
            stats: PassStatistics::new("pass_manager"),
        }
    }

    /// Registers an optimization pass to the scheduler.
    pub fn add_pass(&mut self, pass: Box<dyn OptimizationPass>) {
        self.passes.push(pass);
    }

    /// Runs all scheduled passes on the module up to a fixed point or single pass iteration.
    pub fn run(&mut self, module: &mut Module) -> OptimizationResult {
        let mut overall_changed = false;

        // Run IR verification before optimizations (in debug mode)
        if cfg!(debug_assertions) {
            let verifier = IRVerifier::new();
            if let Err(msg) = verifier.verify(module) {
                panic!("IR Verifier failed before optimization passes: {}", msg);
            }
        }

        for pass in &mut self.passes {
            let start = std::time::Instant::now();
            let res = pass.run(module, &mut self.analyses);
            let duration = start.elapsed();

            let mut pass_stats = res.stats.clone();
            pass_stats.time_taken_ns = duration.as_nanos();

            self.stats.combine(&pass_stats);
            overall_changed |= res.changed;

            if res.changed {
                // Invalidate analysis caches since IR has changed
                self.analyses.invalidate_all();

                if cfg!(debug_assertions) {
                    let verifier = IRVerifier::new();
                    if let Err(msg) = verifier.verify(module) {
                        panic!("IR Verifier failed after pass '{}': {}", pass.name(), msg);
                    }
                }
            }
        }

        if overall_changed {
            OptimizationResult::changed(self.stats.clone())
        } else {
            OptimizationResult::unchanged("pass_manager")
        }
    }
}
