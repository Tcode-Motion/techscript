// ── TechScript .txbc bytecode file format ────────────────────────────
use crate::value::{Function, Value};

const MAGIC: &[u8; 4] = b"TXBC";
const FORMAT_VERSION: u8 = 2;

/// Serialize a compiled function chunk to `.txbc` bytes.
pub fn serialize_function(function: &Function) -> Vec<u8> {
    let chunk = &function.chunk;
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.push(FORMAT_VERSION);
    out.extend_from_slice(&(chunk.code.len() as u32).to_le_bytes());
    out.extend_from_slice(&chunk.code);
    out.extend_from_slice(&(chunk.lines.len() as u32).to_le_bytes());
    for line in &chunk.lines {
        out.extend_from_slice(&(*line as u32).to_le_bytes());
    }
    out.extend_from_slice(&(chunk.constants.len() as u32).to_le_bytes());
    for c in &chunk.constants {
        write_constant(&mut out, c);
    }
    out
}

fn write_constant(out: &mut Vec<u8>, val: &Value) {
    match val {
        Value::None => out.push(0),
        Value::Int(i) => {
            out.push(1);
            out.extend_from_slice(&i.to_le_bytes());
        }
        Value::Float(f) => {
            out.push(2);
            out.extend_from_slice(&f.to_le_bytes());
        }
        Value::Bool(b) => {
            out.push(3);
            out.push(if *b { 1 } else { 0 });
        }
        Value::String(s) => {
            out.push(4);
            let bytes = s.as_bytes();
            out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            out.extend_from_slice(bytes);
        }
        _ => out.push(0), // unsupported complex constants
    }
}

/// Load a `Function` from `.txbc` bytes (script entry only).
pub fn deserialize_function(data: &[u8]) -> Result<Function, String> {
    if data.len() < 6 || &data[0..4] != MAGIC {
        return Err("Invalid .txbc file (bad magic)".into());
    }
    let version = data[4];
    if version != FORMAT_VERSION && version != 1 {
        return Err(format!("Unsupported .txbc version {}", version));
    }
    let pos = 5usize;
    let (code, pos) = read_bytes(data, pos)?;
    let (lines, pos) = read_lines(data, pos)?;
    let (constants, _pos) = if version >= 2 {
        read_constants(data, pos)?
    } else {
        (Vec::new(), pos)
    };

    let mut function = Function::new("<txbc>", 0);
    function.chunk.code = code;
    function.chunk.lines = lines;
    function.chunk.constants = constants;
    Ok(function)
}

fn read_u32(data: &[u8], pos: usize) -> Result<(u32, usize), String> {
    if pos + 4 > data.len() {
        return Err("Truncated .txbc".into());
    }
    let v = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
    Ok((v, pos + 4))
}

fn read_bytes(data: &[u8], pos: usize) -> Result<(Vec<u8>, usize), String> {
    let (len, pos) = read_u32(data, pos)?;
    let end = pos + len as usize;
    if end > data.len() {
        return Err("Truncated .txbc code".into());
    }
    Ok((data[pos..end].to_vec(), end))
}

fn read_lines(data: &[u8], pos: usize) -> Result<(Vec<usize>, usize), String> {
    let (count, mut pos) = read_u32(data, pos)?;
    let mut lines = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let (line, p) = read_u32(data, pos)?;
        lines.push(line as usize);
        pos = p;
    }
    Ok((lines, pos))
}

fn read_constants(data: &[u8], pos: usize) -> Result<(Vec<Value>, usize), String> {
    use std::rc::Rc;
    let (count, mut pos) = read_u32(data, pos)?;
    let mut constants = Vec::with_capacity(count as usize);
    for _ in 0..count {
        if pos >= data.len() {
            return Err("Truncated .txbc constants".into());
        }
        let tag = data[pos];
        pos += 1;
        let val = match tag {
            0 => Value::None,
            1 => {
                if pos + 8 > data.len() {
                    return Err("Truncated int constant".into());
                }
                let i = i64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
                pos += 8;
                Value::Int(i)
            }
            2 => {
                if pos + 8 > data.len() {
                    return Err("Truncated float constant".into());
                }
                let f = f64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
                pos += 8;
                Value::Float(f)
            }
            3 => {
                if pos >= data.len() {
                    return Err("Truncated bool constant".into());
                }
                let b = data[pos] != 0;
                pos += 1;
                Value::Bool(b)
            }
            4 => {
                let (len, p) = read_u32(data, pos)?;
                pos = p;
                let end = pos + len as usize;
                if end > data.len() {
                    return Err("Truncated string constant".into());
                }
                let s = String::from_utf8_lossy(&data[pos..end]).into_owned();
                pos = end;
                Value::String(Rc::new(s))
            }
            _ => Value::None,
        };
        constants.push(val);
    }
    Ok((constants, pos))
}
