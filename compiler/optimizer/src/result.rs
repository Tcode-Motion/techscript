use crate::statistics::PassStatistics;
use serde::{Deserialize, Serialize};

/// The execution outcome of an optimization pass run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub changed: bool,
    pub stats: PassStatistics,
}

impl OptimizationResult {
    /// Creates a success result with no modifications.
    pub fn unchanged(name: &str) -> Self {
        Self {
            changed: false,
            stats: PassStatistics::new(name),
        }
    }

    /// Creates a result carrying changes.
    pub fn changed(stats: PassStatistics) -> Self {
        Self {
            changed: true,
            stats,
        }
    }
}
