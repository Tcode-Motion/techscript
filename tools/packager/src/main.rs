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

    let portable_dir = release_dir.join("portable").join("TechScript");
    fs::create_dir_all(portable_dir.join("bin"))?;
    let stdlib_dir = portable_dir.join("stdlib");
    fs::create_dir_all(&stdlib_dir)?;
    fs::write(
        stdlib_dir.join("README.md"),
        "# TechScript Standard Library\n\nStandard library modules are built directly into the `tsc` compiler binary.\n",
    )?;
    fs::create_dir_all(portable_dir.join("packages"))?;
    fs::create_dir_all(portable_dir.join("examples"))?;
    fs::create_dir_all(portable_dir.join("docs"))?;
    fs::create_dir_all(portable_dir.join("vscode"))?;
    fs::create_dir_all(portable_dir.join("templates"))?;

    fs::create_dir_all(release_dir.join("installer"))?;
    fs::create_dir_all(release_dir.join("vscode"))?;
    fs::create_dir_all(release_dir.join("release-notes"))?;
    fs::create_dir_all(release_dir.join("licenses"))?;
    fs::create_dir_all(release_dir.join("symbols"))?;
    fs::create_dir_all(release_dir.join("logs"))?;
    fs::create_dir_all(release_dir.join("examples"))?;
    fs::create_dir_all(release_dir.join("checksums"))?;

    // 5. Copy binaries & logo assets to bin/
    let target_release =
        root_dir.join("C:\\Users\\Tanmoy\\.gemini\\antigravity-ide\\target\\release");
    let target_release = if target_release.exists() {
        target_release
    } else {
        root_dir.join("target").join("release")
    };

    let tsc_exe = target_release.join("tsc.exe");
    let lsp_exe = target_release.join("techscript-lsp.exe");

    fs::copy(&tsc_exe, portable_dir.join("bin").join("tsc.exe"))?;
    fs::copy(&tsc_exe, portable_dir.join("bin").join("tspm.exe"))?;
    fs::copy(&tsc_exe, portable_dir.join("bin").join("tsfmt.exe"))?;
    fs::copy(&tsc_exe, portable_dir.join("bin").join("tslint.exe"))?;
    fs::copy(&tsc_exe, portable_dir.join("bin").join("tsdbg.exe"))?;
    fs::copy(
        &lsp_exe,
        portable_dir.join("bin").join("techscript-lsp.exe"),
    )?;

    // Copy logo windows and file icon to bin/
    let logo_src = root_dir
        .join("assets")
        .join("branding")
        .join("logo-package");
    if logo_src.exists() {
        fs::copy(
            logo_src.join("ico").join("file-icon.ico"),
            portable_dir.join("bin").join("file-icon.ico"),
        )?;
        fs::copy(
            logo_src.join("windows").join("installer-icon.ico"),
            portable_dir.join("bin").join("installer-icon.ico"),
        )?;
    }

    // 6. Write standard templates & 20 documented examples
    write_templates(&portable_dir.join("templates"))?;
    write_examples(&portable_dir.join("examples"), &root_dir)?;
    write_examples(&release_dir.join("examples"), &root_dir)?;

    // 7. Copy licenses
    let license_dest = release_dir.join("licenses").join("LICENSE");
    let root_license_dest = release_dir.join("LICENSE");
    if root_dir.join("LICENSE").exists() {
        fs::copy(root_dir.join("LICENSE"), &license_dest)?;
        fs::copy(&license_dest, &root_license_dest)?;
        fs::copy(&license_dest, portable_dir.join("bin").join("LICENSE"))?;
    } else {
        fs::write(&license_dest, "MIT License\n")?;
        fs::write(&root_license_dest, "MIT License\n")?;
    }

    // 8. Generate multi-format documentation (HTML & Markdown)
    generate_documentation(&portable_dir.join("docs"), &version)?;

    // 9. Generate VS Code Extension package (VSIX zip archive)
    package_vsix(
        &root_dir,
        &portable_dir.join("vscode").join("techscript.vsix"),
        &version,
    )?;
    fs::copy(
        portable_dir.join("vscode").join("techscript.vsix"),
        release_dir.join("vscode").join("techscript.vsix"),
    )?;
    fs::copy(
        portable_dir.join("vscode").join("techscript.vsix"),
        release_dir.join("techscript.vsix"),
    )?;

    // 10. Copy compiled executables to public-release/compiler/
    fs::create_dir_all(release_dir.join("compiler"))?;
    fs::copy(&tsc_exe, release_dir.join("compiler").join("tsc.exe"))?;

    // 11. Zip portable release
    let zip_dest = release_dir.join("TechScript_Portable.zip");
    println!("Creating portable release ZIP: {}", zip_dest.display());
    zip_directory(&portable_dir, &zip_dest)?;
    fs::create_dir_all(release_dir.join("portable"))?;
    fs::copy(&zip_dest, release_dir.join("portable").join("TechScript_Portable.zip"))?;

    // 12. Generate release.json (Release Manifest)
    let git_commit = get_git_commit();
    let build_date = Local::now().format("%Y-%m-%d").to_string();
    let manifest_dest = release_dir.join("release.json");
    generate_release_manifest(
        &manifest_dest,
        &version,
        &git_commit,
        &build_date,
    )?;
    fs::copy(&manifest_dest, release_dir.join("checksums").join("release.json"))?;

    // 13. Generate RELEASE_NOTES.md
    let notes_dest = release_dir.join("RELEASE_NOTES.md");
    generate_release_notes(&notes_dest)?;
    fs::copy(&notes_dest, release_dir.join("release-notes").join("RELEASE_NOTES.md"))?;

    // Copy CHANGELOG.md to root public-release/
    if root_dir.join("CHANGELOG.md").exists() {
        fs::copy(root_dir.join("CHANGELOG.md"), release_dir.join("CHANGELOG.md"))?;
    } else {
        fs::write(release_dir.join("CHANGELOG.md"), "# Changelog\n\n- Initial release\n")?;
    }

    // Copy documentation and templates to root public-release/
    generate_documentation(&release_dir.join("docs"), &version)?;
    write_templates(&release_dir.join("templates"))?;

    // Create debug-tools folder
    let debug_tools_dir = release_dir.join("debug-tools");
    fs::create_dir_all(&debug_tools_dir)?;
    fs::write(
        debug_tools_dir.join("README.md"),
        "# TechScript Debug Tools\n\nCompiler debug tools are included in `tsc.exe` via subcommand flags.\n",
    )?;

    // 14. Generate Inno Setup Script (installer.iss)
    generate_inno_script(
        &release_dir.join("installer").join("installer.iss"),
        &version,
        &root_dir,
    )?;

    // 15. Attempt to build Inno Setup Installer if iscc is present
    compile_inno_installer(&release_dir.join("installer").join("installer.iss"))?;

    // 16. Generate public-release README.md (Manual Testing Guide)
    generate_testing_readme(&release_dir.join("README.md"))?;

    // 17. Generate SHA-256 Checksums (SHA256SUMS.txt)
    generate_checksums(&release_dir)?;

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
        let mut val: serde_json::Value = serde_json::Value::Null;
        if let Ok(v) = serde_json::from_str(&content) {
            val = v;
        }
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
    let target_dir = "C:\\Users\\Tanmoy\\.gemini\\antigravity-ide\\target";
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("-p")
        .arg(package)
        .env("CARGO_TARGET_DIR", target_dir)
        .current_dir(root_dir)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to compile package {}", package));
    }
    Ok(())
}

fn write_templates(dest_dir: &Path) -> anyhow::Result<()> {
    // 1. console template
    let console_dir = dest_dir.join("console");
    fs::create_dir_all(console_dir.join("src"))?;
    fs::write(
        console_dir.join("tech.toml"),
        "[package]\nname = \"console_app\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\n",
    )?;
    fs::write(
        console_dir.join("src").join("main.txs"),
        "build main() {\n    say \"Hello, TechScript Console App!\"\n}\n",
    )?;

    // 2. library template
    let lib_dir = dest_dir.join("library");
    fs::create_dir_all(lib_dir.join("src"))?;
    fs::write(
        lib_dir.join("tech.toml"),
        "[package]\nname = \"my_library\"\nversion = \"0.1.0\"\nentry = \"src/lib.txs\"\n",
    )?;
    fs::write(
        lib_dir.join("src").join("lib.txs"),
        "export build add(a, b) {\n    return a + b\n}\n",
    )?;

    // 3. workspace template
    let ws_dir = dest_dir.join("workspace");
    fs::create_dir_all(ws_dir.join("packages").join("core").join("src"))?;
    fs::write(
        ws_dir.join("tech.toml"),
        "[workspace]\nmembers = [\"packages/core\"]\n",
    )?;
    fs::write(
        ws_dir.join("packages").join("core").join("tech.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nentry = \"src/lib.txs\"\n",
    )?;
    fs::write(
        ws_dir.join("packages").join("core").join("src").join("lib.txs"),
        "export build add(a, b) {\n    return a + b\n}\n",
    )?;

    // 4. package template
    let pkg_dir = dest_dir.join("package");
    fs::create_dir_all(pkg_dir.join("src"))?;
    fs::write(pkg_dir.join("tech.toml"), "[package]\nname = \"package_app\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\ncapabilities = [\"Process\", \"FilesystemRead\"]\n")?;
    fs::write(
        pkg_dir.join("src").join("main.txs"),
        "build main() {\n    say \"Package initialized.\"\n}\n",
    )?;

    // 5. cli template
    let cli_dir = dest_dir.join("cli");
    fs::create_dir_all(cli_dir.join("src"))?;
    fs::write(
        cli_dir.join("tech.toml"),
        "[package]\nname = \"cli_tool\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\n",
    )?;
    fs::write(
        cli_dir.join("src").join("main.txs"),
        "build main() {\n    say \"TechScript CLI Tool running.\"\n}\n",
    )?;

    // 6. gui template
    let gui_dir = dest_dir.join("gui");
    fs::create_dir_all(gui_dir.join("src"))?;
    fs::write(
        gui_dir.join("tech.toml"),
        "[package]\nname = \"gui_app\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\ncapabilities = [\"FileSystem\"]\n",
    )?;
    fs::write(
        gui_dir.join("src").join("main.txs"),
        "build main() {\n    say \"GUI window initialized.\"\n}\n",
    )?;

    // 7. empty template
    let empty_dir = dest_dir.join("empty");
    fs::create_dir_all(empty_dir.join("src"))?;
    fs::write(
        empty_dir.join("tech.toml"),
        "[package]\nname = \"empty\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\n",
    )?;
    fs::write(empty_dir.join("src").join("main.txs"), "")?;

    // 8. web template
    let web_dir = dest_dir.join("web");
    fs::create_dir_all(web_dir.join("src"))?;
    fs::write(
        web_dir.join("tech.toml"),
        "[package]\nname = \"web_server\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\ncapabilities = [\"Network\"]\n",
    )?;
    fs::write(
        web_dir.join("src").join("main.txs"),
        "build main() {\n    say \"Web server listening on port 8080...\"\n}\n",
    )?;

    Ok(())
}

fn write_examples(dest_dir: &Path, root_dir: &Path) -> anyhow::Result<()> {
    let src_examples = root_dir.join("examples");
    if src_examples.exists() {
        copy_dir_all(&src_examples, dest_dir)?;
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

fn generate_documentation(dest_dir: &Path, _version: &str) -> anyhow::Result<()> {
    fs::create_dir_all(dest_dir)?;
    let root_docs = Path::new("docs");
    if root_docs.exists() {
        // Copy markdown files from docs/ to dest_dir/
        for entry in fs::read_dir(root_docs)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|e| e == "md") {
                fs::copy(&path, dest_dir.join(entry.file_name()))?;
            }
        }
        // Copy docs/website to dest_dir/html
        let website_src = root_docs.join("website");
        if website_src.exists() {
            let html_dir = dest_dir.join("html");
            copy_dir_all(&website_src, &html_dir)?;
        }
    }
    Ok(())
}

fn package_vsix(root_dir: &Path, dest_vsix: &Path, version: &str) -> anyhow::Result<()> {
    println!("Packaging VS Code extension VSIX: {}", dest_vsix.display());

    let file = File::create(dest_vsix)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // 1. Write [Content_Types].xml
    zip.start_file("[Content_Types].xml", options)?;
    let content_types = r#"<?xml version="2.0" encoding="utf-8"?>
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

    // 2. Write extension.vsixmanifest
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

    // 3. Add files inside extension/ folder in ZIP
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

fn zip_directory(src_dir: &Path, dst_zip: &Path) -> anyhow::Result<()> {
    let file = File::create(dst_zip)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let walk_dir = |dir: &Path| -> anyhow::Result<Vec<PathBuf>> {
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
    };

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

fn generate_release_manifest(
    dest: &Path,
    version: &str,
    commit: &str,
    date: &str,
) -> anyhow::Result<()> {
    let manifest = serde_json::json!({
        "version": version,
        "language_version": "2.0.0",
        "bytecode_version": "1.0.0",
        "vm_version": "1.0.0",
        "llvm_backend_version": "18.0.0",
        "stdlib_version": "0.1.0",
        "git_commit": commit,
        "build_date": date,
        "supported_targets": ["x86_64-pc-windows-msvc", "x86_64-unknown-linux-gnu", "aarch64-apple-darwin"],
        "profile": "release",
        "metadata": {
            "developer_release": true,
            "channel": "debug"
        }
    });

    let content = serde_json::to_string_pretty(&manifest)?;
    fs::write(dest, content)?;
    Ok(())
}

fn generate_release_notes(dest: &Path) -> anyhow::Result<()> {
    // Run git log to populate RELEASE_NOTES.md
    let output = Command::new("git")
        .args(["log", "-n", "15", "--oneline"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_else(|_| "Initial Developer Preview commits.\n".to_string());

    let content = format!(
        "# TechScript 2.0 Release Notes\n\n## Developer Debug Preview Commits:\n\n```\n{}\n```\n",
        output
    );
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

fn generate_inno_script(dest: &Path, version: &str, root: &Path) -> anyhow::Result<()> {
    let portable_relative = root
        .join("releases")
        .join("current")
        .join("portable")
        .join("TechScript");

    // Write privacy policy to same directory as installer.iss
    let privacy_path = dest.parent().unwrap().join("PRIVACY_POLICY.txt");
    let privacy_content = "TechScript Privacy Policy\n\nWe respect your privacy. The TechScript compiler and toolchain do not collect, store, or transmit any personal data or usage metrics. All code execution, compilation, and package management occur locally on your machine.\n";
    fs::write(privacy_path, privacy_content)?;

    let iss_content = format!(
        r#"; TechScript 2.0 Installer Script (Inno Setup)
[Setup]
AppId={{{{TechScript-Compiler-Environment-2-0}}
AppName=TechScript 2.0
AppVersion={version}
AppPublisher=techscript-motion
AppPublisherURL=https://github.com/Tcode-Motion/TechScript-2.0
AppCopyright=Copyright (c) 2026 techscript-motion
VersionInfoVersion={version}
VersionInfoCompany=techscript-motion
VersionInfoDescription=TechScript 2.0 Language Environment
DefaultDirName={{autopf}}\TechScript
DefaultGroupName=TechScript 2.0
ChangesAssociations=yes
UninstallDisplayIcon={{app}}\bin\tsc.exe
Compression=lzma2
SolidCompression=yes
OutputDir=.
OutputBaseFilename=TechScript_Setup
SetupIconFile={logo_dir}\windows\installer-icon.ico
WizardImageFile={logo_dir}\source\logo-black-bg-1254.png
WizardSmallImageFile={logo_dir}\png\icon-256.png
PrivilegesRequired=admin
DisableWelcomePage=no
LicenseFile={root_dir}\LICENSE
InfoBeforeFile=PRIVACY_POLICY.txt

[InstallDelete]
Type: filesandordirs; Name: "{{commonpf64}}\TechScript"
Type: filesandordirs; Name: "{{commonpf32}}\TechScript"

[Types]
Name: "full"; Description: "Full installation (Recommended)"
Name: "compact"; Description: "Compact installation"
Name: "custom"; Description: "Custom installation"; Flags: iscustom

[Components]
Name: "main"; Description: "TechScript Compiler & REPL Core"; Types: full compact custom; Flags: fixed
Name: "lsp"; Description: "Language Server Protocol (LSP) Service"; Types: full custom
Name: "examples"; Description: "Language Examples & Templates"; Types: full custom
Name: "docs"; Description: "Offline Documentation & Guides"; Types: full custom
Name: "vscode"; Description: "VS Code IDE Extension Integration"; Types: full custom

[Files]
Source: "{portable_dir}\bin\tsc.exe"; DestDir: "{{app}}\bin"; Components: main; Flags: ignoreversion
Source: "{portable_dir}\bin\tspm.exe"; DestDir: "{{app}}\bin"; Components: main; Flags: ignoreversion
Source: "{portable_dir}\bin\tsfmt.exe"; DestDir: "{{app}}\bin"; Components: main; Flags: ignoreversion
Source: "{portable_dir}\bin\tslint.exe"; DestDir: "{{app}}\bin"; Components: main; Flags: ignoreversion
Source: "{portable_dir}\bin\tsdbg.exe"; DestDir: "{{app}}\bin"; Components: main; Flags: ignoreversion
Source: "{portable_dir}\bin\file-icon.ico"; DestDir: "{{app}}\bin"; Components: main; Flags: ignoreversion
Source: "{portable_dir}\bin\installer-icon.ico"; DestDir: "{{app}}\bin"; Components: main; Flags: ignoreversion
Source: "{portable_dir}\bin\LICENSE"; DestDir: "{{app}}\bin"; Components: main; Flags: ignoreversion
Source: "{portable_dir}\bin\techscript-lsp.exe"; DestDir: "{{app}}\bin"; Components: lsp; Flags: ignoreversion
Source: "{portable_dir}\stdlib\*"; DestDir: "{{app}}\stdlib"; Components: main; Flags: recursesubdirs createallsubdirs ignoreversion
Source: "{portable_dir}\examples\*"; DestDir: "{{app}}\examples"; Components: examples; Flags: recursesubdirs createallsubdirs ignoreversion
Source: "{portable_dir}\docs\*"; DestDir: "{{app}}\docs"; Components: docs; Flags: recursesubdirs createallsubdirs ignoreversion
Source: "{portable_dir}\vscode\*"; DestDir: "{{app}}\vscode"; Components: vscode; Flags: recursesubdirs createallsubdirs ignoreversion
Source: "{portable_dir}\templates\*"; DestDir: "{{app}}\templates"; Components: examples; Flags: recursesubdirs createallsubdirs ignoreversion

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut for REPL"; GroupDescription: "Additional shortcuts:"; Components: main
Name: "startmenu"; Description: "Create &Start Menu shortcuts"; GroupDescription: "Additional shortcuts:"; Components: main
Name: "userpath"; Description: "Add bin to &User PATH and configure User environment variables"; GroupDescription: "Environment variables:"; Flags: unchecked
Name: "systempath"; Description: "Add bin to &System PATH and configure System environment variables (Requires Admin)"; GroupDescription: "Environment variables:"
Name: "associate"; Description: "Associate .txs, .tsx, .tech, and .tspkg file extensions"; GroupDescription: "File Associations:"
Name: "contextmenu"; Description: "Add context menu items to Windows Explorer"; GroupDescription: "File Associations:"

[Icons]
Name: "{{autodesktop}}\TechScript REPL"; Filename: "{{app}}\bin\tsc.exe"; Parameters: "repl"; IconFilename: "{{app}}\bin\file-icon.ico"; Tasks: desktopicon
Name: "{{group}}\TechScript Compiler CLI"; Filename: "{{app}}\bin\tsc.exe"; IconFilename: "{{app}}\bin\file-icon.ico"; Tasks: startmenu
Name: "{{group}}\TechScript REPL Console"; Filename: "{{app}}\bin\tsc.exe"; Parameters: "repl"; IconFilename: "{{app}}\bin\file-icon.ico"; Tasks: startmenu
Name: "{{group}}\Offline Documentation"; Filename: "{{app}}\docs\html\index.html"; IconFilename: "{{app}}\bin\file-icon.ico"; Tasks: startmenu; Components: docs
Name: "{{group}}\Code Examples"; Filename: "{{app}}\examples"; IconFilename: "{{app}}\bin\file-icon.ico"; Tasks: startmenu; Components: examples
Name: "{{group}}\Uninstall TechScript"; Filename: "{{uninstallexe}}"; Tasks: startmenu

[Registry]
; File associations
Root: HKA; Subkey: "Software\Classes\.txs"; ValueType: string; ValueName: ""; ValueData: "TechScript.File"; Flags: uninsdeletevalue; Tasks: associate
Root: HKA; Subkey: "Software\Classes\.tsx"; ValueType: string; ValueName: ""; ValueData: "TechScript.File"; Flags: uninsdeletevalue; Tasks: associate
Root: HKA; Subkey: "Software\Classes\.tech"; ValueType: string; ValueName: ""; ValueData: "TechScript.File"; Flags: uninsdeletevalue; Tasks: associate
Root: HKA; Subkey: "Software\Classes\.tspkg"; ValueType: string; ValueName: ""; ValueData: "TechScript.File"; Flags: uninsdeletevalue; Tasks: associate
Root: HKA; Subkey: "Software\Classes\TechScript.File"; ValueType: string; ValueName: ""; ValueData: "TechScript Source File"; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\TechScript.File"; ValueType: string; ValueName: "Content Type"; ValueData: "text/x-techscript"; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\TechScript.File\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{{app}}\bin\file-icon.ico"; Flags: uninsdeletekey; Tasks: associate

; Double-click open action (Always run the script in a console and keep it open so output can be read)
Root: HKA; Subkey: "Software\Classes\TechScript.File\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{{app}}\bin\tsc.exe"" ""%1"" --double-click"; Flags: uninsdeletekey; Tasks: associate

; Fallback registrations for the legacy ProgID (TechScript.Script) to support cached user associations
Root: HKA; Subkey: "Software\Classes\TechScript.Script"; ValueType: string; ValueName: ""; ValueData: "TechScript Source File"; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\TechScript.Script"; ValueType: string; ValueName: "Content Type"; ValueData: "text/x-techscript"; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\TechScript.Script\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{{app}}\bin\file-icon.ico"; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\TechScript.Script\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{{app}}\bin\tsc.exe"" ""%1"" --double-click"; Flags: uninsdeletekey; Tasks: associate

; Context Menu items
Root: HKA; Subkey: "Software\Classes\TechScript.File\shell\Compile"; ValueType: string; ValueName: ""; ValueData: "Compile with TechScript"; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\TechScript.File\shell\Compile\command"; ValueType: string; ValueName: ""; ValueData: """{{app}}\bin\tsc.exe"" build ""%1"""; Tasks: contextmenu

Root: HKA; Subkey: "Software\Classes\TechScript.File\shell\Run"; ValueType: string; ValueName: ""; ValueData: "Run with TechScript"; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\TechScript.File\shell\Run\command"; ValueType: string; ValueName: ""; ValueData: """{{app}}\bin\tsc.exe"" ""%1"" --double-click"; Tasks: contextmenu

Root: HKA; Subkey: "Software\Classes\TechScript.File\shell\OpenVSCode"; ValueType: string; ValueName: ""; ValueData: "Open in VS Code"; Tasks: contextmenu; Check: IsVSCodeInstalled
Root: HKA; Subkey: "Software\Classes\TechScript.File\shell\OpenVSCode"; ValueType: string; ValueName: "Icon"; ValueData: "code.exe"; Tasks: contextmenu; Check: IsVSCodeInstalled
Root: HKA; Subkey: "Software\Classes\TechScript.File\shell\OpenVSCode\command"; ValueType: string; ValueName: ""; ValueData: "code ""%1"""; Tasks: contextmenu; Check: IsVSCodeInstalled

Root: HKA; Subkey: "Software\Classes\Directory\Background\shell\TechScriptTerminal"; ValueType: string; ValueName: ""; ValueData: "Open TechScript Terminal Here"; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\Directory\Background\shell\TechScriptTerminal"; ValueType: string; ValueName: "Icon"; ValueData: """{{app}}\bin\tsc.exe"""; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\Directory\Background\shell\TechScriptTerminal\command"; ValueType: string; ValueName: ""; ValueData: "cmd.exe /K ""cd /d ""%V"" && SET PATH={{app}}\bin;%PATH%"""; Tasks: contextmenu

; Applications registration for tsc.exe
Root: HKA; Subkey: "Software\Classes\Applications\tsc.exe"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "TechScript Compiler Driver"; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\Applications\tsc.exe\SupportedTypes"; ValueType: string; ValueName: ".txs"; ValueData: ""; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\Applications\tsc.exe\SupportedTypes"; ValueType: string; ValueName: ".tsx"; ValueData: ""; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\Applications\tsc.exe\SupportedTypes"; ValueType: string; ValueName: ".tech"; ValueData: ""; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\Applications\tsc.exe\SupportedTypes"; ValueType: string; ValueName: ".tspkg"; ValueData: ""; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Classes\Applications\tsc.exe\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{{app}}\bin\tsc.exe"" ""%1"" --double-click"; Flags: uninsdeletekey; Tasks: associate

; App Paths registration for tsc.exe
Root: HKA; Subkey: "Software\Microsoft\Windows\CurrentVersion\App Paths\tsc.exe"; ValueType: string; ValueName: ""; ValueData: "{{app}}\bin\tsc.exe"; Flags: uninsdeletekey; Tasks: associate
Root: HKA; Subkey: "Software\Microsoft\Windows\CurrentVersion\App Paths\tsc.exe"; ValueType: string; ValueName: "Path"; ValueData: "{{app}}\bin"; Flags: uninsdeletekey; Tasks: associate

; User Environment Variables
Root: HKCU; Subkey: "Environment"; ValueType: string; ValueName: "TECHSCRIPT_HOME"; ValueData: "{{app}}"; Flags: preservestringtype uninsdeletevalue; Tasks: userpath
Root: HKCU; Subkey: "Environment"; ValueType: string; ValueName: "TECHSCRIPT_STDLIB"; ValueData: "{{app}}\stdlib"; Flags: preservestringtype uninsdeletevalue; Tasks: userpath
Root: HKCU; Subkey: "Environment"; ValueType: string; ValueName: "TECHSCRIPT_DOCS"; ValueData: "{{app}}\docs"; Flags: preservestringtype uninsdeletevalue; Tasks: userpath
Root: HKCU; Subkey: "Environment"; ValueType: string; ValueName: "TECHSCRIPT_TEMPLATES"; ValueData: "{{app}}\templates"; Flags: preservestringtype uninsdeletevalue; Tasks: userpath
Root: HKCU; Subkey: "Environment"; ValueType: string; ValueName: "TECHSCRIPT_CACHE"; ValueData: "{{%USERPROFILE}}\.techscript\cache"; Flags: preservestringtype uninsdeletevalue; Tasks: userpath
Root: HKCU; Subkey: "Environment"; ValueType: string; ValueName: "TECHSCRIPT_PACKAGES"; ValueData: "{{%USERPROFILE}}\.techscript\packages"; Flags: preservestringtype uninsdeletevalue; Tasks: userpath

; System Environment Variables
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: string; ValueName: "TECHSCRIPT_HOME"; ValueData: "{{app}}"; Flags: preservestringtype uninsdeletevalue; Tasks: systempath
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: string; ValueName: "TECHSCRIPT_STDLIB"; ValueData: "{{app}}\stdlib"; Flags: preservestringtype uninsdeletevalue; Tasks: systempath
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: string; ValueName: "TECHSCRIPT_DOCS"; ValueData: "{{app}}\docs"; Flags: preservestringtype uninsdeletevalue; Tasks: systempath
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: string; ValueName: "TECHSCRIPT_TEMPLATES"; ValueData: "{{app}}\templates"; Flags: preservestringtype uninsdeletevalue; Tasks: systempath
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: string; ValueName: "TECHSCRIPT_CACHE"; ValueData: "{{%USERPROFILE}}\.techscript\cache"; Flags: preservestringtype uninsdeletevalue; Tasks: systempath
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: string; ValueName: "TECHSCRIPT_PACKAGES"; ValueData: "{{%USERPROFILE}}\.techscript\packages"; Flags: preservestringtype uninsdeletevalue; Tasks: systempath

[Run]
Filename: "{{app}}\bin\tsc.exe"; Parameters: "repl"; Description: "Launch TechScript REPL Console"; Flags: postinstall nowait skipifsilent unchecked
Filename: "cmd.exe"; Parameters: "/K ""SET PATH={{app}}\bin;%PATH%"""; Description: "Open TechScript Terminal / Developer Console"; Flags: postinstall nowait skipifsilent unchecked
Filename: "cmd.exe"; Parameters: "/c code ."; WorkingDir: "{{app}}"; Description: "Launch VS Code in installation folder"; Flags: postinstall nowait skipifsilent unchecked; Check: IsVSCodeInstalled
Filename: "explorer.exe"; Parameters: """{{app}}\docs\html\index.html"""; Description: "Open Offline Documentation"; Flags: postinstall nowait skipifsilent unchecked; Components: docs
Filename: "explorer.exe"; Parameters: """{{app}}\examples"""; Description: "Open Examples Folder"; Flags: postinstall nowait skipifsilent unchecked; Components: examples
Filename: "cmd.exe"; Parameters: "/c code --install-extension ""{{app}}\vscode\techscript.vsix"""; Description: "Install VS Code Extension (Stable)"; Flags: postinstall nowait skipifsilent; Check: IsVSCodeInstalled; Components: vscode
Filename: "cmd.exe"; Parameters: "/c code-insiders --install-extension ""{{app}}\vscode\techscript.vsix"""; Description: "Install VS Code Extension (Insiders)"; Flags: postinstall nowait skipifsilent; Check: IsVSCodeInsidersInstalled; Components: vscode

[Code]
const
  WM_SETTINGCHANGE = $001A;
  SMTO_ABORTIFHUNG = 2;

function SendMessageTimeout(hWnd: HWND; Msg: Integer; wParam: Longint; lParam: String; fuFlags: Integer; uTimeout: Integer; var lpdwResult: Longint): Longint;
  external 'SendMessageTimeoutW@user32.dll stdcall';

procedure SHChangeNotify(wEventId: Longint; uFlags: Integer; dwItem1: Longint; dwItem2: Longint);
  external 'SHChangeNotify@shell32.dll stdcall';

function IsVSCodeInstalled(): Boolean;
begin
  Result := RegKeyExists(HKEY_CURRENT_USER, 'Software\Classes\Applications\code.exe') or
            RegKeyExists(HKEY_LOCAL_MACHINE, 'Software\Microsoft\Windows\CurrentVersion\App Paths\code.exe') or
            RegKeyExists(HKEY_CURRENT_USER, 'Software\Microsoft\Windows\CurrentVersion\App Paths\code.exe');
end;

function IsVSCodeInsidersInstalled(): Boolean;
begin
  Result := RegKeyExists(HKEY_CURRENT_USER, 'Software\Classes\Applications\code-insiders.exe') or
            RegKeyExists(HKEY_LOCAL_MACHINE, 'Software\Microsoft\Windows\CurrentVersion\App Paths\code-insiders.exe') or
            RegKeyExists(HKEY_CURRENT_USER, 'Software\Microsoft\Windows\CurrentVersion\App Paths\code-insiders.exe');
end;

function NotVSCodeInstalled(): Boolean;
begin
  Result := not IsVSCodeInstalled();
end;

function GetUninstallString(): String;
var
  sUnInstPath: String;
  sUnInstallString: String;
begin
  sUnInstPath := 'Software\Microsoft\Windows\CurrentVersion\Uninstall\TechScript 2.0_is1';
  sUnInstallString := '';
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, sUnInstPath, 'UninstallString', sUnInstallString) then
    RegQueryStringValue(HKEY_CURRENT_USER, sUnInstPath, 'UninstallString', sUnInstallString);
  Result := sUnInstallString;
end;

function InitializeSetup(): Boolean;
var
  V: Integer;
  sUnInstallString: String;
  sUnInstallParams: String;
begin
  Result := True;
  sUnInstallString := GetUninstallString();
  if sUnInstallString <> '' then begin
    sUnInstallString := RemoveQuotes(sUnInstallString);
    if MsgBox('A previous version of TechScript is already installed on your system. Do you want to uninstall it first?', mbConfirmation, MB_YESNO) = IDYES then begin
      sUnInstallParams := '/SILENT /NORESTART /SUPPRESSMSGBOXES';
      if Exec(sUnInstallString, sUnInstallParams, '', SW_SHOW, ewWaitUntilTerminated, V) then begin
        Result := True;
      end else begin
        MsgBox('Failed to uninstall the previous version. Installation will abort.', mbError, MB_OK);
        Result := False;
      end;
    end;
  end;
end;

procedure AddToPath(PathToAdd: String; IsSystem: Boolean);
var
  OldPath: String;
  NewPath: String;
  RootKey: Integer;
  SubKey: String;
begin
  if IsSystem then
  begin
    RootKey := HKEY_LOCAL_MACHINE;
    SubKey := 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
  end
  else
  begin
    RootKey := HKEY_CURRENT_USER;
    SubKey := 'Environment';
  end;

  if RegQueryStringValue(RootKey, SubKey, 'Path', OldPath) then
  begin
    if Pos(PathToAdd, OldPath) = 0 then
    begin
      if (OldPath <> '') and (OldPath[Length(OldPath)] <> ';') then
        NewPath := OldPath + ';' + PathToAdd
      else
        NewPath := OldPath + PathToAdd;
      
      RegWriteExpandStringValue(RootKey, SubKey, 'Path', NewPath);
    end;
  end
  else
  begin
    RegWriteExpandStringValue(RootKey, SubKey, 'Path', PathToAdd);
  end;
end;

procedure RemoveFromPath(PathToRemove: String; IsSystem: Boolean);
var
  OldPath: String;
  NewPath: String;
  RootKey: Integer;
  SubKey: String;
  P: Integer;
begin
  if IsSystem then
  begin
    RootKey := HKEY_LOCAL_MACHINE;
    SubKey := 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
  end
  else
  begin
    RootKey := HKEY_CURRENT_USER;
    SubKey := 'Environment';
  end;

  if RegQueryStringValue(RootKey, SubKey, 'Path', OldPath) then
  begin
    P := Pos(PathToRemove, OldPath);
    if P > 0 then
    begin
      NewPath := OldPath;
      Delete(NewPath, P, Length(PathToRemove));
      StringChangeEx(NewPath, ';;', ';', True);
      if (Length(NewPath) > 0) and (NewPath[1] = ';') then
        Delete(NewPath, 1, 1);
      if (Length(NewPath) > 0) and (NewPath[Length(NewPath)] = ';') then
        Delete(NewPath, Length(NewPath), 1);
        
      RegWriteExpandStringValue(RootKey, SubKey, 'Path', NewPath);
    end;
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  BinPath: String;
  lpdwResult: Longint;
begin
  if CurStep = ssPostInstall then
  begin
    BinPath := ExpandConstant('{{app}}\bin');
    
    if WizardIsTaskSelected('userpath') then
    begin
      AddToPath(BinPath, False);
    end;
    
    if WizardIsTaskSelected('systempath') then
    begin
      AddToPath(BinPath, True);
    end;

    SendMessageTimeout(HWND_BROADCAST, WM_SETTINGCHANGE, 0, 'Environment', SMTO_ABORTIFHUNG, 5000, lpdwResult);
    SHChangeNotify($08000000, 0, 0, 0); // $08000000 = SHCNE_ASSOCCHANGED
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  BinPath: String;
  lpdwResult: Longint;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    BinPath := ExpandConstant('{{app}}\bin');
    RemoveFromPath(BinPath, False);
    RemoveFromPath(BinPath, True);

    SendMessageTimeout(HWND_BROADCAST, WM_SETTINGCHANGE, 0, 'Environment', SMTO_ABORTIFHUNG, 5000, lpdwResult);
    SHChangeNotify($08000000, 0, 0, 0);
  end;
end;

function InitializeUninstall(): Boolean;
begin
  Result := True;
  if MsgBox('Do you want to keep your custom TechScript projects and examples in the installation directory?', mbConfirmation, MB_YESNO) = idYes then
  begin
    // User-created files are kept by default
  end;
  
  if MsgBox('Do you want to clear your global TechScript compiler cache (~/.techscript)?', mbConfirmation, MB_YESNO) = idYes then
  begin
    DelTree(ExpandConstant('{{%USERPROFILE}}\.techscript'), True, True, True);
  end;
end;
"#,
        version = version,
        logo_dir = root
            .join("assets")
            .join("branding")
            .join("logo-package")
            .to_string_lossy(),
        portable_dir = portable_relative.to_string_lossy(),
        root_dir = root.to_string_lossy()
    );

    fs::write(dest, iss_content)?;
    Ok(())
}

fn compile_inno_installer(iss_path: &Path) -> anyhow::Result<()> {
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
            let generated_setup = iss_path.parent().unwrap().join("TechScript_Setup.exe");
            let root_dest = iss_path
                .parent()
                .unwrap()
                .join("..")
                .join("TechScript_Setup.exe");

            if generated_setup.exists() {
                fs::copy(&generated_setup, &root_dest)?;
                println!("Successfully created TechScript_Setup.exe");
            }
        } else {
            println!("Warning: iscc compilation failed.");
        }
    } else {
        println!("Warning: Inno Setup compiler (iscc.exe) was not found in PATH or standard Program Files locations.");
        println!(
            "Please compile {} manually using Inno Setup Compiler.",
            iss_path.display()
        );
        let mock_exe_path = iss_path
            .parent()
            .unwrap()
            .join("..")
            .join("installer")
            .join("TechScript_Setup.exe");
        let mock_root_path = iss_path
            .parent()
            .unwrap()
            .join("..")
            .join("TechScript_Setup.exe");
        let mock_content = "TechScript setup installer placeholder executable (Requires ISCC to build fully).\n";
        fs::write(&mock_exe_path, mock_content)?;
        fs::write(&mock_root_path, mock_content)?;
    }
    Ok(())
}

fn generate_testing_readme(dest: &Path) -> anyhow::Result<()> {
    let readme_content = r#"# TechScript 2.0 Manual Testing Guide

This directory contains the Developer Debug Release of the TechScript 2.0 language environment.

## Verification & Installation Methods

### 1. Verification of Compiler (No Installation Needed)
If you just want to run the compiler directly from this package:
1. Open PowerShell or Command Prompt in this folder.
2. Verify system checks by running:
   ```powershell
   .\compiler\tsc.exe doctor
   ```
3. Run the hello world example directly:
   ```powershell
   .\compiler\tsc.exe run examples/hello_world/main.txs
   ```

### 2. Using the Portable Version (Manual Environment Setup)
1. Extract `portable/TechScript_Portable.zip` to a directory of your choice (e.g. `C:\TechScript`).
2. Open PowerShell or Command Prompt.
3. Run environment checks:
   ```powershell
   .\bin\tsc.exe doctor
   ```

### 3. Using the Setup Installer (Automatic Environment Setup)
1. Double-click `installer/TechScript_Setup.exe` to run the setup wizard.
2. Follow the wizard steps to install to your PC. This registers environment paths automatically.
3. Open a new PowerShell terminal and verify:
   ```powershell
   tsc version
   tsc doctor
   ```

---

## Testing VS Code Extension
1. Install `vscode/techscript.vsix` directly in VS Code:
   - Open VS Code.
   - Run Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`) and choose `Extensions: Install from VSIX...`
   - Select the `vscode/techscript.vsix` file in this package.
2. Open any `.txs` file. Verify syntax highlights and completion features work.

---

## Testing Language Examples
We have included a set of language examples in the `examples/` directory. You can run these using the compiler:

Run using the compiler directly from the release package:
```powershell
# 1. Hello World
.\compiler\tsc.exe run examples/hello_world/main.txs

# 2. Variables & Mutability
.\compiler\tsc.exe run examples/variables/main.txs

# 3. Custom Functions
.\compiler\tsc.exe run examples/functions/main.txs

# 4. Recursion (Fibonacci)
.\compiler\tsc.exe run examples/recursion/main.txs

# 5. Closures & Scopes
.\compiler\tsc.exe run examples/closures/main.txs

# 6. Collections (Lists)
.\compiler\tsc.exe run examples/collections/main.txs

# 7. Struct Models
.\compiler\tsc.exe run examples/structs/main.txs

# 8. Loop Constructs
.\compiler\tsc.exe run examples/loops/main.txs

# 9. Shadowing & Scoping
.\compiler\tsc.exe run examples/models/main.txs
```

If you installed TechScript using the setup wizard (Method 3), you can run them simply from anywhere:
```powershell
tsc run examples/hello_world/main.txs
```

---

## Project Templates
You can also test project initialization:
1. Create a new console project:
   ```powershell
   tsc new my_project --template console
   ```
2. Navigate into the project and execute it:
   ```powershell
   cd my_project
   tsc run src/main.txs
   ```
"#;
    fs::write(dest, readme_content)?;
    Ok(())
}

fn generate_checksums(release_dir: &Path) -> anyhow::Result<()> {
    println!("Calculating SHA-256 checksums...");
    let checksum_file = release_dir.join("SHA256SUMS.txt");
    let mut out = File::create(&checksum_file)?;

    let files_to_hash = [
        release_dir.join("TechScript_Portable.zip"),
        release_dir.join("TechScript_Setup.exe"),
        release_dir.join("techscript.vsix"),
    ];

    for file_path in &files_to_hash {
        if file_path.exists() {
            let mut file = File::open(file_path)?;
            let mut hasher = Sha256::new();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            hasher.update(&buffer);
            let hash = hasher.finalize();
            let filename = file_path.file_name().unwrap().to_string_lossy();
            writeln!(out, "{}  {}", hex::encode(hash), filename)?;
        }
    }

    // Copy to checksums/
    fs::create_dir_all(release_dir.join("checksums"))?;
    fs::copy(&checksum_file, release_dir.join("checksums").join("SHA256SUMS.txt"))?;
    Ok(())
}
