//! # `tsc report` System & Build Diagnostic Report Generator
//!
//! Produces an actionable markdown and terminal diagnostic report of
//! compiler versions, host SDKs, connected devices, and build status.

use crate::commands::app::android::resolve_android_home;
use crate::exit_code::ExitCode;
use colored::Colorize;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn execute() -> ExitCode {
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "            TechScript Diagnostic Report                "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );

    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut report = String::new();

    let _ = writeln!(report, "# TechScript 2.0 Diagnostic & Environment Report\n");
    let _ = writeln!(report, "- **Timestamp**: {}", chrono_timestamp());
    let _ = writeln!(
        report,
        "- **TechScript Compiler Version**: {}",
        techscript_common::TECHSCRIPT_VERSION
    );
    let _ = writeln!(
        report,
        "- **Working Directory**: `{}`",
        current_dir.display()
    );

    println!("Collecting environment metadata...");

    // Host OS & CPU
    let _ = writeln!(report, "\n## Host Environment");
    let _ = writeln!(report, "- **OS**: {}", std::env::consts::OS);
    let _ = writeln!(report, "- **Architecture**: {}", std::env::consts::ARCH);

    // Rust
    if let Ok(out) = Command::new("rustc").arg("--version").output() {
        let _ = writeln!(
            report,
            "- **Rustc**: {}",
            String::from_utf8_lossy(&out.stdout).trim()
        );
    }

    // Java
    if let Ok(out) = Command::new("javac").arg("-version").output() {
        let ver = format!(
            "{} {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        let _ = writeln!(report, "- **JDK (javac)**: {}", ver.trim());
    }

    // Gradle
    if let Ok(out) = Command::new("gradle").arg("-v").output() {
        let text = String::from_utf8_lossy(&out.stdout);
        let first = text
            .lines()
            .find(|l| l.contains("Gradle "))
            .unwrap_or("Gradle installed");
        let _ = writeln!(report, "- **Gradle**: {}", first.trim());
    }

    // Android SDK
    let _ = writeln!(report, "\n## Android Toolchain");
    match resolve_android_home() {
        Ok(sdk) => {
            let _ = writeln!(report, "- **ANDROID_HOME**: `{}`", sdk);
        }
        Err(e) => {
            let _ = writeln!(report, "- **ANDROID_HOME**: Missing ({})", e);
        }
    }

    // ADB
    if let Ok(out) = Command::new("adb").arg("version").output() {
        let line = String::from_utf8_lossy(&out.stdout);
        let _ = writeln!(
            report,
            "- **ADB Version**: {}",
            line.lines().next().unwrap_or("").trim()
        );
    }

    // Devices
    if let Ok(out) = Command::new("adb").arg("devices").output() {
        let text = String::from_utf8_lossy(&out.stdout);
        let mut count = 0;
        for line in text.lines() {
            if line.ends_with("device") {
                count += 1;
                let _ = writeln!(
                    report,
                    "  - Device: `{}`",
                    line.split_whitespace().next().unwrap_or("")
                );
            }
        }
        let _ = writeln!(report, "- **Attached Devices**: {}", count);
    }

    // Save report to reports/report.md
    let reports_dir = current_dir.join("reports");
    let _ = fs::create_dir_all(&reports_dir);
    let report_file = reports_dir.join("system_report.md");
    let _ = fs::write(&report_file, &report);

    println!("{}", report);
    println!(
        "{} Report saved to: {}",
        "✓".green().bold(),
        report_file.display().to_string().cyan().bold()
    );

    ExitCode::Success
}

fn chrono_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
}
