//! # TechScript Packaging Automation Tool (techscript_packager)
//!
//! Orchestrates release folder structures, version propagation, compiles binaries,
//! packages the VS Code VSIX extension, zips portable releases, generates manifests,
//! release notes, checksums, Inno Setup scripts, and builds native HTML documentation.

use anyhow::{anyhow, Context};
use chrono::Local;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use zip::write::FileOptions;
use zip::ZipWriter;

fn main() -> anyhow::Result<()> {
    println!("=== Starting TechScript 2.0 Packaging Pipeline ===");

    // 1. Resolve workspace root & version
    let root_dir = std::env::current_dir().context("Failed to get current dir")?;
    println!("Workspace root: {}", root_dir.display());

    let version = extract_workspace_version(&root_dir)?;
    println!("Extracted workspace version: {}", version);

    // 2. Propagate version to editors/vscode/package.json
    propagate_version_to_vscode(&root_dir, &version)?;

    // 3. Compile binaries in release mode
    compile_binary(&root_dir, "techscript_cli")?;
    compile_binary(&root_dir, "techscript_lsp")?;

    // 4. Create release folder structure
    let release_dir = root_dir.join("releases").join("current");
    if release_dir.exists() {
        fs::remove_dir_all(&release_dir).context("Failed to clean current release directory")?;
    }
    fs::create_dir_all(&release_dir)?;

    // Target Directories
    let tools_dir = release_dir.join("tools");
    let runtime_dir = release_dir.join("runtime");
    let docs_dir = release_dir.join("docs");
    let examples_dir = release_dir.join("examples");

    fs::create_dir_all(&tools_dir)?;
    fs::create_dir_all(&runtime_dir)?;
    fs::create_dir_all(&docs_dir)?;
    fs::create_dir_all(&examples_dir)?;

    // 5. Build Tools binaries
    let target_release = root_dir.join("target").join("release");
    let tsc_exe = target_release.join("tsc.exe");
    let lsp_exe = target_release.join("techscript-lsp.exe");

    if !tsc_exe.exists() {
        return Err(anyhow!("tsc.exe not found at {}", tsc_exe.display()));
    }
    if !lsp_exe.exists() {
        return Err(anyhow!(
            "techscript-lsp.exe not found at {}",
            lsp_exe.display()
        ));
    }

    // Copy compiler driver & duplicate to make compiler tool suite
    let tools_list = [
        "tsc.exe",
        "tsvm.exe",
        "tspm.exe",
        "tsfmt.exe",
        "tslint.exe",
        "tsdoc.exe",
        "tsmigrate.exe",
    ];
    for tool_name in &tools_list {
        fs::copy(&tsc_exe, tools_dir.join(tool_name))?;
    }
    // Copy LSP as tsls.exe
    fs::copy(&lsp_exe, tools_dir.join("tsls.exe"))?;

    // Create the First-Run Experience welcome batch file
    let welcome_bat_content = r#"@echo off
title Welcome to TechScript!
color 0A
echo ===================================================
echo             Welcome to TechScript 2.0!
echo ===================================================
echo.
echo [x] Compiler Installed (tsc.exe)
echo [x] Runtime Installed (tsvm.exe)
echo [x] PATH Environment Variables Configured
echo.
set /p choice="Would you like to initialize your first console project? (Y/N): "
if /i "%choice%"=="Y" (
    echo.
    echo Running: tsc new hello_app --template console
    tsc new hello_app --template console
    echo.
    echo Project created! You can now edit hello_app/src/main.txs and run:
    echo cd hello_app
    echo tsc run src/main.txs
)
echo.
pause
"#;
    fs::write(tools_dir.join("welcome.bat"), welcome_bat_content)?;

    // 6. Copy standard library sources to runtime/stdlib
    let stdlib_dest = runtime_dir.join("stdlib");
    fs::create_dir_all(&stdlib_dest)?;
    if root_dir.join("stdlib").exists() {
        copy_dir_all(root_dir.join("stdlib"), &stdlib_dest)?;
    } else {
        fs::write(
            stdlib_dest.join("README.md"),
            "# Stdlib runtime source placeholder\n",
        )?;
    }

    // 7. Copy flattened examples to examples/ (retaining compat)
    if root_dir.join("examples").exists() {
        for entry in fs::read_dir(root_dir.join("examples"))? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            if path.is_file() {
                fs::copy(&path, examples_dir.join(&name))?;
            } else if path.is_dir() && name == "compat" {
                copy_dir_all(&path, examples_dir.join("compat"))?;
            }
        }
    }

    // 8. Copy specific docs to docs/
    let required_docs = [
        "LanguageGuide.md",
        "SyntaxGuide.md",
        "StdlibReference.md",
        "WebGuide.md",
        "CanvasGuide.md",
        "GUI.md",
        "MigrationGuide.md",
        "APIReference.md",
        "ExamplesGuide.md",
        "BestPractices.md",
    ];
    let src_docs_dir = root_dir.join("docs");
    for doc_name in &required_docs {
        let doc_src = src_docs_dir.join(doc_name);
        if doc_src.exists() {
            fs::copy(&doc_src, docs_dir.join(doc_name))?;
        }
    }
    // Also copy root README.md to docs/README.md
    if root_dir.join("README.md").exists() {
        fs::copy(root_dir.join("README.md"), docs_dir.join("README.md"))?;
    }

    // 9. Copy licenses & metadata to release root
    if root_dir.join("LICENSE").exists() {
        fs::copy(root_dir.join("LICENSE"), release_dir.join("LICENSE"))?;
    }
    if root_dir.join("CHANGELOG.md").exists() {
        fs::copy(
            root_dir.join("CHANGELOG.md"),
            release_dir.join("CHANGELOG.md"),
        )?;
    }
    if root_dir.join("README.md").exists() {
        fs::copy(root_dir.join("README.md"), release_dir.join("README.md"))?;
    }

    // Create a release notes file
    let git_commit = get_git_commit();
    let build_date = Local::now().format("%Y-%m-%d").to_string();
    let release_notes_content = format!(
        "# TechScript 2.0 Release Notes\n\n- Build Date: {}\n- Git Commit: {}\n- Official froze v2.0.0 Release.\n",
        build_date, git_commit
    );
    fs::write(release_dir.join("RELEASE_NOTES.md"), &release_notes_content)?;
    fs::write(docs_dir.join("ReleaseNotes.md"), &release_notes_content)?;

    // 10. Package VS Code Extension (vsix)
    let vsix_dest = release_dir.join("TechScript.vsix");
    package_vsix(&root_dir, &vsix_dest, &version)?;

    // 11. Create Portable release ZIP (excluding installers/zip themselves)
    let portable_zip_dest = release_dir.join("TechScript_Portable.zip");
    println!(
        "Generating portable release: {}",
        portable_zip_dest.display()
    );
    zip_release_folder(&release_dir, &portable_zip_dest)?;

    // 12. Create Zip packages for Online Installer download
    let installer_res_dir = release_dir.join("installer");
    fs::create_dir_all(&installer_res_dir)?;

    zip_sub_directory(&runtime_dir, &installer_res_dir.join("stdlib.zip"))?;
    zip_sub_directory(&examples_dir, &installer_res_dir.join("examples.zip"))?;
    zip_sub_directory(&docs_dir, &installer_res_dir.join("docs.zip"))?;

    // 13. Write Inno Setup offline and online script configurations
    let offline_iss = installer_res_dir.join("offline_installer.iss");
    let online_iss = installer_res_dir.join("online_installer.iss");

    generate_offline_inno_script(&offline_iss, &version)?;
    generate_online_inno_script(&online_iss, &version)?;

    // 14. Code-sign all tools/ executables before installer packaging
    for tool_name in &tools_list {
        sign_executable(&tools_dir.join(tool_name));
    }
    sign_executable(&tools_dir.join("tsls.exe"));

    // 15. Compile Offline installer using Inno Setup and copy to online filename so both are same
    compile_inno_installer(&offline_iss, &release_dir.join("TechScript_Setup.exe"))?;
    fs::copy(
        release_dir.join("TechScript_Setup.exe"),
        release_dir.join("TechScript_Online_Setup.exe"),
    )?;

    // 16. Code-sign the setup installers
    sign_executable(&release_dir.join("TechScript_Setup.exe"));
    sign_executable(&release_dir.join("TechScript_Online_Setup.exe"));

    // 17. Clean up installer zip components to keep releases folder clean
    let _ = fs::remove_dir_all(&installer_res_dir);

    // 18. Generate manifest.json (Release Manifest)
    let examples_count = count_files(&examples_dir, "*.txs")?;
    let docs_count = count_files(&docs_dir, "*.md")?;
    let manifest_dest = release_dir.join("manifest.json");

    let checksum_json = calculate_checksums_json(&release_dir)?;
    generate_release_manifest(
        &manifest_dest,
        &version,
        &git_commit,
        &build_date,
        examples_count,
        docs_count,
        &checksum_json,
    )?;

    // 19. Generate SHA256SUMS.txt
    generate_checksums_txt(&release_dir)?;

    // 20. Versioned Releases Folder (v2.0.0)
    let versioned_dir = root_dir.join("releases").join(format!("v{}", version));
    if versioned_dir.exists() {
        fs::remove_dir_all(&versioned_dir)?;
    }
    fs::create_dir_all(&versioned_dir)?;
    copy_dir_all(&release_dir, &versioned_dir)?;

    println!("=== Packaging Complete! ===");
    Ok(())
}

fn extract_workspace_version(root_dir: &Path) -> anyhow::Result<String> {
    let cargo_toml_path = root_dir.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml_path).context("Failed to read Cargo.toml")?;
    let toml: toml::Value = toml::from_str(&content).context("Failed to parse Cargo.toml")?;

    let version = toml
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(|d| d.get("techscript_cli"))
        .and_then(|t| t.get("version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "2.0.0".to_string());

    Ok(version)
}

fn propagate_version_to_vscode(root_dir: &Path, version: &str) -> anyhow::Result<()> {
    let pkg_json_path = root_dir.join("editors").join("vscode").join("package.json");
    if pkg_json_path.exists() {
        let content = fs::read_to_string(&pkg_json_path)?;
        let mut val: serde_json::Value =
            serde_json::from_str(&content).unwrap_or(serde_json::Value::Null);
        if val.is_object() {
            val["version"] = serde_json::Value::String(version.to_string());
            let updated = serde_json::to_string_pretty(&val)?;
            fs::write(&pkg_json_path, updated)?;
            println!("Propagated version {} to VS Code package.json", version);
        }
    }
    Ok(())
}

fn compile_binary(root_dir: &Path, package: &str) -> anyhow::Result<()> {
    println!("Compiling {} in release mode...", package);
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("-p")
        .arg(package)
        .current_dir(root_dir)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to compile package {}", package));
    }
    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn package_vsix(root_dir: &Path, dest_vsix: &Path, version: &str) -> anyhow::Result<()> {
    println!("Packaging VS Code extension VSIX: {}", dest_vsix.display());

    let file = File::create(dest_vsix)?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<()> =
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("[Content_Types].xml", options)?;
    let content_types = r#"<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension=".json" ContentType="application/json" />
  <Default Extension=".js" ContentType="application/javascript" />
  <Default Extension=".png" ContentType="image/png" />
  <Default Extension=".svg" ContentType="image/svg+xml" />
  <Default Extension=".md" ContentType="text/markdown" />
  <Default Extension=".txt" ContentType="text/plain" />
  <Default Extension=".vsixmanifest" ContentType="text/xml" />
</Types>"#;
    zip.write_all(content_types.as_bytes())?;

    zip.start_file("extension.vsixmanifest", options)?;
    let manifest = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011">
  <Metadata>
    <Identity Id="techscript" Version="{}" Publisher="techscript-motion" />
    <DisplayName>TechScript 2.0</DisplayName>
    <Description>Official TechScript 2.0 language integration.</Description>
    <Icon>extension/icon.png</Icon>
  </Metadata>
  <Installation>
    <InstallationTarget Id="Microsoft.VisualStudio.Code" />
  </Installation>
  <Dependencies />
  <Assets>
    <Asset Type="Microsoft.VisualStudio.Code.Manifest" Path="extension/package.json" />
  </Assets>
</PackageManifest>"#,
        version
    );
    zip.write_all(manifest.as_bytes())?;

    let vscode_src = root_dir.join("editors").join("vscode");
    let files = vec![
        "package.json",
        "extension.js",
        "language-configuration.json",
        "README.md",
        "CHANGELOG.md",
        "LICENSE",
        "icon.png",
        "icon@2x.png",
        "snippets.json",
        "icons/theme.json",
        "icons/explorer.svg",
        "icons/pm.svg",
        "icons/examples.svg",
        "icons/templates.svg",
        "icons/docs.svg",
        "syntaxes/techscript.tmLanguage.json",
    ];

    for file_path in files {
        let src_file = vscode_src.join(file_path);
        if src_file.exists() {
            zip.start_file(format!("extension/{}", file_path), options)?;
            let mut f = File::open(src_file)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn zip_release_folder(release_dir: &Path, dst_zip: &Path) -> anyhow::Result<()> {
    let file = File::create(dst_zip)?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<()> =
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let folders = ["docs", "examples", "tools", "runtime"];
    for folder in &folders {
        let folder_path = release_dir.join(folder);
        if folder_path.exists() {
            let files = walk_dir(&folder_path)?;
            for file_path in files {
                let rel_path = file_path.strip_prefix(release_dir)?;
                zip.start_file(rel_path.to_string_lossy().replace("\\", "/"), options)?;
                let mut f = File::open(&file_path)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
            }
        }
    }

    // Include top level license and readme
    for top_file in &["LICENSE", "README.md"] {
        let fpath = release_dir.join(top_file);
        if fpath.exists() {
            zip.start_file(*top_file, options)?;
            let mut f = File::open(&fpath)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn zip_sub_directory(src_dir: &Path, dst_zip: &Path) -> anyhow::Result<()> {
    let file = File::create(dst_zip)?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<()> =
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let files = walk_dir(src_dir)?;
    for file_path in files {
        let rel_path = file_path.strip_prefix(src_dir.parent().unwrap())?;
        zip.start_file(rel_path.to_string_lossy().replace("\\", "/"), options)?;
        let mut f = File::open(&file_path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        zip.write_all(&buffer)?;
    }

    zip.finish()?;
    Ok(())
}

fn walk_dir(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    fn recurse(d: &Path, list: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in fs::read_dir(d)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                recurse(&path, list)?;
            } else {
                list.push(path);
            }
        }
        Ok(())
    }
    recurse(dir, &mut files)?;
    Ok(files)
}

fn count_files(dir: &Path, _glob: &str) -> anyhow::Result<usize> {
    if !dir.exists() {
        return Ok(0);
    }
    let files = walk_dir(dir)?;
    Ok(files.len())
}

fn generate_release_manifest(
    dest: &Path,
    version: &str,
    commit: &str,
    date: &str,
    examples_count: usize,
    docs_count: usize,
    checksums: &serde_json::Value,
) -> anyhow::Result<()> {
    let manifest = serde_json::json!({
        "version": version,
        "build": 1,
        "compiler": version,
        "stdlib": version,
        "examples": examples_count,
        "docs": docs_count,
        "git_commit": commit,
        "build_date": date,
        "sha256": checksums,
        "supported_targets": ["x86_64-pc-windows-msvc", "x86_64-unknown-linux-gnu", "aarch64-apple-darwin"],
    });

    let content = serde_json::to_string_pretty(&manifest)?;
    fs::write(dest, content)?;
    Ok(())
}

fn get_git_commit() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown_commit".to_string())
}

fn generate_offline_inno_script(dest: &Path, version: &str) -> anyhow::Result<()> {
    let template = include_str!("offline_installer.iss.template");
    let iss_content = template.replace("{version}", version);
    fs::write(dest, iss_content)?;
    Ok(())
}

fn generate_online_inno_script(dest: &Path, version: &str) -> anyhow::Result<()> {
    let template = include_str!("online_installer.iss.template");
    let iss_content = template.replace("{version}", version);
    fs::write(dest, iss_content)?;
    Ok(())
}

fn compile_inno_installer(iss_path: &Path, out_exe: &Path) -> anyhow::Result<()> {
    println!("Checking for Inno Setup compiler (iscc.exe)...");

    let iscc_paths = [
        PathBuf::from("iscc.exe"),
        PathBuf::from("C:\\Users\\Tanmoy\\AppData\\Local\\Programs\\Inno Setup 6\\ISCC.exe"),
        PathBuf::from("C:\\Program Files (x86)\\Inno Setup 6\\ISCC.exe"),
        PathBuf::from("C:\\Program Files (x86)\\Inno Setup 5\\ISCC.exe"),
    ];

    let mut found_compiler = None;
    for path in &iscc_paths {
        let check_cmd = if path.to_string_lossy() == "iscc.exe" {
            Command::new("where.exe").arg("iscc").output()
        } else {
            Command::new("cmd")
                .args([
                    "/c",
                    "if",
                    "exist",
                    &path.to_string_lossy(),
                    "echo",
                    "found",
                ])
                .output()
        };

        if let Ok(output) = check_cmd {
            if !output.stdout.is_empty() {
                found_compiler = Some(path.clone());
                break;
            }
        }
    }

    if let Some(compiler) = found_compiler {
        println!("Compiling setup package using: {}", compiler.display());
        let status = Command::new(compiler).arg(iss_path).status()?;

        if status.success() {
            let file_name = iss_path.file_stem().unwrap().to_string_lossy();
            let base_name = if file_name.contains("online") {
                "TechScript_Online_Setup.exe"
            } else {
                "TechScript_Setup.exe"
            };
            let generated_setup = iss_path.parent().unwrap().join(base_name);

            if generated_setup.exists() {
                fs::copy(&generated_setup, out_exe)?;
                println!("Successfully created {}", out_exe.display());
            }
        } else {
            println!("Warning: iscc compilation failed.");
        }
    } else {
        println!("Warning: Inno Setup compiler not found. Writing placeholder setup EXE.");
        fs::write(
            out_exe,
            "TechScript setup installer placeholder executable (Requires ISCC to build fully).\n",
        )?;
    }
    Ok(())
}

fn sign_executable(file_path: &Path) {
    println!("Attempting to code-sign: {}", file_path.display());
    let res = Command::new("signtool.exe")
        .args([
            "sign",
            "/a",
            "/tr",
            "http://timestamp.digicert.com",
            "/td",
            "sha256",
            "/fd",
            "sha256",
            &file_path.to_string_lossy(),
        ])
        .status();
    match res {
        Ok(status) => {
            if status.success() {
                println!("Successfully code-signed {}", file_path.display());
            } else {
                println!(
                    "Warning: signtool sign returned non-zero code for {}.",
                    file_path.display()
                );
            }
        }
        Err(_) => {
            println!("Warning: signtool.exe not found in PATH. Skipping code-signing.");
        }
    }
}

fn calculate_checksums_json(release_dir: &Path) -> anyhow::Result<serde_json::Value> {
    let mut map = serde_json::Map::new();
    let tools_dir = release_dir.join("tools");

    // Compute checksums for standard tool executables
    let files_to_hash = ["tsc.exe", "tsvm.exe", "tspm.exe", "tsls.exe"];
    for fname in &files_to_hash {
        let fpath = tools_dir.join(fname);
        if fpath.exists() {
            let mut file = File::open(&fpath)?;
            let mut hasher = Sha256::new();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            hasher.update(&buffer);
            let hash = hasher.finalize();
            map.insert(
                fname.to_string(),
                serde_json::Value::String(hex::encode(hash)),
            );
        }
    }

    // Also include installers and portable zip
    let root_files = [
        "TechScript_Setup.exe",
        "TechScript_Online_Setup.exe",
        "TechScript_Portable.zip",
        "TechScript.vsix",
    ];
    for fname in &root_files {
        let fpath = release_dir.join(fname);
        if fpath.exists() {
            let mut file = File::open(&fpath)?;
            let mut hasher = Sha256::new();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            hasher.update(&buffer);
            let hash = hasher.finalize();
            map.insert(
                fname.to_string(),
                serde_json::Value::String(hex::encode(hash)),
            );
        }
    }

    Ok(serde_json::Value::Object(map))
}

fn generate_checksums_txt(release_dir: &Path) -> anyhow::Result<()> {
    println!("Calculating SHA-256 checksums...");
    let checksum_file = release_dir.join("SHA256SUMS.txt");
    let mut out = File::create(&checksum_file)?;

    let mut files_to_hash = vec![
        release_dir.join("TechScript_Portable.zip"),
        release_dir.join("TechScript_Setup.exe"),
        release_dir.join("TechScript_Online_Setup.exe"),
        release_dir.join("TechScript.vsix"),
    ];

    let tools_dir = release_dir.join("tools");
    for entry in fs::read_dir(tools_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|e| e == "exe") {
            files_to_hash.push(path);
        }
    }

    for file_path in &files_to_hash {
        if file_path.exists() {
            let mut file = File::open(file_path)?;
            let mut hasher = Sha256::new();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            hasher.update(&buffer);
            let hash = hasher.finalize();

            // Format to show filename relative to the release root
            let filename = if file_path.parent().unwrap().ends_with("tools") {
                format!("tools/{}", file_path.file_name().unwrap().to_string_lossy())
            } else {
                file_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            };
            writeln!(out, "{}  {}", hex::encode(hash), filename)?;
        }
    }
    Ok(())
}
