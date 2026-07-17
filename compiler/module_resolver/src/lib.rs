use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleSource {
    Stdlib(String),    // e.g. "std.math"
    UserPath(PathBuf), // e.g. "user/library.ts"
}

pub trait ModuleResolver {
    fn resolve(&self, path: &[String]) -> Result<ModuleSource, String>;
}

/// A default resolver implementation that matches the standard library modules.
pub struct DefaultModuleResolver {
    stdlib_modules: std::collections::HashSet<String>,
}

impl Default for DefaultModuleResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultModuleResolver {
    pub fn new() -> Self {
        let mut stdlib_modules = std::collections::HashSet::new();
        let modules = [
            "std/io", "std/fs", "std/net", "std/http", "std/json", "std/xml", "std/csv", "std/yaml",
            "std/time", "std/datetime", "std/env", "std/process", "std/crypto", "std/hash", "std/random",
            "std/math", "std/collections", "std/string", "std/strings", "std/regex", "std/path", "std/thread", "std/sync",
            "std/async", "std/future", "std/channel", "std/testing", "std/logging", "std/compress",
            "std/encoding", "std/base64", "std/hex", "std/uuid", "std/url", "std/system"
        ];
        for m in &modules {
            stdlib_modules.insert(m.to_string());
        }
        Self { stdlib_modules }
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
            // Assume it's a relative/absolute user file import path
            let file_path = PathBuf::from(format!("{}.ts", joined));
            Ok(ModuleSource::UserPath(file_path))
        }
    }
}
