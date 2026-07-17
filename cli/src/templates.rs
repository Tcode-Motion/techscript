//! # TechScript Compiler Driver — Project Templates
//!
//! Provides scaffolding for console, library, package, workspace, CLI,
//! HTTP Server, REST API, Desktop App, Game, Minimal, and Testing templates.

use std::path::{Path, PathBuf};

pub enum ProjectTemplate {
    Console,        // Simple executable package
    Cli,            // Command-line tool package
    Library,        // Standard library package
    Workspace,      // Workspace tech.toml + packages/ structure
    HttpServer,     // HTTP Server using std.http
    RestApi,        // REST API using std.http + std.json
    DesktopApp,     // Simulated desktop app setup
    Game,           // Simple game CLI/runtime loop
    Package,        // Complete package layout with tech.toml + src/ + tests/
    Minimal,        // Bare minimum setup
    TestingProject, // Standard testing suite project
}

impl ProjectTemplate {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "console" => Some(Self::Console),
            "cli" => Some(Self::Cli),
            "library" | "lib" => Some(Self::Library),
            "workspace" | "ws" => Some(Self::Workspace),
            "http_server" | "http-server" | "httpserver" | "web" => Some(Self::HttpServer),
            "rest_api" | "rest-api" | "restapi" => Some(Self::RestApi),
            "desktop_app" | "desktop-app" | "desktop" | "gui" => Some(Self::DesktopApp),
            "game" => Some(Self::Game),
            "package" | "pkg" => Some(Self::Package),
            "minimal" | "empty" => Some(Self::Minimal),
            "testing_project" | "testing" | "test" => Some(Self::TestingProject),
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
            Self::Cli => {
                let toml_path = dir.join("tech.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\ncapabilities = [\"Environment\", \"Process\"]\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let src_dir = dir.join("src");
                std::fs::create_dir_all(&src_dir)?;
                let main_path = src_dir.join("main.txs");
                let main_content = "import std.io;\nimport std.env;\n\nfunction main() {\n    let args = std.env.args();\n    std.io.print(\"CLI arguments:\", args);\n}\n\nmain();\n";
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
            Self::HttpServer => {
                let toml_path = dir.join("tech.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\ncapabilities = [\"Network\"]\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let src_dir = dir.join("src");
                std::fs::create_dir_all(&src_dir)?;
                let main_path = src_dir.join("main.txs");
                let main_content = "import std.io;\nimport std.http;\n\nfunction main() {\n    std.io.print(\"Starting HTTP server on port 8080...\");\n    std.http.listen(8080, function(req) {\n        return \"HTTP Server Response\";\n    });\n}\n\nmain();\n";
                std::fs::write(&main_path, main_content)?;
                created_paths.push(main_path);
            }
            Self::RestApi => {
                let toml_path = dir.join("tech.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\ncapabilities = [\"Network\"]\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let src_dir = dir.join("src");
                std::fs::create_dir_all(&src_dir)?;
                let main_path = src_dir.join("main.txs");
                let main_content = "import std.io;\nimport std.http;\nimport std.json;\n\nfunction main() {\n    std.io.print(\"Starting REST API on port 8080...\");\n    std.http.listen(8080, function(req) {\n        let data = { \"status\": \"success\", \"data\": \"TechScript API v2\" };\n        return std.json.stringify(data);\n    });\n}\n\nmain();\n";
                std::fs::write(&main_path, main_content)?;
                created_paths.push(main_path);
            }
            Self::DesktopApp => {
                let toml_path = dir.join("tech.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"src/main.txs\"\ncapabilities = [\"FileSystem\"]\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let src_dir = dir.join("src");
                std::fs::create_dir_all(&src_dir)?;
                let main_path = src_dir.join("main.txs");
                let main_content = "import std.io;\nimport std.system;\n\nfunction main() {\n    std.io.print(\"Initializing App UI on OS:\", std.system.os());\n}\n\nmain();\n";
                std::fs::write(&main_path, main_content)?;
                created_paths.push(main_path);
            }
            Self::Game => {
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
                let main_content = "import std.io;\nimport std.random;\n\nfunction main() {\n    let secret = std.random.int(1, 100);\n    std.io.print(\"Guess the number game! Secret is generated.\");\n}\n\nmain();\n";
                std::fs::write(&main_path, main_content)?;
                created_paths.push(main_path);
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
            Self::Minimal => {
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
            Self::TestingProject => {
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
                std::fs::write(&main_path, "import std.io;\n\nfunction main() {\n    std.io.print(\"Testing project entry\");\n}\n\nmain();\n")?;
                created_paths.push(main_path);

                let tests_dir = dir.join("tests");
                std::fs::create_dir_all(&tests_dir)?;
                let test_path = tests_dir.join("assert_test.txs");
                let test_content = "import std.testing;\n\nfunction test_assertions() {\n    std.testing.assert_eq(1 + 1, 2, \"1+1 equals 2\");\n}\n\ntest_assertions();\n";
                std::fs::write(&test_path, test_content)?;
                created_paths.push(test_path);
            }
        }

        Ok(created_paths)
    }
}
