use std::path::{Path, PathBuf};
use techscript_common::resolve_source_module;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleSource {
    Stdlib(String),    // e.g. "std.math"
    UserPath(PathBuf), // e.g. "user/library.txs"
}

pub trait ModuleResolver {
    fn resolve(&self, path: &[String]) -> Result<ModuleSource, String>;
}

/// A default resolver implementation that matches the standard library modules
/// and resolves user imports to .txs (default) and specialized TechScript files.
pub struct DefaultModuleResolver {
    stdlib_modules: std::collections::HashSet<String>,
    base_dir: PathBuf,
}

impl Default for DefaultModuleResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultModuleResolver {
    pub fn new() -> Self {
        Self::with_base_dir(PathBuf::from("."))
    }

    pub fn with_base_dir<P: AsRef<Path>>(base_dir: P) -> Self {
        let mut stdlib_modules = std::collections::HashSet::new();
        let modules = [
            "std/io",
            "std/fs",
            "std/net",
            "std/http",
            "std/json",
            "std/xml",
            "std/csv",
            "std/yaml",
            "std/time",
            "std/datetime",
            "std/env",
            "std/process",
            "std/crypto",
            "std/hash",
            "std/random",
            "std/math",
            "std/collections",
            "std/string",
            "std/strings",
            "std/regex",
            "std/path",
            "std/thread",
            "std/sync",
            "std/async",
            "std/future",
            "std/channel",
            "std/testing",
            "std/logging",
            "std/compress",
            "std/encoding",
            "std/base64",
            "std/hex",
            "std/uuid",
            "std/url",
            "std/system",
            "std/web",
        ];
        for m in &modules {
            stdlib_modules.insert(m.to_string());
        }
        Self {
            stdlib_modules,
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }
}

impl ModuleResolver for DefaultModuleResolver {
    fn resolve(&self, path: &[String]) -> Result<ModuleSource, String> {
        if path.is_empty() {
            return Err("Empty module import path".to_string());
        }
        let joined = path.join("/");
        if self.stdlib_modules.contains(&joined) {
            Ok(ModuleSource::Stdlib(path.join(".")))
        } else {
            // Attempt deterministic resolution via TechScript source registry
            match resolve_source_module(&self.base_dir, &joined) {
                Ok(path) => Ok(ModuleSource::UserPath(path)),
                Err(techscript_common::SourceResolutionError::AmbiguousImport { module, candidates }) => {
                    Err(format!(
                        "Ambiguous import for '{}': found multiple matching files with different extensions: {:?}",
                        module, candidates
                    ))
                }
                Err(techscript_common::SourceResolutionError::PathTraversalDetected { path }) => {
                    Err(format!("Security error: path traversal forbidden: '{}'", path))
                }
                Err(_) => {
                    // Default to canonical .txs path when file does not exist on disk yet
                    let file_path = self.base_dir.join(format!("{}.txs", joined));
                    Ok(ModuleSource::UserPath(file_path))
                }
            }
        }
    }
}
