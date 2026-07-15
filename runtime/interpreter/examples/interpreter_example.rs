use techscript_ast::{Program, NodeId};
use techscript_common::Span;
use techscript_semantic::CheckedProgram;
use techscript_semantic::SymbolTable;
use techscript_interpreter::Interpreter;

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
