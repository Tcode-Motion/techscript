//! # TechScript Compiler Driver — Layered Config Manager
//!
//! Merges configuration layers deterministically:
//! 1. Built-in Defaults
//! 2. Global user config (~/.techscript/config.toml)
//! 3. Workspace config (workspace tech.toml [config])
//! 4. Project config (project tech.toml [config])
//! 5. Environment variables (TSC_*)
//! 6. CLI overrides

use crate::diagnostics::DiagnosticOutput;
use crate::logging::LogLevel;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use techscript_optimizer::OptimizationLevel;
use techscript_runtime::context::Capability;

/// Cli options that override other configs.
#[derive(Debug, Clone, Default)]
pub struct CliConfig {
    pub opt_level: Option<OptimizationLevel>,
    pub debug_symbols: Option<bool>,
    pub source_maps: Option<bool>,
    pub strict_mode: Option<bool>,
    pub log_level: Option<LogLevel>,
    pub output_format: Option<DiagnosticOutput>,
    pub parallel_jobs: Option<usize>,
}

/// The final merged configuration used during pipeline execution.
#[derive(Debug, Clone)]
pub struct EffectiveConfig {
    pub optimization_level: OptimizationLevel,
    pub debug_symbols: bool,
    pub source_maps: bool,
    pub strict_mode: bool,
    pub max_recursion: usize,
    pub log_level: LogLevel,
    pub output_format: DiagnosticOutput,
    pub parallel_jobs: usize,
    pub capabilities: HashSet<Capability>,
}

impl Default for EffectiveConfig {
    fn default() -> Self {
        let mut caps = HashSet::new();
        caps.insert(Capability::FileSystem);
        caps.insert(Capability::Environment);
        caps.insert(Capability::Process);
        caps.insert(Capability::Network);
        Self {
            optimization_level: OptimizationLevel::O2,
            debug_symbols: false,
            source_maps: false,
            strict_mode: false,
            max_recursion: 1000,
            log_level: LogLevel::Normal,
            output_format: DiagnosticOutput::Plain,
            parallel_jobs: num_cpus(),
            capabilities: caps,
        }
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

/// Serializable config representation for TOML files.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TomlConfig {
    pub optimization_level: Option<String>,
    pub debug_symbols: Option<bool>,
    pub source_maps: Option<bool>,
    pub strict_mode: Option<bool>,
    pub max_recursion: Option<usize>,
    pub log_level: Option<String>,
    pub output_format: Option<String>,
    pub parallel_jobs: Option<usize>,
    pub capabilities: Option<Vec<String>>,
}

pub struct ConfigManager {
    pub effective: EffectiveConfig,
}

impl ConfigManager {
    /// Loads and merges all config layers.
    pub fn load(
        project_root: Option<&Path>,
        workspace_root: Option<&Path>,
        cli: &CliConfig,
    ) -> anyhow::Result<Self> {
        let mut base = EffectiveConfig::default();

        // 2. Global user config (~/.techscript/config.toml)
        if let Some(home) = dirs::home_dir() {
            let global_path = home.join(".techscript").join("config.toml");
            if let Ok(content) = std::fs::read_to_string(global_path) {
                if let Ok(toml_cfg) = toml::from_str::<TomlConfig>(&content) {
                    apply_toml(&mut base, &toml_cfg);
                }
            }
        }

        // 3. Workspace config
        if let Some(ws) = workspace_root {
            let ws_manifest = ws.join("tech.toml");
            if let Ok(content) = std::fs::read_to_string(ws_manifest) {
                if let Ok(manifest_toml) = toml::from_str::<serde_json::Value>(&content) {
                    if let Some(cfg_val) = manifest_toml.get("config") {
                        if let Ok(toml_cfg) = serde_json::from_value::<TomlConfig>(cfg_val.clone())
                        {
                            apply_toml(&mut base, &toml_cfg);
                        }
                    }
                }
            }
        }

        // 4. Project config
        if let Some(proj) = project_root {
            let proj_manifest = proj.join("tech.toml");
            if let Ok(content) = std::fs::read_to_string(proj_manifest) {
                if let Ok(manifest_toml) = toml::from_str::<serde_json::Value>(&content) {
                    if let Some(cfg_val) = manifest_toml.get("config") {
                        if let Ok(toml_cfg) = serde_json::from_value::<TomlConfig>(cfg_val.clone())
                        {
                            apply_toml(&mut base, &toml_cfg);
                        }
                    }
                }
            }
        }

        // 5. Environment variables (TSC_*)
        apply_env(&mut base);

        // 6. CLI overrides
        apply_cli(&mut base, cli);

        Ok(Self { effective: base })
    }
}

fn apply_toml(base: &mut EffectiveConfig, toml: &TomlConfig) {
    if let Some(opt) = &toml.optimization_level {
        base.optimization_level = match opt.as_str() {
            "O0" => OptimizationLevel::O0,
            "O1" => OptimizationLevel::O1,
            "O3" => OptimizationLevel::O3,
            "Os" => OptimizationLevel::Os,
            _ => OptimizationLevel::O2,
        };
    }
    if let Some(dbg) = toml.debug_symbols {
        base.debug_symbols = dbg;
    }
    if let Some(sm) = toml.source_maps {
        base.source_maps = sm;
    }
    if let Some(strict) = toml.strict_mode {
        base.strict_mode = strict;
    }
    if let Some(rec) = toml.max_recursion {
        base.max_recursion = rec;
    }
    if let Some(log) = &toml.log_level {
        base.log_level = match log.as_str() {
            "quiet" => LogLevel::Quiet,
            "verbose" => LogLevel::Verbose,
            "trace" => LogLevel::Trace,
            _ => LogLevel::Normal,
        };
    }
    if let Some(fmt) = &toml.output_format {
        base.output_format = match fmt.as_str() {
            "colored" => DiagnosticOutput::Colored,
            "json" => DiagnosticOutput::Json,
            _ => DiagnosticOutput::Plain,
        };
    }
    if let Some(jobs) = toml.parallel_jobs {
        base.parallel_jobs = jobs;
    }
    if let Some(caps) = &toml.capabilities {
        base.capabilities.clear();
        for cap in caps {
            match cap.as_str() {
                "FileSystem" => {
                    base.capabilities.insert(Capability::FileSystem);
                }
                "Environment" => {
                    base.capabilities.insert(Capability::Environment);
                }
                "Process" => {
                    base.capabilities.insert(Capability::Process);
                }
                "Network" => {
                    base.capabilities.insert(Capability::Network);
                }
                _ => {}
            }
        }
    }
}

fn apply_env(base: &mut EffectiveConfig) {
    if let Ok(opt) = std::env::var("TSC_OPT_LEVEL") {
        base.optimization_level = match opt.as_str() {
            "O0" => OptimizationLevel::O0,
            "O1" => OptimizationLevel::O1,
            "O3" => OptimizationLevel::O3,
            "Os" => OptimizationLevel::Os,
            _ => OptimizationLevel::O2,
        };
    }
    if let Ok(dbg) = std::env::var("TSC_DEBUG_SYMBOLS") {
        base.debug_symbols = dbg == "true" || dbg == "1";
    }
    if let Ok(sm) = std::env::var("TSC_SOURCE_MAPS") {
        base.source_maps = sm == "true" || sm == "1";
    }
    if let Ok(strict) = std::env::var("TSC_STRICT_MODE") {
        base.strict_mode = strict == "true" || strict == "1";
    }
    if let Ok(rec) = std::env::var("TSC_MAX_RECURSION") {
        if let Ok(parsed) = rec.parse::<usize>() {
            base.max_recursion = parsed;
        }
    }
}

fn apply_cli(base: &mut EffectiveConfig, cli: &CliConfig) {
    if let Some(opt) = cli.opt_level {
        base.optimization_level = opt;
    }
    if let Some(dbg) = cli.debug_symbols {
        base.debug_symbols = dbg;
    }
    if let Some(sm) = cli.source_maps {
        base.source_maps = sm;
    }
    if let Some(strict) = cli.strict_mode {
        base.strict_mode = strict;
    }
    if let Some(log) = cli.log_level {
        base.log_level = log;
    }
    if let Some(fmt) = cli.output_format {
        base.output_format = fmt;
    }
    if let Some(jobs) = cli.parallel_jobs {
        base.parallel_jobs = jobs;
    }
}

mod dirs {
    use std::path::PathBuf;
    pub fn home_dir() -> Option<PathBuf> {
        #[allow(deprecated)]
        std::env::home_dir()
    }
}
