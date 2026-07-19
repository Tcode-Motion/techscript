use techscript_errors::{Diagnostic, DiagnosticLevel, DiagnosticReporter, ErrorCode};
use techscript_lexer::lex;
use techscript_parser::parse;
use techscript_semantic::{analyze, CheckedProgram};

fn check_source(source: &str) -> (Result<CheckedProgram, Vec<Diagnostic>>, Vec<Diagnostic>) {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).expect("lexing should succeed");
    let program = parse(&tokens, &mut reporter).expect("parsing should succeed");
    let result = analyze(program, &mut reporter);
    (result, reporter.get_diagnostics().to_vec())
}

#[test]
fn test_semantic_empty() {
    let (res, diags) = check_source("");
    assert!(res.is_ok());
    assert!(diags.is_empty());
}

#[test]
fn test_semantic_undefined_variable_with_suggestion() {
    let (res, diags) = check_source("make count = 10\nsay cont");
    assert!(res.is_err());
    assert!(!diags.is_empty());
    assert_eq!(diags[0].code, ErrorCode::E0300);
    assert!(diags[0].message.contains("Did you mean 'count'?"));
}

#[test]
fn test_semantic_duplicate_declaration() {
    let (res, diags) = check_source("make x = 10\nmake x = 20");
    assert!(res.is_err());
    assert!(!diags.is_empty());
    assert_eq!(diags[0].code, ErrorCode::E0301);
}

#[test]
fn test_semantic_shadowing_warning() {
    let (res, diags) = check_source("make x = 10\n{\n  make x = 20\n}");
    assert!(res.is_ok()); // Shadowing is allowed, so compile succeeds
    assert!(!diags.is_empty());
    assert_eq!(diags[0].level, DiagnosticLevel::Warning);
    // W0010 shadowing warning
    assert!(matches!(diags[0].code, ErrorCode::W0010));
}

#[test]
fn test_semantic_constant_reassignment() {
    let (res, diags) = check_source("const x = 10\nx = 20");
    assert!(res.is_err());
    assert!(!diags.is_empty());
    assert_eq!(diags[0].code, ErrorCode::E0302);
}

#[test]
fn test_semantic_flow_control_errors() {
    // 1. Return outside function body
    let (res, diags) = check_source("return 42");
    assert!(res.is_err());
    assert_eq!(diags[0].code, ErrorCode::E0312);

    // 2. Break outside loop
    let (res, diags) = check_source("break");
    assert!(res.is_err());
    assert_eq!(diags[0].code, ErrorCode::E0312);

    // 3. Continue outside loop
    let (res, diags) = check_source("continue");
    assert!(res.is_err());
    assert_eq!(diags[0].code, ErrorCode::E0312);
}

#[test]
fn test_semantic_call_arity_mismatch() {
    // Current semantic analysis only rejects calls with *too many* arguments
    // (default params are permitted).  Too-few is caught at runtime.
    let (res, diags) = check_source("build add(a, b) {\n  return a + b\n}\nadd(1, 2, 3)");
    assert!(res.is_err()); // Too many arguments is a compile-time error
    assert!(!diags.is_empty());
    assert_eq!(diags[0].code, ErrorCode::E0311); // Too many arguments
}

#[test]
fn test_semantic_deprecated_fun_keyword() {
    let (res, diags) = check_source("model User {\n  fun greet() {}\n}");
    assert!(res.is_ok());
    assert!(!diags.is_empty());
    assert_eq!(diags[0].code, ErrorCode::W0015); // Deprecated fun warning
}

#[test]
fn test_semantic_self_outside_model() {
    let (res, diags) = check_source("say self");
    assert!(res.is_err());
    assert_eq!(diags[0].code, ErrorCode::E0320); // Self outside model context
}
