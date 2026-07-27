//! # tsc build Command
//!
//! Compiles project sources, runs optimizations, generates bytecode bundles,
//! debug symbols, source maps, compile database, and builds manifests.

use crate::artifacts::{ArtifactManager, BuildManifest, BuildOutput};
use crate::cache::BuildCache;
use crate::config::{CliConfig, ConfigManager};
use crate::events::EventBus;
use crate::exit_code::ExitCode;
use crate::logging::{LogFormat, LogLevel, Logger};
use crate::pipeline::{BuildProfile, CompilationPipeline, ExecutionBackend, PipelineOptions};
use crate::profiler::TimingProfiler;
use crate::project::ProjectBuildGraph;
use crate::watch::FileWatcher;
use colored::Colorize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub fn execute(
    file_path: Option<&str>,
    profile_str: Option<&str>,
    watch: bool,
    time: bool,
    target_str: &str,
    verbose: bool,
) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Resolve build profile
    let profile = match profile_str.unwrap_or("debug").to_lowercase().as_str() {
        "release" => BuildProfile::Release,
        "release-fast" => BuildProfile::ReleaseFast,
        "release-small" => BuildProfile::ReleaseSmall,
        _ => BuildProfile::Debug,
    };

    let target_backend = match target_str.to_lowercase().as_str() {
        "native" => ExecutionBackend::Native,
        _ => ExecutionBackend::Vm,
    };

    let build_closure = || -> Result<(), anyhow::Error> {
        let start_time = Instant::now();

        // 1. Load project configurations
        let cli_cfg = CliConfig {
            log_level: Some(if verbose {
                LogLevel::Verbose
            } else {
                LogLevel::Normal
            }),
            ..Default::default()
        };
        let config_mgr = ConfigManager::load(Some(&current_dir), None, &cli_cfg)?;
        let cache = BuildCache::load(&current_dir.join("build").join(".tsc-cache"))?;
        let artifacts = ArtifactManager::new(&current_dir);
        artifacts.prepare()?;

        let mut events = EventBus::new();
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
            backend: target_backend,
            emit_source_map: true,
            emit_debug_symbols: true,
            emit_compile_db: true,
            emit_build_manifest: true,
        };

        // 2. Discover build graph
        let mut graph = ProjectBuildGraph::discover(&current_dir)?;
        pipeline.source_manager = techscript_common::SourceManager::new();
        graph.resolve_dependencies(&mut pipeline.source_manager)?;
        graph.compute_build_order()?;

        // 3. Compile project
        let results = pipeline.compile_project(&mut graph, &opts)?;

        // 4. Save compilation output
        let mut build_outputs = Vec::new();
        for res in &results {
            if let Some(bytecode) = &res.bytecode {
                let name = res.path.file_stem().unwrap_or_default().to_string_lossy();
                let serialized = techscript_bytecode::BytecodeSerializer::serialize(bytecode)
                    .map_err(|e| anyhow::anyhow!("Bytecode serialization failed: {}", e))?;

                let bc_path = pipeline.artifacts.write_bytecode(&name, &serialized)?;
                build_outputs.push(BuildOutput {
                    path: bc_path.to_string_lossy().to_string(),
                    category: "bytecode".to_string(),
                    size_bytes: serialized.len() as u64,
                });

                // Write debug symbols and source maps
                let dbg_symbols = techscript_bytecode::DebugSymbols {
                    local_names: HashMap::new(),
                    function_names: HashMap::new(),
                };
                let dbg_path = pipeline
                    .artifacts
                    .write_debug_symbols(&name, &dbg_symbols)?;
                build_outputs.push(BuildOutput {
                    path: dbg_path.to_string_lossy().to_string(),
                    category: "debug_symbols".to_string(),
                    size_bytes: std::fs::metadata(&dbg_path)?.len(),
                });

                let source_map = techscript_bytecode::SourceMap {
                    mappings: Vec::new(),
                };
                let sm_path = pipeline.artifacts.write_source_map(&name, &source_map)?;
                build_outputs.push(BuildOutput {
                    path: sm_path.to_string_lossy().to_string(),
                    category: "source_map".to_string(),
                    size_bytes: std::fs::metadata(&sm_path)?.len(),
                });
            }
        }

        // 5. Emit build manifest
        let manifest = BuildManifest {
            compiler_version: techscript_common::TECHSCRIPT_VERSION.to_string(),
            optimization_level: format!("{:?}", config_mgr.effective.optimization_level),
            source_hash: String::new(), // Combined hash if needed
            build_timestamp: format!("{:?}", std::time::SystemTime::now()),
            outputs: build_outputs,
            dependencies: Vec::new(),
            bytecode_version: "1.0.0".to_string(),
            target_backend: "vm".to_string(),
            build_profile: format!("{:?}", profile),
            total_duration_ms: start_time.elapsed().as_millis() as u64,
        };
        pipeline.artifacts.write_build_manifest(&manifest)?;

        // 5.5. Link Native binaries (if native)
        if target_backend == ExecutionBackend::Native {
            for res in &results {
                let name = res.path.file_stem().unwrap_or_default().to_string_lossy();
                invoke_linker(&current_dir, &name)?;
            }
        }

        // 6. Emit compile commands DB
        let mut compile_db = crate::compile_db::CompilationDatabase::new();
        for res in &results {
            compile_db.record(crate::compile_db::CompileCommand {
                directory: current_dir.to_string_lossy().to_string(),
                file: res.path.to_string_lossy().to_string(),
                arguments: vec![
                    "tsc".to_string(),
                    "build".to_string(),
                    res.path.to_string_lossy().to_string(),
                ],
                output: pipeline
                    .artifacts
                    .artifact_path(
                        crate::artifacts::ArtifactCategory::Bytecode,
                        &res.path.file_stem().unwrap().to_string_lossy(),
                    )
                    .to_string_lossy()
                    .to_string(),
            });
        }
        compile_db.write(&current_dir.join("build"))?;

        if time {
            println!("{}", pipeline.profiler.render());
        }

        let duration = start_time.elapsed();
        let main_output = if target_backend == ExecutionBackend::Native {
            "build/bin/main.exe"
        } else {
            "build/bytecode/main.tsb"
        };
        println!(
            "{} Build completed, Output: {}, Time: {:.2?}",
            "✓".green(),
            main_output.cyan(),
            duration
        );
        Ok(())
    };

    if watch {
        let watcher = FileWatcher::new(&current_dir, 500);
        let _ = watcher.watch(|changed| {
            println!(
                "\n[tsc watch] Change detected in: {:?}. Rebuilding...",
                changed
            );
            if let Err(e) = build_closure() {
                eprintln!("Build failed: {}", e);
            }
        });
        ExitCode::Success
    } else {
        match build_closure() {
            Ok(_) => ExitCode::Success,
            Err(e) => {
                eprintln!("Build failed: {}", e);
                ExitCode::CompilationError
            }
        }
    }
}

fn invoke_linker(current_dir: &Path, name: &str) -> Result<(), anyhow::Error> {
    let workspace_root = current_dir
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists())
        .unwrap_or(current_dir);

    let build_profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let mut runtime_lib = None;

    let candidates = vec![
        workspace_root.join("target").join(build_profile),
        workspace_root.join("target").join("release"),
        workspace_root.join("target").join("debug"),
    ];

    for c in candidates {
        let static_lib = if cfg!(windows) {
            c.join("techscript_native_runtime.lib")
        } else {
            c.join("libtechscript_native_runtime.a")
        };
        if static_lib.exists() {
            runtime_lib = Some(static_lib);
            break;
        }
    }

    let runtime_lib_path = runtime_lib.ok_or_else(|| {
        anyhow::anyhow!("Could not find techscript_native_runtime static library. Please run 'cargo build' to compile it.")
    })?;

    let build_dir = current_dir.join("build");
    let obj_ext = if cfg!(windows) { "obj" } else { "o" };
    let exec_ext = if cfg!(windows) { ".exe" } else { "" };

    let obj_path = build_dir.join(format!("{}.{}", name, obj_ext));
    let exec_path = build_dir.join(format!("{}{}", name, exec_ext));

    if !obj_path.exists() {
        return Err(anyhow::anyhow!(
            "Compiled object file not found at {:?}",
            obj_path
        ));
    }

    println!("Linking {}...", name);

    let mut status = if cfg!(windows) {
        // Use MSVC link.exe
        let mut cmd = std::process::Command::new("link.exe");
        cmd.arg(format!("/OUT:{}", exec_path.to_string_lossy()))
            .arg(obj_path.to_string_lossy().to_string())
            .arg(runtime_lib_path.to_string_lossy().to_string())
            .arg("ws2_32.lib")
            .arg("user32.lib")
            .arg("shell32.lib")
            .arg("advapi32.lib")
            .arg("bcrypt.lib")
            .arg("ntdll.lib")
            .arg("msvcrt.lib")
            .status()
    } else {
        // Use standard cc
        let mut cmd = std::process::Command::new("cc");
        cmd.arg("-o")
            .arg(&exec_path)
            .arg(&obj_path)
            .arg(&runtime_lib_path)
            .arg("-lpthread")
            .arg("-ldl")
            .status()
    };

    // If link.exe failed to spawn because it's not in path on Windows, try executing cl.exe or finding VS path,
    // or fallback to print a detailed diagnostic.
    if let Err(ref e) = status {
        if cfg!(windows) && e.kind() == std::io::ErrorKind::NotFound {
            println!("Warning: link.exe not found in PATH. Make sure Visual Studio/MSVC command prompt is active.");
            println!("Attempting to invoke cl /link...");
            status = std::process::Command::new("cl")
                .arg(&obj_path)
                .arg(&runtime_lib_path)
                .arg("/link")
                .arg(format!("/OUT:{}", exec_path.to_string_lossy()))
                .status();
        }
    }

    match status {
        Ok(s) if s.success() => {
            println!("Successfully linked executable: {:?}", exec_path);
            Ok(())
        }
        Ok(s) => Err(anyhow::anyhow!(
            "Linker exited with error code: {:?}",
            s.code()
        )),
        Err(e) => Err(anyhow::anyhow!("Failed to run linker command: {}", e)),
    }
}
