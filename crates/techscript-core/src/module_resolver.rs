// ── TechScript Module Resolver ───────────────────────────────────────
// Resolves `use` / `take from` statements to built-in modules or files.

use std::path::{Path, PathBuf};

/// The result of resolving a module name.
#[derive(Debug)]
pub enum ModuleSource {
    /// A built-in module (api, web, gui, math, fs, os, etc.)
    Builtin(String),
    /// A file-based module (resolved to an absolute path)
    File(PathBuf),
}

/// List of all known built-in module names.
const BUILTIN_MODULES: &[&str] = &[
    "math", "fs", "os", "random", "json", "crypto", "date",
    "api", "web", "gui", "three_d", "anime", "debug",
];

/// Resolve a module name to either a builtin or a file path.
///
/// Search order for file-based modules:
/// 1. `<base_dir>/<name>.txs`
/// 2. `<base_dir>/<name>/mod.txs`
/// 3. `$TECHSCRIPT_PATH/lib/<name>.txs`
/// 4. `~/.techscript/modules/<name>.txs`
pub fn resolve(module_name: &str, base_dir: &Path) -> ModuleSource {
    // Check builtins first
    if BUILTIN_MODULES.contains(&module_name) {
        return ModuleSource::Builtin(module_name.to_string());
    }

    // Search for file-based module
    let candidates = vec![
        base_dir.join(format!("{}.txs", module_name)),
        base_dir.join(module_name).join("mod.txs"),
        base_dir.join(".techscript-modules").join(module_name).join(format!("{}.txs", module_name)),
        base_dir.join(".techscript-modules").join(module_name).join("mod.txs"),
        base_dir.join(".techscript-modules").join(module_name).join("src").join("mod.txs"),
        base_dir.join(".techscript-modules").join(module_name).join("src").join(format!("{}.txs", module_name)),
    ];

    // Add TECHSCRIPT_PATH if set
    if let Ok(lib_path) = std::env::var("TECHSCRIPT_PATH") {
        let lib_candidates = vec![
            PathBuf::from(&lib_path).join("lib").join(format!("{}.txs", module_name)),
        ];
        for candidate in lib_candidates {
            if candidate.exists() {
                return ModuleSource::File(candidate);
            }
        }
    }

    // Add home directory modules
    if let Some(home) = home_dir() {
        let home_module = home.join(".techscript").join("modules").join(format!("{}.txs", module_name));
        if home_module.exists() {
            return ModuleSource::File(home_module);
        }
    }

    // Check local candidates
    for candidate in candidates {
        if candidate.exists() {
            return ModuleSource::File(candidate);
        }
    }

    // Fall back to builtin (the VM will handle the error if it's truly unknown)
    ModuleSource::Builtin(module_name.to_string())
}

/// Get the user's home directory.
fn home_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    }
    #[cfg(not(windows))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}

/// Check if a module name refers to a known builtin.
pub fn is_builtin(name: &str) -> bool {
    BUILTIN_MODULES.contains(&name)
}
