use std::collections::HashMap;
use std::rc::Rc;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;
use crate::stdlib::date::format_unix_ts;

pub fn register_debug_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("trace", native!("debug.trace", |args| {
            for (i, v) in args.iter().enumerate() {
                eprintln!("[TRACE] arg{}: {} (type: {})", i, v.display_string(), v.type_name());
            }
            Ok(Value::None)
        })),
        ("inspect", native!("debug.inspect", |args| {
            if let Some(v) = args.first() {
                let info = format!("Value: {}\nType: {}\nTruthy: {}\nDisplay: {:?}", v.display_string(), v.type_name(), v.is_truthy(), v);
                eprintln!("[INSPECT]\n{}", info);
                Ok(Value::String(Rc::new(info)))
            } else { Ok(Value::None) }
        })),
        ("timer_start", native!("debug.timer_start", |args| {
            let label = args.first().map(|v| v.display_string()).unwrap_or("default".into());
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos() as i64;
            std::env::set_var(format!("_TECH_TIMER_{}", label), now.to_string());
            eprintln!("[TIMER] {} started", label);
            Ok(Value::None)
        })),
        ("timer_end", native!("debug.timer_end", |args| {
            let label = args.first().map(|v| v.display_string()).unwrap_or("default".into());
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos() as i64;
            let start = std::env::var(format!("_TECH_TIMER_{}", label)).ok().and_then(|s| s.parse::<i64>().ok()).unwrap_or(now);
            let elapsed_ns = now - start;
            let elapsed_ms = elapsed_ns as f64 / 1_000_000.0;
            let elapsed_s = elapsed_ns as f64 / 1_000_000_000.0;
            if elapsed_s >= 1.0 { eprintln!("[TIMER] {}: {:.3}s", label, elapsed_s); }
            else { eprintln!("[TIMER] {}: {:.3}ms", label, elapsed_ms); }
            Ok(Value::Float(elapsed_ms))
        })),
        ("benchmark", native!("debug.benchmark", |args| {
            let iterations = args.first().and_then(|v| if let Value::Int(i) = v { Some(*i) } else { None }).unwrap_or(1000);
            let label = args.get(1).map(|v| v.display_string()).unwrap_or("benchmark".into());
            let start = std::time::Instant::now();
            let mut sum = 0i64;
            for i in 0..iterations { sum += i; }
            let _ = sum;
            let elapsed = start.elapsed();
            eprintln!("[BENCHMARK] {}: {} iterations in {:.6}s ({:.0} ops/sec)", label, iterations, elapsed.as_secs_f64(), iterations as f64 / elapsed.as_secs_f64());
            Ok(Value::Float(elapsed.as_secs_f64() * 1000.0))
        })),
        ("assert", native!("debug.assert", |args| {
            let cond = args.first().map(|v| v.is_truthy()).unwrap_or(false);
            let msg = args.get(1).map(|v| v.display_string()).unwrap_or("Assertion failed".into());
            if !cond {
                eprintln!("❌ [ASSERT FAILED] {}", msg);
                return Err(format!("Assertion failed: {}", msg));
            }
            eprintln!("✓ [ASSERT OK] {}", msg);
            Ok(Value::Bool(true))
        })),
        ("log", native!("debug.log", |args| {
            let level = args.first().map(|v| v.display_string()).unwrap_or("INFO".into());
            let msg = args.get(1).map(|v| v.display_string()).unwrap_or_default();
            let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
            eprintln!("[{}] {} | {}", level.to_uppercase(), format_unix_ts(ts), msg);
            Ok(Value::None)
        })),
        ("table", native!("debug.table", |args| {
            if let Some(Value::List(l)) = args.first() {
                eprintln!("┌─────────┬────────────────────────────────┐");
                eprintln!("│ Index   │ Value                          │");
                eprintln!("├─────────┼────────────────────────────────┤");
                for (i, v) in l.borrow().iter().enumerate() {
                    eprintln!("│ {:<7} │ {:<30} │", i, v.display_string());
                }
                eprintln!("└─────────┴────────────────────────────────┘");
            } else if let Some(Value::Map(m)) = args.first() {
                eprintln!("┌──────────────────┬────────────────────────┐");
                eprintln!("│ Key              │ Value                  │");
                eprintln!("├──────────────────┼────────────────────────┤");
                for (k, v) in m.borrow().iter() {
                    eprintln!("│ {:<16} │ {:<22} │", k, v.display_string());
                }
                eprintln!("└──────────────────┴────────────────────────┘");
            }
            Ok(Value::None)
        })),
    ]);
    globals.insert("debug".into(), m);
}
