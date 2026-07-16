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
        stdlib_modules.insert("std/collections".to_string());
        stdlib_modules.insert("std/strings".to_string());
        stdlib_modules.insert("std/math".to_string());
        stdlib_modules.insert("std/json".to_string());
        stdlib_modules.insert("std/io".to_string());
        stdlib_modules.insert("std/fs".to_string());
        stdlib_modules.insert("std/time".to_string());
        stdlib_modules.insert("std/env".to_string());
        stdlib_modules.insert("std/process".to_string());
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
