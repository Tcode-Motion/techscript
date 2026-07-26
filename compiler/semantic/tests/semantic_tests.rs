use techscript_errors::{Diagnostic, DiagnosticLevel, DiagnosticReporter, ErrorCode};
use techscript_lexer::lex;
use techscript_parser::parse;
use techscript_semantic::{analyze, CheckedProgram};

fn check_source(source: &str) -> (Result<CheckedProgram, Vec<Diagnostic>>, Vec<Diagnostic>) {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).expect("lexing should succeed");
    let program = parse(&tokens, &mut reporter).expect("parsing should succeed");
    let result = analyze(program, &mut reporter);
    let filtered_diags = reporter.get_diagnostics()
        .iter()
        .filter(|d| {
            !matches!(
                d.code,
                ErrorCode::TSW1001
                    | ErrorCode::TSW1002
                    | ErrorCode::TSW1003
                    | ErrorCode::TSW1004
                    | ErrorCode::TSW1005
                    | ErrorCode::TSW1006
                    | ErrorCode::TSW1012
            )
        })
        .cloned()
        .collect();
    (result, filtered_diags)
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

// ── DSL semantic validation tests ─────────────────────────────────────

#[test]
fn test_dsl_semantic_valid_block() {
    let (res, diags) = check_source("logo \"MyApp\"\n  text \"TS\"\n  color \"#333\"\nend");
    assert!(res.is_ok(), "DSL block should pass semantic: {:?}", diags);
    let dsl_errors: Vec<_> = diags.iter().filter(|d| matches!(d.code, ErrorCode::E0400 | ErrorCode::E0401 | ErrorCode::E0402 | ErrorCode::E0403)).collect();
    assert!(dsl_errors.is_empty(), "No DSL validation errors: {:?}", dsl_errors);
}

#[test]
fn test_dsl_semantic_unknown_property_warning() {
    let (res, diags) = check_source("logo \"MyApp\"\n  text \"TS\"\n  unknown_prop \"value\"\nend");
    assert!(res.is_ok(), "Unknown property should be warning only: {:?}", diags);
    let warnings: Vec<_> = diags.iter().filter(|d| d.code == ErrorCode::E0401).collect();
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].level, DiagnosticLevel::Warning);
    assert!(warnings[0].message.contains("unknown_prop"));
}

#[test]
fn test_dsl_semantic_duplicate_property_error() {
    let (res, diags) = check_source("button \"Click\"\n  label \"OK\"\n  label \"Cancel\"\nend");
    assert!(res.is_err(), "Duplicate property should be error");
    let errors: Vec<_> = diags.iter().filter(|d| d.code == ErrorCode::E0400).collect();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Duplicate property 'label'"));
}

#[test]
fn test_dsl_semantic_missing_required_property() {
    let (res, diags) = check_source("button \"Click\"\n  color \"red\"\nend");
    assert!(res.is_err(), "Missing required property should be error");
    let errors: Vec<_> = diags.iter().filter(|d| d.code == ErrorCode::E0402).collect();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("missing required property 'label'") ||
            errors[0].message.contains("Missing required property 'label'"));
}

#[test]
fn test_dsl_semantic_invalid_nested_block_warning() {
    // `page` is a valid sub-block within a `page` parent (nested pages allowed by schema),
    // but `page` is NOT listed in `card`'s `allowed_children`.
    let (res, diags) = check_source("card\n  title \"Test\"\n  page \"/nested\"\n    title \"Nested\"\n  end\nend");
    assert!(res.is_ok(), "Invalid nest should be warning only: {:?}", diags);
    let warnings: Vec<_> = diags.iter().filter(|d| d.code == ErrorCode::E0403).collect();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("page"));
    assert!(warnings[0].message.contains("not allowed inside 'card'"));
}

#[test]
fn test_dsl_semantic_nested_valid_block() {
    let (res, diags) = check_source("website \"Portal\"\n  title \"Portal\"\n  page \"/\"\n    title \"Home\"\n  end\nend");
    assert!(res.is_ok(), "Valid nested DSL should pass: {:?}", diags);
    let dsl_errors: Vec<_> = diags.iter().filter(|d| matches!(d.code, ErrorCode::E0400 | ErrorCode::E0401 | ErrorCode::E0402 | ErrorCode::E0403)).collect();
    assert!(dsl_errors.is_empty(), "No DSL validation errors: {:?}", dsl_errors);
}

#[test]
fn test_dsl_semantic_full_web_page() {
    let source = r#"
use web
website "My Site"
  title "My Site"
  page "/"
    title "Home"
    hero
      title "Welcome"
      subtitle "Hello World"
    end
    section
      title "Features"
      card
        title "Fast"
        text "Lightning"
      end
    end
  end
end
"#;
    let (res, diags) = check_source(source);
    assert!(res.is_ok(), "Full web DSL should pass: {:?}", diags);
    let dsl_errors: Vec<_> = diags.iter().filter(|d| matches!(d.code, ErrorCode::E0400 | ErrorCode::E0401 | ErrorCode::E0402 | ErrorCode::E0403)).collect();
    assert!(dsl_errors.is_empty(), "No DSL validation errors: {:?}", dsl_errors);
}
