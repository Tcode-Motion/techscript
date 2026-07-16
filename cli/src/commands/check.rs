//! # tsc check Command
//!
//! Performs syntactic and semantic verification without compiling or executing.

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

pub fn execute(file_path: Option<&str>, watch: bool) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let check_closure = || -> Result<bool, anyhow::Error> {
        let path = if let Some(f) = file_path {
            PathBuf::from(f)
        } else {
            // Find entry point in tech.toml
            let manifest_path = current_dir.join("tech.toml");
            if manifest_path.exists() {
                let content = std::fs::read_to_string(&manifest_path)?;
                let manifest: techscript_package_manager::Manifest = toml::from_str(&content)?;
                current_dir.join(&manifest.package.entry)
            } else {
                return Err(anyhow::anyhow!(
                    "No input file specified and no tech.toml found in current directory."
                ));
            }
        };

        if !path.exists() {
            return Err(anyhow::anyhow!("File not found: {:?}", path));
        }

        let cli_cfg = CliConfig::default();
        let config_mgr = ConfigManager::load(Some(&current_dir), None, &cli_cfg)?;
        let cache = BuildCache::load(&current_dir.join("build").join(".tsc-cache"))?;
        let artifacts = ArtifactManager::new(&current_dir);
        let mut events = EventBus::new();
        let logger = Logger::new(config_mgr.effective.log_level, LogFormat::Human);
        let profiler = TimingProfiler::new(false);

        let mut pipeline = CompilationPipeline::new(
            techscript_common::SourceManager::new(),
            config_mgr.effective.clone(),
            cache,
            artifacts,
            events,
            logger,
            profiler,
        );

        let diags = pipeline.check_unit(&path)?;

        let renderer =
            crate::diagnostics::DiagnosticRenderer::auto_detect(&pipeline.source_manager);
        let mut has_errors = false;
        for diag in &diags {
            renderer.emit(diag);
            if diag.severity == crate::diagnostics::Severity::Error {
                has_errors = true;
            }
        }

        if has_errors {
            println!("Checks failed with semantic or syntax errors.");
            Ok(false)
        } else {
            println!("Checks passed with zero errors.");
            Ok(true)
        }
    };

    if watch {
        let watcher = FileWatcher::new(&current_dir, 500);
        let _ = watcher.watch(|changed| {
            println!(
                "\n[tsc watch] Change detected in: {:?}. Re-checking...",
                changed
            );
            let _ = check_closure();
        });
        ExitCode::Success
    } else {
        match check_closure() {
            Ok(true) => ExitCode::Success,
            Ok(false) => ExitCode::CompilationError,
            Err(e) => {
                eprintln!("Check failed: {}", e);
                ExitCode::CompilationError
            }
        }
    }
}
