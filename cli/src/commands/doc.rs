//! # tsc doc Command
//!
//! Generates HTML and Markdown documentation by extracting triple-slash comments.

use crate::artifacts::ArtifactManager;
use crate::exit_code::ExitCode;
use std::path::{Path, PathBuf};
use techscript_package_manager::DocExtractor;

pub fn execute(path_str: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let path = if let Some(p) = path_str {
        PathBuf::from(p)
    } else {
        // Look for manifest entry file
        let manifest_path = current_dir.join("tech.toml");
        if manifest_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                if let Ok(manifest) =
                    toml::from_str::<techscript_package_manager::Manifest>(&content)
                {
                    current_dir.join(&manifest.package.entry)
                } else {
                    current_dir.join("src/main.txs")
                }
            } else {
                current_dir.join("src/main.txs")
            }
        } else {
            current_dir.join("src/main.txs")
        }
    };

    if !path.exists() {
        eprintln!("Error: Path does not exist: {:?}", path);
        return ExitCode::IoError;
    }

    println!("Generating API documentation for: {:?}", path);

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return ExitCode::IoError;
        }
    };

    let doc_items = DocExtractor::extract_comments(&content);
    if doc_items.is_empty() {
        println!("No triple-slash (///) comments found.");
        return ExitCode::Success;
    }

    // Build structured output
    let mut md_content = String::new();
    md_content.push_str(&format!(
        "# API Documentation — {}\n\n",
        path.file_name().unwrap_or_default().to_string_lossy()
    ));

    for item in &doc_items {
        md_content.push_str(&format!("## {}\n\n", item.name));
        md_content.push_str(&format!("{}\n\n", item.doc));
        md_content.push_str("---\n\n");
    }

    let artifacts = ArtifactManager::new(&current_dir);
    if let Err(e) = artifacts.prepare() {
        eprintln!("Error preparing build directory: {}", e);
        return ExitCode::IoError;
    }

    let name = path.file_stem().unwrap_or_default().to_string_lossy();

    // Save Markdown doc file
    let md_path = artifacts
        .build_dir
        .join("docs")
        .join(format!("{}.md", name));
    if let Err(e) = std::fs::write(&md_path, &md_content) {
        eprintln!("Error writing documentation file: {}", e);
        return ExitCode::IoError;
    }

    // Save HTML doc file
    let html_content = format!(
        "<!DOCTYPE html><html><head><title>API Docs</title><style>body{{font-family:sans-serif;margin:40px;}}code{{background:#f4f4f4;padding:2px 4px;}}</style></head><body>{}</body></html>",
        md_content.replace("\n", "<br>")
    );
    if let Err(e) = artifacts.write_docs(&name, &html_content) {
        eprintln!("Error writing documentation file: {}", e);
        return ExitCode::IoError;
    }

    println!("Documentation generated successfully in build/docs/");
    ExitCode::Success
}
