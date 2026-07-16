use crate::module::BytecodeModule;
use crate::operand::Operand;

/// Performs integrity checks on function chunks, indices, and jump offsets.
pub struct BytecodeValidator;

impl Default for BytecodeValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl BytecodeValidator {
    /// Creates a new BytecodeValidator.
    pub fn new() -> Self {
        Self
    }

    /// Validates the module.
    pub fn validate(&self, module: &BytecodeModule) -> Result<(), String> {
        for func in &module.functions {
            let num_insts = func.chunk.instructions.len();
            let mut stack_height = 0i32;

            for (idx, inst) in func.chunk.instructions.iter().enumerate() {
                // 1. Stack height check
                let effect = inst.op.stack_effect();
                stack_height += effect;
                if stack_height < 0 {
                    return Err(format!(
                        "Validation error in function '{}': Stack underflow at instruction index {}",
                        func.name, idx
                    ));
                }

                // 2. Operand checks
                for op in &inst.operands {
                    match op {
                        Operand::ConstantIndex(c_idx) => {
                            if func.chunk.constants.get(*c_idx).is_none() {
                                return Err(format!(
                                    "Validation error in function '{}': Reference to invalid ConstantPool index {}",
                                    func.name, c_idx
                                ));
                            }
                        }
                        Operand::JumpOffset(offset) => {
                            let target = (idx as i32) + offset;
                            if target < 0 || target >= (num_insts as i32) {
                                return Err(format!(
                                    "Validation error in function '{}': Out of bounds jump offset {} targeting index {}",
                                    func.name, offset, target
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
