use techscript_ast::{Program, NodeId};
use techscript_common::Span;
use techscript_semantic::CheckedProgram;
use techscript_semantic::SymbolTable;
use techscript_interpreter::{interpret, Value};

#[test]
fn test_interpreter_empty() {
    let checked = CheckedProgram {
        program: Program {
            id: NodeId(0),
            statements: vec![],
            span: Span::new(0, 0),
        },
        symbols: SymbolTable::default(),
    };
    let val = interpret(checked).expect("interpret should succeed");
    assert_eq!(val, Value::None);
}
