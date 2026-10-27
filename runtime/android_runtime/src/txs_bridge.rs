// =============================================================================
// TechScript AppForge — Native .txs Execution Bridge
// runtime/android_runtime/src/txs_bridge.rs
//
// Bridges the pure TechScript language engine (Lexer -> Parser -> Semantic ->
// Interpreter) to the Android native runtime.
//
// Executes real .txs scripts from Android assets:
//   - app.txs: build_appforge_ui()
//   - MainActivity.txs: lifecycle & state machine
//   - screens/home.txs: get_home_dashboard()
//   - screens/about.txs: get_about_metadata()
//   - screens/docs.txs: get_cli_commands()
//   - navigation.txs: get_navigation_routes()
//   - theme.txs: get_theme_tokens()
// =============================================================================

use techscript_interpreter::Interpreter;
use techscript_runtime::RuntimeValue;

#[derive(Clone, Debug)]
pub struct QuickAction {
    pub id: String,
    pub label: String,
    pub cmd: String,
    pub icon: String,
}

#[derive(Clone, Debug)]
pub struct MetricItem {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct CliCommandItem {
    pub cmd: String,
    pub desc: String,
}

#[derive(Clone, Debug)]
pub struct VerifiedLinkItem {
    pub label: String,
    pub url: String,
    pub link_type: String,
}

#[derive(Clone, Debug)]
pub struct HomeDashboardData {
    pub title: String,
    pub tagline: String,
    pub quick_actions: Vec<QuickAction>,
    pub metrics: Vec<MetricItem>,
}

#[derive(Clone, Debug)]
pub struct AboutMetadata {
    pub app_name: String,
    pub version: String,
    pub creator: String,
    pub github_identity: String,
    pub official_website: String,
    pub official_repository: String,
    pub developer_contact: String,
    pub license: String,
    pub verified_links: Vec<VerifiedLinkItem>,
}

pub struct TechScriptBridge {
    pub interpreter: Interpreter,
    pub loaded_scripts: Vec<String>,
    pub last_execution_output: String,
    pub script_error: Option<String>,
    pub home_data: HomeDashboardData,
    pub about_data: AboutMetadata,
    pub cli_commands: Vec<CliCommandItem>,
    pub is_txs_engine_active: bool,
}

impl Default for TechScriptBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl TechScriptBridge {
    pub fn new() -> Self {
        let interpreter = Interpreter::new();
        Self {
            interpreter,
            loaded_scripts: Vec::new(),
            last_execution_output: "TechScript v2.0 Native Interpreter Ready".into(),
            script_error: None,
            home_data: HomeDashboardData {
                title: "TechScript AppForge".into(),
                tagline: "The Official Mobile Developer Companion".into(),
                quick_actions: vec![
                    QuickAction {
                        id: "new_project".into(),
                        label: "New Project".into(),
                        cmd: "tsc app create <name>".into(),
                        icon: "plus".into(),
                    },
                    QuickAction {
                        id: "run_check".into(),
                        label: "Semantic Check".into(),
                        cmd: "tsc check src/main.txs".into(),
                        icon: "check".into(),
                    },
                    QuickAction {
                        id: "doctor".into(),
                        label: "Toolchain Doctor".into(),
                        cmd: "tsc app doctor".into(),
                        icon: "cpu".into(),
                    },
                ],
                metrics: vec![
                    MetricItem {
                        label: "Language".into(),
                        value: "TechScript 2.0".into(),
                    },
                    MetricItem {
                        label: "Runtime".into(),
                        value: "Rust NDK Core".into(),
                    },
                    MetricItem {
                        label: "UI Engine".into(),
                        value: "EGL / GLES 2.0".into(),
                    },
                    MetricItem {
                        label: "Bytecode".into(),
                        value: "Native VM / AST".into(),
                    },
                ],
            },
            about_data: AboutMetadata {
                app_name: "TechScript AppForge".into(),
                version: "2.0.0".into(),
                creator: "Tanmoy Majumder".into(),
                github_identity: "Tcode-Motion".into(),
                official_website: "https://techscript.is-a.dev/techscript/".into(),
                official_repository: "https://github.com/Tcode-Motion/techscript".into(),
                developer_contact: "majumdertanmoy510@gmail.com".into(),
                license: "Apache License 2.0".into(),
                verified_links: vec![
                    VerifiedLinkItem {
                        label: "Official Website".into(),
                        url: "https://techscript.is-a.dev/techscript/".into(),
                        link_type: "web".into(),
                    },
                    VerifiedLinkItem {
                        label: "GitHub Repository".into(),
                        url: "https://github.com/Tcode-Motion/techscript".into(),
                        link_type: "code".into(),
                    },
                    VerifiedLinkItem {
                        label: "Report Issue".into(),
                        url: "https://github.com/Tcode-Motion/techscript/issues".into(),
                        link_type: "bug".into(),
                    },
                    VerifiedLinkItem {
                        label: "Security Policy".into(),
                        url: "https://github.com/Tcode-Motion/techscript/blob/main/SECURITY.md"
                            .into(),
                        link_type: "shield".into(),
                    },
                ],
            },
            cli_commands: vec![
                CliCommandItem {
                    cmd: "tsc app create <name>".into(),
                    desc: "Scaffolds an AppForge mobile/desktop app".into(),
                },
                CliCommandItem {
                    cmd: "tsc app build android".into(),
                    desc: "Compiles native Android APK via NDK".into(),
                },
                CliCommandItem {
                    cmd: "tsc app run android".into(),
                    desc: "Installs and launches on attached device".into(),
                },
                CliCommandItem {
                    cmd: "tsc app doctor".into(),
                    desc: "Audits JDK, Android SDK, and toolchains".into(),
                },
                CliCommandItem {
                    cmd: "tsc check <file>".into(),
                    desc: "Performs semantic and type analysis".into(),
                },
                CliCommandItem {
                    cmd: "tsc run <file>".into(),
                    desc: "Executes .txs code through bytecode VM".into(),
                },
                CliCommandItem {
                    cmd: "tsc web dev".into(),
                    desc: "Runs Webforge Wasm dev server".into(),
                },
            ],
            is_txs_engine_active: false,
        }
    }

    /// Evaluates a raw TechScript source string.
    pub fn eval_source(&mut self, script_name: &str, source: &str) -> Result<RuntimeValue, String> {
        let mut reporter = techscript_errors::DiagnosticReporter::new();
        let tokens = techscript_lexer::lex(source, &mut reporter)
            .map_err(|e| format!("Lexer error in {script_name}: {e:?}"))?;

        let program = techscript_parser::parse(&tokens, &mut reporter)
            .map_err(|e| format!("Parser error in {script_name}: {e:?}"))?;

        let result = self
            .interpreter
            .interpret(&program)
            .map_err(|e| format!("Runtime error in {script_name}: {e:?}"))?;

        if !self.loaded_scripts.contains(&script_name.to_string()) {
            self.loaded_scripts.push(script_name.to_string());
        }
        self.is_txs_engine_active = true;
        log::info!("[TechScript Bridge] Successfully executed {script_name}");
        Ok(result)
    }

    /// Executes the real .txs scripts loaded from Android APK assets.
    pub fn load_and_execute_assets<F>(&mut self, mut asset_loader: F)
    where
        F: FnMut(&str) -> Option<String>,
    {
        log::info!("[TechScript Bridge] Initializing real .txs runtime from assets...");

        // 1. Load app.txs
        if let Some(src) = asset_loader("txs/app.txs") {
            match self.eval_source("txs/app.txs", &src) {
                Ok(val) => {
                    log::info!("[TechScript Bridge] app.txs executed successfully");
                    self.extract_app_data(&val);
                }
                Err(e) => {
                    log::warn!("[TechScript Bridge] app.txs execution failed: {e}");
                    self.script_error = Some(e);
                }
            }
        }

        // 2. Load theme.txs
        if let Some(src) = asset_loader("txs/theme.txs") {
            let _ = self.eval_source("txs/theme.txs", &src);
        }

        // 3. Load navigation.txs
        if let Some(src) = asset_loader("txs/navigation.txs") {
            let _ = self.eval_source("txs/navigation.txs", &src);
        }

        // 4. Load screens/home.txs
        if let Some(src) = asset_loader("txs/screens/home.txs") {
            match self.eval_source("txs/screens/home.txs", &src) {
                Ok(_) => {
                    self.invoke_home_dashboard();
                }
                Err(e) => {
                    log::warn!("[TechScript Bridge] home.txs error: {e}");
                    self.script_error = Some(e);
                }
            }
        }

        // 5. Load screens/about.txs
        if let Some(src) = asset_loader("txs/screens/about.txs") {
            match self.eval_source("txs/screens/about.txs", &src) {
                Ok(_) => {
                    self.invoke_about_metadata();
                }
                Err(e) => {
                    log::warn!("[TechScript Bridge] about.txs error: {e}");
                }
            }
        }

        // 6. Load screens/docs.txs
        if let Some(src) = asset_loader("txs/screens/docs.txs") {
            match self.eval_source("txs/screens/docs.txs", &src) {
                Ok(_) => {
                    self.invoke_docs_commands();
                }
                Err(e) => {
                    log::warn!("[TechScript Bridge] docs.txs error: {e}");
                }
            }
        }

        // 7. Load and execute MainActivity.txs
        if let Some(src) = asset_loader("txs/MainActivity.txs") {
            match self.eval_source("txs/MainActivity.txs", &src) {
                Ok(_) => {
                    log::info!("[TechScript Bridge] MainActivity.txs loaded & verified");
                    self.last_execution_output = "TechScript MainActivity initialized".into();
                }
                Err(e) => {
                    log::warn!("[TechScript Bridge] MainActivity.txs error: {e}");
                }
            }
        }

        log::info!(
            "[TechScript Bridge] Initialized with {} real .txs scripts loaded",
            self.loaded_scripts.len()
        );
    }

    fn extract_app_data(&mut self, val: &RuntimeValue) {
        if let RuntimeValue::Map { entries, .. } = val {
            let map = entries.borrow();
            if let Some(name) = map.get("app_name").and_then(|v| v.as_string()) {
                self.home_data.title = name.to_string();
            }
        }
    }

    fn invoke_home_dashboard(&mut self) {
        let func = self.interpreter.env.borrow().lookup("get_home_dashboard");
        if let Ok(RuntimeValue::Function(callable)) = func {
            if let Ok(RuntimeValue::Map { entries, .. }) =
                callable.call(&mut self.interpreter.ctx, vec![])
            {
                let map = entries.borrow();
                if let Some(title) = map.get("title").and_then(|v| v.as_string()) {
                    self.home_data.title = title.to_string();
                }
                if let Some(tagline) = map.get("tagline").and_then(|v| v.as_string()) {
                    self.home_data.tagline = tagline.to_string();
                }
                if let Some(RuntimeValue::List { items, .. }) = map.get("quick_actions") {
                    let mut actions = Vec::new();
                    for item in items.borrow().iter() {
                        if let RuntimeValue::Map {
                            entries: item_entries,
                            ..
                        } = item
                        {
                            let item_map = item_entries.borrow();
                            let id = item_map
                                .get("id")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            let label = item_map
                                .get("label")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            let cmd = item_map
                                .get("cmd")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            let icon = item_map
                                .get("icon")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            actions.push(QuickAction {
                                id,
                                label,
                                cmd,
                                icon,
                            });
                        }
                    }
                    if !actions.is_empty() {
                        self.home_data.quick_actions = actions;
                    }
                }
                if let Some(RuntimeValue::List { items, .. }) = map.get("metrics") {
                    let mut metrics = Vec::new();
                    for item in items.borrow().iter() {
                        if let RuntimeValue::Map {
                            entries: item_entries,
                            ..
                        } = item
                        {
                            let item_map = item_entries.borrow();
                            let label = item_map
                                .get("label")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            let value = item_map
                                .get("value")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            metrics.push(MetricItem { label, value });
                        }
                    }
                    if !metrics.is_empty() {
                        self.home_data.metrics = metrics;
                    }
                }
                log::info!("[TechScript Bridge] Dynamically updated Home dashboard from home.txs");
            }
        }
    }

    fn invoke_about_metadata(&mut self) {
        let func = self.interpreter.env.borrow().lookup("get_about_metadata");
        if let Ok(RuntimeValue::Function(callable)) = func {
            if let Ok(RuntimeValue::Map { entries, .. }) =
                callable.call(&mut self.interpreter.ctx, vec![])
            {
                let map = entries.borrow();
                if let Some(app_name) = map.get("app_name").and_then(|v| v.as_string()) {
                    self.about_data.app_name = app_name.to_string();
                }
                if let Some(creator) = map.get("creator").and_then(|v| v.as_string()) {
                    self.about_data.creator = creator.to_string();
                }
                if let Some(gh) = map.get("github_identity").and_then(|v| v.as_string()) {
                    self.about_data.github_identity = gh.to_string();
                }
                if let Some(web) = map.get("official_website").and_then(|v| v.as_string()) {
                    self.about_data.official_website = web.to_string();
                }
                if let Some(repo) = map.get("official_repository").and_then(|v| v.as_string()) {
                    self.about_data.official_repository = repo.to_string();
                }
                if let Some(RuntimeValue::List { items, .. }) = map.get("verified_links") {
                    let mut links = Vec::new();
                    for item in items.borrow().iter() {
                        if let RuntimeValue::Map {
                            entries: item_entries,
                            ..
                        } = item
                        {
                            let item_map = item_entries.borrow();
                            let label = item_map
                                .get("label")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            let url = item_map
                                .get("url")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            let link_type = item_map
                                .get("type")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            links.push(VerifiedLinkItem {
                                label,
                                url,
                                link_type,
                            });
                        }
                    }
                    if !links.is_empty() {
                        self.about_data.verified_links = links;
                    }
                }
                log::info!("[TechScript Bridge] Dynamically updated About metadata from about.txs");
            }
        }
    }

    fn invoke_docs_commands(&mut self) {
        let func = self.interpreter.env.borrow().lookup("get_cli_commands");
        if let Ok(RuntimeValue::Function(callable)) = func {
            if let Ok(RuntimeValue::List { items, .. }) =
                callable.call(&mut self.interpreter.ctx, vec![])
            {
                let mut commands = Vec::new();
                for item in items.borrow().iter() {
                    if let RuntimeValue::Map { entries, .. } = item {
                        let map = entries.borrow();
                        let cmd = map
                            .get("cmd")
                            .and_then(|v| v.as_string())
                            .unwrap_or("")
                            .to_string();
                        let desc = map
                            .get("desc")
                            .and_then(|v| v.as_string())
                            .unwrap_or("")
                            .to_string();
                        commands.push(CliCommandItem { cmd, desc });
                    }
                }
                if !commands.is_empty() {
                    self.cli_commands = commands;
                }
                log::info!(
                    "[TechScript Bridge] Dynamically loaded {} CLI commands from docs.txs",
                    self.cli_commands.len()
                );
            }
        }
    }

    /// Evaluates dynamic code snippet entered or selected by user.
    pub fn run_snippet(&mut self, code: &str) -> Result<String, String> {
        let mut reporter = techscript_errors::DiagnosticReporter::new();
        let tokens = techscript_lexer::lex(code, &mut reporter)
            .map_err(|e| format!("Lexer Error: {e:?}"))?;

        let program = techscript_parser::parse(&tokens, &mut reporter)
            .map_err(|e| format!("Parser Error: {e:?}"))?;

        let mut local_interpreter = Interpreter::new();
        let res = local_interpreter
            .interpret(&program)
            .map_err(|e| format!("Execution Error: {e:?}"))?;

        let output = format!("{res:?}");
        self.last_execution_output = output.clone();
        Ok(output)
    }
}
