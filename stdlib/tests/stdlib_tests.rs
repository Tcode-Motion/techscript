use techscript_interpreter::Value;
use techscript_stdlib::StdlibRegistry;

#[test]
fn test_stdlib_math_abs() {
    let registry = StdlibRegistry::new();
    assert!(registry.has_module("math"));
    assert!(registry.has_module("io"));

    let math = registry.get_module("math").expect("math module exists");
    let abs_func = math.functions.get("abs").expect("abs function exists");
    
    let args = vec![Value::Int(-42)];
    let val = abs_func(&args).expect("abs should run");
    assert_eq!(val, Value::Int(42));
}
