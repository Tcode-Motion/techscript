use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;

fn unix_year(secs: u64) -> u32 {
    let mut days = secs / 86400;
    let mut year = 1970u32;
    loop { let dy = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 366 } else { 365 }; if days < dy { break; } days -= dy; year += 1; }
    year
}
fn unix_month(secs: u64) -> u32 {
    let year = unix_year(secs);
    let mut days = (secs / 86400) - days_before_year(year);
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let months = if leap { [31u32,29,31,30,31,30,31,31,30,31,30,31] } else { [31,28,31,30,31,30,31,31,30,31,30,31] };
    for (i, &m) in months.iter().enumerate() { if days < m as u64 { return i as u32 + 1; } days -= m as u64; }
    12
}
fn unix_day(secs: u64) -> u32 {
    let year = unix_year(secs);
    let month = unix_month(secs);
    let mut days = (secs / 86400) - days_before_year(year);
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let months = if leap { [31u32,29,31,30,31,30,31,31,30,31,30,31] } else { [31,28,31,30,31,30,31,31,30,31,30,31] };
    for (i, &m) in months.iter().enumerate() { if i + 1 == month as usize { break; } days -= m as u64; }
    (days + 1) as u32
}
fn days_before_year(year: u32) -> u64 {
    let y = (year - 1970) as u64;
    y * 365 + y / 4 - y / 100 + y / 400
}
pub fn format_unix_ts(secs: u64) -> String {
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", unix_year(secs), unix_month(secs), unix_day(secs), h, m, s)
}

pub fn register_date_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("unix",   native!("unix",   |_| { Ok(Value::Int(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as i64)) })),
        ("unix_ms",native!("unix_ms",|_| { Ok(Value::Int(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64)) })),
        ("now",    native!("now",    |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::String(Rc::new(format_unix_ts(secs)))) })),
        ("year",   native!("year",   |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(unix_year(secs) as i64)) })),
        ("month",  native!("month",  |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(unix_month(secs) as i64)) })),
        ("day",    native!("day",    |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(unix_day(secs) as i64)) })),
        ("hour",   native!("hour",   |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(((secs % 86400) / 3600) as i64)) })),
        ("minute", native!("minute", |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int(((secs % 3600) / 60) as i64)) })),
        ("second", native!("second", |_| { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(); Ok(Value::Int((secs % 60) as i64)) })),
    ]);
    globals.insert("date".into(), m);
}
