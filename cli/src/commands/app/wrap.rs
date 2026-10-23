//! # `tsc app wrap web` Webforge Containerization Engine
//!
//! Wraps Webforge web applications into installable native mobile & desktop
//! applications with local asset streaming and native system bridges.

use crate::commands::app::android;
use crate::commands::app::create::scaffold_project;
use crate::commands::app::project::AppforgeManifest;
use crate::commands::app::windows;
use crate::exit_code::ExitCode;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub fn execute(target: &str, platform: &str, is_release: bool, verbose: bool) -> ExitCode {
    if target.to_lowercase() != "web" {
        eprintln!(
            "{} Only 'web' target is currently supported for app wrapping (e.g. 'tsc app wrap web')",
            "error:".red().bold()
        );
        return ExitCode::Failure;
    }

    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    println!(
        "{} Wrapping Webforge application for native platform: {}...",
        "🌐📦".bold(),
        platform.cyan().bold()
    );

    // 1. Build the web project to dist/
    println!("Step 1/3: Compiling Webforge production web assets...");
    let build_res = crate::commands::build::execute(
        Some("web"),
        Some(if is_release { "release" } else { "debug" }),
        false,
        false,
        "web",
        verbose,
    );
    if build_res != ExitCode::Success {
        eprintln!(
            "{} Failed to compile Webforge website to dist/",
            "error:".red().bold()
        );
        return build_res;
    }

    let dist_dir = current_dir.join("dist");
    if !dist_dir.exists() {
        eprintln!(
            "{} dist/ directory was not produced by web build.",
            "error:".red().bold()
        );
        return ExitCode::CompilationError;
    }

    // 2. Ensure Appforge host scaffolding exists
    println!("Step 2/3: Configuring native host scaffolding...");
    let manifest_path = current_dir.join("appforge.toml");
    if !manifest_path.exists() {
        let app_name = current_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("WebforgeApp");
        let _ = scaffold_project(&current_dir, app_name, None, verbose);
    }

    let manifest = match AppforgeManifest::load_from_dir(&current_dir) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{} {}", "error:".red().bold(), e);
            return ExitCode::Failure;
        }
    };

    // 3. Stage dist/ assets into host assets
    let p = platform.to_lowercase();
    match p.as_str() {
        "android" => {
            let android_assets = current_dir
                .join("android")
                .join("app")
                .join("src")
                .join("main")
                .join("assets");
            let _ = fs::create_dir_all(&android_assets);

            // Copy all dist contents into android assets
            if let Err(e) = copy_dir_all(&dist_dir, &android_assets) {
                eprintln!(
                    "{} Failed to copy web assets to Android host: {}",
                    "error:".red().bold(),
                    e
                );
                return ExitCode::Failure;
            }

            println!("Step 3/3: Packaging native Android APK...");
            match android::build(&manifest, &current_dir, is_release, false, verbose) {
                Ok(res) => {
                    println!(
                        "{}",
                        "\n========================================================"
                            .green()
                            .bold()
                    );
                    println!("  ✨ Webforge Web-to-Native Android Packaging SUCCESSFUL!");
                    println!(
                        "{}",
                        "========================================================"
                            .green()
                            .bold()
                    );
                    println!(
                        "Artifact: {}",
                        res.artifact_path.display().to_string().cyan().bold()
                    );
                    println!(
                        "Size:     {:.2} MB",
                        res.artifact_size_bytes as f64 / (1024.0 * 1024.0)
                    );
                    println!("Duration: {:.2} seconds\n", res.duration_secs);
                    ExitCode::Success
                }
                Err(e) => {
                    eprintln!(
                        "{} Native Android packaging failed: {}",
                        "error:".red().bold(),
                        e
                    );
                    ExitCode::CompilationError
                }
            }
        }
        "windows" => {
            println!("Step 3/3: Packaging native Windows application...");
            match windows::build(&manifest, &current_dir, is_release, verbose) {
                Ok(res) => {
                    println!(
                        "Wrapped Windows artifact: {}",
                        res.artifact_path.display().to_string().cyan().bold()
                    );
                    ExitCode::Success
                }
                Err(e) => {
                    eprintln!(
                        "{} Native Windows packaging failed: {}",
                        "error:".red().bold(),
                        e
                    );
                    ExitCode::CompilationError
                }
            }
        }
        _ => {
            eprintln!(
                "{} Platform '{}' is not supported for web wrapping.",
                "error:".red().bold(),
                platform
            );
            ExitCode::Failure
        }
    }
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
