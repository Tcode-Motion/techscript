use techscript_common::Span;
use techscript_syntax::{Precedence, Token, TokenKind};

#[test]
fn test_syntax_tokens() {
    let span = Span::new(0, 4);
    let token = Token::new(TokenKind::Make, "make".to_string(), span);
    assert_eq!(token.kind, TokenKind::Make);
    assert_eq!(token.lexeme, "make");

    assert!(Precedence::Factor > Precedence::Term);
}
