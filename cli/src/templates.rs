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
    Website,        // Basic WebForge website
    FullStack,      // Full-stack WebForge application with pages, server, scenes
    Api,            // WebForge REST API service
    Scene3D,        // WebForge 3D WebGL website
}

impl ProjectTemplate {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "website" | "webforge" | "site" | "web" => Some(Self::Website),
            "fullstack" | "full-stack" | "app" => Some(Self::FullStack),
            "api" | "rest_api" | "rest-api" | "restapi" => Some(Self::Api),
            "3d" | "scene3d" | "scene-3d" | "webgl" => Some(Self::Scene3D),
            "console" => Some(Self::Console),
            "cli" => Some(Self::Cli),
            "library" | "lib" => Some(Self::Library),
            "workspace" | "ws" => Some(Self::Workspace),
            "http_server" | "http-server" | "httpserver" => Some(Self::HttpServer),
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
                let toml_path = dir.join("techscript.toml");
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
            Self::Website => {
                let toml_path = dir.join("webforge.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"app.txs\"\ncapabilities = [\"Network\", \"FileSystem\"]\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let app_path = dir.join("app.txs");
                let app_content = format!(
                    "make a website called {}\n\nheader:\n    title is \"{}\"\n    nav:\n        link:\n            label is \"Home\"\n            url is \"/\"\n        link:\n            label is \"About\"\n            url is \"/about\"\n        link:\n            label is \"Docs\"\n            url is \"https://is-a.dev/techscript/\"\n\nfooter:\n    section:\n        title is \"{}\"\n        link:\n            label is \"Home\"\n            url is \"/\"\n        link:\n            label is \"About\"\n            url is \"/about\"\n    section:\n        title is \"TechScript\"\n        link:\n            label is \"Documentation\"\n            url is \"https://is-a.dev/techscript/\"\n        link:\n            label is \"GitHub Repository\"\n            url is \"https://github.com/Tcode-Motion/techscript\"\n\nstart website\n",
                    name, name, name
                );
                std::fs::write(&app_path, app_content)?;
                created_paths.push(app_path);

                let pages_dir = dir.join("pages");
                std::fs::create_dir_all(&pages_dir)?;

                let home_path = pages_dir.join("home.txs");
                let home_content = format!(
                    "page Home \"/\":\n    title \"Welcome to {}\"\n    text \"A clean, modern website built with TechScript & WebForge.\"\n    text \"Edit pages/home.txs or app.txs to start customizing your application.\"\n    button \"Explore Documentation\" -> open \"https://is-a.dev/techscript/\"\n    button \"About This Site\" -> open About\n",
                    name
                );
                std::fs::write(&home_path, home_content)?;
                created_paths.push(home_path);

                let about_path = pages_dir.join("about.txs");
                let about_content = format!(
                    "page About \"/about\":\n    title \"About {}\"\n    text \"This project is powered by TechScript 2.0 and WebForge.\"\n    text \"TechScript provides a readable, full-stack programming experience with native compilation and zero-config servers.\"\n    button \"Back to Home\" -> open Home\n",
                    name
                );
                std::fs::write(&about_path, about_content)?;
                created_paths.push(about_path);

                let assets_dir = dir.join("assets");
                std::fs::create_dir_all(assets_dir.join("images"))?;
                std::fs::create_dir_all(assets_dir.join("models"))?;

                let readme_path = dir.join("README.md");
                let readme_content = format!(
                    "# {}\n\nA clean, modern website built with **TechScript & WebForge**.\n\n## Project Structure\n```text\n├── webforge.toml       # Project configuration\n├── app.txs             # Application entry (header, navigation bar, footer)\n├── pages/\n│   ├── home.txs        # Welcome / home page\n│   └── about.txs       # About page\n├── assets/\n│   ├── images/         # Images\n│   └── models/         # 3D models\n└── README.md\n```\n\n## Getting Started\n\nStart live development server:\n```bash\ntsc dev\n```\nVisit `http://localhost:3000` in your browser.\n\n## Production Build\n```bash\ntsc build --target web\n```\n",
                    name
                );
                std::fs::write(&readme_path, readme_content)?;
                created_paths.push(readme_path);
            }
            Self::FullStack => {
                let toml_path = dir.join("webforge.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"app.txs\"\ncapabilities = [\"Network\", \"FileSystem\"]\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let app_path = dir.join("app.txs");
                let app_content = format!("site \"{}\"\n", name);
                std::fs::write(&app_path, app_content)?;
                created_paths.push(app_path);

                let pages_dir = dir.join("pages");
                let server_dir = dir.join("server");
                let scenes_dir = dir.join("scenes");
                let assets_dir = dir.join("assets");
                let tests_dir = dir.join("tests");

                std::fs::create_dir_all(&pages_dir)?;
                std::fs::create_dir_all(&server_dir)?;
                std::fs::create_dir_all(&scenes_dir)?;
                std::fs::create_dir_all(assets_dir.join("images"))?;
                std::fs::create_dir_all(assets_dir.join("models"))?;
                std::fs::create_dir_all(&tests_dir)?;

                // 1. Home Page (.txs)
                let page_path = pages_dir.join("home.txs");
                let page_content = format!(
                    "page Home \"/\":\n    title \"Welcome to {}\"\n    text \"Modular full-stack architecture built with TechScript + WebForge\"\n    input \"Add an item\" called ItemInput\n    button \"Save Item\" called SaveBtn\n\nwhen SaveBtn is clicked:\n    send ItemInput to /api/items\n    show message \"Item successfully added!\"\n",
                    name
                );
                std::fs::write(&page_path, page_content)?;
                created_paths.push(page_path);

                // 2. Server Endpoints & Database (.txs)
                let srv_path = server_dir.join("api.txs");
                let srv_content = "data AppData:\n    table items:\n        title is text\n        content is text\n";
                std::fs::write(&srv_path, srv_content)?;
                created_paths.push(srv_path);

                // 3. 3D Scene Viewport (.txs)
                let scene_path = scenes_dir.join("world.txs");
                let scene_content = "scene World:\n    camera\n    light\n    cube Box:\n        color \"#6366F1\"\n        position (0, 0, 0)\n        size 2\n        rotate auto\n";
                std::fs::write(&scene_path, scene_content)?;
                created_paths.push(scene_path);

                // 4. Test suite (.txs)
                let test_path = tests_dir.join("app_test.txs");
                let test_content = "import std.testing;\n\nfunction test_app_loads() {\n    std.testing.assert_eq(1 + 1, 2, \"app test initializes\");\n}\n\ntest_app_loads();\n";
                std::fs::write(&test_path, test_content)?;
                created_paths.push(test_path);

                // 5. README
                let readme_path = dir.join("README.md");
                let readme_content = format!(
                    "# {}\n\nFull-stack application built with **TechScript + WebForge**.\n\n## Project Structure\n```text\n├── webforge.toml       # Canonical WebForge configuration\n├── app.txs             # Application entry\n├── pages/\n│   └── home.txs        # Interactive UI pages\n├── server/\n│   └── api.txs         # Backend APIs & SQLite tables\n├── scenes/\n│   └── world.txs       # Three.js 3D WebGL viewport\n├── assets/             # Static assets and 3D models\n└── tests/\n    └── app_test.txs    # Integration tests\n```\n\n## Getting Started\n\n```bash\n# Start development server\nwebforge dev\n\n# Inspect application routes\nwebforge routes\n\n# Compile production bundle into dist/\nwebforge build web --release\n```\n",
                    name
                );
                std::fs::write(&readme_path, readme_content)?;
                created_paths.push(readme_path);
            }
            Self::Api => {
                let toml_path = dir.join("webforge.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"app.txs\"\ncapabilities = [\"Network\", \"FileSystem\"]\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let app_path = dir.join("app.txs");
                let app_content = format!(
                    "site \"{}\"\n\ndata Records:\n    table items:\n        title is text\n        content is text\n",
                    name
                );
                std::fs::write(&app_path, app_content)?;
                created_paths.push(app_path);

                let readme_path = dir.join("README.md");
                let readme_content = format!(
                    "# {}\n\nREST API service built with **TechScript + WebForge**.\n\n## Getting Started\n\nRun server:\n```bash\nwebforge dev\n```\n\n## Endpoints\n- `GET  /api/items` - Fetch rows from SQLite\n- `POST /api/items` - Insert row into SQLite\n",
                    name
                );
                std::fs::write(&readme_path, readme_content)?;
                created_paths.push(readme_path);
            }
            Self::Scene3D => {
                let toml_path = dir.join("webforge.toml");
                let toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nentry = \"app.txs\"\ncapabilities = [\"Network\"]\n",
                    name
                );
                std::fs::write(&toml_path, toml_content)?;
                created_paths.push(toml_path);

                let app_path = dir.join("app.txs");
                let app_content = format!(
                    "site \"{}\"\n\npage Home \"/\":\n    title \"Interactive 3D Experience\"\n    text \"Powered by TechScript + WebForge & Three.js WebGL Engine\"\n\nscene Showcase:\n    camera\n    light\n    cube Box:\n        color \"#06B6D4\"\n        rotate auto\n",
                    name
                );
                std::fs::write(&app_path, app_content)?;
                created_paths.push(app_path);

                let readme_path = dir.join("README.md");
                let readme_content = format!(
                    "# {}\n\n3D WebGL website built with **TechScript + WebForge**.\n\n## Getting Started\n\nRun development server:\n```bash\nwebforge dev\n```\nVisit `http://localhost:3000` to interact with the 3D scene canvas.\n",
                    name
                );
                std::fs::write(&readme_path, readme_content)?;
                created_paths.push(readme_path);
            }
        }

        Ok(created_paths)
    }
}
