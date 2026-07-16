//! # TechScript Compiler Driver — Crash Recovery
//!
//! Installs a custom panic hook to capture Internal Compiler Errors (ICE)
//! and write structured crash reports.

use crate::exit_code::ExitCode;
use std::panic;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Structured crash report details for debugging.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CrashReport {
    pub compiler_version: String,
    pub command: String,
    pub source_file: Option<String>,
    pub build_profile: String,
    pub backtrace: String,
    pub os_info: String,
    pub timestamp: String,
}

impl CrashReport {
    /// Renders a human-readable crash report summary.
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "================================================================================\n",
        );
        out.push_str(
            "                        INTERNAL COMPILER ERROR (ICE)                           \n",
        );
        out.push_str(
            "================================================================================\n",
        );
        out.push_str(&format!("Compiler Version: {}\n", self.compiler_version));
        out.push_str(&format!("CLI Command:      {}\n", self.command));
        out.push_str(&format!("Timestamp:        {}\n", self.timestamp));
        out.push_str(&format!("OS Info:          {}\n", self.os_info));
        if let Some(file) = &self.source_file {
            out.push_str(&format!("Active File:      {}\n", file));
        }
        out.push_str(
            "--------------------------------------------------------------------------------\n",
        );
        out.push_str("Backtrace:\n");
        out.push_str(&self.backtrace);
        out.push_str(
            "\n================================================================================\n",
        );
        out
    }

    /// Writes the crash report to a file in the project build directory.
    pub fn write_to_file(&self, build_dir: &Path) -> std::io::Result<PathBuf> {
        std::fs::create_dir_all(build_dir)?;
        let name = format!(
            "crash-report-{}.txt",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        let path = build_dir.join(name);
        std::fs::write(&path, self.render())?;
        Ok(path)
    }
}

/// Installs the global panic hook for structured ICE reports.
pub fn install_panic_hook(build_dir: PathBuf) {
    panic::set_hook(Box::new(move |info| {
        let backtrace = format!("{:?}", std::backtrace::Backtrace::capture());

        let os = if cfg!(target_os = "windows") {
            "Windows"
        } else if cfg!(target_os = "macos") {
            "macOS"
        } else if cfg!(target_os = "linux") {
            "Linux"
        } else {
            "Unknown OS"
        }
        .to_string();

        let report = CrashReport {
            compiler_version: techscript_common::TECHSCRIPT_VERSION.to_string(),
            command: std::env::args().collect::<Vec<_>>().join(" "),
            source_file: None, // Can be populated dynamically if needed
            build_profile: if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            }
            .to_string(),
            backtrace,
            os_info: os,
            timestamp: format!("{:?}", SystemTime::now()),
        };

        eprintln!("{}", report.render());

        if let Ok(report_path) = report.write_to_file(&build_dir) {
            eprintln!(
                "A crash report has been saved to: {}",
                report_path.display()
            );
        }

        eprintln!(
            "Please report this bug at: https://github.com/Tcode-Motion/TechScript-2.0/issues"
        );

        std::process::exit(ExitCode::InternalError.code());
    }));
}
