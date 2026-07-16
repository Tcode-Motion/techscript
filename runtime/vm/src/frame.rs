/// Exception handler tracking execution jump catch points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExceptionHandler {
    pub catch_ip: usize,
    pub stack_depth: usize,
}

/// Call frame containing current instruction pointer and local variable offsets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallFrame {
    pub function_idx: u32,
    pub ip: usize,
    pub base_pointer: usize,
    pub handlers: Vec<ExceptionHandler>,
}

impl CallFrame {
    /// Creates a new CallFrame.
    pub fn new(function_idx: u32, base_pointer: usize) -> Self {
        Self {
            function_idx,
            ip: 0,
            base_pointer,
            handlers: Vec::new(),
        }
    }
}
