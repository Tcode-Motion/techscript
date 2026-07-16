use techscript_ast::LiteralVal;
use techscript_errors::DiagnosticReporter;
use techscript_ir::instruction::Op;
use techscript_ir::lower;
use techscript_lexer::lex;
use techscript_optimizer::{optimize, OptimizationContext, OptimizationLevel};
use techscript_parser::parse;
use techscript_semantic::analyze;

fn optimize_source(source: &str, level: OptimizationLevel) -> techscript_ir::Module {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).expect("lexing should succeed");
    let program = parse(&tokens, &mut reporter).expect("parsing should succeed");
    let checked = analyze(program, &mut reporter).expect("semantic should succeed");

    let res = lower(&checked.program, "test_module");
    let mut module = res.module;

    let mut ctx = OptimizationContext::new();
    ctx.level = level;
    let _opt_res = optimize(&mut module, &ctx);
    module
}

#[test]
fn test_optimizer_constant_folding() {
    let source = "make x = 5 + 10";
    let module = optimize_source(source, OptimizationLevel::O2);

    // The addition "5 + 10" should fold to a single constant 15
    let main = &module.functions[0];
    let insts = &main.blocks[0].instructions;

    let has_15 = insts.iter().any(|inst| {
        if let Op::Constant(LiteralVal::Int(val)) = &inst.op {
            *val == 15
        } else {
            false
        }
    });
    assert!(has_15);
}

#[test]
fn test_optimizer_algebraic_simplification() {
    let source = r#"
make x = 100
make y = x + 0
"#;
    let module = optimize_source(source, OptimizationLevel::O3);

    // "x + 0" should simplify to loading x directly
    let main = &module.functions[0];
    let insts = &main.blocks[0].instructions;

    // Verify no binary operator exists for addition with 0
    let has_add_zero = insts.iter().any(|inst| {
        matches!(
            &inst.op,
            Op::BinaryOp {
                right: techscript_ir::value::Value::Const(LiteralVal::Int(0)),
                ..
            }
        )
    });
    assert!(!has_add_zero);
}

#[test]
fn test_optimizer_dead_code_elimination() {
    let source = r#"
make x = 10
make y = 20
say x
"#;
    let module = optimize_source(source, OptimizationLevel::O3);

    // y is completely unused, its definition should be eliminated in O3
    let main = &module.functions[0];
    let insts = &main.blocks[0].instructions;

    let defines_20 = insts
        .iter()
        .any(|inst| matches!(&inst.op, Op::Constant(LiteralVal::Int(20))));
    assert!(!defines_20);
}

#[test]
fn test_optimizer_branch_simplification() {
    let source = r#"
if true {
    say 1
} else {
    say 2
}
"#;
    let module = optimize_source(source, OptimizationLevel::O3);
    let main = &module.functions[0];

    // The conditional branch should simplify to an unconditional jump
    let has_conditional_jump = main.blocks.iter().any(|b| {
        if let Some(ref term) = b.terminator {
            matches!(
                term.kind,
                techscript_ir::instruction::TerminatorKind::ConditionalJump { .. }
            )
        } else {
            false
        }
    });
    assert!(!has_conditional_jump);
}
