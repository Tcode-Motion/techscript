use techscript_errors::DiagnosticReporter;
use techscript_interpreter::Interpreter;
use techscript_lexer::lex;
use techscript_parser::parse;
use techscript_runtime::{RuntimeError, RuntimeErrorKind, RuntimeValue};
use techscript_semantic::analyze;

fn run_source(source: &str) -> Result<RuntimeValue, RuntimeError> {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).map_err(|_| {
        RuntimeError::new(
            RuntimeErrorKind::UserError("Lexing failed".to_string()),
            None,
            None,
        )
    })?;
    let program = parse(&tokens, &mut reporter).map_err(|_| {
        RuntimeError::new(
            RuntimeErrorKind::UserError("Parsing failed".to_string()),
            None,
            None,
        )
    })?;
    let checked = analyze(program, &mut reporter).map_err(|_| {
        RuntimeError::new(
            RuntimeErrorKind::UserError("Semantic analysis failed".to_string()),
            None,
            None,
        )
    })?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program)
}

#[test]
fn test_interpreter_empty() {
    let res = run_source("");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), RuntimeValue::Null);
}

#[test]
fn test_interpreter_basic_arithmetic() {
    let res = run_source("make x = 2 * 3 + 4\nsay(x)");
    assert!(res.is_ok());
}

#[test]
fn test_interpreter_factorial_recursion() {
    let source = r#"
build factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
make result = factorial(5)
"#;
    let res = run_source(source);
    assert!(res.is_ok());
    // Lookup "result" inside interpreter's environment!
    // Standard REPLs or scripts can bind a variable and then we lookup the variable value from the interpreter's environment!
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).unwrap();
    let program = parse(&tokens, &mut reporter).unwrap();
    let checked = analyze(program, &mut reporter).unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program).unwrap();

    let val = interpreter.env.borrow().lookup("result").unwrap();
    assert_eq!(val, RuntimeValue::Int(120));
}

#[test]
fn test_interpreter_loops_and_conditionals() {
    let source = r#"
make sum = 0
make i = 1
while i <= 5 {
    sum = sum + i
    i = i + 1
}
"#;
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).unwrap();
    let program = parse(&tokens, &mut reporter).unwrap();
    let checked = analyze(program, &mut reporter).unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program).unwrap();

    let val = interpreter.env.borrow().lookup("sum").unwrap();
    assert_eq!(val, RuntimeValue::Int(15));
}

#[test]
fn test_interpreter_short_circuiting() {
    let source = r#"
make x = true or (1 / 0 == 0)
make y = false and (1 / 0 == 0)
make z = none ?? 100
"#;
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).unwrap();
    let program = parse(&tokens, &mut reporter).unwrap();
    let checked = analyze(program, &mut reporter).unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program).unwrap();

    assert_eq!(
        interpreter.env.borrow().lookup("x").unwrap(),
        RuntimeValue::Bool(true)
    );
    assert_eq!(
        interpreter.env.borrow().lookup("y").unwrap(),
        RuntimeValue::Bool(false)
    );
    assert_eq!(
        interpreter.env.borrow().lookup("z").unwrap(),
        RuntimeValue::Int(100)
    );
}

#[test]
fn test_interpreter_closures() {
    let source = r#"
build make_counter() {
    make count = 0
    build count_up() {
        count = count + 1
        return count
    }
    return count_up
}
make counter = make_counter()
make val1 = counter()
make val2 = counter()
"#;
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).unwrap();
    let program = parse(&tokens, &mut reporter).unwrap();
    let checked = analyze(program, &mut reporter).unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program).unwrap();

    assert_eq!(
        interpreter.env.borrow().lookup("val1").unwrap(),
        RuntimeValue::Int(1)
    );
    assert_eq!(
        interpreter.env.borrow().lookup("val2").unwrap(),
        RuntimeValue::Int(2)
    );
}

#[test]
fn test_interpreter_optional_chaining() {
    let source = r#"
make x = none
make val = x?.foo
"#;
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).unwrap();
    let program = parse(&tokens, &mut reporter).unwrap();
    let checked = analyze(program, &mut reporter).unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program).unwrap();

    assert_eq!(
        interpreter.env.borrow().lookup("val").unwrap(),
        RuntimeValue::Null
    );
}

#[test]
fn test_interpreter_try_catch() {
    let source = r#"
make error_msg = ""
try {
    throw "critical exception"
} catch e {
    error_msg = e
}
"#;
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).unwrap();
    let program = parse(&tokens, &mut reporter).unwrap();
    let checked = analyze(program, &mut reporter).unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program).unwrap();

    let val = interpreter.env.borrow().lookup("error_msg").unwrap();
    assert_eq!(val, RuntimeValue::Str("critical exception".to_string()));
}
