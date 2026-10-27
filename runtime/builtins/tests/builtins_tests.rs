use techscript_builtins::BuiltinRegistry;
use techscript_interpreter::Value;

#[test]
fn test_builtins_len() {
    let registry = BuiltinRegistry::new();
    assert!(registry.has_function("len"));
    assert!(registry.has_function("say"));

    let args = vec![Value::Str("hello".to_string())];
    let val = registry.call("len", &args).expect("len should run");
    assert_eq!(val, Value::Int(5));
}

#[test]
fn test_builtins_fstring_concat() {
    let registry = BuiltinRegistry::new();
    assert!(registry.has_function("fstring_concat"));

    let args = vec![
        Value::Str("hello ".to_string()),
        Value::Int(42),
        Value::Str(" world ".to_string()),
        Value::Bool(true),
        Value::Str(" ".to_string()),
        Value::Null,
    ];
    let val = registry
        .call("fstring_concat", &args)
        .expect("fstring_concat should run");
    assert_eq!(val, Value::Str("hello 42 world true none".to_string()));
}

#[test]
fn bench_fstring_concat() {
    let registry = BuiltinRegistry::new();
    let args = vec![
        Value::Str("user_".to_string()),
        Value::Int(12345),
        Value::Str(": status = ".to_string()),
        Value::Bool(true),
        Value::Str(", score = ".to_string()),
        Value::Float(98.5),
    ];
    let start = std::time::Instant::now();
    for _ in 0..100_000 {
        let _ = registry.call("fstring_concat", &args);
    }
    let elapsed = start.elapsed();
    println!(
        "Baseline/Optimized fstring_concat (100k iterations): {:?}",
        elapsed
    );
}
