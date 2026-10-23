// =============================================================================
// TechScript AppForge — Native Application Engine & State Machine
// runtime/android_runtime/src/txs_runtime.rs
//
// Authoritative state machine for TechScript AppForge:
//   - 5 Primary Destinations: Home, Explorer, Labs, Projects, Docs
//   - Secondary Destinations: Diagnostics, Settings, About
//   - Persistent navigation with back stack preservation
//   - 3D Procedural Engine state (Mesh, Material, Camera, Shaders)
//   - Interactive Explorer gallery controls & live metrics
//   - Real TechScript ecosystem projects with syntax inspector
//   - Verified TechScript architecture documentation
// =============================================================================

use crate::ui_model::AppSpec;
use crate::geometry3d::{MeshType, MaterialPreset};

#[derive(Clone, Debug)]
pub struct ProjectEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub path: &'static str,
    pub description: &'static str,
    pub code_sample: &'static str,
    pub lines: usize,
    pub category: &'static str,
}

#[derive(Clone, Debug)]
pub struct DocTopic {
    pub title: &'static str,
    pub category: &'static str,
    pub content: &'static str,
    pub code_snippet: Option<&'static str>,
}

pub struct TechScriptRuntime {
    spec: AppSpec,
    active_screen: String,
    nav_history: Vec<String>,
    theme_mode: String,
    is_dark: bool,
    initialized: bool,

    // ── 3D Engine Interactive State ──────────────────────────────────────────
    pub mesh_type: MeshType,
    pub material: MaterialPreset,
    pub rot_x: f32,
    pub rot_y: f32,
    pub zoom: f32,
    pub auto_rotate: bool,
    pub anim_speed: f32,

    // ── Explorer Gallery State ───────────────────────────────────────────────
    pub explorer_category: usize,
    pub demo_counter: i32,
    pub demo_toggle: bool,
    pub demo_slider: f32,
    pub explorer_viewing_code: bool,

    // ── Real Projects State ──────────────────────────────────────────────────
    pub projects: Vec<ProjectEntry>,
    pub selected_project: usize,
    pub show_source: bool,
    pub project_validated: bool,

    // ── Docs & Architecture State ────────────────────────────────────────────
    pub doc_topics: Vec<DocTopic>,
    pub selected_doc_topic: usize,

    // ── Diagnostics & Telemetry ──────────────────────────────────────────────
    pub frame_count: u64,
    pub scroll_y: f32,
}

impl TechScriptRuntime {
    pub fn new(spec: AppSpec) -> Self {
        let projects = vec![
            ProjectEntry {
                id: "hello_world",
                name: "Hello World",
                path: "examples/hello_world/hello.txs",
                description: "Canonical entry point: stdout interpolation and immutable variables.",
                code_sample: "# TechScript Canonical Hello World\nsay \"Hello, TechScript Native World!\"\nname = \"Developer\"\nsay $\"Welcome to AppForge, {name}!\"",
                lines: 4,
                category: "Basics",
            },
            ProjectEntry {
                id: "calculator",
                name: "CLI Calculator",
                path: "examples/calculator/calculator.txs",
                description: "Arithmetic expressions, function definitions, and control flow.",
                code_sample: "do add(a, b) -> a + b\ndo sub(a, b) -> a - b\nx = 42\ny = 18\nsay $\"Sum: {add(x, y)}\"\nsay $\"Diff: {sub(x, y)}\"",
                lines: 6,
                category: "Logic",
            },
            ProjectEntry {
                id: "todo_cli",
                name: "Todo CLI",
                path: "examples/todo_cli/todo.txs",
                description: "Task collections, iteration, and state management in TechScript.",
                code_sample: "tasks = [\"Build Runtime\", \"Design UI\", \"Native Release\"]\nfor task in tasks:\n    say $\"● {task}\"\nend",
                lines: 4,
                category: "Collections",
            },
            ProjectEntry {
                id: "canvas_draw",
                name: "Canvas Draw",
                path: "examples/canvas/draw.txs",
                description: "Procedural graphics rendering and frame loop execution.",
                code_sample: "model Canvas:\n    do render_frame(time_ms)\n        say $\"Draw frame at {time_ms}ms\"\n    end\nend",
                lines: 5,
                category: "Graphics",
            },
            ProjectEntry {
                id: "json_parser",
                name: "JSON Parser",
                path: "examples/json_parser/parser.txs",
                description: "Zero-dependency tokenizer and abstract syntax tree evaluator.",
                code_sample: "payload = '{\"engine\": \"TechScript\", \"version\": 2.0}'\nparsed = parse_json(payload)\nsay $\"Engine: {parsed.engine}\"",
                lines: 3,
                category: "Data",
            },
            ProjectEntry {
                id: "web_api",
                name: "Web API",
                path: "examples/web_api/web_api.txs",
                description: "HTTP endpoint handler and networking integration for Webforge.",
                code_sample: "endpoint \"/api/status\":\n    respond_json({\"status\": \"ok\", \"runtime\": \"Rust NDK\"})\nend",
                lines: 3,
                category: "Network",
            },
            ProjectEntry {
                id: "ai_prompt",
                name: "AI Prompt",
                path: "examples/ai/prompt.txs",
                description: "LLM prompt builder and token streaming pipeline.",
                code_sample: "prompt = \"Summarize TechScript architecture\"\nsay $\"Dispatching prompt: {prompt}\"",
                lines: 2,
                category: "AI",
            },
        ];

        let doc_topics = vec![
            DocTopic {
                title: "Getting Started",
                category: "Basics",
                content: "TechScript is an independent application language and toolchain.\nCompile and run directly with:\n  tsc run app.txs\nCreate and build Android AppForge apps with:\n  tsc app create my_app\n  tsc app build android",
                code_snippet: Some("say \"Welcome to TechScript!\"\nversion = 2.0\nsay $\"Running v{version}\""),
            },
            DocTopic {
                title: "AppForge Architecture",
                category: "Architecture",
                content: "AppForge compiles .txs apps into high-performance native binaries.\n• Zero Java/Kotlin: Android OS NativeActivity entry point\n• Hardware EGL 1.4 context on ANativeWindow\n• Hardware-accelerated GLES 2.0 liquid-glass SDF rendering\n• Rust core runtime: deterministic memory, 60 FPS loop",
                code_snippet: Some("# AppForge configuration\napp TechScriptApp:\n    name = \"AppForge Showcase\"\n    package = \"com.techscript.appforge\"\n    renderer = \"egl_gles2\"\nend"),
            },
            DocTopic {
                title: "3D Procedural Engine",
                category: "Graphics",
                content: "Hardware-accelerated 3D geometry engine built into runtime:\n• 4 Procedural 3D Meshes: Crystal, Octahedron, HexPrism, StarGem\n• Blinn-Phong specular lighting with Glass Fresnel refraction\n• Touch-controlled drag rotation, camera zoom, and material presets\n• 100% lightweight procedural vertices with zero 3D model bloat",
                code_snippet: Some("mesh = Procedural3D.create(\"crystal\")\nmesh.set_material(\"indigo_prism\")\nmesh.rotate_y(0.015)"),
            },
            DocTopic {
                title: "Webforge Integration",
                category: "Ecosystem",
                content: "TechScript unifies cross-platform native and web development:\n• Webforge translates .txs into high-speed WebAssembly (Wasm)\n• Shared data models, validation logic, and AST between mobile & web\n• Unified CLI compiler for desktop, web, and mobile targets",
                code_snippet: Some("export model User:\n    id: int\n    name: string\nend"),
            },
            DocTopic {
                title: "Language Syntax & Types",
                category: "Reference",
                content: "Clean, Python/Ruby-inspired expressive syntax:\n• Variables: dynamically or statically inferred (x = 10)\n• String interpolation: $\"Value is {x}\"\n• Functions: do fn(a, b) -> a + b\n• Iteration: for item in list: ... end\n• Models: model Name: ... end",
                code_snippet: Some("do calculate_tax(amount, rate) -> amount * rate\ntotal = calculate_tax(100.0, 0.18)\nsay $\"Tax: {total}\""),
            },
        ];

        Self {
            spec,
            active_screen: "home".into(),
            nav_history: vec!["home".into()],
            theme_mode: "dark".into(),
            is_dark: true,
            initialized: false,
            mesh_type: MeshType::Crystal,
            material: MaterialPreset::TechScriptIndigo,
            rot_x: 0.25,
            rot_y: 0.0,
            zoom: 1.0,
            auto_rotate: true,
            anim_speed: 1.0,
            explorer_category: 0,
            demo_counter: 42,
            demo_toggle: true,
            demo_slider: 0.65,
            explorer_viewing_code: false,
            projects,
            selected_project: 0,
            show_source: false,
            project_validated: false,
            doc_topics,
            selected_doc_topic: 0,
            frame_count: 0,
            scroll_y: 0.0,
        }
    }

    pub fn spec(&self) -> &AppSpec {
        &self.spec
    }

    pub fn active_screen(&self) -> &str {
        &self.active_screen
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }

    pub fn theme_mode(&self) -> &str {
        &self.theme_mode
    }

    // ── Android Lifecycle Callbacks ─────────────────────────────────────────

    pub fn on_create(&mut self) {
        log::info!("[TechScript .txs] NativeActivity.onCreate() — runtime active");
        self.initialized = true;
    }

    pub fn on_resume(&mut self) {
        log::info!("[TechScript .txs] NativeActivity.onResume() — focus active");
    }

    pub fn on_pause(&mut self) {
        log::info!("[TechScript .txs] NativeActivity.onPause() — state preserved");
    }

    pub fn on_destroy(&mut self) {
        log::info!("[TechScript .txs] NativeActivity.onDestroy() — runtime exit");
        self.initialized = false;
    }

    // ── Navigation Engine ───────────────────────────────────────────────────

    pub fn navigate(&mut self, screen_id: &str) -> bool {
        let valid_screens = ["home", "explorer", "labs", "projects", "docs", "about"];
        if valid_screens.contains(&screen_id) {
            log::info!("[TechScript .txs] navigate(\"{screen_id}\")");
            if self.active_screen != screen_id {
                self.nav_history.push(self.active_screen.clone());
                self.active_screen = screen_id.to_string();
                self.scroll_y = 0.0;
                self.show_source = false;
                self.explorer_viewing_code = false;
            }
            true
        } else {
            log::warn!("[TechScript .txs] navigate: unknown destination '{screen_id}'");
            false
        }
    }

    pub fn on_back_pressed(&mut self) -> Option<String> {
        if self.show_source {
            self.show_source = false;
            Some(self.active_screen.clone())
        } else if self.explorer_viewing_code {
            self.explorer_viewing_code = false;
            Some(self.active_screen.clone())
        } else if let Some(prev) = self.nav_history.pop() {
            log::info!("[TechScript .txs] onBackPressed — returning to '{prev}'");
            self.active_screen = prev.clone();
            self.scroll_y = 0.0;
            Some(prev)
        } else if self.active_screen != "home" {
            log::info!("[TechScript .txs] onBackPressed — fallback to home");
            self.active_screen = "home".into();
            self.scroll_y = 0.0;
            Some("home".into())
        } else {
            log::info!("[TechScript .txs] onBackPressed at Home — exiting");
            None
        }
    }

    // ── Theme Controls ───────────────────────────────────────────────────────

    pub fn cycle_theme(&mut self) {
        self.theme_mode = match self.theme_mode.as_str() {
            "dark"  => "light".into(),
            "light" => "system".into(),
            _       => "dark".into(),
        };
        self.is_dark = match self.theme_mode.as_str() {
            "light" => false,
            _       => true,
        };
        log::info!("[TechScript .txs] Theme changed: {}", self.theme_mode);
    }

    // ── 3D Engine Controls ───────────────────────────────────────────────────

    pub fn cycle_mesh(&mut self) {
        self.mesh_type = self.mesh_type.cycle();
        log::info!("[TechScript 3D] Mesh: {}", self.mesh_type.name());
    }

    pub fn cycle_material(&mut self) {
        self.material = self.material.cycle();
        log::info!("[TechScript 3D] Material: {}", self.material.name());
    }

    pub fn toggle_auto_rotate(&mut self) {
        self.auto_rotate = !self.auto_rotate;
    }

    pub fn reset_camera(&mut self) {
        self.rot_x = 0.25;
        self.rot_y = 0.0;
        self.zoom = 1.0;
        self.auto_rotate = true;
    }

    pub fn adjust_zoom(&mut self, delta: f32) {
        self.zoom = (self.zoom + delta).clamp(0.6, 2.2);
    }

    pub fn toggle_speed(&mut self) {
        self.anim_speed = if self.anim_speed >= 1.9 { 1.0 } else { 2.0 };
    }

    // ── Explorer Gallery Controls ───────────────────────────────────────────

    pub fn select_explorer_category(&mut self, cat: usize) {
        self.explorer_category = cat;
        self.explorer_viewing_code = false;
    }

    pub fn increment_counter(&mut self) {
        self.demo_counter += 1;
    }

    pub fn toggle_switch(&mut self) {
        self.demo_toggle = !self.demo_toggle;
    }

    pub fn toggle_explorer_code(&mut self) {
        self.explorer_viewing_code = !self.explorer_viewing_code;
    }

    // ── Projects Controls ───────────────────────────────────────────────────

    pub fn select_project(&mut self, idx: usize) {
        if idx < self.projects.len() {
            self.selected_project = idx;
            self.show_source = true;
            self.project_validated = false;
        }
    }

    pub fn toggle_source_view(&mut self) {
        self.show_source = !self.show_source;
        self.project_validated = false;
    }

    pub fn validate_selected_project(&mut self) {
        self.project_validated = true;
    }

    // ── Docs Controls ───────────────────────────────────────────────────────

    pub fn select_doc_topic(&mut self, idx: usize) {
        if idx < self.doc_topics.len() {
            self.selected_doc_topic = idx;
        }
    }
}
