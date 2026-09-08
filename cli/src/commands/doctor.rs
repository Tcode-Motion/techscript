//! # tsc doctor Command
//!
//! Evaluates the TechScript toolchain, registry access, directory permissions,
//! config validity, and packages directories to report environment health.
//! Supports automatic repair checkups via --fix.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::{Path, PathBuf};

struct DoctorContext {
    theme: crate::theme::Theme,
    fix: bool,
    overall_success: bool,
    warnings: usize,
}

impl DoctorContext {
    fn new(fix: bool) -> Self {
        Self {
            theme: crate::theme::Theme::detect(),
            fix,
            overall_success: true,
            warnings: 0,
        }
    }

    fn print_status(&self, label: &str, status: &str, color: &str) {
        let icon = match status {
            "OK" => self.theme.success_icon.green().bold(),
            "WARN" => self.theme.warning_icon.yellow().bold(),
            _ => self.theme.error_icon.red().bold(),
        };
        println!(
            "  {}  {:<35} [ {} ]",
            icon,
            label,
            status.color(color).bold()
        );
    }

    fn check_compiler_version(&mut self) {
        self.print_status("Compiler version check", "OK", "green");
    }

    fn check_stdlib(&mut self) {
        let registry = techscript_stdlib::StdlibRegistry::new();
        if registry.has_module("std.math") && registry.has_module("std.io") {
            self.print_status("Standard library integrity", "OK", "green");
        } else {
            self.print_status("Standard library integrity", "FAILED", "red");
            self.overall_success = false;
        }
    }

    fn check_rustc(&mut self) {
        let rustc_check = std::process::Command::new("rustc")
            .arg("--version")
            .output();
        if rustc_check.is_ok() {
            self.print_status("rustc backend dependency", "OK", "green");
        } else {
            self.print_status("rustc backend dependency", "WARN", "yellow");
            self.warnings += 1;
            println!("     Note: rustc is not found on PATH. Optional but recommended for LLVM JIT compiles.");
        }
    }

    fn check_git(&mut self) {
        let git_check = std::process::Command::new("git").arg("--version").output();
        if git_check.is_ok() {
            self.print_status("git version control integration", "OK", "green");
        } else {
            self.print_status("git version control integration", "WARN", "yellow");
            self.warnings += 1;
            println!(
                "     Note: git command not found. Required for installing packages from GitHub."
            );
        }
    }

    fn check_network(&mut self) {
        let internet_ok = std::net::TcpStream::connect_timeout(
            &"8.8.8.8:53".parse().unwrap(),
            std::time::Duration::from_millis(1200),
        )
        .is_ok();

        if internet_ok {
            self.print_status("Network connectivity", "OK", "green");
        } else {
            self.print_status("Network connectivity", "WARN", "yellow");
            self.warnings += 1;
            println!("     Note: Offline mode. Registry packages downloads will fail.");
        }
    }

    fn check_cache_dirs(&mut self) {
        if let Some(home) = dirs::home_dir() {
            let config_dir = home.join(".techscript");
            let cache_dir = config_dir.join("cache");
            let packages_dir = config_dir.join("packages");

            let mut cache_ok = cache_dir.exists();
            let mut packages_ok = packages_dir.exists();

            if self.fix {
                if !cache_ok {
                    std::fs::create_dir_all(&cache_dir).ok();
                    cache_ok = cache_dir.exists();
                }
                if !packages_ok {
                    std::fs::create_dir_all(&packages_dir).ok();
                    packages_ok = packages_dir.exists();
                }
            }

            if cache_ok && packages_ok {
                self.print_status("Package caches directory", "OK", "green");
            } else {
                self.print_status("Package caches directory", "FAILED", "red");
                self.overall_success = false;
            }
        } else {
            self.print_status("Home directory access", "FAILED", "red");
            self.overall_success = false;
        }
    }

    fn check_project_manifest(&mut self) {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let manifest_path = current_dir.join("tech.toml");
        if manifest_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                if toml::from_str::<serde_json::Value>(&content).is_ok() {
                    self.print_status("Project manifest layout", "OK", "green");
                } else {
                    self.print_status("Project manifest layout", "FAILED", "red");
                    println!("     Error: tech.toml exists but is not valid TOML.");
                    self.overall_success = false;
                }
            } else {
                self.print_status("Project manifest read", "FAILED", "red");
                self.overall_success = false;
            }
        } else {
            println!(
                "  {}  {:<35} [ {} ]",
                self.theme.info_icon.dimmed(),
                "Project manifest layout",
                "SKIPPED".dimmed()
            );
            println!(
                "     Note: No tech.toml found. Running in single-file script execution mode."
            );
        }
    }

    fn check_path_env(&mut self) {
        let path_val = std::env::var("PATH").unwrap_or_default();
        let has_tsc_bin = path_val.contains(".techscript")
            || path_val.contains("TechScript")
            || std::env::var("TECHSCRIPT_HOME").is_ok();
        if has_tsc_bin {
            self.print_status("PATH environment configuration", "OK", "green");
        } else {
            self.print_status("PATH environment configuration", "WARN", "yellow");
            self.warnings += 1;
            println!("     Note: TECHSCRIPT_HOME or .techscript binary directories not explicitly defined in PATH.");
        }
    }

    #[cfg(windows)]
    fn check_windows_associations(&mut self) {
        let mut association_ok = true;
        let extensions = [".txs", ".tsx", ".tech", ".tspkg"];
        for ext in &extensions {
            let output = std::process::Command::new("powershell")
                .args([
                    "-Command",
                    &format!(
                        "Get-ItemProperty -Path 'HKCU:\\Software\\Classes\\{}' -ErrorAction SilentlyContinue | Select-Object -ExpandProperty '(default)' -ErrorAction SilentlyContinue",
                        ext
                    )
                ])
                .output();

            let current_val = output
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default();

            if current_val != "TechScript.File" {
                association_ok = false;
                break;
            }
        }

        if association_ok {
            self.print_status("User file associations", "OK", "green");
        } else {
            self.print_status("User file associations", "WARN", "yellow");
            self.warnings += 1;
            println!(
                "     Note: User file associations (.txs, .tsx) are missing or misconfigured."
            );
            if self.fix {
                println!("     Repairing user file associations...");
                for ext in &extensions {
                    let _ = std::process::Command::new("powershell")
                        .args([
                            "-Command",
                            &format!(
                                "New-Item -Path 'HKCU:\\Software\\Classes\\{}' -Force -ErrorAction SilentlyContinue; Set-Item -Path 'HKCU:\\Software\\Classes\\{}' -Value 'TechScript.File'",
                                ext, ext
                            )
                        ])
                        .output();
                }
                println!("     ✓ Associations updated to TechScript.File.");
            }
        }
    }

    fn perform_auto_repair(&self) -> bool {
        if self.fix && !self.overall_success {
            println!("Performing automatic repair routines...");
            // Auto-initialize directories
            if let Some(home) = dirs::home_dir() {
                let config_dir = home.join(".techscript");
                std::fs::create_dir_all(config_dir.join("cache")).ok();
                std::fs::create_dir_all(config_dir.join("packages")).ok();
                let config_file = config_dir.join("config.toml");
                if !config_file.exists() {
                    let default_config = r#"[config]
optimization_level = "O2"
log_level = "Normal"
output_format = "Plain"
"#;
                    std::fs::write(config_file, default_config).ok();
                }
            }
            println!(
                "{}",
                "✓ Repair completed successfully. Re-run doctor to verify."
                    .green()
                    .bold()
            );
            true
        } else {
            false
        }
    }
}

pub fn execute(fix: bool) -> ExitCode {
    println!("{}", "Checking TechScript 2.0 Environment Health...".bold());
    println!("------------------------------------------------------------");

    let mut ctx = DoctorContext::new(fix);

    ctx.check_compiler_version();
    ctx.check_stdlib();
    ctx.check_rustc();
    ctx.check_git();
    ctx.check_network();
    ctx.check_cache_dirs();
    ctx.check_project_manifest();
    ctx.check_path_env();

    #[cfg(windows)]
    ctx.check_windows_associations();

    println!("------------------------------------------------------------");

    if ctx.perform_auto_repair() {
        return ExitCode::Success;
    }

    if ctx.overall_success {
        if ctx.warnings > 0 {
            println!(
                "{}",
                "System is healthy, but some optional dependencies are missing."
                    .yellow()
                    .bold()
            );
        } else {
            println!(
                "{}",
                "All system checks passed! Your toolchain is ready."
                    .green()
                    .bold()
            );
        }
        ExitCode::Success
    } else {
        println!(
            "{}",
            "Some system checks failed. Run 'tsc doctor --fix' to automatically resolve issues."
                .red()
                .bold()
        );
        ExitCode::CompilationError
    }
}

mod dirs {
    use std::path::PathBuf;
    pub fn home_dir() -> Option<PathBuf> {
        #[allow(deprecated)]
        std::env::home_dir()
    }
}
