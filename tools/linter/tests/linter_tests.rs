use techscript_ast::{Program, NodeId};
use techscript_common::Span;
use techscript_errors::Diagnostic;
use techscript_semantic::{CheckedProgram, SymbolTable};
use techscript_linter::{Linter, LintRule};

struct DummyRule;

impl LintRule for DummyRule {
    fn name(&self) -> &'static str {
        "dummy"
    }

    fn check(&self, _program: &CheckedProgram) -> Vec<Diagnostic> {
        vec![]
    }
}

#[test]
fn test_linter_engine() {
    let mut linter = Linter::new();
    linter.add_rule(Box::new(DummyRule));
    let checked = CheckedProgram {
        program: Program {
            id: NodeId(0),
            statements: vec![],
            span: Span::new(0, 0),
        },
        symbols: SymbolTable::default(),
    };
    let diags = linter.lint(&checked);
    assert_eq!(diags.len(), 0);
}
