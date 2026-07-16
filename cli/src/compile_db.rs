//! # TechScript Compiler Driver — Compilation Database
//!
//! Generates compile_commands.json recording compile options, source files, and outputs.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Record in the compilation database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileCommand {
    pub directory: String,
    pub file: String,
    pub arguments: Vec<String>,
    pub output: String,
}

pub struct CompilationDatabase {
    pub commands: Vec<CompileCommand>,
}

impl Default for CompilationDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl CompilationDatabase {
    /// Creates an empty compilation database.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Records a compilation step.
    pub fn record(&mut self, cmd: CompileCommand) {
        self.commands.push(cmd);
    }

    /// Serializes and writes compile_commands.json to the build folder.
    pub fn write(&self, build_dir: &Path) -> anyhow::Result<PathBuf> {
        std::fs::create_dir_all(build_dir)?;
        let dest = build_dir.join("compile_commands.json");
        let content = serde_json::to_string_pretty(&self.commands)?;
        std::fs::write(&dest, content)?;
        Ok(dest)
    }
}
