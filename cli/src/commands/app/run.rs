//! # `tsc app run` Device Execution & Installation Engine
//!
//! Deploys and launches Appforge applications on physical devices, emulators,
//! and desktop hosts via ADB and native process launchers.

use crate::commands::app::android;
use crate::commands::app::project::AppforgeManifest;
use crate::commands::app::windows;
use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::PathBuf;
use std::process::Command;

pub fn execute(platform: &str, device: Option<&str>, is_release: bool, verbose: bool) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let manifest = match AppforgeManifest::load_from_dir(&current_dir) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{} {}", "error:".red().bold(), e);
            return ExitCode::Failure;
        }
    };

    let p = platform.to_lowercase();
    match p.as_str() {
        "android" => {
            // 1. Discover ADB devices
            let devices = match list_adb_devices() {
                Ok(devs) => devs,
                Err(e) => {
                    eprintln!(
                        "{} Failed to query ADB devices: {}",
                        "error:".red().bold(),
                        e
                    );
                    return ExitCode::Failure;
                }
            };

            if devices.is_empty() {
                eprintln!(
                    "{} No authorized Android devices or emulators attached via ADB.",
                    "error:".red().bold()
                );
                eprintln!("Please connect an Android phone with USB debugging enabled, or launch an emulator.");
                return ExitCode::Failure;
            }

            let target_device = match device {
                Some(id) => {
                    if !devices.contains(&id.to_string()) {
                        eprintln!(
                            "{} Specified device '{}' is not among attached devices: {:?}",
                            "error:".red().bold(),
                            id,
                            devices
                        );
                        return ExitCode::Failure;
                    }
                    id.to_string()
                }
                None => {
                    let chosen = devices[0].clone();
                    if devices.len() > 1 {
                        println!(
                            "Multiple devices found: {:?}. Targeting first device: {}",
                            devices,
                            chosen.cyan().bold()
                        );
                    }
                    chosen
                }
            };

            println!(
                "{} Deploying to Android device: {}...",
                "📱".bold(),
                target_device.cyan().bold()
            );

            // 2. Build APK
            let build_result =
                match android::build(&manifest, &current_dir, is_release, false, verbose) {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("{} Android build failed: {}", "error:".red().bold(), e);
                        return ExitCode::CompilationError;
                    }
                };

            // 3. Install APK via ADB
            println!(
                "{} Installing APK: '{}' on device [{}]...",
                "📦".bold(),
                build_result.artifact_path.display(),
                target_device
            );

            let install_output = Command::new("adb")
                .args([
                    "-s",
                    &target_device,
                    "install",
                    "-r",
                    &build_result.artifact_path.to_string_lossy(),
                ])
                .output();

            match install_output {
                Ok(out) => {
                    let out_str = String::from_utf8_lossy(&out.stdout);
                    let err_str = String::from_utf8_lossy(&out.stderr);
                    if !out.status.success() || out_str.contains("Failure") {
                        eprintln!(
                            "{} Installation failed: {}{}",
                            "error:".red().bold(),
                            out_str,
                            err_str
                        );
                        return ExitCode::Failure;
                    }
                    println!("  ✓ Installation complete: {}", "Success".green().bold());
                }
                Err(e) => {
                    eprintln!(
                        "{} Failed to invoke adb install: {}",
                        "error:".red().bold(),
                        e
                    );
                    return ExitCode::Failure;
                }
            }

            // 4. Launch main activity
            let activity_target = format!("{}/.MainActivity", manifest.android.package);
            println!(
                "{} Launching activity: {}...",
                "🚀".bold(),
                activity_target.cyan().bold()
            );

            let launch_output = Command::new("adb")
                .args([
                    "-s",
                    &target_device,
                    "shell",
                    "am",
                    "start",
                    "-n",
                    &activity_target,
                ])
                .output();

            match launch_output {
                Ok(out) => {
                    let out_str = String::from_utf8_lossy(&out.stdout);
                    if out_str.contains("Error") {
                        eprintln!("{} Launch failed: {}", "error:".red().bold(), out_str);
                        return ExitCode::Failure;
                    }
                    println!("  ✓ Application launched successfully on device!\n");
                    println!(
                        "{}",
                        "========================================================"
                            .green()
                            .bold()
                    );
                    println!(
                        "  🎉 Appforge Application is RUNNING on {}",
                        target_device.bold()
                    );
                    println!(
                        "{}",
                        "========================================================"
                            .green()
                            .bold()
                    );
                    ExitCode::Success
                }
                Err(e) => {
                    eprintln!(
                        "{} Failed to launch application: {}",
                        "error:".red().bold(),
                        e
                    );
                    ExitCode::Failure
                }
            }
        }
        "windows" => {
            let build_result = match windows::build(&manifest, &current_dir, is_release, verbose) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("{} Windows build failed: {}", "error:".red().bold(), e);
                    return ExitCode::CompilationError;
                }
            };
            println!(
                "{} Spawning Windows application: {}...",
                "🚀".bold(),
                build_result.artifact_path.display()
            );
            let _ = Command::new(&build_result.artifact_path).spawn();
            ExitCode::Success
        }
        _ => {
            eprintln!(
                "{} 'tsc app run' for platform '{}' is not supported on this host.",
                "error:".red().bold(),
                platform
            );
            ExitCode::Failure
        }
    }
}

pub fn install(platform: &str, device: Option<&str>, verbose: bool) -> ExitCode {
    execute(platform, device, false, verbose)
}

fn list_adb_devices() -> Result<Vec<String>, anyhow::Error> {
    let output = Command::new("adb").arg("devices").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("List of devices") || trimmed.starts_with('*')
        {
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "device" {
            devices.push(parts[0].to_string());
        }
    }
    Ok(devices)
}
