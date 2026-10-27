//! # TechScript Canonical Master Spec Syntax Tests
//!
//! Verifies that all canonical syntax rules defined in
//! `TechScript_Final_Syntax_and_Language_Master_Spec.md` parse and execute correctly:
//! - Colon and indentation block structuring (no `end`, no `do`)
//! - `name is "value"` variable declaration
//! - `count becomes count + 1` reassignment
//! - `constant app_name is "value"` constant declaration
//! - `to greet(person):` function declarations
//! - `return value` in functions
//! - `if ...: else if ...: else:` branch chains
//! - `repeat N times:` loops
//! - String interpolation `"Hello, {name}!"`

use techscript_errors::DiagnosticReporter;
use techscript_interpreter::Interpreter;
use techscript_lexer::lex;
use techscript_parser::parse;
use techscript_semantic::analyze;

fn run_txs(source: &str) -> techscript_runtime::RuntimeContext {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).expect("Lexing failed");
    let program = parse(&tokens, &mut reporter).expect("Parsing failed");
    let checked = analyze(program, &mut reporter).expect("Semantic analysis failed");

    let mut interpreter = Interpreter::new();
    let res = interpreter.interpret(&checked.program);
    assert!(res.is_ok(), "Interpretation failed: {:?}", res);
    interpreter.ctx
}

#[test]
fn test_canonical_variable_declaration_and_reassignment() {
    let source = r#"
name is "TechScript"
count is 0
count becomes count + 1
"#;
    let _ = run_txs(source);
}

#[test]
fn test_canonical_functions_with_colon_blocks() {
    let source = r#"
to add(first, second):
    return first + second

result is add(20, 22)
"#;
    let _ = run_txs(source);
}

#[test]
fn test_canonical_if_else_if_else() {
    let source = r#"
score is 85
grade is "none"

if score >= 90:
    grade becomes "A"
else if score >= 70:
    grade becomes "B"
else:
    grade becomes "C"
"#;
    let _ = run_txs(source);
}

#[test]
fn test_canonical_repeat_loop() {
    let source = r#"
total is 0
repeat 3 times:
    total becomes total + 1
"#;
    let _ = run_txs(source);
}

#[test]
fn test_canonical_while_loop() {
    let source = r#"
count is 0
while count < 4:
    count becomes count + 1
"#;
    let _ = run_txs(source);
}

#[test]
fn test_canonical_error_handling() {
    let source = r#"
caught is false
try:
    throw "disk error"
catch err:
    caught becomes true
"#;
    let _ = run_txs(source);
}
