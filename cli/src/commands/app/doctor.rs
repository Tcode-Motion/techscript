//! # `tsc app doctor` Toolchain Health Check
//!
//! Validates Android SDK, JDK, Gradle, ADB, devices, and desktop build toolchains.

use crate::commands::app::android::resolve_android_home;
use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn execute(fix: bool) -> ExitCode {
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "           Appforge Toolchain Doctor                    "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "========================================================"
            .cyan()
            .bold()
    );
    println!("Evaluating mobile, desktop, and cross-platform build environments...\n");

    let mut has_warnings = false;
    let mut has_errors = false;

    // 1. JDK
    print!("  Java Development Kit (JDK)       ... ");
    let java_check = Command::new("javac").arg("-version").output();
    match java_check {
        Ok(out) => {
            let ver = format!(
                "{} {}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            println!("{} ({})", "[ OK ]".green().bold(), ver.trim());
        }
        Err(_) => {
            println!("{}", "[ FAILED ]".red().bold());
            eprintln!(
                "     JDK compiler (javac) not found. Android builds require JDK 17 or JDK 21."
            );
            has_errors = true;
        }
    }

    // 2. Android SDK
    print!("  Android SDK Location             ... ");
    match resolve_android_home() {
        Ok(sdk_dir) => {
            println!("{} ({})", "[ OK ]".green().bold(), sdk_dir);

            // Check platforms
            let p_dir = Path::new(&sdk_dir).join("platforms");
            if p_dir.exists() {
                let mut apis = Vec::new();
                if let Ok(entries) = std::fs::read_dir(&p_dir) {
                    for e in entries.flatten() {
                        if let Some(name) = e.file_name().to_str() {
                            apis.push(name.to_string());
                        }
                    }
                }
                println!("    Platforms Installed:           {:?}", apis);
            }

            // Check build-tools
            let bt_dir = Path::new(&sdk_dir).join("build-tools");
            if bt_dir.exists() {
                let mut bts = Vec::new();
                if let Ok(entries) = std::fs::read_dir(&bt_dir) {
                    for e in entries.flatten() {
                        if let Some(name) = e.file_name().to_str() {
                            bts.push(name.to_string());
                        }
                    }
                }
                println!("    Build Tools Installed:         {:?}", bts);
            }
        }
        Err(e) => {
            println!("{}", "[ FAILED ]".red().bold());
            eprintln!("     {}", e);
            has_errors = true;
        }
    }

    // 3. Gradle
    print!("  Gradle Build Engine              ... ");
    let mut gradle_cmd = "gradle".to_string();
    let candidates = [
        "C:\\Gradle\\gradle-8.7\\bin\\gradle.bat",
        "C:\\Gradle\\bin\\gradle.bat",
    ];
    for c in &candidates {
        if Path::new(c).exists() {
            gradle_cmd = c.to_string();
            break;
        }
    }
    let gradle_check = Command::new(&gradle_cmd).arg("-v").output();
    match gradle_check {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout);
            let first_line = text
                .lines()
                .find(|l| l.contains("Gradle "))
                .unwrap_or("Gradle installed");
            println!("{} ({})", "[ OK ]".green().bold(), first_line.trim());
        }
        Err(_) => {
            println!("{}", "[ WARN ]".yellow().bold());
            println!(
                "     System Gradle not found on PATH. Gradle Wrapper (gradlew) will be used."
            );
            has_warnings = true;
        }
    }

    // 4. Android Debug Bridge (ADB)
    print!("  Android Debug Bridge (ADB)       ... ");
    let adb_check = Command::new("adb").arg("version").output();
    match adb_check {
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout);
            let line = s.lines().next().unwrap_or("ADB active");
            println!("{} ({})", "[ OK ]".green().bold(), line.trim());

            // Connected devices
            if let Ok(dev_out) = Command::new("adb").arg("devices").output() {
                let dev_str = String::from_utf8_lossy(&dev_out.stdout);
                let count = dev_str.lines().filter(|l| l.ends_with("device")).count();
                if count > 0 {
                    println!(
                        "    Attached Devices:              {} active device(s)",
                        count.to_string().green().bold()
                    );
                } else {
                    println!("    Attached Devices:              {} (connect via USB debugging or start emulator)", "0 devices".yellow());
                }
            }
        }
        Err(_) => {
            println!("{}", "[ WARN ]".yellow().bold());
            println!("     ADB not found on PATH. Device deployment commands will be unavailable.");
            has_warnings = true;
        }
    }

    // 5. Windows Toolchain
    #[cfg(windows)]
    {
        print!("  Windows Native Toolchain         ... ");
        let rustc_check = Command::new("rustc").arg("--version").output();
        if rustc_check.is_ok() {
            println!("{}", "[ OK ]".green().bold());
        } else {
            println!("{}", "[ WARN ]".yellow().bold());
            has_warnings = true;
        }
    }

    println!("\n--------------------------------------------------------");
    if has_errors {
        eprintln!(
            "{}",
            "Doctor found critical missing dependencies for mobile builds."
                .red()
                .bold()
        );
        ExitCode::Failure
    } else if has_warnings {
        println!(
            "{}",
            "Doctor found warnings, but required core toolchains are present."
                .yellow()
                .bold()
        );
        ExitCode::Success
    } else {
        println!(
            "{}",
            "All Appforge mobile and desktop toolchains are healthy and ready!"
                .green()
                .bold()
        );
        ExitCode::Success
    }
}
