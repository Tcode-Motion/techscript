use techscript_common::Span;
use techscript_errors::{Diagnostic, DiagnosticLevel, DiagnosticReporter, ErrorCode};

#[test]
fn test_errors_reporter() {
    let mut reporter = DiagnosticReporter::new();
    assert!(!reporter.has_errors());

    let diag = Diagnostic::new(
        DiagnosticLevel::Error,
        ErrorCode::E0001,
        "Unexpected character".to_string(),
        Span::new(0, 1),
    );
    reporter.report(diag);
    assert!(reporter.has_errors());
}
