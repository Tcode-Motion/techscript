use crate::error::VMError;
use techscript_runtime::RuntimeValue;

/// Unified value stack for variables and execution evaluations.
pub struct ValueStack {
    data: Vec<RuntimeValue>,
    max_size: usize,
}

impl ValueStack {
    /// Creates a value stack with fixed capacity.
    pub fn new(max_size: usize) -> Self {
        Self {
            data: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Pushes a value to the stack, checking for overflow.
    pub fn push(&mut self, val: RuntimeValue) -> Result<(), VMError> {
        if self.data.len() >= self.max_size {
            return Err(VMError::StackOverflow);
        }
        self.data.push(val);
        Ok(())
    }

    /// Pops a value off the stack, checking for underflow.
    pub fn pop(&mut self) -> Result<RuntimeValue, VMError> {
        self.data.pop().ok_or(VMError::StackUnderflow)
    }

    /// Returns a reference to the top value.
    pub fn peek(&self) -> Result<&RuntimeValue, VMError> {
        self.data.last().ok_or(VMError::StackUnderflow)
    }

    /// Returns a reference to the value at offset index from base.
    pub fn get(&self, idx: usize) -> Result<&RuntimeValue, VMError> {
        self.data.get(idx).ok_or(VMError::StackUnderflow)
    }

    /// Sets the value at offset index.
    pub fn set(&mut self, idx: usize, val: RuntimeValue) -> Result<(), VMError> {
        if idx >= self.data.len() {
            // Fill intermediates with Null if needed
            while self.data.len() <= idx {
                if self.data.len() >= self.max_size {
                    return Err(VMError::StackOverflow);
                }
                self.data.push(RuntimeValue::Null);
            }
        }
        self.data[idx] = val;
        Ok(())
    }

    /// Truncates the stack size to a fixed depth.
    pub fn truncate(&mut self, size: usize) {
        self.data.truncate(size);
    }

    /// Returns the current stack height.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Debug print utility helper.
    pub fn get_dump(&self) -> Vec<RuntimeValue> {
        self.data.clone()
    }
}
