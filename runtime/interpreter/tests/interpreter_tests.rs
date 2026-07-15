use techscript_ast::{NodeId, Program};
use techscript_common::Span;
use techscript_interpreter::{interpret, Value};
use techscript_semantic::CheckedProgram;
use techscript_semantic::SymbolTable;

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
