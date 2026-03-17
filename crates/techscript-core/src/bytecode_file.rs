// ── TechScript Bytecode File Format ──────────────────────────────────
// `tech build` — serialize/deserialize compiled bytecode to `.txc` files.
//
// File format:
//   4 bytes: magic "TXSC"
//   1 byte:  version (currently 1)
//   4 bytes: constant pool count (little-endian u32)
//   N × constant entries
//   4 bytes: code length (little-endian u32)
//   N × code bytes
//   4 bytes: line info length (little-endian u32)
//   N × line numbers (each as little-endian u32)
//   1 byte:  arity
//   string:  function name (length-prefixed)
//   1 byte:  upvalue count

use crate::value::{Function, Value};
use crate::chunk::Chunk;
use std::io::{Read, Write, Cursor};
use std::rc::Rc;

const MAGIC: &[u8; 4] = b"TXSC";
const FORMAT_VERSION: u8 = 1;

/// Serialize a compiled Function to bytecode bytes.
pub fn serialize(function: &Function) -> Vec<u8> {
    let mut buf = Vec::new();

    // Magic header
    buf.extend_from_slice(MAGIC);
    buf.push(FORMAT_VERSION);

    // Function name
    write_string(&mut buf, &function.name);

    // Arity
    buf.push(function.arity as u8);

    // Upvalue count
    buf.push(function.upvalue_count as u8);

    // Constants
    write_u32(&mut buf, function.chunk.constants.len() as u32);
    for constant in &function.chunk.constants {
        write_value(&mut buf, constant);
    }

    // Code
    write_u32(&mut buf, function.chunk.code.len() as u32);
    buf.extend_from_slice(&function.chunk.code);

    // Lines
    write_u32(&mut buf, function.chunk.lines.len() as u32);
    for &line in &function.chunk.lines {
        write_u32(&mut buf, line as u32);
    }

    buf
}

/// Deserialize bytecode bytes back into a Function.
pub fn deserialize(data: &[u8]) -> Result<Function, String> {
    let mut cursor = Cursor::new(data);

    // Check magic
    let mut magic = [0u8; 4];
    cursor.read_exact(&mut magic).map_err(|e| format!("Invalid bytecode: {}", e))?;
    if &magic != MAGIC {
        return Err("Not a valid TechScript bytecode file (bad magic header)".into());
    }

    // Version
    let mut ver = [0u8; 1];
    cursor.read_exact(&mut ver).map_err(|_| "Cannot read version")?;
    if ver[0] != FORMAT_VERSION {
        return Err(format!("Unsupported bytecode version: {} (expected {})", ver[0], FORMAT_VERSION));
    }

    // Function name
    let name = read_string(&mut cursor)?;

    // Arity
    let mut arity_buf = [0u8; 1];
    cursor.read_exact(&mut arity_buf).map_err(|_| "Cannot read arity")?;
    let arity = arity_buf[0] as usize;

    // Upvalue count
    let mut uv_buf = [0u8; 1];
    cursor.read_exact(&mut uv_buf).map_err(|_| "Cannot read upvalue count")?;
    let upvalue_count = uv_buf[0] as usize;

    // Constants
    let const_count = read_u32(&mut cursor)? as usize;
    let mut constants = Vec::with_capacity(const_count);
    for _ in 0..const_count {
        constants.push(read_value(&mut cursor)?);
    }

    // Code
    let code_len = read_u32(&mut cursor)? as usize;
    let mut code = vec![0u8; code_len];
    cursor.read_exact(&mut code).map_err(|_| "Cannot read bytecode")?;

    // Lines
    let lines_len = read_u32(&mut cursor)? as usize;
    let mut lines = Vec::with_capacity(lines_len);
    for _ in 0..lines_len {
        lines.push(read_u32(&mut cursor)? as usize);
    }

    Ok(Function {
        name,
        arity,
        chunk: Chunk { code, constants, lines },
        upvalue_count,
    })
}

// ── Helpers ─────────────────────────────────────────────────────────

fn write_u32(buf: &mut Vec<u8>, val: u32) {
    buf.extend_from_slice(&val.to_le_bytes());
}

fn read_u32(cursor: &mut Cursor<&[u8]>) -> Result<u32, String> {
    let mut bytes = [0u8; 4];
    cursor.read_exact(&mut bytes).map_err(|_| "Unexpected end of bytecode".to_string())?;
    Ok(u32::from_le_bytes(bytes))
}

fn write_string(buf: &mut Vec<u8>, s: &str) {
    write_u32(buf, s.len() as u32);
    buf.extend_from_slice(s.as_bytes());
}

fn read_string(cursor: &mut Cursor<&[u8]>) -> Result<String, String> {
    let len = read_u32(cursor)? as usize;
    let mut bytes = vec![0u8; len];
    cursor.read_exact(&mut bytes).map_err(|_| "Cannot read string")?;
    String::from_utf8(bytes).map_err(|_| "Invalid UTF-8 in bytecode".into())
}

/// Value type tags for serialization.
const TAG_INT: u8 = 0;
const TAG_FLOAT: u8 = 1;
const TAG_BOOL: u8 = 2;
const TAG_NONE: u8 = 3;
const TAG_STRING: u8 = 4;
const TAG_FUNCTION: u8 = 5;

fn write_value(buf: &mut Vec<u8>, val: &Value) {
    match val {
        Value::Int(i) => {
            buf.push(TAG_INT);
            buf.extend_from_slice(&i.to_le_bytes());
        }
        Value::Float(f) => {
            buf.push(TAG_FLOAT);
            buf.extend_from_slice(&f.to_le_bytes());
        }
        Value::Bool(b) => {
            buf.push(TAG_BOOL);
            buf.push(if *b { 1 } else { 0 });
        }
        Value::None => {
            buf.push(TAG_NONE);
        }
        Value::String(s) => {
            buf.push(TAG_STRING);
            write_string(buf, s);
        }
        Value::Function(f) => {
            buf.push(TAG_FUNCTION);
            let nested = serialize(f);
            write_u32(buf, nested.len() as u32);
            buf.extend_from_slice(&nested);
        }
        // Other value types cannot appear in the constant pool
        _ => {
            buf.push(TAG_NONE);
        }
    }
}

fn read_value(cursor: &mut Cursor<&[u8]>) -> Result<Value, String> {
    let mut tag = [0u8; 1];
    cursor.read_exact(&mut tag).map_err(|_| "Cannot read value tag")?;
    match tag[0] {
        TAG_INT => {
            let mut bytes = [0u8; 8];
            cursor.read_exact(&mut bytes).map_err(|_| "Cannot read int")?;
            Ok(Value::Int(i64::from_le_bytes(bytes)))
        }
        TAG_FLOAT => {
            let mut bytes = [0u8; 8];
            cursor.read_exact(&mut bytes).map_err(|_| "Cannot read float")?;
            Ok(Value::Float(f64::from_le_bytes(bytes)))
        }
        TAG_BOOL => {
            let mut b = [0u8; 1];
            cursor.read_exact(&mut b).map_err(|_| "Cannot read bool")?;
            Ok(Value::Bool(b[0] != 0))
        }
        TAG_NONE => Ok(Value::None),
        TAG_STRING => {
            let s = read_string(cursor)?;
            Ok(Value::String(Rc::new(s)))
        }
        TAG_FUNCTION => {
            let len = read_u32(cursor)? as usize;
            let mut nested = vec![0u8; len];
            cursor.read_exact(&mut nested).map_err(|_| "Cannot read nested function")?;
            let func = deserialize(&nested)?;
            Ok(Value::Function(Rc::new(func)))
        }
        _ => Err(format!("Unknown value tag: {}", tag[0])),
    }
}

/// Get the `.txc` output path for a `.txs` source path.
pub fn txc_path(txs_path: &str) -> String {
    if txs_path.ends_with(".txs") {
        format!("{}c", txs_path)
    } else if txs_path.ends_with(".tx") {
        format!("{}c", txs_path)
    } else {
        format!("{}.txc", txs_path)
    }
}
