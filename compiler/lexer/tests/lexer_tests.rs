use techscript_errors::DiagnosticReporter;
use techscript_lexer::lex;
use techscript_syntax::TokenKind;

#[test]
fn test_lexer_eof() {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex("", &mut reporter).expect("lex should succeed");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Eof);
}
