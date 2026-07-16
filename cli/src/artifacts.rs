//! # TechScript Compiler Driver — Artifact Manager
//!
//! Manages output files, directory layout, build/manifest.json, and bytecode/debug files.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Output metadata for build/manifest.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildOutput {
    pub path: String,
    pub category: String,
    pub size_bytes: u64,
}

/// The build manifest serialized to build/manifest.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManifest {
    pub compiler_version: String,
    pub optimization_level: String,
    pub source_hash: String,
    pub build_timestamp: String,
    pub outputs: Vec<BuildOutput>,
    pub dependencies: Vec<String>,
    pub bytecode_version: String,
    pub target_backend: String,
    pub build_profile: String,
    pub total_duration_ms: u64,
}

pub enum ArtifactCategory {
    Bytecode,
    DebugSymbols,
    SourceMap,
    Documentation,
    Cache,
    Temp,
}

pub struct ArtifactManager {
    pub build_dir: PathBuf,
}

impl ArtifactManager {
    /// Creates a new ArtifactManager for the project root.
    pub fn new(project_root: &Path) -> Self {
        Self {
            build_dir: project_root.join("build"),
        }
    }

    /// Prepares the output directory layout.
    pub fn prepare(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.build_dir)?;
        std::fs::create_dir_all(self.build_dir.join("bytecode"))?;
        std::fs::create_dir_all(self.build_dir.join("debug"))?;
        std::fs::create_dir_all(self.build_dir.join("sourcemaps"))?;
        std::fs::create_dir_all(self.build_dir.join("docs"))?;
        std::fs::create_dir_all(self.build_dir.join(".tsc-cache"))?;
        std::fs::create_dir_all(self.build_dir.join("temp"))?;
        Ok(())
    }

    /// Writes compiled bytecode to build/bytecode/.
    pub fn write_bytecode(&self, name: &str, data: &[u8]) -> anyhow::Result<PathBuf> {
        let dest = self
            .build_dir
            .join("bytecode")
            .join(format!("{}.tsb", name));
        std::fs::write(&dest, data)?;
        Ok(dest)
    }

    /// Writes debug symbols to build/debug/.
    pub fn write_debug_symbols(
        &self,
        name: &str,
        symbols: &techscript_bytecode::DebugSymbols,
    ) -> anyhow::Result<PathBuf> {
        let dest = self
            .build_dir
            .join("debug")
            .join(format!("{}.tsb.dbg", name));
        let data = serde_json::to_string_pretty(symbols)?;
        std::fs::write(&dest, data)?;
        Ok(dest)
    }

    /// Writes source maps to build/sourcemaps/.
    pub fn write_source_map(
        &self,
        name: &str,
        map: &techscript_bytecode::SourceMap,
    ) -> anyhow::Result<PathBuf> {
        let dest = self
            .build_dir
            .join("sourcemaps")
            .join(format!("{}.tsb.map", name));
        let data = serde_json::to_string_pretty(map)?;
        std::fs::write(&dest, data)?;
        Ok(dest)
    }

    /// Writes generated documentation.
    pub fn write_docs(&self, name: &str, content: &str) -> anyhow::Result<PathBuf> {
        let dest = self.build_dir.join("docs").join(format!("{}.html", name));
        std::fs::write(&dest, content)?;
        Ok(dest)
    }

    /// Writes a temporary file for intermediate stages.
    pub fn write_temp(&self, name: &str, data: &[u8]) -> anyhow::Result<PathBuf> {
        let dest = self.build_dir.join("temp").join(name);
        std::fs::write(&dest, data)?;
        Ok(dest)
    }

    /// Emits the build manifest file.
    pub fn write_build_manifest(&self, manifest: &BuildManifest) -> anyhow::Result<PathBuf> {
        let dest = self.build_dir.join("manifest.json");
        let data = serde_json::to_string_pretty(manifest)?;
        std::fs::write(&dest, data)?;
        Ok(dest)
    }

    /// Removes all generated artifacts (tsc clean).
    pub fn clean(&self) -> anyhow::Result<()> {
        if self.build_dir.exists() {
            std::fs::remove_dir_all(&self.build_dir)?;
        }
        Ok(())
    }

    /// Gets the path to a named artifact category file.
    pub fn artifact_path(&self, category: ArtifactCategory, name: &str) -> PathBuf {
        match category {
            ArtifactCategory::Bytecode => self
                .build_dir
                .join("bytecode")
                .join(format!("{}.tsb", name)),
            ArtifactCategory::DebugSymbols => self
                .build_dir
                .join("debug")
                .join(format!("{}.tsb.dbg", name)),
            ArtifactCategory::SourceMap => self
                .build_dir
                .join("sourcemaps")
                .join(format!("{}.tsb.map", name)),
            ArtifactCategory::Documentation => {
                self.build_dir.join("docs").join(format!("{}.html", name))
            }
            ArtifactCategory::Cache => self.build_dir.join(".tsc-cache").join(name),
            ArtifactCategory::Temp => self.build_dir.join("temp").join(name),
        }
    }
}
