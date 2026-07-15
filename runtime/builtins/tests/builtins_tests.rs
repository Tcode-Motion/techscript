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
