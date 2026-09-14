use crate::function::BytecodeFunction;
use crate::module::BytecodeModule;
use crate::operand::Operand;
use std::fmt::Write;

/// Disassembler rendering human-readable bytecode instructions.
pub struct BytecodeDisassembler;

impl BytecodeDisassembler {
    /// Renders a whole module to a formatted string.
    pub fn disassemble_module(module: &BytecodeModule) -> String {
        let mut output = String::new();
        let _ = write!(output, "Module: {}\n\n", module.name);

        for func in &module.functions {
            output.push_str(&Self::disassemble_function(func));
            output.push('\n');
        }
        output
    }

    /// Renders a single function's chunk to formatted string.
    pub fn disassemble_function(func: &BytecodeFunction) -> String {
        let mut output = String::new();
        let _ = write!(
            output,
            "Function: {} (params: {}, locals: {}, max_stack: {})\n",
            func.name, func.param_count, func.local_count, func.max_stack_size
        );

        // Bolt performance optimization: Avoid unnecessary string allocation for inst.op.
        // `inst.op` implements `Debug`, so we can format it directly into the final string
        // instead of pre-allocating intermediate Strings with `format!`.
        for (idx, inst) in func.chunk.instructions.iter().enumerate() {
            let _ = write!(output, "{:04}  {:<15?}", idx, inst.op);

            for op in &inst.operands {
                match op {
                    Operand::ConstantIndex(c_idx) => {
                        if let Some(lit) = func.chunk.constants.get(*c_idx) {
                            let _ = write!(output, "  #{:<3} ({:?})", c_idx, lit);
                        } else {
                            let _ = write!(output, "  #{:<3} (invalid)", c_idx);
                        }
                    }
                    Operand::LocalIndex(l_idx) => {
                        if let Some(name) = func.debug_symbols.local_names.get(l_idx) {
                            let _ = write!(output, "  local_{:<2} ({})", l_idx, name);
                        } else {
                            let _ = write!(output, "  local_{}", l_idx);
                        }
                    }
                    Operand::JumpOffset(offset) => {
                        let target = (idx as i32) + offset;
                        let _ = write!(output, "  offset_{:<3} (target: {:04})", offset, target);
                    }
                    Operand::Count(n) => {
                        let _ = write!(output, "  count_{}", n);
                    }
                    Operand::GlobalIndex(g_idx) => {
                        let _ = write!(output, "  global_{}", g_idx);
                    }
                    Operand::Register(r_idx) => {
                        let _ = write!(output, "  reg_{}", r_idx);
                    }
                    Operand::FunctionIndex(f_idx) => {
                        let _ = write!(output, "  func_{}", f_idx);
                    }
                }
            }
            output.push('\n');
        }
        output
    }
}
