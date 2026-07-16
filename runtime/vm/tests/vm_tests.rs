use techscript_bytecode::compile;
use techscript_errors::DiagnosticReporter;
use techscript_ir::lower;
use techscript_lexer::lex;
use techscript_optimizer::{optimize, OptimizationContext, OptimizationLevel};
use techscript_parser::parse;
use techscript_runtime::RuntimeValue;
use techscript_semantic::analyze;
use techscript_vm::{run, VMError};

fn execute_source(source: &str) -> Result<RuntimeValue, VMError> {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).expect("lexing should succeed");
    let program = parse(&tokens, &mut reporter).expect("parsing should succeed");
    let checked = analyze(program, &mut reporter).expect("semantic should succeed");

    let res = lower(&checked.program, "test_module");
    let mut module = res.module;

    let mut ctx = OptimizationContext::new();
    ctx.level = OptimizationLevel::O2;
    let _opt_res = optimize(&mut module, &ctx);

    let bc_module = compile(&module);
    run(bc_module)
}

#[test]
fn test_vm_arithmetic() {
    let res = execute_source(
        r#"
build main() {
    return 10 + 20
}
"#,
    )
    .expect("Execution failed");
    assert_eq!(res, RuntimeValue::Int(30));
}

#[test]
fn test_vm_variables() {
    let res = execute_source(
        r#"
build main() {
    make x = 50
    return x
}
"#,
    )
    .expect("Execution failed");
    assert_eq!(res, RuntimeValue::Int(50));
}

#[test]
fn test_vm_conditions() {
    let res = execute_source(
        r#"
build main() {
    make x = 10
    if x > 5 {
        return 100
    } else {
        return 200
    }
}
"#,
    )
    .expect("Execution failed");
    assert_eq!(res, RuntimeValue::Int(100));
}

#[test]
fn test_vm_functions() {
    let res = execute_source(
        r#"
build add(a, b) {
    return a + b
}
build main() {
    return add(40, 2)
}
"#,
    )
    .expect("Execution failed");
    assert_eq!(res, RuntimeValue::Int(42));
}

#[test]
fn test_vm_recursion() {
    let res = execute_source(
        r#"
build fact(n) {
    if n <= 1 {
        return 1
    }
    return n * fact(n - 1)
}
build main() {
    return fact(5)
}
"#,
    )
    .expect("Execution failed");
    assert_eq!(res, RuntimeValue::Int(120));
}

#[test]
fn test_vm_native_call() {
    let res = execute_source(
        r#"
build len(s) {
    return 0
}
build main() {
    return len("hello")
}
"#,
    )
    .expect("Execution failed");
    assert_eq!(res, RuntimeValue::Int(5));
}

#[test]
fn test_vm_division_by_zero() {
    let res = execute_source(
        r#"
build main() {
    return 10 / 0
}
"#,
    );
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), VMError::DivisionByZero);
}
