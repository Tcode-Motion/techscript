// ── Native GUI Module (eframe/egui) ──────────────────────────────────
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::value::{NativeFnObj, Value};

thread_local! {
    static GUI_CTX: RefCell<GuiBuilder> = RefCell::new(GuiBuilder::default());
}

#[derive(Default)]
struct GuiBuilder {
    title: String,
    labels: Vec<String>,
    buttons: Vec<(String, String)>, // label, click handler (say messages joined by |)
    inputs: Vec<(String, String)>,
    #[allow(dead_code)]
    pending_handler: Option<String>,
}

pub fn register(globals: &mut HashMap<String, Value>) {
    for (name, func) in [
        ("__gui_window", gui_window as fn(&[Value]) -> Result<Value, String>),
        ("__gui_button", gui_button),
        ("__gui_input", gui_input),
        ("__gui_label", gui_label),
        ("__gui_run", gui_run),
    ] {
        globals.insert(
            name.into(),
            Value::NativeFunction(Rc::new(NativeFnObj {
                name: name.to_string(),
                func: Box::new(func),
            })),
        );
    }
}

fn gui_window(args: &[Value]) -> Result<Value, String> {
    let title = args.first().map(|v| v.display_string()).unwrap_or_else(|| "App".into());
    GUI_CTX.with(|ctx| ctx.borrow_mut().title = title);
    Ok(Value::None)
}

fn gui_button(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("gui_button needs label".into());
    }
    let label = args[0].display_string();
    let handler = args.get(1).map(|v| v.display_string()).unwrap_or_default();
    GUI_CTX.with(|ctx| ctx.borrow_mut().buttons.push((label, handler)));
    Ok(Value::None)
}

fn gui_input(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("gui_input needs name".into());
    }
    let name = args[0].display_string();
    let placeholder = args.get(1).map(|v| v.display_string()).unwrap_or_default();
    GUI_CTX.with(|ctx| ctx.borrow_mut().inputs.push((name, placeholder)));
    Ok(Value::None)
}

fn gui_label(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("gui_label needs text".into());
    }
    GUI_CTX.with(|ctx| ctx.borrow_mut().labels.push(args[0].display_string()));
    Ok(Value::None)
}

fn gui_run(_args: &[Value]) -> Result<Value, String> {
    if std::env::var("TECHSCRIPT_GUI_TEST").is_ok() {
        println!("[gui] window run skipped (TECHSCRIPT_GUI_TEST)");
        return Ok(Value::None);
    }
    let builder = GUI_CTX.with(|ctx| ctx.borrow().clone_snapshot());
    run_gui_app(builder);
    Ok(Value::None)
}

impl GuiBuilder {
    fn clone_snapshot(&self) -> GuiSnapshot {
        GuiSnapshot {
            title: self.title.clone(),
            labels: self.labels.clone(),
            buttons: self.buttons.clone(),
            inputs: self.inputs.clone(),
        }
    }
}

struct GuiSnapshot {
    title: String,
    labels: Vec<String>,
    buttons: Vec<(String, String)>, // label, click handler (say messages joined by |)
    inputs: Vec<(String, String)>,
}

struct GuiApp {
    snapshot: GuiSnapshot,
    input_values: HashMap<String, String>,
    messages: Arc<Mutex<Vec<String>>>,
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.snapshot.title);
            for label in &self.snapshot.labels {
                ui.label(label);
            }
            for (name, placeholder) in &self.snapshot.inputs {
                let val = self.input_values.entry(name.clone()).or_default();
                ui.horizontal(|ui| {
                    ui.label(name);
                    ui.text_edit_singleline(val).on_hover_text(placeholder);
                });
            }
            for (label, handler) in &self.snapshot.buttons {
                if ui.button(label).clicked() {
                    if handler.is_empty() {
                        self.messages.lock().unwrap().push(format!("Clicked: {}", label));
                        println!("Button clicked: {}", label);
                    } else {
                        for msg in handler.split('|') {
                            if !msg.is_empty() {
                                println!("{}", msg);
                                self.messages.lock().unwrap().push(msg.to_string());
                            }
                        }
                    }
                }
            }
            let msgs = self.messages.lock().unwrap().clone();
            for msg in msgs {
                ui.label(&msg);
            }
        });
    }
}

fn run_gui_app(snapshot: GuiSnapshot) {
    let title = snapshot.title.clone();
    let messages = Arc::new(Mutex::new(Vec::new()));
    let msgs = messages.clone();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title(title),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "TechScript GUI",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(GuiApp {
                snapshot: snapshot.clone(),
                input_values: HashMap::new(),
                messages: msgs,
            }))
        }),
    );
}

impl Clone for GuiSnapshot {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            labels: self.labels.clone(),
            buttons: self.buttons.clone(),
            inputs: self.inputs.clone(),
        }
    }
}
