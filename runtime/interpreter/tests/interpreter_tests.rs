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
    error_msg = e.message
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

#[test]
fn test_interpreter_uncaught_throw() {
    let source = r#"
throw "uncaught exception"
"#;
    let res = run_source(source);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.message, "uncaught exception");
    match err.kind {
        RuntimeErrorKind::UserError(msg) => assert_eq!(msg, "uncaught exception"),
        _ => panic!("Expected UserError"),
    }
}

#[test]
fn test_v108_runtime_compatibility() {
    let source = r#"
keep limit be 4
make x be 1
repeat x < limit {
    x = x + 1
}
make last = 0
each i in 1..3 then
    last = i
end
build greet(name, greeting = "Hello ") {
    give greeting + name
}
make greeting = greet("TechScript")
make caught = ""
attempt {
    make impossible = 1 / 0
} catch err {
    caught = err.message
}
"#;
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).unwrap();
    let program = parse(&tokens, &mut reporter).unwrap();
    let checked = analyze(program, &mut reporter).unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program).unwrap();

    assert_eq!(
        interpreter.env.borrow().lookup("x").unwrap(),
        RuntimeValue::Int(4)
    );
    assert_eq!(
        interpreter.env.borrow().lookup("last").unwrap(),
        RuntimeValue::Int(3)
    );
    assert_eq!(
        interpreter.env.borrow().lookup("greeting").unwrap(),
        RuntimeValue::Str("Hello TechScript".to_string())
    );
    assert_eq!(
        interpreter.env.borrow().lookup("caught").unwrap(),
        RuntimeValue::Str("division by zero".to_string())
    );
}

#[test]
fn test_interpreter_dsl_block_produces_value() {
    let source = r#"
page "/"
  title "Home"
  hero
    title "Welcome"
  end
end
"#;
    let res = run_source(source);
    assert!(res.is_ok(), "DSL block interpretation should succeed");
    // Check that _dsl_blocks list was populated
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).unwrap();
    let program = parse(&tokens, &mut reporter).unwrap();
    let checked = analyze(program, &mut reporter).unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&checked.program).unwrap();
    let blocks = interpreter.env.borrow().lookup("_dsl_blocks").unwrap();
    if let RuntimeValue::List { items, .. } = blocks {
        let list = items.borrow();
        assert_eq!(list.len(), 1);
        if let RuntimeValue::DslBlock(dsl) = &list[0] {
            assert_eq!(dsl.kind, "page");
            assert_eq!(dsl.args.len(), 1);
            assert_eq!(dsl.properties.len(), 1); // title "Home"
            assert_eq!(dsl.children.len(), 1); // hero sub-block
            assert_eq!(dsl.children[0].kind, "hero");
        } else {
            panic!("Expected DslBlock value");
        }
    } else {
        panic!("Expected List value");
    }
}

#[test]
fn test_interpreter_dsl_block_with_code() {
    let source = r#"
button "Click Me"
  label "Click Me"
  code
    say "Button clicked!"
  end
end
"#;
    let res = run_source(source);
    assert!(res.is_ok());
}
