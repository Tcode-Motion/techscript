use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;

pub fn register_os_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("env_get",  native!("env_get",  |args| { let k = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(std::env::var(&k).map(|v| Value::String(Rc::new(v))).unwrap_or(Value::None)) })),
        ("env_set",  native!("env_set",  |args| { let k = args.first().map(|v| v.display_string()).unwrap_or_default(); let v = args.get(1).map(|v| v.display_string()).unwrap_or_default(); std::env::set_var(&k, &v); Ok(Value::None) })),
        ("env_del",  native!("env_del",  |args| { let k = args.first().map(|v| v.display_string()).unwrap_or_default(); std::env::remove_var(&k); Ok(Value::None) })),
        ("system",   native!("system",   |args| { let cmd = args.first().map(|v| v.display_string()).unwrap_or_default(); let status = std::process::Command::new(if cfg!(windows) { "cmd" } else { "sh" }).args(if cfg!(windows) { vec!["/C", &cmd] } else { vec!["-c", &cmd] }).status().map_err(|e| format!("os.system: {}", e))?; Ok(Value::Int(status.code().unwrap_or(0) as i64)) })),
        ("popen",    native!("popen",    |args| { let cmd = args.first().map(|v| v.display_string()).unwrap_or_default(); let out = std::process::Command::new(if cfg!(windows) { "cmd" } else { "sh" }).args(if cfg!(windows) { vec!["/C", &cmd] } else { vec!["-c", &cmd] }).output().map_err(|e| format!("os.popen: {}", e))?; Ok(Value::String(Rc::new(String::from_utf8_lossy(&out.stdout).into_owned()))) })),
        ("name",     native!("name",     |_| { Ok(Value::String(Rc::new(if cfg!(windows) { "windows" } else if cfg!(target_os = "macos") { "macos" } else { "linux" }.to_string()))) })),
        ("arch",     native!("arch",     |_| { Ok(Value::String(Rc::new(std::env::consts::ARCH.to_string()))) })),
        ("cpu_count",native!("cpu_count",|_| { Ok(Value::Int(std::thread::available_parallelism().map(|n| n.get() as i64).unwrap_or(1))) })),
        ("pid",      native!("pid",      |_| { Ok(Value::Int(std::process::id() as i64)) })),
        ("args",     native!("args",     |_| { let items: Vec<Value> = std::env::args().skip(1).map(|a| Value::String(Rc::new(a))).collect(); Ok(Value::List(Rc::new(RefCell::new(items)))) })),
        ("chdir",    native!("chdir",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::env::set_current_dir(&p).map_err(|e| format!("os.chdir: {}", e))?; Ok(Value::None) })),
    ]);
    globals.insert("os".into(), m);
}
