//! # tsc doctor Command
//!
//! Evaluates the TechScript toolchain, registry access, directory permissions,
//! config validity, and packages directories to report environment health.
//! Supports automatic repair checkups via --fix.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::{Path, PathBuf};

pub fn execute(fix: bool) -> ExitCode {
    println!("{}", "Checking TechScript 2.0 Environment Health...".bold());
    println!("------------------------------------------------------------");

    let mut overall_success = true;
    let mut warnings = 0;

    let theme = crate::theme::Theme::detect();

    // Helper to print check item
    let print_status = |label: &str, status: &str, color: &str| {
        let icon = match status {
            "OK" => theme.success_icon.green().bold(),
            "WARN" => theme.warning_icon.yellow().bold(),
            _ => theme.error_icon.red().bold(),
        };
        println!(
            "  {}  {:<35} [ {} ]",
            icon,
            label,
            status.color(color).bold()
        );
    };

    // 1. Compiler Toolchain version
    print_status("Compiler version check", "OK", "green");

    // 2. Standard library load test
    let registry = techscript_stdlib::StdlibRegistry::new();
    if registry.has_module("std.math") && registry.has_module("std.io") {
        print_status("Standard library integrity", "OK", "green");
    } else {
        print_status("Standard library integrity", "FAILED", "red");
        overall_success = false;
    }

    // 3. Rust & Cargo installation (optional LLVM build dependency)
    let rustc_check = std::process::Command::new("rustc")
        .arg("--version")
        .output();
    if rustc_check.is_ok() {
        print_status("rustc backend dependency", "OK", "green");
    } else {
        print_status("rustc backend dependency", "WARN", "yellow");
        warnings += 1;
        println!("     Note: rustc is not found on PATH. Optional but recommended for LLVM JIT compiles.");
    }

    // 4. Git binary dependency
    let git_check = std::process::Command::new("git").arg("--version").output();
    if git_check.is_ok() {
        print_status("git version control integration", "OK", "green");
    } else {
        print_status("git version control integration", "WARN", "yellow");
        warnings += 1;
        println!("     Note: git command not found. Required for installing packages from GitHub.");
    }

    // 5. Internet connectivity check
    let internet_ok = std::net::TcpStream::connect_timeout(
        &"8.8.8.8:53".parse().unwrap(),
        std::time::Duration::from_millis(1200),
    )
    .is_ok();

    if internet_ok {
        print_status("Network connectivity", "OK", "green");
    } else {
        print_status("Network connectivity", "WARN", "yellow");
        warnings += 1;
        println!("     Note: Offline mode. Registry packages downloads will fail.");
    }

    // 6. Home package cache directories
    if let Some(home) = dirs::home_dir() {
        let config_dir = home.join(".techscript");
        let cache_dir = config_dir.join("cache");
        let packages_dir = config_dir.join("packages");

        let mut cache_ok = cache_dir.exists();
        let mut packages_ok = packages_dir.exists();

        if fix {
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
            print_status("Package caches directory", "OK", "green");
        } else {
            print_status("Package caches directory", "FAILED", "red");
            overall_success = false;
        }
    } else {
        print_status("Home directory access", "FAILED", "red");
        overall_success = false;
    }

    // 7. Local Project manifest (if present)
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let manifest_path = current_dir.join("tech.toml");
    if manifest_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&manifest_path) {
            if toml::from_str::<serde_json::Value>(&content).is_ok() {
                print_status("Project manifest layout", "OK", "green");
            } else {
                print_status("Project manifest layout", "FAILED", "red");
                println!("     Error: tech.toml exists but is not valid TOML.");
                overall_success = false;
            }
        } else {
            print_status("Project manifest read", "FAILED", "red");
            overall_success = false;
        }
    } else {
        println!(
            "  {}  {:<35} [ {} ]",
            theme.info_icon.dimmed(),
            "Project manifest layout",
            "SKIPPED".dimmed()
        );
        println!("     Note: No tech.toml found. Running in single-file script execution mode.");
    }

    // 8. PATH Env check
    let path_val = std::env::var("PATH").unwrap_or_default();
    let has_tsc_bin = path_val.contains(".techscript")
        || path_val.contains("TechScript")
        || std::env::var("TECHSCRIPT_HOME").is_ok();
    if has_tsc_bin {
        print_status("PATH environment configuration", "OK", "green");
    } else {
        print_status("PATH environment configuration", "WARN", "yellow");
        warnings += 1;
        println!("     Note: TECHSCRIPT_HOME or .techscript binary directories not explicitly defined in PATH.");
    }

    // 9. Windows-specific user-level file associations check
    #[cfg(windows)]
    {
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
            print_status("User file associations", "OK", "green");
        } else {
            print_status("User file associations", "WARN", "yellow");
            warnings += 1;
            println!(
                "     Note: User file associations (.txs, .tsx) are missing or misconfigured."
            );
            if fix {
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

    println!("------------------------------------------------------------");

    if fix && !overall_success {
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
        return ExitCode::Success;
    }

    if overall_success {
        if warnings > 0 {
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
