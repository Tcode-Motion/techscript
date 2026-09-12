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

        for (idx, inst) in func.chunk.instructions.iter().enumerate() {
            let op_str = format!("{:?}", inst.op);
            let mut line = format!("{:04}  {:<15}", idx, op_str);

            for op in &inst.operands {
                match op {
                    Operand::ConstantIndex(c_idx) => {
                        if let Some(lit) = func.chunk.constants.get(*c_idx) {
                            let _ = write!(line, "  #{:<3} ({:?})", c_idx, lit);
                        } else {
                            let _ = write!(line, "  #{:<3} (invalid)", c_idx);
                        }
                    }
                    Operand::LocalIndex(l_idx) => {
                        if let Some(name) = func.debug_symbols.local_names.get(l_idx) {
                            let _ = write!(line, "  local_{:<2} ({})", l_idx, name);
                        } else {
                            let _ = write!(line, "  local_{}", l_idx);
                        }
                    }
                    Operand::JumpOffset(offset) => {
                        let target = (idx as i32) + offset;
                        let _ = write!(line, "  offset_{:<3} (target: {:04})", offset, target);
                    }
                    Operand::Count(n) => {
                        let _ = write!(line, "  count_{}", n);
                    }
                    Operand::GlobalIndex(g_idx) => {
                        let _ = write!(line, "  global_{}", g_idx);
                    }
                    Operand::Register(r_idx) => {
                        let _ = write!(line, "  reg_{}", r_idx);
                    }
                    Operand::FunctionIndex(f_idx) => {
                        let _ = write!(line, "  func_{}", f_idx);
                    }
                }
            }
            output.push_str(&line);
            output.push('\n');
        }
        output
    }
}
