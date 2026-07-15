use techscript_errors::DiagnosticReporter;
use techscript_lexer::Lexer;

fn main() {
    let mut reporter = DiagnosticReporter::new();
    let mut lexer = Lexer::new("make x = 42");
    if let Ok(tokens) = lexer.lex(&mut reporter) {
        println!("Scanned {} tokens successfully.", tokens.len());
    }
}
