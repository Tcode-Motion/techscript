use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;

pub fn register_fs_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("read",       native!("read",       |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::fs::read_to_string(&p).map(|c| Value::String(Rc::new(c))).map_err(|e| format!("fs.read: {}", e)) })),
        ("write",      native!("write",      |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let d = args.get(1).map(|v| v.display_string()).unwrap_or_default(); std::fs::write(&p, &d).map_err(|e| format!("fs.write: {}", e))?; Ok(Value::None) })),
        ("append",     native!("append",     |args| { use std::io::Write; let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let d = args.get(1).map(|v| v.display_string()).unwrap_or_default(); let mut f = std::fs::OpenOptions::new().append(true).create(true).open(&p).map_err(|e| format!("fs.append: {}", e))?; f.write_all(d.as_bytes()).map_err(|e| format!("{}", e))?; Ok(Value::None) })),
        ("read_lines", native!("read_lines", |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let content = std::fs::read_to_string(&p).map_err(|e| format!("fs.read_lines: {}", e))?; let lines: Vec<Value> = content.lines().map(|l| Value::String(Rc::new(l.to_string()))).collect(); Ok(Value::List(Rc::new(RefCell::new(lines)))) })),
        ("exists",     native!("exists",     |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::Bool(std::path::Path::new(&p).exists())) })),
        ("is_file",    native!("is_file",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::Bool(std::path::Path::new(&p).is_file())) })),
        ("is_dir",     native!("is_dir",     |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::Bool(std::path::Path::new(&p).is_dir())) })),
        ("make_dir",   native!("make_dir",   |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::fs::create_dir_all(&p).map_err(|e| format!("fs.make_dir: {}", e))?; Ok(Value::None) })),
        ("remove_file",native!("remove_file",|args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::fs::remove_file(&p).map_err(|e| format!("fs.remove_file: {}", e))?; Ok(Value::None) })),
        ("remove_dir", native!("remove_dir", |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); std::fs::remove_dir_all(&p).map_err(|e| format!("fs.remove_dir: {}", e))?; Ok(Value::None) })),
        ("rename",     native!("rename",     |args| { let from = args.first().map(|v| v.display_string()).unwrap_or_default(); let to = args.get(1).map(|v| v.display_string()).unwrap_or_default(); std::fs::rename(&from, &to).map_err(|e| format!("fs.rename: {}", e))?; Ok(Value::None) })),
        ("copy",       native!("copy",       |args| { let src = args.first().map(|v| v.display_string()).unwrap_or_default(); let dst = args.get(1).map(|v| v.display_string()).unwrap_or_default(); std::fs::copy(&src, &dst).map_err(|e| format!("fs.copy: {}", e))?; Ok(Value::None) })),
        ("size",       native!("size",       |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let meta = std::fs::metadata(&p).map_err(|e| format!("fs.size: {}", e))?; Ok(Value::Int(meta.len() as i64)) })),
        ("cwd",        native!("cwd",        |_| { std::env::current_dir().map(|p| Value::String(Rc::new(p.to_string_lossy().into_owned()))).map_err(|e| format!("fs.cwd: {}", e)) })),
        ("abspath",    native!("abspath",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let abs = std::fs::canonicalize(&p).map_err(|e| format!("fs.abspath: {}", e))?; Ok(Value::String(Rc::new(abs.to_string_lossy().into_owned()))) })),
        ("basename",   native!("basename",   |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let base = std::path::Path::new(&p).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(); Ok(Value::String(Rc::new(base))) })),
        ("dirname",    native!("dirname",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let dir = std::path::Path::new(&p).parent().map(|d| d.to_string_lossy().into_owned()).unwrap_or_default(); Ok(Value::String(Rc::new(dir))) })),
        ("extname",    native!("extname",    |args| { let p = args.first().map(|v| v.display_string()).unwrap_or_default(); let ext = std::path::Path::new(&p).extension().map(|e| e.to_string_lossy().into_owned()).unwrap_or_default(); Ok(Value::String(Rc::new(ext))) })),
        ("list_dir",   native!("list_dir",   |args| { let p = args.first().map(|v| v.display_string()).unwrap_or(".".into()); let entries = std::fs::read_dir(&p).map_err(|e| format!("fs.list_dir: {}", e))?; let items: Vec<Value> = entries.filter_map(|e| e.ok()).map(|e| Value::String(Rc::new(e.file_name().to_string_lossy().into_owned()))).collect(); Ok(Value::List(Rc::new(RefCell::new(items)))) })),
    ]);
    globals.insert("fs".into(), m);
}
