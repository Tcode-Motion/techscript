//! # tsc test Command
//!
//! Discovers test files (*_test.txs or tests/ directory), executes them,
//! and displays execution timings and test results.

use crate::artifacts::ArtifactManager;
use crate::cache::BuildCache;
use crate::config::{CliConfig, ConfigManager};
use crate::events::EventBus;
use crate::exit_code::ExitCode;
use crate::logging::{LogFormat, LogLevel, Logger};
use crate::pipeline::{BuildProfile, CompilationPipeline, ExecutionBackend, PipelineOptions};
use crate::profiler::TimingProfiler;
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub fn execute(
    dir_str: Option<&str>,
    filter: Option<&str>,
    parallel: bool,
    ignored: bool,
) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let target_dir = dir_str.map(PathBuf::from).unwrap_or(current_dir);

    println!("Discovering tests in {}...", target_dir.display());

    let mut test_files = Vec::new();
    find_test_files(&target_dir, &mut test_files);

    if test_files.is_empty() {
        println!("No test files discovered.");
        return ExitCode::Success;
    }

    // Apply filters
    if let Some(f) = filter {
        let f_lower = f.to_lowercase();
        test_files.retain(|path| {
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                .to_lowercase();
            name.contains(&f_lower)
        });
    }

    println!("Running {} test file(s)...", test_files.len());

    let start_time = Instant::now();
    let mut passed = 0;
    let mut failed = 0;

    for test_file in &test_files {
        let test_start = Instant::now();
        print!(
            "  Running test {} ... ",
            test_file.file_name().unwrap().to_string_lossy()
        );

        // Execute compilation and VM run
        let mut reporter = techscript_errors::DiagnosticReporter::new();
        let content = std::fs::read_to_string(test_file).unwrap_or_default();
        let run_res = || -> Result<(), anyhow::Error> {
            let tokens = techscript_lexer::lex(&content, &mut reporter)
                .map_err(|e| anyhow::anyhow!("Lexing failed: {:?}", e))?;
            let ast = techscript_parser::parse(&tokens, &mut reporter)
                .map_err(|e| anyhow::anyhow!("Parsing failed: {:?}", e))?;
            let checked = techscript_semantic::analyze(ast, &mut reporter)
                .map_err(|e| anyhow::anyhow!("Semantic analysis failed: {:?}", e))?;
            let lowered = techscript_ir::lower(&checked.program, "test");
            let mut module = lowered.module;
            let opt_ctx = techscript_optimizer::OptimizationContext::new();
            let _ = techscript_optimizer::optimize(&mut module, &opt_ctx);
            let bytecode = techscript_bytecode::compile(&module);
            let _ = techscript_vm::run(bytecode)
                .map_err(|e| anyhow::anyhow!("VM run failed: {:?}", e))?;
            Ok(())
        }();

        let elapsed = test_start.elapsed();
        if run_res.is_ok() && !reporter.has_errors() {
            println!("{}", "PASSED".green());
            passed += 1;
        } else {
            println!("{}", "FAILED".red());
            failed += 1;
        }
    }

    println!("------------------------------------------------------------");
    let duration = start_time.elapsed();
    println!(
        "Test Summary: {} passed, {} failed, finished in {:.2}s",
        passed,
        failed,
        duration.as_secs_f64()
    );

    if failed > 0 {
        ExitCode::TestFailure
    } else {
        ExitCode::Success
    }
}

fn find_test_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if name != "build" && name != ".git" && name != "target" {
                        find_test_files(&path, files);
                    }
                } else {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if name.contains("_test")
                        || path
                            .parent()
                            .map(|p| p.file_name().unwrap().to_string_lossy() == "tests")
                            .unwrap_or(false)
                    {
                        let ext = path.extension().unwrap_or_default().to_string_lossy();
                        if ext == "txs" || ext == "ts" {
                            files.push(path);
                        }
                    }
                }
            }
        }
    }
}
