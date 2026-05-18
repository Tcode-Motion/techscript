use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectManifest {
    pub project: ProjectSection,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectSection {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default = "default_entry")]
    pub entry: String,
}

fn default_version() -> String { "0.1.0".into() }
fn default_entry() -> String { "main.txs".into() }

pub fn load_manifest(path: &Path) -> Result<ProjectManifest, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| e.to_string())
}

pub fn find_manifest(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        let candidate = dir.join("techscript.toml");
        if candidate.exists() {
            return Some(candidate);
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

pub fn pkg_init(name: &str) -> Result<(), String> {
    let manifest = format!(
        r#"[project]
name = "{name}"
version = "0.1.0"
entry = "main.txs"

[dependencies]
"#
    );
    fs::write("techscript.toml", manifest).map_err(|e| e.to_string())
}

const BUILTIN_MODULES: &[&str] = &["web", "gui", "3d", "anime"];

pub fn pkg_install() -> Result<(), String> {
    let manifest_path = find_manifest(Path::new(".")).ok_or("No techscript.toml found")?;
    let manifest = load_manifest(&manifest_path)?;
    let cache = Path::new(".tech/cache");
    fs::create_dir_all(cache).map_err(|e| e.to_string())?;
    for (dep, ver) in &manifest.dependencies {
        if !BUILTIN_MODULES.contains(&dep.as_str()) {
            return Err(format!(
                "Unknown dependency '{}'. Built-in modules: {}",
                dep,
                BUILTIN_MODULES.join(", ")
            ));
        }
        let marker = cache.join(format!("{}-{}.ok", dep, ver));
        let meta = format!("builtin module {} v{}", dep, ver);
        fs::write(&marker, meta).map_err(|e| e.to_string())?;
        println!("  installed {} = {} (built-in)", dep, ver);
    }
    Ok(())
}

pub fn pkg_list() -> Result<String, String> {
    let manifest_path = find_manifest(Path::new(".")).ok_or("No techscript.toml found")?;
    let manifest = load_manifest(&manifest_path)?;
    let mut out = format!("Project: {} v{}\n", manifest.project.name, manifest.project.version);
    out.push_str(&format!("Entry: {}\n\nDependencies:\n", manifest.project.entry));
    if manifest.dependencies.is_empty() {
        out.push_str("  (none)\n");
    } else {
        for (k, v) in &manifest.dependencies {
            out.push_str(&format!("  {} = {}\n", k, v));
        }
    }
    Ok(out)
}
