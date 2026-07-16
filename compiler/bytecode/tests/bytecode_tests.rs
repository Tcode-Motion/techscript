use techscript_bytecode::{compile, BytecodeDisassembler, BytecodeSerializer, BytecodeValidator};
use techscript_errors::DiagnosticReporter;
use techscript_ir::lower;
use techscript_lexer::lex;
use techscript_optimizer::{optimize, OptimizationContext, OptimizationLevel};
use techscript_parser::parse;
use techscript_semantic::analyze;

fn compile_source(source: &str) -> techscript_bytecode::BytecodeModule {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).expect("lexing should succeed");
    let program = parse(&tokens, &mut reporter).expect("parsing should succeed");
    let checked = analyze(program, &mut reporter).expect("semantic should succeed");

    let res = lower(&checked.program, "test_module");
    let mut module = res.module;

    let mut ctx = OptimizationContext::new();
    ctx.level = OptimizationLevel::O2;
    let _opt_res = optimize(&mut module, &ctx);

    compile(&module)
}

#[test]
fn test_bytecode_compilation_and_validation() {
    let source = r#"
make x = 10 + 20
if x > 15 {
    say x
}
"#;
    let bc_module = compile_source(source);

    // The validator should pass successfully
    let validator = BytecodeValidator::new();
    let val_res = validator.validate(&bc_module);
    assert!(val_res.is_ok(), "Validation failed: {:?}", val_res.err());
}

#[test]
fn test_bytecode_constant_deduplication() {
    let source = r#"
make a = "hello"
make b = "hello"
make c = "hello"
"#;
    let bc_module = compile_source(source);
    let main = &bc_module.functions[0];

    // The constant pool should have exactly one instance of "hello" (and possibly Null/None placeholders)
    let hello_count = main
        .chunk
        .constants
        .constants
        .iter()
        .filter(|c| {
            if let techscript_ast::LiteralVal::Str(s) = c {
                s == "hello"
            } else {
                false
            }
        })
        .count();
    assert_eq!(hello_count, 1, "Constant pool did not deduplicate strings!");
}

#[test]
fn test_bytecode_serialization_roundtrip() {
    let source = r#"
make x = 100
make y = 200
make z = x + y
"#;
    let bc_module = compile_source(source);

    let serialized = BytecodeSerializer::serialize(&bc_module).expect("Serialization failed");
    assert!(!serialized.is_empty());

    let deserialized =
        BytecodeSerializer::deserialize(&serialized).expect("Deserialization failed");
    assert_eq!(bc_module.name, deserialized.name);
    assert_eq!(bc_module.functions.len(), deserialized.functions.len());
    assert_eq!(bc_module.functions[0].name, deserialized.functions[0].name);
}

#[test]
fn test_bytecode_disassembler() {
    let source = r#"
build f() {
    make x = 5
    return x
}
"#;
    let bc_module = compile_source(source);
    let dis = BytecodeDisassembler::disassemble_module(&bc_module);

    assert!(!dis.is_empty());
    assert!(dis.contains("Function: f"));
    assert!(dis.contains("Return"));
}
