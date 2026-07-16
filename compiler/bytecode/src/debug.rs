use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Debug symbols retaining identifier names for debug printing and tracing.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DebugSymbols {
    pub local_names: HashMap<u32, String>,
    pub function_names: HashMap<u32, String>,
}

impl DebugSymbols {
    /// Creates an empty DebugSymbols table.
    pub fn new() -> Self {
        Self::default()
    }
}
