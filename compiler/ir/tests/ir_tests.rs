use techscript_errors::DiagnosticReporter;
use techscript_ir::instruction::TerminatorKind;
use techscript_ir::lower;
use techscript_lexer::lex;
use techscript_parser::parse;
use techscript_semantic::analyze;

fn lower_source(source: &str) -> techscript_ir::Module {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).expect("lexing should succeed");
    let program = parse(&tokens, &mut reporter).expect("parsing should succeed");
    let checked = analyze(program, &mut reporter).expect("semantic should succeed");

    let res = lower(&checked.program, "test_module");
    assert!(res.diagnostics.is_empty());
    res.module
}

#[test]
fn test_ir_empty() {
    let module = lower_source("");
    assert_eq!(module.name, "test_module");
    assert_eq!(module.functions.len(), 1);
    assert_eq!(module.functions[0].name, "main");
    assert_eq!(module.functions[0].blocks.len(), 1);
}

#[test]
fn test_ir_basic_arithmetic() {
    let module = lower_source("make x = 2 * 3 + 4");
    let main = &module.functions[0];
    assert_eq!(main.blocks.len(), 1);

    // Check that we have a sequence of load, binary arithmetic, and store instructions
    let insts = &main.blocks[0].instructions;
    assert!(!insts.is_empty());
}

#[test]
fn test_ir_if_conditional_branching() {
    let source = r#"
make x = 10
if x > 5 {
    x = 1
} else {
    x = 2
}
"#;
    let module = lower_source(source);
    let main = &module.functions[0];

    // Should split into entry, then, else, and merge blocks
    assert!(main.blocks.len() >= 4);

    // Find entry block
    let entry = &main.blocks[0];
    assert!(entry.terminator.is_some());

    // Terminator should be a ConditionalJump
    if let Some(ref term) = entry.terminator {
        match &term.kind {
            TerminatorKind::ConditionalJump { .. } => {}
            _ => panic!("Expected ConditionalJump terminator"),
        }
    }
}

#[test]
fn test_ir_while_loop_cfg() {
    let source = r#"
make i = 0
while i < 10 {
    i = i + 1
}
"#;
    let module = lower_source(source);
    let main = &module.functions[0];

    // Exit block, cond block, body block, entry block
    assert!(main.blocks.len() >= 4);
}

#[test]
fn test_ir_optional_chaining_lower() {
    let source = r#"
make x = none
make y = x?.foo
"#;
    let module = lower_source(source);
    let main = &module.functions[0];

    // Should contain opt_not_null, opt_null, opt_merge blocks
    let block_labels: Vec<String> = main.blocks.iter().map(|b| b.label.clone()).collect();
    assert!(block_labels.contains(&"opt_not_null".to_string()));
    assert!(block_labels.contains(&"opt_null".to_string()));
    assert!(block_labels.contains(&"opt_merge".to_string()));
}

#[test]
fn test_ir_null_coalescing_lower() {
    let source = r#"
make x = none
make y = x ?? 100
"#;
    let module = lower_source(source);
    let main = &module.functions[0];

    // Should contain coal_null, coal_not_null, coal_merge blocks
    let block_labels: Vec<String> = main.blocks.iter().map(|b| b.label.clone()).collect();
    assert!(block_labels.contains(&"coal_null".to_string()));
    assert!(block_labels.contains(&"coal_not_null".to_string()));
    assert!(block_labels.contains(&"coal_merge".to_string()));
}

#[test]
fn test_ir_dsl_block_empty() {
    let source = r#"
use web
hero
  title "Empty"
end
"#;
    let module = lower_source(source);
    // Should have a DslBlockIR entry
    assert_eq!(module.dsl_blocks.len(), 1);
    assert_eq!(module.dsl_blocks[0].kind, "hero");
    assert!(module.dsl_blocks[0].args.is_empty());
    assert_eq!(module.dsl_blocks[0].properties.len(), 1);
    assert!(module.dsl_blocks[0].children.is_empty());
}

#[test]
fn test_ir_dsl_block_with_properties() {
    let source = r#"
use canvas
logo "TS"
  text "My Logo"
  color "hash333"
end
"#;
    let module = lower_source(source);
    assert_eq!(module.dsl_blocks.len(), 1);
    let block = &module.dsl_blocks[0];
    assert_eq!(block.kind, "logo");
    assert_eq!(block.args.len(), 1);
    assert_eq!(block.properties.len(), 2);
}

#[test]
fn test_ir_dsl_block_nested() {
    let source = r#"
use web
website
  title "My Site"
  page "/"
    title "Home"
    hero
      title "Welcome"
    end
  end
end
"#;
    let module = lower_source(source);
    assert!(module.dsl_blocks.len() >= 3);
    let dsl_blocks = &module.dsl_blocks;
    // Children are traversed first, so the first block registered is "hero" (innermost)
    assert_eq!(dsl_blocks[0].kind, "hero");
    assert_eq!(dsl_blocks[1].kind, "page");
    assert_eq!(dsl_blocks[2].kind, "website");
    assert_eq!(dsl_blocks[2].children.len(), 1);
    assert_eq!(dsl_blocks[2].children[0].1, "page");
}
