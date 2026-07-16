/// Metrics profiling instruction metrics, stack ranges, and memory.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VMProfiler {
    pub instruction_count: usize,
    pub function_calls: usize,
    pub max_stack_height: usize,
}

impl VMProfiler {
    /// Creates a new VMProfiler tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records instruction invocation metrics.
    pub fn record_instruction(&mut self) {
        self.instruction_count += 1;
    }

    /// Records function call triggers.
    pub fn record_call(&mut self) {
        self.function_calls += 1;
    }

    /// Logs stack heights dynamically.
    pub fn record_stack_height(&mut self, current: usize) {
        if current > self.max_stack_height {
            self.max_stack_height = current;
        }
    }
}
