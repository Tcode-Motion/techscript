//! # TechScript Compiler Driver — Project Templates
//!
//! Provides scaffolding for console, library, package, workspace, and empty templates.

use std::path::{Path, PathBuf};

pub enum ProjectTemplate {
    Console,   // Simple executable package
    Library,   // Standard library package
    Package,   // Complete package layout with tech.toml + src/ + tests/
    Workspace, // Workspace tech.toml + packages/ structure
    Empty,     // Bare minimum setup
}

impl ProjectTemplate {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "console" => Some(Self::Console),
            "library" | "lib" => Some(Self::Library),
            "package" | "pkg" => Some(Self::Package),
            "workspace" | "ws" => Some(Self::Workspace),
            "empty" => Some(Self::Empty),
            _ => None,
        }
    }

    /// Scaffolds files for the chosen template.
    pub fn scaffold(&self, name: &str, dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
        std::fs::create_dir_all(dir)?;
        let mut created_paths = Vec::new();

        match self {
            Self::Console => {
                let toml_path = dir.join("tech.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let src_dir = dir.join("src");
                std::fs::create_dir_all(&src_dir)?;
                let main_path = src_dir.join("main.txs");
                let main_content = "import std.io;\n\nfunction main() {\n    std.io.print(\"Hello, TechScript!\");\n}\n\nmain();\n";
                std::fs::write(&main_path, main_content)?;
                created_paths.push(main_path);
            }
            Self::Library => {
                let toml_path = dir.join("tech.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"src/lib.txs\"\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let src_dir = dir.join("src");
                std::fs::create_dir_all(&src_dir)?;
                let lib_path = src_dir.join("lib.txs");
                let lib_content = "export function add(a, b) {\n    return a + b;\n}\n";
                std::fs::write(&lib_path, lib_content)?;
                created_paths.push(lib_path);
            }
            Self::Package => {
                let toml_path = dir.join("tech.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\n\n[dependencies]\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let src_dir = dir.join("src");
                std::fs::create_dir_all(&src_dir)?;
                let main_path = src_dir.join("main.txs");
                let main_content = "import std.io;\n\nfunction main() {\n    std.io.print(\"Initializing package...\");\n}\n\nmain();\n";
                std::fs::write(&main_path, main_content)?;
                created_paths.push(main_path);

                let tests_dir = dir.join("tests");
                std::fs::create_dir_all(&tests_dir)?;
                let test_path = tests_dir.join("unit_test.txs");
                let test_content = "import std.io;\n\nfunction test_first() {\n    std.io.print(\"Test passed!\");\n}\n\ntest_first();\n";
                std::fs::write(&test_path, test_content)?;
                created_paths.push(test_path);
            }
            Self::Workspace => {
                let toml_path = dir.join("tech.toml");
                let toml_content =
                    "[workspace]\nmembers = [\n    \"packages/core\",\n    \"packages/app\"\n]\n";
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let pkgs_dir = dir.join("packages");
                std::fs::create_dir_all(pkgs_dir.join("core"))?;
                std::fs::create_dir_all(pkgs_dir.join("app"))?;

                let core_toml = pkgs_dir.join("core").join("tech.toml");
                std::fs::write(
                    &core_toml,
                    "[package]\nname = \"core\"\nversion = \"0.1.0\"\nentry = \"src/lib.txs\"\n",
                )?;
                created_paths.push(core_toml);

                let app_toml = pkgs_dir.join("app").join("tech.toml");
                std::fs::write(
                    &app_toml,
                    "[package]\nname = \"app\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\n",
                )?;
                created_paths.push(app_toml);
            }
            Self::Empty => {
                let toml_path = dir.join("tech.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"main.txs\"\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let main_path = dir.join("main.txs");
                std::fs::write(&main_path, "\n")?;
                created_paths.push(main_path);
            }
        }

        Ok(created_paths)
    }
}
