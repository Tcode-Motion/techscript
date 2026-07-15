use techscript_ast::{NodeId, Program};
use techscript_common::Span;
use techscript_interpreter::Interpreter;
use techscript_semantic::CheckedProgram;
use techscript_semantic::SymbolTable;

fn main() {
    let checked = CheckedProgram {
        program: Program {
            id: NodeId(0),
            statements: vec![],
            span: Span::new(0, 0),
        },
        symbols: SymbolTable::default(),
    };
    let mut interpreter = Interpreter::new();
    if let Ok(val) = interpreter.interpret(checked) {
        println!("Interpreter exited with value: {:?}", val);
    }
}
