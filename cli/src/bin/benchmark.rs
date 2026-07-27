// cli/src/bin/benchmark.rs
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting TechScript 2.0 compiler performance benchmark...");

    // Sample code: Fibonacci 25 to stress the lexer, parser, semantic analysis, and VM runtime
    let source_code = r#"
        fun fib(n) {
            if (n < 2) {
                return n;
            }
            return fib(n - 1) + fib(n - 2);
        }

        build main() {
            make result = fib(25);
            say result;
        }
    "#;

    // 1. Benchmark Lexer
    let mut reporter = techscript_errors::DiagnosticReporter::new();
    let start_lex = Instant::now();
    let tokens = techscript_lexer::lex_recovered(source_code, &mut reporter);
    let duration_lex = start_lex.elapsed();
    println!("- Lexing: {:.3} ms", duration_lex.as_secs_f64() * 1000.0);

    // 2. Benchmark Parser
    let start_parse = Instant::now();
    let program = techscript_parser::parse_recovered(&tokens, &mut reporter);
    let duration_parse = start_parse.elapsed();
    println!("- Parsing: {:.3} ms", duration_parse.as_secs_f64() * 1000.0);

    // 3. Benchmark Semantic Analysis
    let start_semantic = Instant::now();
    let _checked = techscript_semantic::analyze(program.clone(), &mut reporter);
    let duration_semantic = start_semantic.elapsed();
    println!(
        "- Semantic Analysis: {:.3} ms",
        duration_semantic.as_secs_f64() * 1000.0
    );

    // 4. Benchmark IR lowering & optimization
    let start_ir = Instant::now();
    let lowered = techscript_ir::lower(&program, "main");
    let mut module = lowered.module;
    let opt_ctx = techscript_optimizer::OptimizationContext::new();
    let _opt_res = techscript_optimizer::optimize(&mut module, &opt_ctx);
    let duration_ir = start_ir.elapsed();
    println!(
        "- IR Lowering & Optimization: {:.3} ms",
        duration_ir.as_secs_f64() * 1000.0
    );

    // 5. Benchmark Bytecode Generation
    let start_bytecode = Instant::now();
    let bytecode = techscript_bytecode::compile(&module);
    let duration_bytecode = start_bytecode.elapsed();
    println!(
        "- Bytecode Generation: {:.3} ms",
        duration_bytecode.as_secs_f64() * 1000.0
    );

    // 6. Benchmark VM Execution
    let start_vm = Instant::now();
    let mut vm = techscript_vm::VM::new(bytecode);
    let vm_res = vm.run();
    let duration_vm = start_vm.elapsed();
    println!(
        "- VM Execution (Fibonacci 25): {:.3} ms",
        duration_vm.as_secs_f64() * 1000.0
    );

    if let Ok(res) = vm_res {
        println!("  Result value: {}", res);
    }

    // Generate Markdown report
    let report_content = format!(
        r#"# TechScript 2.0 Compiler Performance Report

Generated on standard test environment (Windows target execution).

## Compilation Phase Speeds
| Phase | Duration |
| :--- | :--- |
| **Lexing & Tokenization** | {:.3} ms |
| **Pratt Parsing & AST Building** | {:.3} ms |
| **Semantic Check & Name Binding** | {:.3} ms |
| **SSA IR Lowering & Optimization** | {:.3} ms |
| **Bytecode Generation** | {:.3} ms |
| **VM Execution (Fibonacci 25)** | {:.3} ms |

## Benchmark Details
- **Test File**: Recursive Fibonacci 25 calculation (`fib(25)`)
- **Optimization Level**: SSA optimizations enabled (Constant Folding, Dead Code Elimination)
- **Garbage Collector**: Mark-sweep tracing enabled
"#,
        duration_lex.as_secs_f64() * 1000.0,
        duration_parse.as_secs_f64() * 1000.0,
        duration_semantic.as_secs_f64() * 1000.0,
        duration_ir.as_secs_f64() * 1000.0,
        duration_bytecode.as_secs_f64() * 1000.0,
        duration_vm.as_secs_f64() * 1000.0
    );

    let report_dir = Path::new("docs/benchmarks");
    fs::create_dir_all(report_dir)?;
    fs::write(report_dir.join("benchmark_report.md"), report_content)?;
    println!("Successfully generated benchmark report at docs/benchmarks/benchmark_report.md");

    Ok(())
}
