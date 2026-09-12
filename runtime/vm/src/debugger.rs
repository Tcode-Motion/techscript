use std::collections::HashSet;
use std::fmt::Write;
use techscript_bytecode::{BytecodeFunction, Operand};

/// VM debugger supporting breakpoint registries and opcode single-step tracing.
pub struct VMDebugger {
    breakpoints: HashSet<(u32, usize)>, // (FunctionIndex, InstructionOffset)
    enabled: bool,
}

impl Default for VMDebugger {
    fn default() -> Self {
        Self::new()
    }
}

impl VMDebugger {
    /// Creates a debugger context.
    pub fn new() -> Self {
        Self {
            breakpoints: HashSet::new(),
            enabled: false,
        }
    }

    /// Toggles active trace logging.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Registers a breakpoint.
    pub fn add_breakpoint(&mut self, func_idx: u32, offset: usize) {
        self.breakpoints.insert((func_idx, offset));
    }

    /// Evaluates if a breakpoint exists at current execution coordinates.
    pub fn check_breakpoint(&self, func_idx: u32, offset: usize) -> bool {
        self.breakpoints.contains(&(func_idx, offset))
    }

    /// Prints a trace of the instruction and the current stack values.
    pub fn trace_instruction(
        &self,
        func: &BytecodeFunction,
        ip: usize,
        op_code: techscript_bytecode::Opcode,
        operands: &[techscript_bytecode::Operand],
        stack_dump: &[techscript_runtime::RuntimeValue],
    ) {
        if !self.enabled {
            return;
        }

        let op_str = format!("{:?}", op_code);
        let mut operands_str = String::new();

        for op in operands {
            match op {
                Operand::ConstantIndex(c_idx) => {
                    if let Some(lit) = func.chunk.constants.get(*c_idx) {
                        let _ = write!(operands_str, " #{:<3} ({:?})", c_idx, lit);
                    } else {
                        let _ = write!(operands_str, " #{:<3} (invalid)", c_idx);
                    }
                }
                Operand::LocalIndex(l_idx) => {
                    if let Some(name) = func.debug_symbols.local_names.get(l_idx) {
                        let _ = write!(operands_str, " local_{:<2} ({})", l_idx, name);
                    } else {
                        let _ = write!(operands_str, " local_{}", l_idx);
                    }
                }
                Operand::JumpOffset(offset) => {
                    let target = (ip as i32) + offset;
                    let _ = write!(
                        operands_str,
                        " offset_{:<3} (target: {:04})",
                        offset, target
                    );
                }
                Operand::Count(n) => {
                    let _ = write!(operands_str, " count_{}", n);
                }
                Operand::GlobalIndex(g_idx) => {
                    let _ = write!(operands_str, " global_{}", g_idx);
                }
                _ => {}
            }
        }

        println!(
            "[{:04}]  {:<15} {}  | Stack: {:?}",
            ip, op_str, operands_str, stack_dump
        );
    }
}
