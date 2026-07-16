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
use std::path::{Path, PathBuf};
use std::time::Instant;

pub fn execute(
    file_path: Option<&str>,
    profile_str: Option<&str>,
    watch: bool,
    time: bool,
) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Resolve build profile
    let profile = match profile_str.unwrap_or("debug").to_lowercase().as_str() {
        "release" => BuildProfile::Release,
        "release-fast" => BuildProfile::ReleaseFast,
        "release-small" => BuildProfile::ReleaseSmall,
        _ => BuildProfile::Debug,
    };

    let build_closure = || -> Result<(), anyhow::Error> {
        let start_time = Instant::now();

        // 1. Load project configurations
        let cli_cfg = CliConfig::default();
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
            backend: ExecutionBackend::Vm,
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

        println!("Build completed successfully.");
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

use std::collections::HashMap;
