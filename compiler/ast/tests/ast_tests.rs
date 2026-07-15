use techscript_common::{NodeId, Span};
use techscript_ast::{Program, Block, Statement};

#[test]
fn test_ast_program() {
    let program = Program {
        id: NodeId(1),
        statements: vec![
            Statement::Block(Block {
                id: NodeId(2),
                statements: vec![],
                span: Span::new(0, 2),
            })
        ],
        span: Span::new(0, 2),
    };

    assert_eq!(program.statements.len(), 1);
}
