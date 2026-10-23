//! # `tsc report` System & Build Diagnostic Report Generator
//!
//! Produces an actionable markdown and terminal diagnostic report of
//! compiler versions, host SDKs, connected devices, and build status.

use crate::commands::app::android::resolve_android_home;
use crate::exit_code::ExitCode;
use colored::Colorize;
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

    report.push_str("# TechScript 2.0 Diagnostic & Environment Report\n\n");
    report.push_str(&format!("- **Timestamp**: {}\n", chrono_timestamp()));
    report.push_str(&format!(
        "- **TechScript Compiler Version**: {}\n",
        techscript_common::TECHSCRIPT_VERSION
    ));
    report.push_str(&format!(
        "- **Working Directory**: `{}`\n",
        current_dir.display()
    ));

    println!("Collecting environment metadata...");

    // Host OS & CPU
    report.push_str("\n## Host Environment\n");
    report.push_str(&format!("- **OS**: {}\n", std::env::consts::OS));
    report.push_str(&format!("- **Architecture**: {}\n", std::env::consts::ARCH));

    // Rust
    if let Ok(out) = Command::new("rustc").arg("--version").output() {
        report.push_str(&format!(
            "- **Rustc**: {}\n",
            String::from_utf8_lossy(&out.stdout).trim()
        ));
    }

    // Java
    if let Ok(out) = Command::new("javac").arg("-version").output() {
        let ver = format!(
            "{} {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        report.push_str(&format!("- **JDK (javac)**: {}\n", ver.trim()));
    }

    // Gradle
    if let Ok(out) = Command::new("gradle").arg("-v").output() {
        let text = String::from_utf8_lossy(&out.stdout);
        let first = text
            .lines()
            .find(|l| l.contains("Gradle "))
            .unwrap_or("Gradle installed");
        report.push_str(&format!("- **Gradle**: {}\n", first.trim()));
    }

    // Android SDK
    report.push_str("\n## Android Toolchain\n");
    match resolve_android_home() {
        Ok(sdk) => report.push_str(&format!("- **ANDROID_HOME**: `{}`\n", sdk)),
        Err(e) => report.push_str(&format!("- **ANDROID_HOME**: Missing ({})\n", e)),
    }

    // ADB
    if let Ok(out) = Command::new("adb").arg("version").output() {
        let line = String::from_utf8_lossy(&out.stdout);
        report.push_str(&format!(
            "- **ADB Version**: {}\n",
            line.lines().next().unwrap_or("").trim()
        ));
    }

    // Devices
    if let Ok(out) = Command::new("adb").arg("devices").output() {
        let text = String::from_utf8_lossy(&out.stdout);
        let mut count = 0;
        for line in text.lines() {
            if line.ends_with("device") {
                count += 1;
                report.push_str(&format!(
                    "  - Device: `{}`\n",
                    line.split_whitespace().next().unwrap_or("")
                ));
            }
        }
        report.push_str(&format!("- **Attached Devices**: {}\n", count));
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
