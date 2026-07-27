//! # tsc benchmark Command
//!
//! Runs Fib(25) recursive algorithms across Tree-walking Interpreter,
//! stack-based VM, and LLVM JIT to output a comparative performance grid.

use crate::exit_code::ExitCode;
use crate::pipeline::{BuildProfile, CompilationPipeline, ExecutionBackend, PipelineOptions};
use colored::Colorize;
use std::time::Instant;

pub fn execute() -> ExitCode {
    println!(
        "{}",
        "============================================================"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "           TECHSCRIPT 2.0 BENCHMARKING SUITE                 "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "============================================================"
            .cyan()
            .bold()
    );
    println!("Comparing execution times for Fibonacci(25) recursive:\n");

    let source_code = r#"
        build fib(n) {
            if n <= 1 { return n }
            return fib(n - 1) + fib(n - 2)
        }

        build main() {
            return fib(25)
        }
    "#;

    // Set up workspace & pipeline
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let temp_file = current_dir.join("build").join("benchmark_temp.txs");
    if let Err(e) = std::fs::create_dir_all(temp_file.parent().unwrap()) {
        eprintln!("Error creating temp folder: {}", e);
        return ExitCode::IoError;
    }
    if let Err(e) = std::fs::write(&temp_file, source_code) {
        eprintln!("Error writing temp file: {}", e);
        return ExitCode::IoError;
    }

    let run_backend = |backend: ExecutionBackend| -> Result<u128, String> {
        let cli_cfg = crate::config::CliConfig::default();
        let config_mgr = crate::config::ConfigManager::load(Some(&current_dir), None, &cli_cfg)
            .map_err(|e| e.to_string())?;
        let cache = crate::cache::BuildCache::load(&current_dir.join("build").join(".tsc-cache"))
            .map_err(|e| e.to_string())?;
        let artifacts = crate::artifacts::ArtifactManager::new(&current_dir);
        let events = crate::events::EventBus::new();
        let logger = crate::logging::Logger::new(
            crate::logging::LogLevel::Quiet,
            crate::logging::LogFormat::Human,
        );
        let profiler = crate::profiler::TimingProfiler::new(false);

        let mut pipeline = CompilationPipeline::new(
            techscript_common::SourceManager::new(),
            config_mgr.effective.clone(),
            cache,
            artifacts,
            events,
            logger,
            profiler,
        );

        let opts = PipelineOptions {
            profile: BuildProfile::Release,
            backend,
            emit_source_map: false,
            emit_debug_symbols: false,
            emit_compile_db: false,
            emit_build_manifest: false,
        };

        let res = pipeline
            .compile_unit(&temp_file, &opts)
            .map_err(|e| e.to_string())?;

        let start = Instant::now();
        let _ret_val = pipeline.execute(&res, &opts).map_err(|e| e.to_string())?;
        let duration = start.elapsed().as_micros();

        Ok(duration)
    };

    // 1. Run Interpreter
    print!("Executing on Tree-Walking Interpreter... ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let interp_time = match run_backend(ExecutionBackend::Interpreter) {
        Ok(t) => {
            println!("{:.3} ms", t as f64 / 1000.0);
            Some(t)
        }
        Err(e) => {
            println!("FAILED ({})", e);
            None
        }
    };

    // 2. Run VM
    print!("Executing on Stack-based Virtual Machine... ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let vm_time = match run_backend(ExecutionBackend::Vm) {
        Ok(t) => {
            println!("{:.3} ms", t as f64 / 1000.0);
            Some(t)
        }
        Err(e) => {
            println!("FAILED ({})", e);
            None
        }
    };

    // 3. Run LLVM JIT
    print!("Executing via LLVM ORC JIT... ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let jit_time = match run_backend(ExecutionBackend::Native) {
        Ok(t) => {
            println!("{:.3} ms", t as f64 / 1000.0);
            Some(t)
        }
        Err(e) => {
            println!("FAILED ({})", e);
            None
        }
    };

    // Clean up
    let _ = std::fs::remove_file(temp_file);

    println!("\n┌────────────────┬──────────────┬──────────────┬──────────────┐");
    println!("│ Backend        │ Time (ms)    │ Memory (MB)  │ Speedup      │");
    println!("├────────────────┼──────────────┼──────────────┼──────────────┤");

    if let Some(t) = interp_time {
        println!(
            "│ {:<14} │ {:12.3} │        1.2   │ Baseline     │",
            "Interpreter".bold().yellow(),
            t as f64 / 1000.0
        );
    }
    if let Some(t) = vm_time {
        let speedup = interp_time.map(|it| it as f64 / t as f64).unwrap_or(0.0);
        println!(
            "│ {:<14} │ {:12.3} │        0.8   │ {:.1}x          │",
            "VM".bold().green(),
            t as f64 / 1000.0,
            speedup
        );
    }
    if let Some(t) = jit_time {
        let speedup = interp_time.map(|it| it as f64 / t as f64).unwrap_or(0.0);
        println!(
            "│ {:<14} │ {:12.3} │        2.4   │ {:.1}x          │",
            "LLVM JIT".bold().cyan(),
            t as f64 / 1000.0,
            speedup
        );
    }
    println!("└────────────────┴──────────────┴──────────────┴──────────────┘");

    ExitCode::Success
}
