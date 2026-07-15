use techscript_errors::DiagnosticReporter;
use techscript_parser::parse;

#[test]
fn test_parser_empty() {
    let mut reporter = DiagnosticReporter::new();
    let program = parse(&[], &mut reporter).expect("parse should succeed");
    assert_eq!(program.statements.len(), 0);
}
