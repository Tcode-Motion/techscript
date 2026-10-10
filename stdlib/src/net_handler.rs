use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::Rc;
use techscript_runtime::{
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
    RuntimeContext,
};

pub fn tcp_listen_fn(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let port = args[0].try_into_int()?;
    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("TCP bind error: {}", e)),
            None,
            None,
        )
    })?;
    let handle_id = ctx.resources.borrow_mut().insert(listener);
    let mut listener_map = IndexMap::new();
    listener_map.insert("port".to_string(), RuntimeValue::Int(port));
    listener_map.insert("_handle".to_string(), RuntimeValue::Int(handle_id as i64));
    Ok(RuntimeValue::Map {
        entries: Rc::new(RefCell::new(listener_map)),
        is_const: false,
    })
}

pub fn tcp_connect_fn(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    let ip = args[0].try_into_string()?;
    let port = args[1].try_into_int()?;
    let stream = std::net::TcpStream::connect(format!("{}:{}", ip, port)).map_err(|e| {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(format!("TCP connect error: {}", e)),
            None,
            None,
        )
    })?;
    let handle_id = ctx.resources.borrow_mut().insert(stream);
    let mut stream_map = IndexMap::new();
    stream_map.insert("ip".to_string(), RuntimeValue::Str(ip));
    stream_map.insert("port".to_string(), RuntimeValue::Int(port));
    stream_map.insert("_handle".to_string(), RuntimeValue::Int(handle_id as i64));
    Ok(RuntimeValue::Map {
        entries: Rc::new(RefCell::new(stream_map)),
        is_const: false,
    })
}

pub fn tcp_send_fn(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if let RuntimeValue::Map { entries, .. } = &args[0] {
        let handle_id = entries
            .borrow()
            .get("_handle")
            .cloned()
            .unwrap_or(RuntimeValue::Null)
            .try_into_int()? as u32;
        let mut resources = ctx.resources.borrow_mut();
        if let Some(stream) = resources.get_mut::<std::net::TcpStream>(handle_id) {
            use std::io::Write;
            let msg = args[1].try_into_string()?;
            stream.write_all(msg.as_bytes()).ok();
        }
    }
    Ok(RuntimeValue::Null)
}

pub fn tcp_recv_fn(
    ctx: &mut RuntimeContext,
    args: Vec<RuntimeValue>,
) -> Result<RuntimeValue, RuntimeError> {
    if let RuntimeValue::Map { entries, .. } = &args[0] {
        let handle_id = entries
            .borrow()
            .get("_handle")
            .cloned()
            .unwrap_or(RuntimeValue::Null)
            .try_into_int()? as u32;
        let mut resources = ctx.resources.borrow_mut();
        if let Some(stream) = resources.get_mut::<std::net::TcpStream>(handle_id) {
            use std::io::Read;
            let mut buf = [0; 512];
            if let Ok(n) = stream.read(&mut buf) {
                return Ok(RuntimeValue::Str(
                    String::from_utf8_lossy(&buf[..n]).to_string(),
                ));
            }
        }
    }
    Ok(RuntimeValue::Str(String::new()))
}
