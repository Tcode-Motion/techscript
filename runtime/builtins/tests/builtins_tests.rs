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
fn test_fstring_concat() {
    let registry = BuiltinRegistry::new();
    assert!(registry.has_function("fstring_concat"));

    let args = vec![
        Value::Str("Hello, ".to_string()),
        Value::Str("World".to_string()),
        Value::Str("! Number: ".to_string()),
        Value::Int(42),
        Value::Str(" Bool: ".to_string()),
        Value::Bool(true),
    ];

    let val = registry
        .call("fstring_concat", &args)
        .expect("fstring_concat should run");
    assert_eq!(
        val,
        Value::Str("Hello, World! Number: 42 Bool: true".to_string())
    );
}

#[test]
fn benchmark_fstring_concat() {
    let registry = BuiltinRegistry::new();
    let args = vec![
        Value::Str("Hello, ".to_string()),
        Value::Str("World".to_string()),
        Value::Str("! Number: ".to_string()),
        Value::Int(42),
        Value::Str(" Bool: ".to_string()),
        Value::Bool(true),
    ];

    let start = std::time::Instant::now();
    for _ in 0..100_000 {
        let _ = registry.call("fstring_concat", &args);
    }
    let elapsed = start.elapsed();
    println!("100,000 fstring_concat runs took: {:?}", elapsed);
}
