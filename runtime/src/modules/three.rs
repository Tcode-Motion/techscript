// ── Native 3D Module (minimal scene MVP) ─────────────────────────────
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::{NativeFnObj, Value};

thread_local! {
    static THREE_CTX: RefCell<SceneBuilder> = RefCell::new(SceneBuilder::default());
}

#[derive(Default, Clone)]
struct SceneBuilder {
    name: String,
    camera_pos: [f32; 3],
    meshes: Vec<(String, String)>,
    has_light: bool,
}

pub fn register(globals: &mut HashMap<String, Value>) {
    for (name, func) in [
        ("__3d_scene", scene_begin as fn(&[Value]) -> Result<Value, String>),
        ("__3d_camera", scene_camera),
        ("__3d_light", scene_light),
        ("__3d_mesh", scene_mesh),
        ("__3d_run", scene_run),
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

fn scene_begin(args: &[Value]) -> Result<Value, String> {
    let name = args.first().map(|v| v.display_string()).unwrap_or_else(|| "world".into());
    THREE_CTX.with(|ctx| {
        *ctx.borrow_mut() = SceneBuilder {
            name,
            ..Default::default()
        };
    });
    Ok(Value::None)
}

fn scene_camera(args: &[Value]) -> Result<Value, String> {
    if args.len() >= 3 {
        THREE_CTX.with(|ctx| {
            let mut c = ctx.borrow_mut();
            c.camera_pos = [
                args[0].as_f64().unwrap_or(0.0) as f32,
                args[1].as_f64().unwrap_or(0.0) as f32,
                args[2].as_f64().unwrap_or(5.0) as f32,
            ];
        });
    }
    Ok(Value::None)
}

fn scene_light(_args: &[Value]) -> Result<Value, String> {
    THREE_CTX.with(|ctx| ctx.borrow_mut().has_light = true);
    Ok(Value::None)
}

fn scene_mesh(args: &[Value]) -> Result<Value, String> {
    let shape = args.first().map(|v| v.display_string()).unwrap_or_else(|| "cube".into());
    let color = args.get(1).map(|v| v.display_string()).unwrap_or_else(|| "#7c3aed".into());
    THREE_CTX.with(|ctx| ctx.borrow_mut().meshes.push((shape, color)));
    Ok(Value::None)
}

fn scene_run(_args: &[Value]) -> Result<Value, String> {
    if std::env::var("TECHSCRIPT_3D_TEST").is_ok() {
        println!("[3d] scene run skipped (TECHSCRIPT_3D_TEST)");
        return Ok(Value::None);
    }
    let scene = THREE_CTX.with(|ctx| ctx.borrow().clone());
    run_3d_viewer(scene);
    Ok(Value::None)
}

struct ThreeApp {
    scene: SceneBuilder,
    angle: f32,
}

impl eframe::App for ThreeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.angle += 0.02;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("3D Scene: {}", self.scene.name));
            ui.label(format!(
                "Camera: [{:.1}, {:.1}, {:.1}]",
                self.scene.camera_pos[0], self.scene.camera_pos[1], self.scene.camera_pos[2]
            ));
            if self.scene.has_light {
                ui.label("Light: ambient");
            }
            let (rect, _) = ui.allocate_exact_size(egui::vec2(400.0, 400.0), egui::Sense::hover());
            let painter = ui.painter_at(rect);
            let center = rect.center();
            let scale = 80.0;
            for (shape, color) in &self.scene.meshes {
                let col = parse_color(color);
                let x = center.x + self.angle.cos() * scale;
                let y = center.y + self.angle.sin() * 20.0;
                match shape.as_str() {
                    "cube" => {
                        let size = egui::vec2(60.0, 60.0);
                        painter.rect_filled(
                            egui::Rect::from_center_size(egui::pos2(x, y), size),
                            4.0,
                            col,
                        );
                    }
                    _ => {
                        painter.circle_filled(egui::pos2(x, y), 30.0, col);
                    }
                }
            }
            ui.label("Rotating preview — full 3D engine in future releases");
        });
        ctx.request_repaint();
    }
}

fn parse_color(s: &str) -> egui::Color32 {
    if s.starts_with('#') && s.len() >= 7 {
        let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(124);
        let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(58);
        let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(237);
        egui::Color32::from_rgb(r, g, b)
    } else {
        egui::Color32::from_rgb(124, 58, 237)
    }
}

fn run_3d_viewer(scene: SceneBuilder) {
    let title = format!("TechScript 3D — {}", scene.name);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title(title),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "TechScript 3D",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(ThreeApp {
                scene: scene.clone(),
                angle: 0.0,
            }))
        }),
    );
}
