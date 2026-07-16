use serde::{Deserialize, Serialize};

/// Detailed metrics tracking IR changes during optimization passes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassStatistics {
    pub pass_name: String,
    pub changed: bool,
    pub instructions_removed: usize,
    pub instructions_added: usize,
    pub blocks_removed: usize,
    pub blocks_added: usize,
    pub constants_folded: usize,
    pub branches_simplified: usize,
    pub time_taken_ns: u128,
}

impl PassStatistics {
    /// Creates empty statistics for a pass name.
    pub fn new(name: &str) -> Self {
        Self {
            pass_name: name.to_string(),
            ..Default::default()
        }
    }

    /// Combines metrics from another pass run.
    pub fn combine(&mut self, other: &PassStatistics) {
        self.changed |= other.changed;
        self.instructions_removed += other.instructions_removed;
        self.instructions_added += other.instructions_added;
        self.blocks_removed += other.blocks_removed;
        self.blocks_added += other.blocks_added;
        self.constants_folded += other.constants_folded;
        self.branches_simplified += other.branches_simplified;
        self.time_taken_ns += other.time_taken_ns;
    }
}
