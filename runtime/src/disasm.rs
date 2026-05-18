// ── Bytecode disassembler for debug mode ─────────────────────────────
use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::value::Value;

pub fn disassemble_chunk(name: &str, chunk: &Chunk) -> String {
    let mut out = format!("== {} ==\n", name);
    let mut offset = 0;
    while offset < chunk.code.len() {
        let line = chunk.lines.get(offset).copied().unwrap_or(0);
        out.push_str(&format!("{:04} {:4} ", offset, line));
        let byte = chunk.code[offset];
        offset += 1;
        match OpCode::try_from(byte) {
            Ok(op) => {
                out.push_str(&format!("{:?}", op));
                match op {
                    OpCode::Constant | OpCode::GetGlobal | OpCode::SetGlobal
                    | OpCode::DefineGlobal | OpCode::GetProperty | OpCode::SetProperty
                    | OpCode::Method | OpCode::Import => {
                        if offset + 1 < chunk.code.len() {
                            let idx = ((chunk.code[offset] as u16) << 8) | chunk.code[offset + 1] as u16;
                            offset += 2;
                            if (idx as usize) < chunk.constants.len() {
                                out.push_str(&format!(" {}", format_constant(&chunk.constants[idx as usize])));
                            }
                        }
                    }
                    OpCode::GetLocal | OpCode::SetLocal | OpCode::GetUpvalue | OpCode::SetUpvalue
                    | OpCode::Call | OpCode::Print | OpCode::BuildList | OpCode::BuildMap
                    | OpCode::FormatString => {
                        if offset < chunk.code.len() {
                            out.push_str(&format!(" {}", chunk.code[offset]));
                            offset += 1;
                        }
                    }
                    OpCode::Jump | OpCode::JumpIfFalse | OpCode::Loop | OpCode::IterNext
                    | OpCode::SetupTry => {
                        if offset + 1 < chunk.code.len() {
                            let off = ((chunk.code[offset] as u16) << 8) | chunk.code[offset + 1] as u16;
                            offset += 2;
                            out.push_str(&format!(" ->{}", off));
                        }
                    }
                    OpCode::Closure => {
                        if offset + 1 < chunk.code.len() {
                            let idx = ((chunk.code[offset] as u16) << 8) | chunk.code[offset + 1] as u16;
                            offset += 2;
                            out.push_str(&format!(" fn#{}", idx));
                        }
                    }
                    _ => {}
                }
                out.push('\n');
            }
            Err(()) => {
                out.push_str(&format!("INVALID 0x{:02X}\n", byte));
            }
        }
    }
    out
}

fn format_constant(v: &Value) -> String {
    match v {
        Value::String(s) => format!("\"{}\"", s),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::None => "none".into(),
        _ => format!("<{}>", v.type_name()),
    }
}
