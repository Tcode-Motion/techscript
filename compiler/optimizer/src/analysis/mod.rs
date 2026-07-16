pub mod cfg;
pub mod dominators;
pub mod liveness;
pub mod use_def;

pub use cfg::CFGAnalysis;
pub use dominators::DominatorAnalysis;
pub use liveness::LivenessAnalysis;
pub use use_def::UseDefAnalysis;

use std::collections::HashMap;
use techscript_ir::{Function, FunctionId};

/// Caches and invalidates intermediate IR analysis results dynamically.
#[derive(Default)]
pub struct AnalysisManager {
    cfg_cache: HashMap<FunctionId, CFGAnalysis>,
    dom_cache: HashMap<FunctionId, DominatorAnalysis>,
    live_cache: HashMap<FunctionId, LivenessAnalysis>,
    ud_cache: HashMap<FunctionId, UseDefAnalysis>,
}

impl AnalysisManager {
    /// Creates a new empty AnalysisManager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Invalidates all cached analyses for a function.
    pub fn invalidate(&mut self, func_id: FunctionId) {
        self.cfg_cache.remove(&func_id);
        self.dom_cache.remove(&func_id);
        self.live_cache.remove(&func_id);
        self.ud_cache.remove(&func_id);
    }

    /// Invalidates all caches completely.
    pub fn invalidate_all(&mut self) {
        self.cfg_cache.clear();
        self.dom_cache.clear();
        self.live_cache.clear();
        self.ud_cache.clear();
    }

    /// Retrieves or computes CFG analysis for a function.
    pub fn get_cfg(&mut self, func: &Function) -> &CFGAnalysis {
        self.cfg_cache
            .entry(func.id)
            .or_insert_with(|| CFGAnalysis::analyze(func))
    }

    /// Retrieves or computes Dominator analysis for a function.
    pub fn get_dominators(&mut self, func: &Function) -> &DominatorAnalysis {
        self.dom_cache
            .entry(func.id)
            .or_insert_with(|| DominatorAnalysis::analyze(func))
    }

    /// Retrieves or computes Liveness analysis for a function.
    pub fn get_liveness(&mut self, func: &Function) -> &LivenessAnalysis {
        self.live_cache
            .entry(func.id)
            .or_insert_with(|| LivenessAnalysis::analyze(func))
    }

    /// Retrieves or computes Use-Def analysis for a function.
    pub fn get_use_def(&mut self, func: &Function) -> &UseDefAnalysis {
        self.ud_cache
            .entry(func.id)
            .or_insert_with(|| UseDefAnalysis::analyze(func))
    }
}
