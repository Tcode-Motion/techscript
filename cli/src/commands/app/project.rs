//! # Appforge Project Model & Manifest Parser (`appforge.toml`)

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppforgeManifest {
    pub app: AppMeta,
    #[serde(default)]
    pub theme: AppTheme,
    #[serde(default)]
    pub android: AndroidConfig,
    #[serde(default)]
    pub windows: WindowsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMeta {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default = "default_version_code")]
    pub version_code: u32,
    #[serde(default = "default_entry")]
    pub entry: String,
    #[serde(default)]
    pub description: Option<String>,
}

fn default_version_code() -> u32 {
    1
}

fn default_entry() -> String {
    "src/main.txs".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTheme {
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_primary")]
    pub primary: String,
    #[serde(default = "default_bg")]
    pub background: String,
    #[serde(default = "default_text")]
    pub text: String,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self {
            mode: default_mode(),
            primary: default_primary(),
            background: default_bg(),
            text: default_text(),
        }
    }
}

fn default_mode() -> String {
    "dark".to_string()
}
fn default_primary() -> String {
    "#4f46e5".to_string()
}
fn default_bg() -> String {
    "#0f172a".to_string()
}
fn default_text() -> String {
    "#f8fafc".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidConfig {
    #[serde(default = "default_min_sdk")]
    pub min_sdk: u32,
    #[serde(default = "default_target_sdk")]
    pub target_sdk: u32,
    #[serde(default = "default_compile_sdk")]
    pub compile_sdk: u32,
    #[serde(default = "default_package")]
    pub package: String,
    #[serde(default = "default_orientation")]
    pub orientation: String,
    #[serde(default = "default_permissions")]
    pub permissions: Vec<String>,
}

impl Default for AndroidConfig {
    fn default() -> Self {
        Self {
            min_sdk: default_min_sdk(),
            target_sdk: default_target_sdk(),
            compile_sdk: default_compile_sdk(),
            package: default_package(),
            orientation: default_orientation(),
            permissions: default_permissions(),
        }
    }
}

fn default_min_sdk() -> u32 {
    26
}
fn default_target_sdk() -> u32 {
    34
}
fn default_compile_sdk() -> u32 {
    34
}
fn default_package() -> String {
    "com.techscript.app".to_string()
}
fn default_orientation() -> String {
    "portrait".to_string()
}
fn default_permissions() -> Vec<String> {
    vec!["android.permission.INTERNET".to_string()]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsConfig {
    #[serde(default = "default_exe_name")]
    pub executable_name: String,
    #[serde(default = "default_company")]
    pub company: String,
}

impl Default for WindowsConfig {
    fn default() -> Self {
        Self {
            executable_name: default_exe_name(),
            company: default_company(),
        }
    }
}

fn default_exe_name() -> String {
    "app.exe".to_string()
}
fn default_company() -> String {
    "TechScript".to_string()
}

impl AppforgeManifest {
    pub fn load_from_dir(dir: &Path) -> Result<Self, anyhow::Error> {
        let manifest_path = dir.join("appforge.toml");
        if !manifest_path.exists() {
            anyhow::bail!(
                "appforge.toml not found in '{}'. Are you in an Appforge project directory?",
                dir.display()
            );
        }
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: AppforgeManifest = toml::from_str(&content)?;
        Ok(manifest)
    }

    pub fn to_toml_string(&self) -> Result<String, anyhow::Error> {
        Ok(toml::to_string_pretty(self)?)
    }
}
