use serde::{Deserialize, Serialize};

/// Supported optimization presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// No optimizations, verifier checks only.
    O0,
    /// Basic safe optimizations (DCE, CFG cleanup, Copy Propagation).
    O1,
    /// Default production optimizations (Constant folding/propagation, Peephole).
    O2,
    /// Aggressive optimizations (Fixed point runs, intensive algebraic simplifications).
    O3,
    /// Prefer smaller code footprint.
    Os,
}

/// Global settings configuring the optimizer run behaviors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationContext {
    pub level: OptimizationLevel,
    pub debug_mode: bool,
    pub size_mode: bool,
    pub fast_math: bool,
    pub unsafe_opts: bool,
}

impl Default for OptimizationContext {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationContext {
    /// Creates a new OptimizationContext with O2 defaults.
    pub fn new() -> Self {
        Self {
            level: OptimizationLevel::O2,
            debug_mode: false,
            size_mode: false,
            fast_math: false,
            unsafe_opts: false,
        }
    }
}
