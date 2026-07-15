use techscript_ast::{Program, NodeId};
use techscript_common::Span;
use techscript_errors::DiagnosticReporter;
use techscript_semantic::analyze;

#[test]
fn test_semantic_empty() {
    let mut reporter = DiagnosticReporter::new();
    let program = Program {
        id: NodeId(0),
        statements: vec![],
        span: Span::new(0, 0),
    };
    let checked = analyze(program, &mut reporter).expect("semantic check should succeed");
    assert_eq!(checked.program.id.0, 0);
}
