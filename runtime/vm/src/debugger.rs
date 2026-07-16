use std::collections::HashSet;
use techscript_bytecode::{BytecodeFunction, BytecodeInstruction, Operand};

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
        inst: &BytecodeInstruction,
        stack_dump: &[techscript_runtime::RuntimeValue],
    ) {
        if !self.enabled {
            return;
        }

        let op_str = format!("{:?}", inst.op);
        let mut operands_str = String::new();

        for op in &inst.operands {
            match op {
                Operand::ConstantIndex(c_idx) => {
                    if let Some(lit) = func.chunk.constants.get(*c_idx) {
                        operands_str.push_str(&format!(" #{:<3} ({:?})", c_idx, lit));
                    } else {
                        operands_str.push_str(&format!(" #{:<3} (invalid)", c_idx));
                    }
                }
                Operand::LocalIndex(l_idx) => {
                    if let Some(name) = func.debug_symbols.local_names.get(l_idx) {
                        operands_str.push_str(&format!(" local_{:<2} ({})", l_idx, name));
                    } else {
                        operands_str.push_str(&format!(" local_{}", l_idx));
                    }
                }
                Operand::JumpOffset(offset) => {
                    let target = (ip as i32) + offset;
                    operands_str
                        .push_str(&format!(" offset_{:<3} (target: {:04})", offset, target));
                }
                Operand::Count(n) => {
                    operands_str.push_str(&format!(" count_{}", n));
                }
                Operand::GlobalIndex(g_idx) => {
                    operands_str.push_str(&format!(" global_{}", g_idx));
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
