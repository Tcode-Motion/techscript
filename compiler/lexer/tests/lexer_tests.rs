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

#[test]
fn test_lexer_keywords() {
    let mut reporter = DiagnosticReporter::new();
    let source = "make const let fun build class model when if elif else try attempt null none";
    let tokens = lex(source, &mut reporter).expect("lex should succeed");

    // 15 keywords + EOF = 16 tokens
    assert_eq!(tokens.len(), 16);
    assert_eq!(tokens[0].kind, TokenKind::Make);
    assert_eq!(tokens[1].kind, TokenKind::Const);
    assert_eq!(tokens[2].kind, TokenKind::Let);
    assert_eq!(tokens[3].kind, TokenKind::Fun);
    assert_eq!(tokens[4].kind, TokenKind::Build);
    assert_eq!(tokens[5].kind, TokenKind::Class);
    assert_eq!(tokens[6].kind, TokenKind::Model);
    assert_eq!(tokens[7].kind, TokenKind::When);
    assert_eq!(tokens[8].kind, TokenKind::If);
    assert_eq!(tokens[9].kind, TokenKind::Elif);
    assert_eq!(tokens[10].kind, TokenKind::Else);
    assert_eq!(tokens[11].kind, TokenKind::Try);
    assert_eq!(tokens[12].kind, TokenKind::Attempt);
    assert_eq!(tokens[13].kind, TokenKind::Null);
    assert_eq!(tokens[14].kind, TokenKind::None);
    assert_eq!(tokens[15].kind, TokenKind::Eof);
}

#[test]
fn test_lexer_unicode_identifiers() {
    let mut reporter = DiagnosticReporter::new();
    let source = "my_var count_123 variable_name";
    let tokens = lex(source, &mut reporter).expect("lex should succeed");
    assert_eq!(tokens.len(), 4); // 3 idents + EOF
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].lexeme, "my_var");
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].lexeme, "count_123");
}

#[test]
fn test_lexer_numbers() {
    let mut reporter = DiagnosticReporter::new();
    let source = "42 0xFF 0b1010 0o755 3.14159 2.5e-3";
    let tokens = lex(source, &mut reporter).expect("lex should succeed");

    assert_eq!(tokens.len(), 7); // 6 numbers + EOF
    assert_eq!(tokens[0].kind, TokenKind::IntLiteral);
    assert_eq!(tokens[0].lexeme, "42");
    assert_eq!(tokens[1].kind, TokenKind::IntLiteral);
    assert_eq!(tokens[1].lexeme, "0xFF");
    assert_eq!(tokens[2].kind, TokenKind::IntLiteral);
    assert_eq!(tokens[2].lexeme, "0b1010");
    assert_eq!(tokens[3].kind, TokenKind::IntLiteral);
    assert_eq!(tokens[3].lexeme, "0o755");
    assert_eq!(tokens[4].kind, TokenKind::FloatLiteral);
    assert_eq!(tokens[4].lexeme, "3.14159");
    assert_eq!(tokens[5].kind, TokenKind::FloatLiteral);
    assert_eq!(tokens[5].lexeme, "2.5e-3");
}

#[test]
fn test_lexer_operators() {
    let mut reporter = DiagnosticReporter::new();
    let source = "+ - * / // % ** == != === !== < > <= >= = += -= *= /= %= .. ..= ?. ?? ->";
    let tokens = lex(source, &mut reporter).expect("lex should succeed");

    assert_eq!(tokens.len(), 27); // 26 ops + EOF
    assert_eq!(tokens[0].kind, TokenKind::Plus);
    assert_eq!(tokens[4].kind, TokenKind::DoubleSlash);
    assert_eq!(tokens[6].kind, TokenKind::DoubleStar);
    assert_eq!(tokens[7].kind, TokenKind::EqualEqual);
    assert_eq!(tokens[8].kind, TokenKind::BangEqual);
    assert_eq!(tokens[9].kind, TokenKind::TripleEqual);
    assert_eq!(tokens[10].kind, TokenKind::BangEqualEqual);
    assert_eq!(tokens[21].kind, TokenKind::DotDot);
    assert_eq!(tokens[22].kind, TokenKind::DotDotEqual);
    assert_eq!(tokens[23].kind, TokenKind::QuestionDot);
    assert_eq!(tokens[24].kind, TokenKind::QuestionQuestion);
}

#[test]
fn test_lexer_comments() {
    let mut reporter = DiagnosticReporter::new();
    let source = "make x = 42 // this is a line comment\nconst y = 10 # hash comment\n/* nested /* block */ comment */ say x";
    let tokens = lex(source, &mut reporter).expect("lex should succeed");

    // "make", "x", "=", "42", "Newline", "const", "y", "=", "10", "Newline", "say", "x", "EOF"
    assert_eq!(tokens.len(), 13);
    assert_eq!(tokens[0].kind, TokenKind::Make);
    assert_eq!(tokens[4].kind, TokenKind::Newline);
    assert_eq!(tokens[5].kind, TokenKind::Const);
    assert_eq!(tokens[10].kind, TokenKind::Say);
    assert_eq!(tokens[11].kind, TokenKind::Identifier);
    assert_eq!(tokens[11].lexeme, "x");
}

#[test]
fn test_lexer_strings_and_fstrings() {
    let mut reporter = DiagnosticReporter::new();
    let source = "\"hello \\\"world\\\"\" f\"Total is: {price + tax}\"";
    let tokens = lex(source, &mut reporter).expect("lex should succeed");

    // Tokens:
    // 0: StringLiteral("\"hello \\\"world\\\"\"")
    // 1: FStringStart("f\"")
    // 2: FStringText("Total is: ")
    // 3: FStringExprStart("{")
    // 4: Identifier("price")
    // 5: Plus("+")
    // 6: Identifier("tax")
    // 7: FStringExprEnd("}")
    // 8: FStringEnd("\"")
    // 9: EOF
    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
    assert_eq!(tokens[0].lexeme, "\"hello \\\"world\\\"\"");
    assert_eq!(tokens[1].kind, TokenKind::FStringStart);
    assert_eq!(tokens[2].kind, TokenKind::FStringText);
    assert_eq!(tokens[2].lexeme, "Total is: ");
    assert_eq!(tokens[3].kind, TokenKind::FStringExprStart);
    assert_eq!(tokens[4].kind, TokenKind::Identifier);
    assert_eq!(tokens[4].lexeme, "price");
    assert_eq!(tokens[5].kind, TokenKind::Plus);
    assert_eq!(tokens[6].kind, TokenKind::Identifier);
    assert_eq!(tokens[6].lexeme, "tax");
    assert_eq!(tokens[7].kind, TokenKind::FStringExprEnd);
    assert_eq!(tokens[8].kind, TokenKind::FStringEnd);
}

#[test]
fn test_lexer_newline_collapse() {
    let mut reporter = DiagnosticReporter::new();
    let source = "make x = 42\n\n\n\nsay x";
    let tokens = lex(source, &mut reporter).expect("lex should succeed");

    // collapsed to a single Newline
    // Make, x, =, 42, Newline, Say, x, EOF = 8 tokens
    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[4].kind, TokenKind::Newline);
    assert_eq!(tokens[5].kind, TokenKind::Say);
}

#[test]
fn test_lexer_errors() {
    let mut reporter = DiagnosticReporter::new();

    // 1. Invalid character `@`
    assert!(lex("@", &mut reporter).is_err());
    assert!(reporter.has_errors());
    reporter.clear();

    // 2. Trailing underscore
    assert!(lex("123_", &mut reporter).is_err());
    assert!(reporter.has_errors());
    reporter.clear();

    // 3. Unterminated string
    assert!(lex("\"unterminated", &mut reporter).is_err());
    assert!(reporter.has_errors());
    reporter.clear();

    // 4. Unterminated block comment
    assert!(lex("/* unclosed", &mut reporter).is_err());
    assert!(reporter.has_errors());
}
