//! # tsc run Command
//!
//! Compiles and executes a source file or project.
//! Supports watch mode, timing/memory profiler, and backend selection.

use crate::artifacts::ArtifactManager;
use crate::cache::BuildCache;
use crate::config::{CliConfig, ConfigManager};
use crate::events::EventBus;
use crate::exit_code::ExitCode;
use crate::logging::{LogFormat, LogLevel, Logger};
use crate::pipeline::{BuildProfile, CompilationPipeline, ExecutionBackend, PipelineOptions};
use crate::profiler::TimingProfiler;
use crate::watch::FileWatcher;
use std::path::{Path, PathBuf};

pub fn execute(
    file_path: &str,
    profile_str: Option<&str>,
    backend_str: Option<&str>,
    watch: bool,
    time: bool,
    verbose: bool,
    native: bool,
    show_return: bool,
    debug: bool,
    double_click: bool,
) -> ExitCode {
    let path = PathBuf::from(file_path);
    if !path.exists() {
        eprintln!("Error: Source file does not exist: {}", file_path);
        if double_click {
            println!("\n[Process completed. Press Enter to exit...]");
            let mut buffer = String::new();
            let _ = std::io::stdin().read_line(&mut buffer);
        }
        return ExitCode::IoError;
    }

    let current_dir = path.parent().unwrap_or(&path).to_path_buf();

    // Resolve build profile
    let profile = match profile_str.unwrap_or("debug").to_lowercase().as_str() {
        "release" => BuildProfile::Release,
        "release-fast" => BuildProfile::ReleaseFast,
        "release-small" => BuildProfile::ReleaseSmall,
        _ => BuildProfile::Debug,
    };

    // Resolve execution backend
    let backend = if native {
        ExecutionBackend::Native
    } else {
        match backend_str.unwrap_or("vm").to_lowercase().as_str() {
            "interpreter" | "interp" => ExecutionBackend::Interpreter,
            "native" => ExecutionBackend::Native,
            _ => ExecutionBackend::Vm,
        }
    };

    if watch {
        // Watch mode
        let watcher = FileWatcher::new(&current_dir, 500);
        let _ = watcher.watch(|changed| {
            println!(
                "\n[tsc watch] Change detected in: {:?}. Re-running...",
                changed
            );
            if let Err(e) = run_once(&path, &current_dir, profile, backend, time, verbose, show_return, debug) {
                eprintln!("Run failed: {}", e);
            }
        });
        ExitCode::Success
    } else {
        let res = run_once(&path, &current_dir, profile, backend, time, verbose, show_return, debug);
        if double_click {
            println!("\n[Process completed. Press Enter to exit...]");
            let mut buffer = String::new();
            let _ = std::io::stdin().read_line(&mut buffer);
        }
        match res {
            Ok(_) => ExitCode::Success,
            Err(e) => {
                eprintln!("Run failed: {}", e);
                ExitCode::CompilationError
            }
        }
    }
}

fn run_once(
    path: &Path,
    current_dir: &Path,
    profile: BuildProfile,
    backend: ExecutionBackend,
    time: bool,
    verbose: bool,
    show_return: bool,
    debug: bool,
) -> Result<(), anyhow::Error> {
    let cli_cfg = CliConfig {
        log_level: Some(if debug {
            LogLevel::Trace
        } else if verbose {
            LogLevel::Verbose
        } else {
            LogLevel::Normal
        }),
        ..Default::default()
    };
    let config_mgr = ConfigManager::load(Some(current_dir), None, &cli_cfg)?;
    let cache = BuildCache::load(&current_dir.join("build").join(".tsc-cache"))?;
    let artifacts = ArtifactManager::new(current_dir);
    let events = EventBus::new();
    let logger = Logger::new(config_mgr.effective.log_level, LogFormat::Human);
    let profiler = TimingProfiler::new(time);

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
        profile,
        backend,
        emit_source_map: true,
        emit_debug_symbols: true,
        emit_compile_db: true,
        emit_build_manifest: true,
    };

    // Compile
    let res = pipeline.compile_unit(path, &opts)?;

    // Report diagnostics
    let renderer = crate::diagnostics::DiagnosticRenderer::auto_detect(&pipeline.source_manager);
    for diag in &res.diagnostics {
        renderer.emit(diag);
    }

    if res.bytecode.is_none() && opts.backend != ExecutionBackend::Native {
        return Err(anyhow::anyhow!("Compilation failed"));
    }

    // Execute
    let start_time = std::time::Instant::now();
    let eval_val = pipeline.execute(&res, &opts)?;
    let run_duration = start_time.elapsed();

    // If timing profiler is requested, print table
    if time {
        println!("{}", pipeline.profiler.render());
    }

    // Print final return value
    let is_null = eval_val == techscript_runtime::RuntimeValue::Null;
    if show_return || verbose || debug || !is_null {
        if verbose || debug {
            println!("\n✓ Program completed successfully");
            println!("Time: {:.1?}", run_duration);
            println!("Return value: {:?}", eval_val);
        } else {
            println!("Execution finished with value: {:?}", eval_val);
        }
    }

    Ok(())
}
