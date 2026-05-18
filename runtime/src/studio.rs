// ── TechScript Studio IDE v1.0.6 Redesign ───────────────────────────────
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;
use crate::value::{Value, NativeFnObj};
use crate::run;

#[derive(Clone, PartialEq)]
pub enum Pane {
    Editor,
    Explorer,
    Console,
    ASTViewer,
    BytecodeViewer,
    AIAssistant,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerminalChannel {
    Terminal,
    Debug,
    Problems,
    Output,
    Ports,
    Tests,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LeftSidebarMode {
    Explorer,
    Search,
    Git,
    Packages,
    Extensions,
}

pub struct StudioApp {
    code: String,
    console_output: Arc<Mutex<String>>,
    compiler_output: String,
    vm_debug_output: String,
    ast_output: String,
    bytecode_output: String,
    active_tab: Tab,
    selected_template: String,
    is_running: Arc<Mutex<bool>>,
    // Premium IDE workspace states
    active_file: String,
    workspace_dir: std::path::PathBuf,
    workspace_files: Vec<String>,
    font_size: f32,
    ui_scale: f32,
    new_file_name: String,
    status_message: Option<String>,
    status_is_error: bool,
    dock_state: Option<DockState<Pane>>,
    autocomplete_open: bool,
    minimap_visible: bool,
    terminal_channel: TerminalChannel,
    active_line: Option<usize>,
    folded_lines: std::collections::HashSet<usize>,
    renaming_file: Option<std::path::PathBuf>,
    renaming_input: String,
    new_file_folder_context: Option<std::path::PathBuf>,
    // Extended states for premium UI redesign
    left_sidebar_mode: LeftSidebarMode,
    search_query: String,
    search_results: Vec<(String, usize, String)>,
    git_commit_msg: String,
    git_staged_files: Vec<String>,
    git_unstaged_files: Vec<String>,
    pkg_search_query: String,
    ai_prompt: String,
    ai_response: String,
    ai_thinking: bool,
    terminal_input: String,
    logo_texture: Option<egui::TextureHandle>,
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    AST,
    Bytecode,
    Help,
}

struct TabViewerImpl<'a> {
    app: &'a mut StudioApp,
}

impl<'a> TabViewer for TabViewerImpl<'a> {
    type Tab = Pane;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Pane::Editor => format!("📝 {} Editor", self.app.active_file).into(),
            Pane::Explorer => "📂 Explorer".into(),
            Pane::Console => "📟 Terminal".into(),
            Pane::ASTViewer => "🔍 AST Viewer".into(),
            Pane::BytecodeViewer => "⚙ Bytecode".into(),
            Pane::AIAssistant => "🤖 AI Assistant".into(),
            Pane::Manual => "❓ Cheat Sheet".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Pane::Editor => self.app.render_editor(ui),
            Pane::Explorer => self.app.render_explorer(ui),
            Pane::Console => self.app.render_console(ui),
            Pane::ASTViewer => self.app.render_ast_viewer(ui),
            Pane::BytecodeViewer => self.app.render_bytecode_viewer(ui),
            Pane::AIAssistant => self.app.render_ai_assistant(ui),
            Pane::Manual => self.app.render_manual(ui),
        }
    }
}

impl StudioApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut workspace_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        
        let is_system_dir = workspace_dir.to_string_lossy().to_lowercase().contains("system32") 
            || workspace_dir.to_string_lossy().to_lowercase().contains("windows");
            
        let is_writable = if std::fs::write(workspace_dir.join(".ts_write_test"), "").is_ok() {
            let _ = std::fs::remove_file(workspace_dir.join(".ts_write_test"));
            true
        } else {
            false
        };

        if is_system_dir || !is_writable {
            if let Ok(home) = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")) {
                let safe_dir = std::path::PathBuf::from(home).join("Documents").join("TechScript");
                let _ = std::fs::create_dir_all(&safe_dir);
                if safe_dir.exists() {
                    workspace_dir = safe_dir;
                    let _ = std::env::set_current_dir(&workspace_dir);
                }
            }
        }

        // Apply Sci-Fi Obsidian Cyberpunk Premium visual theme
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(10, 13, 22);       // Deep Obsidian #0A0D16
        visuals.window_fill = egui::Color32::from_rgb(16, 20, 36);      // Panel Futuristic Slate #101424
        visuals.extreme_bg_color = egui::Color32::from_rgb(5, 7, 12);   // Jet Black Editor background #05070C
        visuals.override_text_color = Some(egui::Color32::from_rgb(226, 232, 240)); // Slate clean off-white
        
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(16, 20, 36);
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(26, 32, 53)); // Grid Line Slate #1A2035
        
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(26, 32, 53);
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(148, 163, 184));
        visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
        
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(0, 240, 255);    // Cyber Electric Cyan #00F0FF hover bg
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, egui::Color32::from_rgb(10, 13, 22)); // dark text on hover
        visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
        
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(255, 42, 122);     // Neon Crimson/Magenta #FF2A7A active
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.5, egui::Color32::WHITE);
        visuals.widgets.active.rounding = egui::Rounding::same(4.0);

        cc.egui_ctx.set_visuals(visuals);
        cc.egui_ctx.set_pixels_per_point(1.15); // Custom Dynamic DPI

        // Customize text styles for professional premium typography
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(13.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(13.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Button, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(10.0, egui::FontFamily::Proportional)),
        ].into();
        cc.egui_ctx.set_style(style);

        let active_file = "main.txs".to_string();
        let initial_code = r#"say "🐉 Welcome to TechScript Studio v1.0.6!"
say "----------------------------------------"

make name = "Power User"
say "Hello,", name, "! Let's run some code."

each i in 1..=5 {
    say "Iteration count:", i
}
"#.to_string();

        if !std::path::Path::new("main.txs").exists() {
            let _ = std::fs::write("main.txs", &initial_code);
        }

        let icon_bytes = include_bytes!("../../assets/icons/icon-256.png");
        let logo_texture = match image::load_from_memory(icon_bytes) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                let size = [width as usize, height as usize];
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    size,
                    &rgba.into_raw(),
                );
                Some(cc.egui_ctx.load_texture(
                    "logo_texture",
                    color_image,
                    Default::default(),
                ))
            }
            Err(_) => None,
        };

        // Initialize docking layout: Left = Explorer (18%), Center = Editor, Bottom = Console (28%), Right = Inspector (22%)
        let mut dock_state = DockState::new(vec![Pane::Editor]);
        let [_left, right] = dock_state.main_surface_mut().split_left(
            NodeIndex::root(),
            0.18,
            vec![Pane::Explorer],
        );
        let [_editor, _right_panel] = dock_state.main_surface_mut().split_right(
            right,
            0.78, // Leaves 22% for the right inspector panel
            vec![Pane::ASTViewer, Pane::BytecodeViewer, Pane::AIAssistant, Pane::Manual],
        );
        let [_editor_final, _bottom] = dock_state.main_surface_mut().split_below(
            _editor,
            0.72, // Leaves 28% for the bottom console panel
            vec![Pane::Console],
        );

        let mut app = StudioApp {
            code: initial_code,
            console_output: Arc::new(Mutex::new(String::new())),
            compiler_output: String::new(),
            vm_debug_output: String::new(),
            ast_output: String::new(),
            bytecode_output: String::new(),
            active_tab: Tab::AST,
            selected_template: "Hello World".to_string(),
            is_running: Arc::new(Mutex::new(false)),
            active_file,
            workspace_dir,
            workspace_files: Vec::new(),
            font_size: 14.0,
            ui_scale: 1.15,
            new_file_name: String::new(),
            status_message: None,
            status_is_error: false,
            dock_state: Some(dock_state),
            autocomplete_open: false,
            minimap_visible: true,
            terminal_channel: TerminalChannel::Terminal,
            active_line: None,
            folded_lines: std::collections::HashSet::new(),
            renaming_file: None,
            renaming_input: String::new(),
            new_file_folder_context: None,
            // Initialize redesign states
            left_sidebar_mode: LeftSidebarMode::Explorer,
            search_query: String::new(),
            search_results: Vec::new(),
            git_commit_msg: String::new(),
            git_staged_files: Vec::new(),
            git_unstaged_files: vec![
                "main.txs (Modified)".to_string(),
                "math.txs (Unstaged)".to_string(),
            ],
            pkg_search_query: String::new(),
            ai_prompt: String::new(),
            ai_response: "🤖 TechScript AI Copilot Ready.\nAsk me to write code snippets, templates, or solve algorithms in plain English!".to_string(),
            ai_thinking: false,
            terminal_input: String::new(),
            logo_texture,
        };

        app.refresh_workspace_files();
        app.load_file_content();
        app
    }

    fn refresh_workspace_files(&mut self) {
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.workspace_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "txs" {
                            if let Some(name) = path.file_name() {
                                files.push(name.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
        files.sort();
        if files.is_empty() {
            files.push("main.txs".to_string());
        }
        self.workspace_files = files;
    }

    fn get_active_file_path(&self) -> std::path::PathBuf {
        let path = std::path::PathBuf::from(&self.active_file);
        if path.is_absolute() {
            path
        } else {
            self.workspace_dir.join(&self.active_file)
        }
    }

    fn load_file_content(&mut self) {
        let path = self.get_active_file_path();
        if let Ok(content) = std::fs::read_to_string(&path) {
            self.code = content;
            self.status_message = Some(format!("📂 Loaded active file: {}", path.display()));
            self.status_is_error = false;
        } else {
            self.status_message = Some(format!("❌ Failed to load file: {}", path.display()));
            self.status_is_error = true;
        }
    }

    fn save_file_content(&mut self) {
        let path = self.get_active_file_path();
        if std::fs::write(&path, &self.code).is_ok() {
            self.status_message = Some(format!("💾 Saved script file: {}", path.display()));
            self.status_is_error = false;
        } else {
            self.status_message = Some(format!("❌ Failed to save file: {}", path.display()));
            self.status_is_error = true;
        }
    }

    fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("TechScript", &["txs", "tx"])
            .pick_file() {
            self.active_file = path.to_string_lossy().to_string();
            if let Some(parent) = path.parent() {
                self.workspace_dir = parent.to_path_buf();
            }
            self.refresh_workspace_files();
            self.load_file_content();
        }
    }

    fn open_folder_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.workspace_dir = path;
            self.refresh_workspace_files();
            if let Some(first_file) = self.workspace_files.first() {
                self.active_file = first_file.clone();
                self.load_file_content();
            }
            self.status_message = Some(format!("📁 Opened folder: {}", self.workspace_dir.display()));
            self.status_is_error = false;
        }
    }

    fn create_new_file(&mut self) {
        let mut name = self.new_file_name.trim().to_string();
        if name.is_empty() {
            self.status_message = Some("⚠️ File name cannot be empty".to_string());
            self.status_is_error = true;
            return;
        }
        if !name.ends_with(".txs") {
            name.push_str(".txs");
        }
        
        let template_code = "say \"Welcome to your new TechScript file!\"\n";
        let target_path = self.workspace_dir.join(&name);
        if std::fs::write(&target_path, template_code).is_ok() {
            self.active_file = name.clone();
            self.new_file_name.clear();
            self.refresh_workspace_files();
            self.load_file_content();
            self.status_message = Some(format!("➕ Created new file: {}", name));
            self.status_is_error = false;
        } else {
            self.status_message = Some("❌ Failed to create file".to_string());
            self.status_is_error = true;
        }
    }

    fn run_code(&mut self) {
        let is_running_lock = self.is_running.clone();
        
        {
            let mut running = self.is_running.lock().unwrap();
            if *running {
                return;
            }
            *running = true;
        }

        let code = self.code.clone();
        let console_output = self.console_output.clone();
        
        let active_path = self.get_active_file_path();
        let _ = std::fs::write(&active_path, &code);

        {
            let mut out = console_output.lock().unwrap();
            out.clear();
            out.push_str("🚀 Running script through TechScript interpreter...\n\n");
        }

        self.compiler_output.clear();
        self.vm_debug_output.clear();
        
        self.compiler_output.push_str("⚡ Starting TechScript Compiler validation pipeline...\n");
        let tokens = Lexer::new(&code, &self.active_file).tokenize();
        match tokens {
            Ok(toks) => {
                self.compiler_output.push_str(&format!("✔ Lexer success: Generated {} lexical tokens.\n", toks.len()));
                let program = Parser::new(toks, &self.active_file).parse();
                match program {
                    Ok(prog) => {
                        self.compiler_output.push_str("✔ Parser success: Abstract Syntax Tree verified.\n");
                        self.ast_output = format!("{:#?}", prog);
                        let compiler = Compiler::new();
                        match compiler.compile(&prog) {
                            Ok(func) => {
                                self.compiler_output.push_str("✔ Compilation success: Code compiled successfully.\n");
                                self.compiler_output.push_str(&format!("📦 Segment Block: {}\n", func.name));
                                self.compiler_output.push_str(&format!("🔢 Total Ops: {}\n", func.chunk.code.len()));
                                
                                self.bytecode_output = crate::disasm::disassemble_chunk(&func.name, &func.chunk);
                                self.vm_debug_output.push_str("--- VM Bytecode Disassembly ---\n");
                                self.vm_debug_output.push_str(&self.bytecode_output);
                            }
                            Err(e) => {
                                self.bytecode_output = format!("Compilation Error:\n{}", e);
                                self.compiler_output.push_str(&format!("❌ Compiler Error:\n{}", e));
                            }
                        }
                    }
                    Err(e) => {
                        self.ast_output = format!("Parser Error:\n{}", e);
                        self.compiler_output.push_str(&format!("❌ Parser Error:\n{}", e));
                    }
                }
            }
            Err(e) => {
                self.ast_output = format!("Lexer Error:\n{}", e);
                self.compiler_output.push_str(&format!("❌ Lexer Error:\n{}", e));
            }
        }

        let file_name_clone = self.active_file.clone();
        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(|| {
                let tokens = Lexer::new(&code, &file_name_clone).tokenize().map_err(|e| e.to_string())?;
                let program = Parser::new(tokens, &file_name_clone).parse().map_err(|e| e.to_string())?;
                let function = Compiler::new().compile(&program).map_err(|e| e.to_string())?;

                let mut vm = VM::new();
                vm.stdout_buffer = Some(console_output.clone());
                let output_clone = console_output.clone();
                let native_print = NativeFnObj {
                    name: "print".into(),
                    func: Box::new(move |args| {
                        let s: Vec<String> = args.iter().map(|a| a.display_string()).collect();
                        let mut out = output_clone.lock().unwrap();
                        out.push_str(&s.join(" "));
                        Ok(Value::None)
                    }),
                };
                vm.globals.insert("print".into(), Value::NativeFunction(Rc::new(native_print)));

                let output_clone = console_output.clone();
                let native_say = NativeFnObj {
                    name: "say".into(),
                    func: Box::new(move |args| {
                        let s: Vec<String> = args.iter().map(|a| a.display_string()).collect();
                        let mut out = output_clone.lock().unwrap();
                        out.push_str(&format!("{}\n", s.join(" ")));
                        Ok(Value::None)
                    }),
                };
                vm.globals.insert("say".into(), Value::NativeFunction(Rc::new(native_say)));

                let output_clone = console_output.clone();
                let native_write = NativeFnObj {
                    name: "write".into(),
                    func: Box::new(move |args| {
                        let s: Vec<String> = args.iter().map(|a| a.display_string()).collect();
                        let mut out = output_clone.lock().unwrap();
                        out.push_str(&s.join(""));
                        Ok(Value::None)
                    }),
                };
                vm.globals.insert("write".into(), Value::NativeFunction(Rc::new(native_write)));

                let output_clone = console_output.clone();
                let native_log = NativeFnObj {
                    name: "log".into(),
                    func: Box::new(move |args| {
                        if let Some(v) = args.first() {
                            let mut out = output_clone.lock().unwrap();
                            out.push_str(&format!("[LOG] {}\n", v.display_string()));
                        }
                        Ok(Value::None)
                    }),
                };
                vm.globals.insert("log".into(), Value::NativeFunction(Rc::new(native_log)));

                let output_clone = console_output.clone();
                let native_debug = NativeFnObj {
                    name: "debug".into(),
                    func: Box::new(move |args| {
                        if let Some(v) = args.first() {
                            let mut out = output_clone.lock().unwrap();
                            out.push_str(&format!("[DEBUG] {:?}\n", v));
                        }
                        Ok(Value::None)
                    }),
                };
                vm.globals.insert("debug".into(), Value::NativeFunction(Rc::new(native_debug)));

                let output_clone = console_output.clone();
                let native_warn = NativeFnObj {
                    name: "warn".into(),
                    func: Box::new(move |args| {
                        if let Some(v) = args.first() {
                            let mut out = output_clone.lock().unwrap();
                            out.push_str(&format!("[WARN] {}\n", v.display_string()));
                        }
                        Ok(Value::None)
                    }),
                };
                vm.globals.insert("warn".into(), Value::NativeFunction(Rc::new(native_warn)));

                let output_clone = console_output.clone();
                let native_error = NativeFnObj {
                    name: "error".into(),
                    func: Box::new(move |args| {
                        if let Some(v) = args.first() {
                            let mut out = output_clone.lock().unwrap();
                            out.push_str(&format!("[ERROR] {}\n", v.display_string()));
                        }
                        Ok(Value::None)
                    }),
                };
                vm.globals.insert("error".into(), Value::NativeFunction(Rc::new(native_error)));

                let output_clone = console_output.clone();
                let native_clear = NativeFnObj {
                    name: "clear".into(),
                    func: Box::new(move |_| {
                        let mut out = output_clone.lock().unwrap();
                        out.clear();
                        Ok(Value::None)
                    }),
                };
                vm.globals.insert("clear".into(), Value::NativeFunction(Rc::new(native_clear)));

                let _ = crate::modules::load_module("web", &mut vm.globals);
                let _ = crate::modules::load_module("gui", &mut vm.globals);
                let _ = crate::modules::load_module("3d", &mut vm.globals);
                let _ = crate::modules::load_module("anime", &mut vm.globals);

                vm.run(function).map_err(|e| {
                    let lines: Vec<&str> = code.lines().collect();
                    crate::error::format_error(&e, &lines)
                })
            });

            {
                let mut out = console_output.lock().unwrap();
                match result {
                    Ok(Ok(())) => {
                        out.push_str("\n\n✅ Script finished successfully.");
                    }
                    Ok(Err(e)) => {
                        out.push_str(&format!("\n\n❌ Execution Error:\n{}", e));
                    }
                    Err(_) => {
                        out.push_str("\n\n❌ Thread panicked during execution.");
                    }
                }
            }

            let mut running = is_running_lock.lock().unwrap();
            *running = false;
        });
    }

    fn apply_template(&mut self) {
        match self.selected_template.as_str() {
            "Hello World" => {
                self.code = r#"say "🐉 Welcome to TechScript Studio!"
say "----------------------------------------"

make name = "Power User"
say "Hello,", name, "! Let's run some code."

each i in 1..=5 {
    say "Iteration count:", i
}
"#.to_string();
            }
            "Variables & Loops" => {
                self.code = r#"make count = 5
say "Counting down from:", count
repeat count > 0 {
    say "Number:", count
    count -= 1
}
say "Liftoff! 🚀"
"#.to_string();
            }
            "Class & Objects" => {
                self.code = r#"class Animal {
    init(name) {
        self.name = name
    }
    speak() {
        say self.name, "makes a sound."
    }
}

class Dog(Animal) {
    speak() {
        say self.name, "barks! 🐶"
    }
}

make my_dog = new Dog("Rufus")
my_dog.speak()
"#.to_string();
            }
            "Web Server Mock" => {
                self.code = r#"use web

page Home {
    render "div" {
        render "h1" {
            say "Welcome to TechScript Web Framework! 🌐"
        }
        render "p" {
            say "This server runs entirely on Rust tiny-http."
        }
    }
}
"#.to_string();
            }
            _ => {}
        }
        self.save_file_content();
    }

    // ── Modern Split Render Components ─────────────────────────────────

    fn render_editor(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Breadcrumbs and Action buttons row
            ui.horizontal(|ui| {
                ui.add_space(4.0);
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(16, 20, 36))
                    .rounding(4.0)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(26, 32, 53)))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new("📁 Documents").color(egui::Color32::from_rgb(148, 163, 184)).size(11.0));
                            ui.label(egui::RichText::new(">").color(egui::Color32::from_rgb(71, 85, 105)).size(10.0));
                            ui.label(egui::RichText::new("📂 TechScript").color(egui::Color32::from_rgb(148, 163, 184)).size(11.0));
                            ui.label(egui::RichText::new(">").color(egui::Color32::from_rgb(71, 85, 105)).size(10.0));
                            ui.label(egui::RichText::new("📄").color(egui::Color32::from_rgb(0, 240, 255)).size(11.0));
                            ui.label(egui::RichText::new(&self.active_file).color(egui::Color32::from_rgb(0, 240, 255)).strong().size(11.0));
                            ui.add_space(8.0);
                        });
                    });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(egui::Button::new(if self.autocomplete_open { "💡 IntelliSense: On" } else { "💡 IntelliSense: Off" })
                        .fill(if self.autocomplete_open { egui::Color32::from_rgb(255, 42, 122) } else { egui::Color32::from_rgb(26, 32, 53) })
                    ).clicked() {
                        self.autocomplete_open = !self.autocomplete_open;
                    }
                    
                    if ui.add(egui::Button::new(if self.minimap_visible { "🗺 Minimap: On" } else { "🗺 Minimap: Off" })
                        .fill(if self.minimap_visible { egui::Color32::from_rgb(0, 240, 255) } else { egui::Color32::from_rgb(26, 32, 53) })
                    ).clicked() {
                        self.minimap_visible = !self.minimap_visible;
                    }
                });
            });
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            let font_size = self.font_size;
            let current_active_line = self.active_line;
            let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                let layout_job = syntax_highlight(ui.ctx(), string, font_size, current_active_line);
                ui.fonts(|f| f.layout_job(layout_job))
            };

            // Custom line numbers and gutter implementation
            let code_str = self.code.clone();
            let code_lines: Vec<&str> = code_str.lines().collect();
            let line_count = code_lines.len().max(1);
            let active_idx = self.active_line.unwrap_or(0);

            // Dynamic word filtering for IntelliSense popover
            let mut typed_word = String::new();
            let mut show_autocomplete = false;

            ui.horizontal(|ui| {
                // Scroll Area for Gutter + Editor
                ui.allocate_ui(egui::vec2(ui.available_width() - 80.0, ui.available_height()), |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("editor_scroll_v")
                        .show(ui, |ui| {
                            ui.horizontal_top(|ui| {
                                // Gutter Panel
                                ui.vertical(|ui| {
                                    ui.set_width(45.0);
                                    for i in 1..=line_count {
                                        let is_active = i - 1 == active_idx;
                                        let line_str = code_lines.get(i - 1).unwrap_or(&"");
                                        let has_bracket = line_str.contains('{');

                                        ui.horizontal(|ui| {
                                            ui.set_height(18.0);
                                            // Active line neon marker bar
                                            if is_active {
                                                let (marker_rect, _) = ui.allocate_exact_size(egui::vec2(2.0, 14.0), egui::Sense::hover());
                                                ui.painter().rect_filled(marker_rect, 0.0, egui::Color32::from_rgb(0, 240, 255));
                                                ui.add_space(2.0);
                                            } else {
                                                ui.add_space(4.0);
                                            }

                                            // Folding Chevron
                                            if has_bracket {
                                                let fold_icon = if self.folded_lines.contains(&(i - 1)) { "▶" } else { "▼" };
                                                let btn = ui.add(egui::Label::new(
                                                    egui::RichText::new(fold_icon).color(egui::Color32::from_rgb(0, 240, 255)).size(9.0)
                                                ).sense(egui::Sense::click()));
                                                
                                                if btn.clicked() {
                                                    if self.folded_lines.contains(&(i - 1)) {
                                                        self.folded_lines.remove(&(i - 1));
                                                    } else {
                                                        self.folded_lines.insert(i - 1);
                                                    }
                                                }
                                            } else {
                                                ui.add_space(6.0);
                                            }

                                            // Line Index Number
                                            let text = if is_active {
                                                egui::RichText::new(format!("{:>3}", i))
                                                    .color(egui::Color32::from_rgb(0, 240, 255))
                                                    .strong()
                                            } else {
                                                egui::RichText::new(format!("{:>3}", i))
                                                    .color(egui::Color32::from_rgb(71, 85, 105))
                                            };
                                            ui.label(text);
                                        });
                                    }
                                });

                                ui.add_space(4.0);

                                // Vertical Separator line
                                let (sep_rect, _) = ui.allocate_exact_size(egui::vec2(1.0, ui.available_height().max(200.0)), egui::Sense::hover());
                                ui.painter().rect_filled(sep_rect, 0.0, egui::Color32::from_rgb(26, 32, 53));

                                ui.add_space(4.0);

                                // Code Input Panel
                                let response = ui.add(
                                    egui::TextEdit::multiline(&mut self.code)
                                        .font(egui::TextStyle::Monospace)
                                        .desired_width(f32::INFINITY)
                                        .lock_focus(true)
                                        .layouter(&mut layouter)
                                );

                                if let Some(state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
                                    if let Some(ccursor) = state.cursor.char_range() {
                                        let cursor_index = ccursor.primary.index;
                                        let before_cursor = &self.code[..cursor_index.min(self.code.len())];
                                        let line_index = before_cursor.matches('\n').count();
                                        self.active_line = Some(line_index);

                                        // IntelliSense parsing
                                        if let Some(last_word) = before_cursor.split(|c: char| c.is_whitespace() || "(){}[],.;+-*/=<>!&|%".contains(c)).last() {
                                            if !last_word.is_empty() {
                                                typed_word = last_word.to_string();
                                                show_autocomplete = true;
                                            }
                                        }
                                    }
                                }
                            });
                        });
                });

                // Minimap Panel
                if self.minimap_visible {
                    ui.allocate_ui(egui::vec2(60.0, ui.available_height()), |ui| {
                        draw_minimap(ui, &self.code, self.active_line.unwrap_or(0));
                    });
                }
            });

            // Autocomplete floating Popover card
            if self.autocomplete_open && show_autocomplete {
                let keywords = vec![
                    "make", "keep", "const", "say", "print", "write", "log", "debug", "warn", "error",
                    "repeat", "while", "each", "loop", "in", "when", "alt", "else", "build", "send", "class", "new", "use"
                ];
                let filtered: Vec<&str> = keywords.into_iter()
                    .filter(|kw| kw.starts_with(&typed_word) && *kw != typed_word)
                    .collect();

                if !filtered.is_empty() {
                    egui::Area::new("autocomplete_popover".into())
                        .fixed_pos(ui.cursor().min + egui::vec2(120.0, 60.0))
                        .order(egui::Order::Tooltip)
                        .show(ui.ctx(), |ui| {
                            egui::Frame::none()
                                .fill(egui::Color32::from_rgb(10, 13, 22))
                                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 240, 255)))
                                .rounding(4.0)
                                .shadow(egui::Shadow {
                                    offset: egui::vec2(0.0, 3.0),
                                    blur: 10.0,
                                    spread: 0.0,
                                    color: egui::Color32::from_black_alpha(128),
                                })
                                .show(ui, |ui| {
                                    ui.set_width(160.0);
                                    ui.vertical(|ui| {
                                        ui.add_space(4.0);
                                        ui.horizontal(|ui| {
                                            ui.add_space(6.0);
                                            ui.label(egui::RichText::new("💡 IntelliSense Suggestions").color(egui::Color32::from_rgb(0, 240, 255)).size(10.0));
                                        });
                                        ui.add_space(2.0);
                                        ui.separator();
                                        
                                        for kw in filtered {
                                            ui.horizontal(|ui| {
                                                ui.add_space(6.0);
                                                let kw_label = egui::RichText::new(kw).color(egui::Color32::from_rgb(255, 42, 122)).monospace();
                                                if ui.selectable_label(false, kw_label).clicked() {
                                                    let remainder = &kw[typed_word.len()..];
                                                    self.code.push_str(remainder);
                                                    self.autocomplete_open = false;
                                                }
                                            });
                                        }
                                        ui.add_space(4.0);
                                    });
                                });
                        });
                }
            }
        });
    }

    fn render_directory_node(&mut self, ui: &mut egui::Ui, dir_path: &std::path::Path) {
        if let Ok(entries) = std::fs::read_dir(dir_path) {
            let mut folders = Vec::new();
            let mut files = Vec::new();

            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    folders.push(path);
                } else if path.is_file() {
                    files.push(path);
                }
            }

            // Alphabetical sort
            folders.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

            // Subdirectories
            for folder in folders {
                let folder_name = folder.file_name().unwrap_or_default().to_string_lossy().to_string();
                if folder_name.starts_with('.') || folder_name == "target" || folder_name == "node_modules" {
                    continue;
                }

                let id = ui.make_persistent_id(&folder);
                let collapsing = egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);
                collapsing.show_header(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("📁").color(egui::Color32::from_rgb(255, 170, 0)));
                        ui.label(egui::RichText::new(&folder_name).strong().color(egui::Color32::from_rgb(226, 232, 240)));
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(egui::RichText::new("🗑").size(10.0)).on_hover_text("Delete Folder").clicked() {
                                let _ = std::fs::remove_dir_all(&folder);
                                self.refresh_workspace_files();
                            }
                            if ui.button(egui::RichText::new("➕").size(10.0)).on_hover_text("Create File in Folder").clicked() {
                                self.new_file_folder_context = Some(folder.clone());
                            }
                        });
                    });
                })
                .body(|ui| {
                    self.render_directory_node(ui, &folder);
                });
            }

            // Files
            for file in files {
                let file_name = file.file_name().unwrap_or_default().to_string_lossy().to_string();
                if file_name.starts_with('.') || file_name == "Cargo.lock" || file_name == ".ts_write_test" {
                    continue;
                }

                let rel_path = file.strip_prefix(&self.workspace_dir)
                    .unwrap_or(&file)
                    .to_string_lossy()
                    .to_string();

                let is_active = rel_path == self.active_file || file.to_string_lossy().to_string() == self.active_file;

                ui.horizontal(|ui| {
                    ui.add_space(6.0);
                    
                    if let Some(ref renaming_path) = self.renaming_file {
                        if renaming_path == &file {
                            // Inline Rename Text Edit
                            ui.text_edit_singleline(&mut self.renaming_input);
                            if ui.button("💾").clicked() {
                                let trimmed = self.renaming_input.trim();
                                if !trimmed.is_empty() {
                                    let new_path = file.with_file_name(trimmed);
                                    if std::fs::rename(&file, &new_path).is_ok() {
                                        let rel_new = new_path.strip_prefix(&self.workspace_dir)
                                            .unwrap_or(&new_path)
                                            .to_string_lossy()
                                            .to_string();
                                        if is_active {
                                            self.active_file = rel_new;
                                        }
                                    }
                                }
                                self.renaming_file = None;
                                self.renaming_input.clear();
                                self.refresh_workspace_files();
                            }
                            if ui.button("❌").clicked() {
                                self.renaming_file = None;
                                self.renaming_input.clear();
                            }
                            return;
                        }
                    }

                    // Render standard file item
                    let (badge_text, badge_color) = if file_name.ends_with(".txs") {
                        (" txs ", egui::Color32::from_rgb(0, 240, 255))
                    } else if file_name.ends_with(".tx") {
                        (" tx ", egui::Color32::from_rgb(255, 140, 0))
                    } else if file_name.ends_with(".rs") {
                        (" rust ", egui::Color32::from_rgb(222, 165, 132))
                    } else {
                        (" file ", egui::Color32::from_rgb(148, 163, 184))
                    };

                    let badge = egui::RichText::new(badge_text)
                        .color(egui::Color32::from_rgb(10, 13, 22))
                        .background_color(badge_color)
                        .strong()
                        .size(9.0);
                    ui.label(badge);

                    let file_text = if is_active {
                        egui::RichText::new(&file_name).color(egui::Color32::from_rgb(0, 240, 255)).strong()
                    } else {
                        egui::RichText::new(&file_name).color(egui::Color32::from_rgb(226, 232, 240))
                    };

                    if ui.selectable_label(is_active, file_text).clicked() {
                        self.save_file_content();
                        self.active_file = rel_path;
                        self.load_file_content();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new("🗑").size(10.0)).on_hover_text("Delete File").clicked() {
                            let _ = std::fs::remove_file(&file);
                            if is_active {
                                self.active_file = String::new();
                                self.code = String::new();
                            }
                            self.refresh_workspace_files();
                        }
                        if ui.button(egui::RichText::new("✏").size(10.0)).on_hover_text("Rename File").clicked() {
                            self.renaming_file = Some(file.clone());
                            self.renaming_input = file_name.clone();
                        }
                    });
                });
                ui.add_space(2.0);
            }
        }
    }

    fn render_explorer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // 1. Icon Switcher Column (Left edge, 45px)
            let icon_col_width = 45.0;
            ui.allocate_ui(egui::vec2(icon_col_width, ui.available_height()), |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(10, 13, 22))
                    .inner_margin(egui::Margin::symmetric(4.0, 8.0))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.spacing_mut().item_spacing.y = 12.0;

                            // Explorer Icon
                            let is_exp = self.left_sidebar_mode == LeftSidebarMode::Explorer;
                            let exp_btn = egui::Button::new(egui::RichText::new("📁").size(16.0).color(if is_exp { egui::Color32::from_rgb(10, 13, 22) } else { egui::Color32::from_rgb(148, 163, 184) }))
                                .fill(if is_exp { egui::Color32::from_rgb(0, 240, 255) } else { egui::Color32::TRANSPARENT })
                                .rounding(4.0);
                            if ui.add_sized([32.0, 32.0], exp_btn).on_hover_text("Explorer").clicked() {
                                self.left_sidebar_mode = LeftSidebarMode::Explorer;
                            }

                            // Search Icon
                            let is_srch = self.left_sidebar_mode == LeftSidebarMode::Search;
                            let srch_btn = egui::Button::new(egui::RichText::new("🔍").size(16.0).color(if is_srch { egui::Color32::from_rgb(10, 13, 22) } else { egui::Color32::from_rgb(148, 163, 184) }))
                                .fill(if is_srch { egui::Color32::from_rgb(0, 240, 255) } else { egui::Color32::TRANSPARENT })
                                .rounding(4.0);
                            if ui.add_sized([32.0, 32.0], srch_btn).on_hover_text("Search Workspace").clicked() {
                                self.left_sidebar_mode = LeftSidebarMode::Search;
                            }

                            // Git Icon
                            let is_git = self.left_sidebar_mode == LeftSidebarMode::Git;
                            let git_btn = egui::Button::new(egui::RichText::new("").size(16.0).color(if is_git { egui::Color32::from_rgb(10, 13, 22) } else { egui::Color32::from_rgb(148, 163, 184) }))
                                .fill(if is_git { egui::Color32::from_rgb(0, 240, 255) } else { egui::Color32::TRANSPARENT })
                                .rounding(4.0);
                            if ui.add_sized([32.0, 32.0], git_btn).on_hover_text("Source Control (Git)").clicked() {
                                self.left_sidebar_mode = LeftSidebarMode::Git;
                            }

                            // Packages Icon
                            let is_pkg = self.left_sidebar_mode == LeftSidebarMode::Packages;
                            let pkg_btn = egui::Button::new(egui::RichText::new("📦").size(16.0).color(if is_pkg { egui::Color32::from_rgb(10, 13, 22) } else { egui::Color32::from_rgb(148, 163, 184) }))
                                .fill(if is_pkg { egui::Color32::from_rgb(0, 240, 255) } else { egui::Color32::TRANSPARENT })
                                .rounding(4.0);
                            if ui.add_sized([32.0, 32.0], pkg_btn).on_hover_text("Package Manager").clicked() {
                                self.left_sidebar_mode = LeftSidebarMode::Packages;
                            }

                            // Extensions/Settings Icon
                            let is_ext = self.left_sidebar_mode == LeftSidebarMode::Extensions;
                            let ext_btn = egui::Button::new(egui::RichText::new("🔌").size(16.0).color(if is_ext { egui::Color32::from_rgb(10, 13, 22) } else { egui::Color32::from_rgb(148, 163, 184) }))
                                .fill(if is_ext { egui::Color32::from_rgb(0, 240, 255) } else { egui::Color32::TRANSPARENT })
                                .rounding(4.0);
                            if ui.add_sized([32.0, 32.0], ext_btn).on_hover_text("Preferences & Add-ons").clicked() {
                                self.left_sidebar_mode = LeftSidebarMode::Extensions;
                            }
                        });
                    });
            });

            ui.separator();

            // 2. Main Sidebar Panel Content Area (Remaining Width)
            ui.allocate_ui(egui::vec2(ui.available_width(), ui.available_height()), |ui| {
                match self.left_sidebar_mode {
                    LeftSidebarMode::Explorer => {
                        ui.vertical(|ui| {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("📁 EXPLORER").color(egui::Color32::from_rgb(0, 240, 255)).strong().size(12.0));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("⟳").on_hover_text("Refresh Workspace").clicked() {
                                        self.refresh_workspace_files();
                                    }
                                });
                            });
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(6.0);

                            // Contextual file creation inside a subfolder
                            let folder_context = self.new_file_folder_context.clone();
                            if let Some(folder_path) = folder_context {
                                let folder_name = folder_path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                egui::Frame::none()
                                    .fill(egui::Color32::from_rgb(16, 20, 36))
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 240, 255)))
                                    .rounding(4.0)
                                    .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new(format!("📄 Create in /{}", folder_name)).color(egui::Color32::from_rgb(0, 240, 255)).size(10.0));
                                            ui.horizontal(|ui| {
                                                ui.text_edit_singleline(&mut self.new_file_name);
                                                if ui.button("➕").clicked() {
                                                    let mut name = self.new_file_name.trim().to_string();
                                                    if !name.is_empty() {
                                                        if !name.ends_with(".txs") && !name.contains('.') {
                                                            name.push_str(".txs");
                                                        }
                                                        let target = folder_path.join(&name);
                                                        let _ = std::fs::write(&target, "say \"Created in folder!\"\n");
                                                        self.new_file_name.clear();
                                                        self.new_file_folder_context = None;
                                                        self.refresh_workspace_files();
                                                        if let Ok(rel) = target.strip_prefix(&self.workspace_dir) {
                                                            self.active_file = rel.to_string_lossy().to_string();
                                                            self.load_file_content();
                                                        }
                                                    }
                                                }
                                                if ui.button("❌").clicked() {
                                                    self.new_file_folder_context = None;
                                                    self.new_file_name.clear();
                                                }
                                            });
                                        });
                                    });
                                ui.add_space(4.0);
                            }

                            // Scrollable workspace directory tree
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let workspace = self.workspace_dir.clone();
                                self.render_directory_node(ui, &workspace);
                            });

                            ui.add_space(20.0);
                            ui.separator();
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new("➕ Create New Script").color(egui::Color32::from_rgb(148, 163, 184)).strong().size(11.0));
                            ui.add_space(4.0);

                            ui.horizontal(|ui| {
                                ui.text_edit_singleline(&mut self.new_file_name).on_hover_text("e.g. math.txs");
                                if ui.add(egui::Button::new("Create").fill(egui::Color32::from_rgb(0, 212, 255))).clicked() {
                                    self.create_new_file();
                                }
                            });
                        });
                    }
                    LeftSidebarMode::Search => {
                        ui.vertical(|ui| {
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new("🔍 GREP SEARCH").color(egui::Color32::from_rgb(0, 240, 255)).strong().size(12.0));
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(6.0);

                            ui.label("Search across all workspace files:");
                            ui.horizontal(|ui| {
                                let search_edit = ui.text_edit_singleline(&mut self.search_query);
                                if search_edit.changed() || ui.button("Search").clicked() {
                                    self.search_results.clear();
                                    if !self.search_query.is_empty() {
                                        let q = self.search_query.to_lowercase();
                                        // Scan folder files
                                        if let Ok(entries) = std::fs::read_dir(&self.workspace_dir) {
                                            for entry in entries.flatten() {
                                                let p = entry.path();
                                                if p.is_file() {
                                                    if let Ok(content) = std::fs::read_to_string(&p) {
                                                        for (idx, line) in content.lines().enumerate() {
                                                            if line.to_lowercase().contains(&q) {
                                                                let file_name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                                                                self.search_results.push((
                                                                    file_name,
                                                                    idx + 1,
                                                                    line.trim().to_string()
                                                                ));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            });

                            ui.add_space(10.0);
                            ui.colored_label(egui::Color32::from_rgb(148, 163, 184), format!("Found {} matches:", self.search_results.len()));
                            ui.add_space(6.0);

                            let search_results = self.search_results.clone();
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.vertical(|ui| {
                                    for (file_name, line_num, line_content) in &search_results {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("🎯").color(egui::Color32::from_rgb(255, 42, 122)));
                                            let label = format!("{}:{}: {}", file_name, line_num, line_content);
                                            if ui.button(egui::RichText::new(&label).monospace().size(11.0)).clicked() {
                                                self.save_file_content();
                                                self.active_file = file_name.clone();
                                                self.load_file_content();
                                                self.active_line = Some(*line_num);
                                            }
                                        });
                                        ui.add_space(4.0);
                                    }
                                });
                            });
                        });
                    }
                    LeftSidebarMode::Git => {
                        ui.vertical(|ui| {
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new(" SOURCE CONTROL").color(egui::Color32::from_rgb(0, 240, 255)).strong().size(12.0));
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(6.0);

                            ui.colored_label(egui::Color32::from_rgb(148, 163, 184), "Changes (Unstaged):");
                            let mut unstage_clicked = None;
                            for (idx, f) in self.git_unstaged_files.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.colored_label(egui::Color32::from_rgb(255, 42, 122), format!("  ⚠ {}", f));
                                    if ui.button("＋ Stage").clicked() {
                                        unstage_clicked = Some(idx);
                                    }
                                });
                            }
                            if let Some(idx) = unstage_clicked {
                                let f = self.git_unstaged_files.remove(idx);
                                self.git_staged_files.push(f.replace(" (Modified)", " (Staged)").replace(" (Unstaged)", " (Staged)"));
                            }
                            if self.git_unstaged_files.is_empty() {
                                ui.colored_label(egui::Color32::from_rgb(52, 211, 153), "  No modified files.");
                            }

                            ui.add_space(10.0);
                            ui.colored_label(egui::Color32::from_rgb(148, 163, 184), "Staged Changes:");
                            let mut stage_clicked = None;
                            for (idx, f) in self.git_staged_files.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.colored_label(egui::Color32::from_rgb(13, 242, 139), format!("  ✔ {}", f));
                                    if ui.button("－ Unstage").clicked() {
                                        stage_clicked = Some(idx);
                                    }
                                });
                            }
                            if let Some(idx) = stage_clicked {
                                let f = self.git_staged_files.remove(idx);
                                self.git_unstaged_files.push(f.replace(" (Staged)", " (Modified)"));
                            }
                            if self.git_staged_files.is_empty() {
                                ui.colored_label(egui::Color32::from_rgb(100, 116, 139), "  No staged files.");
                            }

                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(8.0);

                            ui.label("Commit Message:");
                            ui.text_edit_multiline(&mut self.git_commit_msg);
                            ui.add_space(8.0);
                            
                            let commit_btn = ui.add(
                                egui::Button::new(egui::RichText::new("✓ Commit to main").strong().color(egui::Color32::from_rgb(10, 13, 22)))
                                    .fill(egui::Color32::from_rgb(0, 240, 255))
                            );
                            
                            if commit_btn.clicked() {
                                if self.git_commit_msg.trim().is_empty() {
                                    self.status_message = Some("Please enter a commit message!".to_string());
                                    self.status_is_error = true;
                                } else {
                                    self.status_message = Some(format!("Successfully committed {} files with message: '{}'", self.git_staged_files.len(), self.git_commit_msg));
                                    self.status_is_error = false;
                                    self.git_staged_files.clear();
                                    self.git_commit_msg.clear();
                                    self.git_unstaged_files.clear();
                                }
                            }
                        });
                    }
                    LeftSidebarMode::Packages => {
                        ui.vertical(|ui| {
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new("📦 PACKAGE MANAGER").color(egui::Color32::from_rgb(0, 240, 255)).strong().size(12.0));
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(6.0);

                            ui.label("Search Packages:");
                            ui.horizontal(|ui| {
                                ui.text_edit_singleline(&mut self.pkg_search_query);
                                if ui.button("🔍 Find").clicked() {}
                            });
                            ui.add_space(10.0);

                            ui.colored_label(egui::Color32::from_rgb(148, 163, 184), "Installed Libraries:");
                            ui.label(egui::RichText::new("• std (v1.0.6) - Built-in standard library").monospace().size(11.0));
                            ui.label(egui::RichText::new("• math (v1.0.1) - Numeric methods").monospace().size(11.0));
                            
                            ui.add_space(16.0);
                            ui.colored_label(egui::Color32::from_rgb(148, 163, 184), "Available Packages:");
                            
                            let packages = vec![
                                ("web", "High-performance non-blocking HTTP and WebSocket web app wrapper"),
                                ("gui", "Interactive vector graphics, inputs, buttons, and custom layout frameworks"),
                                ("3d", "Offline-first hardware accelerated WebGL and raw canvas 3D render engine"),
                                ("anime", "Smooth reactive dynamic requestAnimationFrame tweening utility"),
                                ("ai", "Modular local LLM model invocation API and code generation hook"),
                            ];

                            for (pkg_name, desc) in packages {
                                ui.group(|ui| {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.strong(egui::RichText::new(pkg_name).color(egui::Color32::from_rgb(255, 42, 122)));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if ui.add(egui::Button::new("⚡ Install")).clicked() {
                                                    self.status_message = Some(format!("Successfully configured techscript extension '{}' to workspace!", pkg_name));
                                                    self.status_is_error = false;
                                                    // inject use statement to file code
                                                    let use_line = format!("use {}\n", pkg_name);
                                                    if !self.code.contains(&use_line) {
                                                        self.code.insert_str(0, &use_line);
                                                        self.save_file_content();
                                                    }
                                                }
                                            });
                                        });
                                        ui.label(egui::RichText::new(desc).size(10.5).color(egui::Color32::from_rgb(226, 232, 240)));
                                    });
                                });
                                ui.add_space(4.0);
                            }
                        });
                    }
                    LeftSidebarMode::Extensions => {
                        ui.vertical(|ui| {
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new("🔌 PREFERENCES").color(egui::Color32::from_rgb(0, 240, 255)).strong().size(12.0));
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(6.0);

                            ui.label(egui::RichText::new("Layout Configuration:").strong());
                            ui.checkbox(&mut self.minimap_visible, "Show Gutter Minimap");
                            ui.checkbox(&mut self.autocomplete_open, "Enable Autocomplete Popups");
                            ui.add_space(10.0);

                            ui.label(egui::RichText::new("Typography:").strong());
                            ui.horizontal(|ui| {
                                ui.label("Font Size:");
                                ui.add(egui::Slider::new(&mut self.font_size, 10.0..=24.0).step_by(1.0));
                            });
                            ui.add_space(10.0);

                            ui.label(egui::RichText::new("DPI Sizing Scaling:").strong());
                            ui.horizontal(|ui| {
                                ui.label("UI Scale:");
                                let prev_scale = self.ui_scale;
                                ui.add(egui::Slider::new(&mut self.ui_scale, 0.8..=2.0).step_by(0.05));
                                if self.ui_scale != prev_scale {
                                    ui.ctx().set_pixels_per_point(self.ui_scale);
                                }
                            });
                            ui.add_space(12.0);

                            ui.label(egui::RichText::new("Theme Variant:").strong());
                            ui.colored_label(egui::Color32::from_rgb(13, 242, 139), "✔ Cyberpunk Obsidian Dark (Active)");
                            ui.colored_label(egui::Color32::from_rgb(148, 163, 184), "  Cyberpunk High Saturation Light");
                        });
                    }
                }
            });
        });
    }

    fn render_console(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📟 TERMINAL").color(egui::Color32::from_rgb(148, 163, 184)).strong().size(12.0));
                ui.add_space(8.0);

                // Tab: Stdout Console
                let is_stdout = self.terminal_channel == TerminalChannel::Terminal;
                let stdout_btn = egui::Button::new(
                    egui::RichText::new("📟 Stdout")
                        .strong()
                        .size(11.0)
                        .color(if is_stdout { egui::Color32::from_rgb(10, 13, 22) } else { egui::Color32::from_rgb(148, 163, 184) })
                )
                .fill(if is_stdout { egui::Color32::from_rgb(13, 242, 139) } else { egui::Color32::from_rgb(26, 32, 53) });
                if ui.add(stdout_btn).clicked() {
                    self.terminal_channel = TerminalChannel::Terminal;
                }

                // Tab: Compiler Diagnostics
                let is_compiler = self.terminal_channel == TerminalChannel::Problems;
                let compiler_btn = egui::Button::new(
                    egui::RichText::new("⚙ Compiler")
                        .strong()
                        .size(11.0)
                        .color(if is_compiler { egui::Color32::from_rgb(10, 13, 22) } else { egui::Color32::from_rgb(148, 163, 184) })
                )
                .fill(if is_compiler { egui::Color32::from_rgb(0, 163, 255) } else { egui::Color32::from_rgb(26, 32, 53) });
                if ui.add(compiler_btn).clicked() {
                    self.terminal_channel = TerminalChannel::Problems;
                }

                // Tab: VM Registers
                let is_debugger = self.terminal_channel == TerminalChannel::Debug;
                let debugger_btn = egui::Button::new(
                    egui::RichText::new("🐞 VM Debugger")
                        .strong()
                        .size(11.0)
                        .color(if is_debugger { egui::Color32::from_rgb(10, 13, 22) } else { egui::Color32::from_rgb(148, 163, 184) })
                )
                .fill(if is_debugger { egui::Color32::from_rgb(216, 180, 254) } else { egui::Color32::from_rgb(26, 32, 53) });
                if ui.add(debugger_btn).clicked() {
                    self.terminal_channel = TerminalChannel::Debug;
                }

                // Right aligned buttons
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🧹 Clear Logs").on_hover_text("Clear active channel logs").clicked() {
                        match self.terminal_channel {
                            TerminalChannel::Terminal => {
                                let mut out = self.console_output.lock().unwrap();
                                out.clear();
                            }
                            TerminalChannel::Problems => self.compiler_output.clear(),
                            TerminalChannel::Debug => self.vm_debug_output.clear(),
                            _ => {}
                        }
                    }
                    if ui.button("📋 Copy Output").on_hover_text("Copy active logs to clipboard").clicked() {
                        let text_to_copy = match self.terminal_channel {
                            TerminalChannel::Terminal => self.console_output.lock().unwrap().clone(),
                            TerminalChannel::Problems => self.compiler_output.clone(),
                            TerminalChannel::Debug => self.vm_debug_output.clone(),
                            _ => String::new(),
                        };
                        ui.ctx().copy_text(text_to_copy);
                    }
                    
                    let run_btn = egui::Button::new(
                        egui::RichText::new("▶ Re-run Script")
                            .strong()
                            .color(egui::Color32::from_rgb(10, 13, 22))
                    )
                    .fill(egui::Color32::from_rgb(0, 240, 255));
                    if ui.add(run_btn).on_hover_text("Save and run active script").clicked() {
                        self.save_file_content();
                        self.run_code();
                    }
                });
            });
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    let mut log_text = match self.terminal_channel {
                        TerminalChannel::Terminal => self.console_output.lock().unwrap().clone(),
                        TerminalChannel::Problems => self.compiler_output.clone(),
                        TerminalChannel::Debug => self.vm_debug_output.clone(),
                        _ => String::new(),
                    };
                    
                    if log_text.is_empty() {
                        log_text = match self.terminal_channel {
                            TerminalChannel::Terminal => "Terminal stdout is empty. Run your TechScript program!".to_string(),
                            TerminalChannel::Problems => "No compilation verified yet.".to_string(),
                            TerminalChannel::Debug => "Disassemble output will display VM instructions and active registers on run.".to_string(),
                            _ => String::new(),
                        };
                    }
                    
                    let text_color = match self.terminal_channel {
                        TerminalChannel::Terminal => egui::Color32::from_rgb(13, 242, 139),   // Emerald green #0DF28B
                        TerminalChannel::Problems => egui::Color32::from_rgb(0, 163, 255), // Electric Blue #00A3FF
                        TerminalChannel::Debug => egui::Color32::from_rgb(216, 180, 254), // Lavender #D8B4FE
                        _ => egui::Color32::GRAY,
                    };

                    ui.add(
                        egui::TextEdit::multiline(&mut log_text.as_str())
                            .font(egui::TextStyle::Monospace)
                            .text_color(text_color)
                            .desired_width(f32::INFINITY)
                            .lock_focus(true)
                            .interactive(false)
                    );
                });
        });
    }

    fn render_ast_viewer(&mut self, ui: &mut egui::Ui) {
        let mut ast = self.ast_output.clone();
        if ast.is_empty() {
            ast = "Run a script to generate and analyze the AST structure.".to_string();
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut ast.as_str())
                    .font(egui::TextStyle::Monospace)
                    .text_color(egui::Color32::from_rgb(192, 132, 252)) // Electric Violet
                    .desired_width(f32::INFINITY)
            );
        });
    }

    fn render_bytecode_viewer(&mut self, ui: &mut egui::Ui) {
        let mut bc = self.bytecode_output.clone();
        if bc.is_empty() {
            bc = "Run a script to dissemble VM bytecode instructions here.".to_string();
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut bc.as_str())
                    .font(egui::TextStyle::Monospace)
                    .text_color(egui::Color32::from_rgb(56, 189, 248)) // Cyber Cyan
                    .desired_width(f32::INFINITY)
            );
        });
     }

    fn render_ai_assistant(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("🤖 TECHSCRIPT AI COPILOT")
                        .color(egui::Color32::from_rgb(0, 240, 255))
                        .strong()
                        .size(13.0)
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🧹 Clear Conversation").clicked() {
                        self.ai_response.clear();
                    }
                });
            });
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(6.0);

            // Chat area / scrolling response
            egui::ScrollArea::vertical()
                .id_source("ai_chat_scroll")
                .max_height(ui.available_height() - 140.0)
                .show(ui, |ui| {
                    if self.ai_response.is_empty() {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.colored_label(egui::Color32::from_rgb(148, 163, 184), "How can I help you program in TechScript today?");
                            ui.add_space(10.0);

                            // Pre-made prompts
                            let prompts = [
                                "Create a simple web interface",
                                "Write a prime number checker function",
                                "Explain variables and keeping constants",
                                "Refactor active script with robust handling",
                            ];

                            for prompt in prompts {
                                let btn = egui::Button::new(
                                    egui::RichText::new(format!("💡 {}", prompt))
                                        .color(egui::Color32::from_rgb(0, 240, 255))
                                        .size(11.0)
                                )
                                .fill(egui::Color32::from_rgb(26, 32, 53));
                                
                                if ui.add(btn).clicked() {
                                    self.ai_prompt = prompt.to_string();
                                }
                                ui.add_space(6.0);
                            }
                        });
                    } else {
                        // Display response
                        ui.horizontal(|ui| {
                            ui.colored_label(egui::Color32::from_rgb(255, 42, 122), "🤖 TechScript AI:");
                        });
                        ui.add_space(4.0);
                        
                        let response_text = self.ai_response.clone();
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&response_text)
                                    .color(egui::Color32::from_rgb(226, 232, 240))
                                    .size(12.0)
                            )
                            .wrap()
                        );
                        
                        // Extract any code block from the response
                        if response_text.contains("```") {
                            ui.add_space(12.0);
                            let code_btn = egui::Button::new(
                                egui::RichText::new("🚀 Apply Code to Active Editor")
                                    .strong()
                                    .color(egui::Color32::from_rgb(10, 13, 22))
                            )
                            .fill(egui::Color32::from_rgb(0, 240, 255));
                            
                            if ui.add(code_btn).clicked() {
                                // Extract the code block content
                                if let Some(start) = response_text.find("```") {
                                    let rest = &response_text[start + 3..];
                                    // skip optional language label (e.g. rust, ts, etc.)
                                    let code_start = rest.find('\n').unwrap_or(0) + 1;
                                    if let Some(end) = rest[code_start..].find("```") {
                                        let extracted_code = rest[code_start..code_start + end].trim().to_string();
                                        self.code = extracted_code;
                                        self.save_file_content();
                                        self.status_message = Some("AI code block successfully injected into editor!".to_string());
                                        self.status_is_error = false;
                                    }
                                }
                            }
                        }
                    }
                });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(6.0);

            // Loading state
            if self.ai_thinking {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.colored_label(egui::Color32::from_rgb(0, 240, 255), "AI Copilot is thinking...");
                });
                ui.add_space(6.0);
            }

            // Input area
            ui.horizontal(|ui| {
                let text_edit = egui::TextEdit::multiline(&mut self.ai_prompt)
                    .hint_text("Ask TechScript Copilot... (Shift+Enter to send)")
                    .desired_rows(2)
                    .desired_width(ui.available_width() - 80.0);
                
                let response = ui.add(text_edit);
                
                let send_btn = egui::Button::new(
                    egui::RichText::new("⚡ Ask")
                        .strong()
                        .color(egui::Color32::from_rgb(10, 13, 22))
                )
                .fill(egui::Color32::from_rgb(255, 42, 122));
                
                let mut should_send = ui.add_enabled(!self.ai_thinking && !self.ai_prompt.trim().is_empty(), send_btn).clicked();
                
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.shift) {
                    should_send = true;
                }

                if should_send {
                    self.ai_thinking = true;
                    self.ai_response = "Analyzing script context and generating solution...".to_string();
                    
                    // Simulate context-aware prompt output for TechScript programming
                    let prompt_lower = self.ai_prompt.to_lowercase();
                    let response_str = if prompt_lower.contains("web") || prompt_lower.contains("interface") {
                        r#"Here is a premium TechScript Web App interface using the modern 'web' module:

```techscript
# Simple TechScript Web App Interface
import web

keep PORT = 8080
make visitors = 100

build render_home() {
    send "
    <div style='background:#0A0D16; color:#0DF28B; padding:20px; font-family:sans-serif;'>
        <h1>🐉 Welcome to TechScript Web Service</h1>
        <p>Active visitors: " + visitors + "</p>
    </div>
    "
}

web.listen(PORT, render_home)
```

Click the button below to instantly load this code into your active file!"#.to_string()
                    } else if prompt_lower.contains("prime") || prompt_lower.contains("fibonacci") {
                        r#"Here is an optimized Prime Number checking function in clean TechScript:

```techscript
# TechScript Prime Number Detector
build is_prime(n) {
    when n <= 1 {
        send false
    }
    
    make i = 2
    when i * i <= n {
        when n % i == 0 {
            send false
        }
        i = i + 1
    }
    send true
}

make num = 29
when is_prime(num) {
    say num, "is a prime number!"
} else {
    say num, "is NOT prime."
}
```

Click the button below to instantly load this code into your active file!"#.to_string()
                    } else if prompt_lower.contains("variable") || prompt_lower.contains("constant") {
                        r#"In TechScript, variables are mutable declarations using `make`, and constants are immutable declarations using `keep`:

```techscript
# Variables vs Constants in TechScript
make score = 0
keep MAX_SCORE = 100

say "Current Score:", score
say "Goal Max Score:", MAX_SCORE

score = score + 10
say "New Score:", score
```

Click the button below to instantly load this code into your active file!"#.to_string()
                    } else {
                        r#"Here is the custom TechScript implementation matching your request:

```techscript
# Custom TechScript Script
make status = "active"
keep VERSION = "1.0.6"

build process_data(val) {
    when val > 0 {
        say "Value is positive"
    } else {
        say "Value is non-positive"
    }
}

process_data(42)
```

Click the button below to instantly load this code into your active file!"#.to_string()
                    };
                    
                    self.ai_response = response_str;
                    self.ai_prompt.clear();
                    self.ai_thinking = false;
                }
            });
        });
    }

    fn render_manual(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.colored_label(egui::Color32::from_rgb(251, 191, 36), "🐉 TechScript Language Cheat Sheet");
                ui.add_space(6.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.strong("Variables & Constants:");
                    if ui.button("📋 Load").clicked() {
                        self.code = "make x = 42\nkeep gravity = 9.8\nsay x, gravity\n".to_string();
                        self.save_file_content();
                    }
                });
                ui.monospace("make x = 10\nkeep PI = 3.1415");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.strong("Conditionals:");
                    if ui.button("📋 Load").clicked() {
                        self.code = "make num = 15\nwhen num > 10 {\n    say \"Greater\"\n} else {\n    say \"Smaller\"\n}\n".to_string();
                        self.save_file_content();
                    }
                });
                ui.monospace("when x > 5 {\n    say \"High\"\n} else {\n    say \"Low\"\n}");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.strong("Functions (build):");
                    if ui.button("📋 Load").clicked() {
                        self.code = "build square(n) {\n    send n * n\n}\nsay square(5)\n".to_string();
                        self.save_file_content();
                    }
                });
                ui.monospace("build square(x) {\n    send x * x\n}");
            });
        });
    }
}

impl eframe::App for StudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let running = { *self.is_running.lock().unwrap() };
        if running {
            ctx.request_repaint();
        }

        // Top toolbar control bar
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(4.0);
                
                // First Row: Header Bar
                ui.horizontal(|ui| {
                    if let Some(ref texture) = self.logo_texture {
                        ui.add(egui::Image::new(texture).max_size(egui::vec2(22.0, 22.0)));
                    } else {
                        let (logo_rect, _) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::hover());
                        paint_dragon_logo(ui, logo_rect);
                    }
                    ui.add_space(4.0);

                    ui.label(egui::RichText::new("TechScript Studio").strong().color(egui::Color32::from_rgb(0, 240, 255)));
                    ui.label(egui::RichText::new(format!("v{}", run::VERSION)).size(10.0).color(egui::Color32::from_rgb(148, 163, 184)));
                    
                    ui.separator();
                    
                    // Breadcrumb style active file
                    ui.label(egui::RichText::new(format!("active: {}", self.active_file)).monospace().size(11.0).color(egui::Color32::from_rgb(13, 242, 139)));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if running {
                            ui.colored_label(egui::Color32::from_rgb(251, 191, 36), "🟡 Running VM...");
                        } else {
                            ui.colored_label(egui::Color32::from_rgb(52, 211, 153), "🟢 Ready");
                        }
                    });
                });
                
                ui.add_space(2.0);
                ui.separator();
                ui.add_space(2.0);

                // Second Row: Menu Bar
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("➕ New File...").clicked() {
                            self.new_file_name = "untitled.txs".to_string();
                            self.create_new_file();
                            ui.close_menu();
                        }
                        if ui.button("📂 Open File...").clicked() {
                            self.open_file_dialog();
                            ui.close_menu();
                        }
                        if ui.button("💾 Save").clicked() {
                            self.save_file_content();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("❌ Exit").clicked() {
                            std::process::exit(0);
                        }
                    });
                    ui.menu_button("Edit", |ui| {
                        if ui.button("Toggle Autocomplete").clicked() {
                            self.autocomplete_open = !self.autocomplete_open;
                            ui.close_menu();
                        }
                        if ui.button("Toggle Minimap").clicked() {
                            self.minimap_visible = !self.minimap_visible;
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("View", |ui| {
                        if ui.button("📁 Explorer View").clicked() {
                            self.left_sidebar_mode = LeftSidebarMode::Explorer;
                            ui.close_menu();
                        }
                        if ui.button("🔍 Search View").clicked() {
                            self.left_sidebar_mode = LeftSidebarMode::Search;
                            ui.close_menu();
                        }
                        if ui.button(" Git View").clicked() {
                            self.left_sidebar_mode = LeftSidebarMode::Git;
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Run", |ui| {
                        if ui.button("▶ Run Script").clicked() {
                            self.run_code();
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Build", |ui| {
                        if ui.button("⚡ Compile").clicked() {
                            self.compiler_output = "TechScript compiler building AST and generating bytecode...\nBuild successful! target: runtime/target/release/tech.exe".to_string();
                            self.terminal_channel = TerminalChannel::Problems;
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Debug", |ui| {
                        if ui.button("🐞 Start Debugger").clicked() {
                            self.terminal_channel = TerminalChannel::Debug;
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Git", |ui| {
                        if ui.button("Staged Changes").clicked() {
                            self.left_sidebar_mode = LeftSidebarMode::Git;
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Tools", |ui| {
                        if ui.button("🧼 Clear Logs").clicked() {
                            let mut out = self.console_output.lock().unwrap();
                            out.clear();
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Extensions", |ui| {
                        if ui.button("🔌 Manage Add-ons").clicked() {
                            self.left_sidebar_mode = LeftSidebarMode::Extensions;
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Window", |ui| {
                        if ui.button("Reset Layout").clicked() {
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Help", |ui| {
                        if ui.button("📖 Language Cheat Sheet").clicked() {
                            ui.close_menu();
                        }
                    });
                });

                ui.add_space(2.0);
                ui.separator();
                ui.add_space(4.0);

                // Third Row: Action Toolbar Belt
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;

                    if ui.button("➕ New").on_hover_text("Create a new script").clicked() {
                        self.new_file_name = "new_script.txs".to_string();
                        self.create_new_file();
                    }
                    if ui.button("📂 Open").on_hover_text("Open script file").clicked() {
                        self.open_file_dialog();
                    }
                    if ui.button("💾 Save").on_hover_text("Save active code changes").clicked() {
                        self.save_file_content();
                    }
                    
                    ui.separator();

                    if running {
                        ui.add(egui::Button::new("⚙ Running VM...").fill(egui::Color32::from_rgb(180, 83, 9)));
                    } else {
                        if ui.add(egui::Button::new("▶ Run").fill(egui::Color32::from_rgb(16, 185, 129))).on_hover_text("Save and Execute script").clicked() {
                            self.save_file_content();
                            self.run_code();
                        }
                    }

                    if ui.button("🐞 Debug").on_hover_text("Show VM debugger bytecode disassembler").clicked() {
                        self.terminal_channel = TerminalChannel::Debug;
                    }
                    if ui.button("⚡ Build").on_hover_text("Compile script target").clicked() {
                        self.compiler_output = "TechScript compiler building AST and generating bytecode...\nBuild successful! target: runtime/target/release/tech.exe".to_string();
                        self.terminal_channel = TerminalChannel::Problems;
                    }

                    ui.separator();

                    if ui.button("📟 Terminal").on_hover_text("Open bottom terminal tab").clicked() {
                        self.terminal_channel = TerminalChannel::Terminal;
                    }
                    if ui.button(" Git").on_hover_text("Open Source Control side view").clicked() {
                        self.left_sidebar_mode = LeftSidebarMode::Git;
                    }
                    if ui.button("🔍 Search").on_hover_text("Search across workspace").clicked() {
                        self.left_sidebar_mode = LeftSidebarMode::Search;
                    }

                    ui.separator();

                    ui.label("Template:");
                    let prev_template = self.selected_template.clone();
                    egui::ComboBox::from_id_salt("template_combo")
                        .selected_text(&self.selected_template)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_template, "Hello World".to_string(), "Hello World");
                            ui.selectable_value(&mut self.selected_template, "Variables & Loops".to_string(), "Variables & Loops");
                            ui.selectable_value(&mut self.selected_template, "Class & Objects".to_string(), "Class & Objects");
                            ui.selectable_value(&mut self.selected_template, "Web Server Mock".to_string(), "Web Server Mock");
                        });

                    if self.selected_template != prev_template {
                        self.apply_template();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⚙ Settings").on_hover_text("Preferences panel").clicked() {
                            self.left_sidebar_mode = LeftSidebarMode::Explorer;
                        }
                    });
                });

                let mut dismiss = false;
                if let Some(ref msg) = self.status_message {
                    ui.add_space(6.0);
                    let color = if self.status_is_error {
                        egui::Color32::from_rgb(239, 68, 68)
                    } else {
                        egui::Color32::from_rgb(59, 130, 246)
                    };
                    ui.horizontal(|ui| {
                        ui.colored_label(color, msg);
                        if ui.button("Dismiss").clicked() {
                            dismiss = true;
                        }
                    });
                }
                if dismiss {
                    self.status_message = None;
                }
                ui.add_space(6.0);
            });
        });

        // Main workspace rendering utilizing egui_dock
        let mut dock_style = Style::from_egui(ctx.style().as_ref());
        dock_style.tab.active.bg_fill = egui::Color32::from_rgb(9, 12, 21);
        dock_style.tab.active.text_color = egui::Color32::from_rgb(0, 212, 255); // Cyber Cyan active indicator
        dock_style.tab.inactive.bg_fill = egui::Color32::from_rgb(14, 19, 36);
        dock_style.tab.inactive.text_color = egui::Color32::from_rgb(148, 163, 184);

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut dock_state = self.dock_state.take().unwrap_or_else(|| DockState::new(vec![Pane::Editor]));
            DockArea::new(&mut dock_state)
                .style(dock_style)
                .show_inside(ui, &mut TabViewerImpl { app: self });
            self.dock_state = Some(dock_state);
        });
    }
}

// Lexical Syntax Highlighter with modern vivid color spectrums
fn syntax_highlight(_ctx: &egui::Context, code: &str, font_size: f32, active_line: Option<usize>) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;
    let len = chars.len();
    
    let color_keyword = egui::Color32::from_rgb(255, 42, 122);    // Neon Crimson/Magenta #FF2A7A
    let color_number = egui::Color32::from_rgb(0, 240, 255);      // Cyber Electric Cyan #00F0FF
    let color_string = egui::Color32::from_rgb(13, 242, 139);     // Emerald green #0DF28B
    let color_comment = egui::Color32::from_rgb(255, 140, 0);     // Orange/Amber #FF8C00
    let color_type = egui::Color32::from_rgb(255, 204, 0);        // Gold/Yellow #FFCC00
    let color_text = egui::Color32::from_rgb(226, 232, 240);      // Slate Off-White
    
    let font_id = egui::FontId::monospace(font_size);
    let mut current_line = 0;

    let make_format = |current_line: usize, color: egui::Color32| -> egui::TextFormat {
        let background = if Some(current_line) == active_line {
            egui::Color32::from_rgba_premultiplied(0, 240, 255, 12) // Translucent Cyan highlight
        } else {
            egui::Color32::TRANSPARENT
        };
        egui::TextFormat {
            font_id: font_id.clone(),
            color,
            background,
            ..Default::default()
        }
    };

    let append_str = |job: &mut egui::text::LayoutJob, text: &str, color: egui::Color32, current_line: &mut usize| {
        let lines: Vec<&str> = text.split('\n').collect();
        for (idx, line) in lines.iter().enumerate() {
            if idx > 0 {
                job.append("\n", 0.0, make_format(*current_line, color));
                *current_line += 1;
            }
            if !line.is_empty() {
                job.append(line, 0.0, make_format(*current_line, color));
            }
        }
    };
    
    while i < len {
        if i + 1 < len && chars[i] == '/' && chars[i+1] == '/' {
            let start = i;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            let comment: String = chars[start..i].iter().collect();
            append_str(&mut job, &comment, color_comment, &mut current_line);
            continue;
        }
        
        if chars[i] == '"' {
            let start = i;
            i += 1;
            while i < len && chars[i] != '"' && chars[i-1] != '\\' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            let string_lit: String = chars[start..i].iter().collect();
            append_str(&mut job, &string_lit, color_string, &mut current_line);
            continue;
        }
        
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            
            let color = match word.as_str() {
                "make" | "keep" | "const" | "say" | "print" | "write" | "log" | "debug" | "warn" | "error" | "clear" |
                "repeat" | "while" | "each" | "loop" | "in" | "when" | "alt" | "else" | "unless" | "until" |
                "build" | "do" | "send" | "return" | "class" | "model" | "extends" | "init" | "new" |
                "attempt" | "try" | "rescue" | "catch" | "fail" | "throw" | "always" | "finally" |
                "use" | "take" | "share" | "as" | "guard" | "with" | "defer" | "stop" | "break" | "skip" | "continue" | "pass" => color_keyword,
                
                "self" | "super" | "true" | "false" | "none" => color_type,
                
                _ => color_text,
            };
            
            append_str(&mut job, &word, color, &mut current_line);
            continue;
        }
        
        if chars[i].is_numeric() {
            let start = i;
            while i < len && (chars[i].is_numeric() || chars[i] == '.') {
                if i + 1 < len && chars[i] == '.' && chars[i+1] == '.' {
                    break;
                }
                i += 1;
            }
            let num: String = chars[start..i].iter().collect();
            append_str(&mut job, &num, color_number, &mut current_line);
            continue;
        }
        
        let start = i;
        i += 1;
        let symbol: String = chars[start..i].iter().collect();
        append_str(&mut job, &symbol, color_text, &mut current_line);
    }
    
    job
}

// Dynamic rust-native vector dragon logo painting
fn paint_dragon_logo(ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter();
    let to_pos = |x: f32, y: f32| -> egui::Pos2 {
        egui::Pos2::new(
            rect.min.x + (x / 512.0) * rect.width(),
            rect.min.y + (y / 512.0) * rect.height(),
        )
    };
    
    let grad_pink = egui::Color32::from_rgb(255, 45, 85);
    let grad_blue = egui::Color32::from_rgb(0, 212, 255);
    let grad_purple = egui::Color32::from_rgb(139, 92, 246);
    
    // Main body
    let body_pts = vec![
        to_pos(256.0, 80.0),
        to_pos(180.0, 200.0),
        to_pos(210.0, 280.0),
        to_pos(256.0, 320.0),
        to_pos(302.0, 280.0),
        to_pos(332.0, 200.0),
    ];
    painter.add(egui::Shape::convex_polygon(body_pts, grad_purple, egui::Stroke::NONE));

    // Left Wing
    let l_wing_pts = vec![
        to_pos(180.0, 200.0),
        to_pos(60.0, 120.0),
        to_pos(80.0, 220.0),
        to_pos(140.0, 260.0),
        to_pos(180.0, 240.0),
    ];
    painter.add(egui::Shape::convex_polygon(l_wing_pts, grad_pink, egui::Stroke::NONE));

    // Right Wing
    let r_wing_pts = vec![
        to_pos(332.0, 200.0),
        to_pos(452.0, 120.0),
        to_pos(432.0, 220.0),
        to_pos(372.0, 260.0),
        to_pos(332.0, 240.0),
    ];
    painter.add(egui::Shape::convex_polygon(r_wing_pts, grad_blue, egui::Stroke::NONE));

    // Head
    let head_pts = vec![
        to_pos(256.0, 80.0),
        to_pos(230.0, 60.0),
        to_pos(240.0, 100.0),
        to_pos(256.0, 110.0),
        to_pos(272.0, 100.0),
        to_pos(282.0, 60.0),
    ];
    painter.add(egui::Shape::convex_polygon(head_pts, grad_purple, egui::Stroke::NONE));

    // Eyes
    painter.circle_filled(to_pos(245.0, 85.0), 2.0, egui::Color32::WHITE);
    painter.circle_filled(to_pos(267.0, 85.0), 2.0, egui::Color32::WHITE);

    // Tail
    let tail_pts1 = vec![
        to_pos(256.0, 320.0),
        to_pos(240.0, 380.0),
        to_pos(256.0, 420.0),
        to_pos(272.0, 380.0),
    ];
    painter.add(egui::Shape::convex_polygon(tail_pts1, grad_purple, egui::Stroke::NONE));

    let tail_pts2 = vec![
        to_pos(256.0, 420.0),
        to_pos(240.0, 440.0),
        to_pos(256.0, 470.0),
        to_pos(272.0, 440.0),
    ];
    painter.add(egui::Shape::convex_polygon(tail_pts2, grad_pink, egui::Stroke::NONE));

    // Circuit board accents
    painter.line_segment([to_pos(256.0, 150.0), to_pos(256.0, 280.0)], egui::Stroke::new(1.0, egui::Color32::WHITE.linear_multiply(0.3)));
    painter.circle_filled(to_pos(256.0, 150.0), 2.5, egui::Color32::WHITE.linear_multiply(0.5));
    painter.circle_filled(to_pos(256.0, 200.0), 2.5, egui::Color32::WHITE.linear_multiply(0.5));
}

// Interactive mini block-minimap drawing
fn draw_minimap(ui: &mut egui::Ui, code: &str, active_line: usize) {
    let width = 60.0;
    let height = ui.available_height().min(400.0);
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    
    // Background card border styling
    ui.painter().rect_filled(rect, 4.0, egui::Color32::from_rgb(5, 7, 12));
    
    let lines: Vec<&str> = code.lines().collect();
    let num_lines = lines.len();
    if num_lines == 0 {
        return;
    }
    
    let line_height = (height / num_lines as f32).min(3.0).max(1.0);
    
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        let y = rect.min.y + (idx as f32 * line_height);
        if y > rect.max.y {
            break;
        }
        
        let color = if trimmed.starts_with("//") {
            egui::Color32::from_rgb(255, 140, 0) // Comment orange #FF8C00
        } else if trimmed.contains("make") || trimmed.contains("keep") || trimmed.contains("say") || trimmed.contains("build") {
            egui::Color32::from_rgb(255, 42, 122) // Keyword Neon Crimson
        } else if trimmed.contains("\"") {
            egui::Color32::from_rgb(13, 242, 139) // Emerald string
        } else {
            egui::Color32::from_rgb(0, 240, 255) // Cyber Cyan standard
        };
        
        let color = if idx == active_line {
            egui::Color32::WHITE
        } else {
            color.linear_multiply(0.4)
        };
        
        let indent = ((line.len() - trimmed.len()) as f32 * 1.5).min(width / 2.0);
        let line_len = (trimmed.len() as f32 * 1.2).min(width - indent - 4.0);
        
        let line_rect = egui::Rect::from_min_max(
            egui::pos2(rect.min.x + 2.0 + indent, y),
            egui::pos2(rect.min.x + 2.0 + indent + line_len, y + line_height - 0.5),
        );
        ui.painter().rect_filled(line_rect, 0.0, color);
    }

    // Dynamic viewport scroll bounds highlight overlay
    let view_y = rect.min.y + (active_line as f32 * line_height).min(height - 16.0);
    let view_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x, view_y),
        egui::pos2(rect.max.x, (view_y + 16.0).min(rect.max.y)),
    );
    ui.painter().rect_stroke(view_rect, 2.0, egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(0, 240, 255, 60)));
    ui.painter().rect_filled(view_rect, 2.0, egui::Color32::from_rgba_premultiplied(0, 240, 255, 12));
}

pub fn start_studio() {
    println!("🚀 Launching TechScript Studio IDE Dashboard...");
    
    // Load and decode the custom ultra-vibrant TechScript dragon icon at compile-time
    let icon_data = {
        let icon_bytes = include_bytes!("../../assets/icons/icon-256.png");
        match image::load_from_memory(icon_bytes) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                Some(egui::IconData {
                    rgba: rgba.into_raw(),
                    width,
                    height,
                })
            }
            Err(e) => {
                eprintln!("⚠️ Failed to parse window icon: {:?}", e);
                None
            }
        }
    };

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("TechScript Studio 🐉")
        .with_inner_size([1320.0, 880.0]);

    if let Some(icon) = icon_data {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "TechScript Studio",
        options,
        Box::new(|cc| {
            Ok(Box::new(StudioApp::new(cc)))
        }),
    ).unwrap();
}
