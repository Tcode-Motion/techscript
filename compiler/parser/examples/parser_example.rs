use techscript_errors::DiagnosticReporter;
use techscript_parser::Parser;
use techscript_syntax::Token;

fn main() {
    let mut reporter = DiagnosticReporter::new();
    let tokens: Vec<Token> = vec![];
    let mut parser = Parser::new(&tokens);
    if let Ok(program) = parser.parse(&mut reporter) {
        println!("Parsed AST Program with Node ID {:?}", program.id);
    }
}
