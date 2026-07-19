use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use techscript_runtime::{
    context::RuntimeContext, error::RuntimeError, function::Callable, value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

/// Convert a DslBlockValue to SVG string.
fn dsl_to_svg(val: &RuntimeValue) -> String {
    match val {
        RuntimeValue::DslBlock(dsl) => {
            let mut svg = String::new();
            match dsl.kind.as_str() {
                "logo" => {
                    let text = dsl.properties.iter()
                        .find(|p| p.name == "text")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "Logo".to_string());
                    let color = dsl.properties.iter()
                        .find(|p| p.name == "color")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "#333".to_string());
                    let size = dsl.properties.iter()
                        .find(|p| p.name == "size")
                        .and_then(|p| p.value.as_ref())
                        .and_then(|v| v.try_into_int().ok())
                        .unwrap_or(48);
                    svg.push_str(&format!(
                        r#"<text x="50%" y="50%" text-anchor="middle" dominant-baseline="central" font-size="{}" fill="{}" font-family="Arial,sans-serif">{}</text>"#,
                        size, color, text
                    ));
                }
                "rings" => {
                    let count = dsl.properties.iter()
                        .find(|p| p.name == "count")
                        .and_then(|p| p.value.as_ref())
                        .and_then(|v| v.try_into_int().ok())
                        .unwrap_or(3);
                    let color = dsl.properties.iter()
                        .find(|p| p.name == "color")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "#666".to_string());
                    let size = dsl.properties.iter()
                        .find(|p| p.name == "size")
                        .and_then(|p| p.value.as_ref())
                        .and_then(|v| v.try_into_int().ok())
                        .unwrap_or(80);
                    let thickness = dsl.properties.iter()
                        .find(|p| p.name == "thickness")
                        .and_then(|p| p.value.as_ref())
                        .and_then(|v| v.try_into_int().ok())
                        .unwrap_or(6);
                    for i in 0..count {
                        let offset = i as i64 * (size + 10);
                        svg.push_str(&format!(
                            r#"<circle cx="{}" cy="50%" r="{}" fill="none" stroke="{}" stroke-width="{}"/>"#,
                            size / 2 + offset, size / 2, color, thickness
                        ));
                    }
                }
                "emblem" => {
                    let color = dsl.properties.iter()
                        .find(|p| p.name == "color")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "#007bff".to_string());
                    let size = dsl.properties.iter()
                        .find(|p| p.name == "size")
                        .and_then(|p| p.value.as_ref())
                        .and_then(|v| v.try_into_int().ok())
                        .unwrap_or(64);
                    svg.push_str(&format!(
                        r#"<rect x="0" y="0" width="{}" height="{}" rx="{}" fill="{}"/>"#,
                        size, size, size / 4, color
                    ));
                }
                "letter" => {
                    let ch = dsl.properties.iter()
                        .find(|p| p.name == "char")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "T".to_string());
                    let color = dsl.properties.iter()
                        .find(|p| p.name == "color")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "#fff".to_string());
                    let size = dsl.properties.iter()
                        .find(|p| p.name == "size")
                        .and_then(|p| p.value.as_ref())
                        .and_then(|v| v.try_into_int().ok())
                        .unwrap_or(32);
                    svg.push_str(&format!(
                        r#"<text x="50%" y="50%" text-anchor="middle" dominant-baseline="central" font-size="{}" fill="{}">{}</text>"#,
                        size, color, ch
                    ));
                }
                "core" => {
                    let color = dsl.properties.iter()
                        .find(|p| p.name == "color")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "#007bff".to_string());
                    let size = dsl.properties.iter()
                        .find(|p| p.name == "size")
                        .and_then(|p| p.value.as_ref())
                        .and_then(|v| v.try_into_int().ok())
                        .unwrap_or(40);
                    svg.push_str(&format!(
                        r#"<circle cx="{}" cy="{}" r="{}" fill="{}"/>"#,
                        size / 2, size / 2, size / 2, color
                    ));
                }
                "circuits" => {
                    let color = dsl.properties.iter()
                        .find(|p| p.name == "color")
                        .and_then(|p| p.value.as_ref())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "#00ff88".to_string());
                    let width = dsl.properties.iter()
                        .find(|p| p.name == "width")
                        .and_then(|p| p.value.as_ref())
                        .and_then(|v| v.try_into_int().ok())
                        .unwrap_or(200);
                    svg.push_str(&format!(
                        r#"<rect x="0" y="0" width="{}" height="100" fill="none" stroke="{}" stroke-width="1" stroke-dasharray="4 4"/>"#,
                        width, color
                    ));
                }
                _ => {}
            }
            svg
        }
        _ => String::new(),
    }
}

struct CanvasFn {
    name: String,
    min_arity: usize,
    max_arity: usize,
    buffer: Rc<RefCell<String>>,
    kind: CanvasOp,
}

enum CanvasOp {
    Create, Background, Rect, Circle, Text, Polygon, Line,
    Close, Save, Reset, Content, Append, Stroke,
}

impl Callable for CanvasFn {
    fn name(&self) -> &str { &self.name }
    fn arity(&self) -> usize { self.min_arity }
    fn accepts_arity(&self, count: usize) -> bool {
        count >= self.min_arity && count <= self.max_arity
    }
    fn call(&self, _ctx: &mut RuntimeContext, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
        let mut buf = self.buffer.borrow_mut();
        match self.kind {
            CanvasOp::Create => {
                let w = args[0].try_into_int().unwrap_or(800);
                let h = args[1].try_into_int().unwrap_or(600);
                *buf = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#, w, h, w, h);
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Background => {
                let w = parse_width(&buf);
                let h = parse_height(&buf);
                let fill = args[0].to_string();
                buf.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" fill="{}"/>"#, w, h, fill));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Rect => {
                let x = args[0].try_into_int().unwrap_or(0);
                let y = args[1].try_into_int().unwrap_or(0);
                let w = args[2].try_into_int().unwrap_or(100);
                let h = args[3].try_into_int().unwrap_or(100);
                let fill = args[4].to_string();
                buf.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#, x, y, w, h, fill));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Circle => {
                let cx = args[0].try_into_int().unwrap_or(50);
                let cy = args[1].try_into_int().unwrap_or(50);
                let r = args[2].try_into_int().unwrap_or(40);
                let fill = args[3].to_string();
                buf.push_str(&format!(r#"<circle cx="{}" cy="{}" r="{}" fill="{}"/>"#, cx, cy, r, fill));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Text => {
                let x = args[0].try_into_int().unwrap_or(10);
                let y = args[1].try_into_int().unwrap_or(30);
                let txt = args[2].to_string();
                let size = if args.len() > 3 { args[3].try_into_int().unwrap_or(16) } else { 16 };
                let color = if args.len() > 4 { args[4].to_string() } else { "black".to_string() };
                buf.push_str(&format!(r#"<text x="{}" y="{}" font-size="{}" font-family="Arial,sans-serif" fill="{}">{}</text>"#, x, y, size, color, txt));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Polygon => {
                let points = args[0].to_string();
                let fill = args[1].to_string();
                buf.push_str(&format!(r#"<polygon points="{}" fill="{}"/>"#, points, fill));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Line => {
                let x1 = args[0].try_into_int().unwrap_or(0);
                let y1 = args[1].try_into_int().unwrap_or(0);
                let x2 = args[2].try_into_int().unwrap_or(100);
                let y2 = args[3].try_into_int().unwrap_or(100);
                let stroke = args[4].to_string();
                buf.push_str(&format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="2"/>"#, x1, y1, x2, y2, stroke));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Close => {
                buf.push_str("</svg>");
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Save => {
                let path = args[0].to_string();
                let content = buf.clone();
                std::fs::write(&path, &content)
                    .map_err(|e| RuntimeError::new(techscript_runtime::error::RuntimeErrorKind::InvalidOperation(e.to_string()), None, None))?;
                Ok(RuntimeValue::Str(format!("Saved to {}", path)))
            }
            CanvasOp::Append => {
                buf.push_str(&args[0].to_string());
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Stroke => {
                let cx = args[0].try_into_int().unwrap_or(50);
                let cy = args[1].try_into_int().unwrap_or(50);
                let r = args[2].try_into_int().unwrap_or(40);
                let color = args[3].to_string();
                let width = if args.len() > 4 { args[4].to_string() } else { "2".to_string() };
                let opacity = if args.len() > 5 { args[5].to_string() } else { "1".to_string() };
                buf.push_str(&format!(r#"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="{}" stroke-width="{}" opacity="{}"/>"#, cx, cy, r, color, width, opacity));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Reset => {
                *buf = String::new();
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Content => {
                Ok(RuntimeValue::Str(buf.clone()))
            }
        }
    }
}

fn parse_width(svg: &str) -> i64 {
    if let Some(start) = svg.find("width=\"") {
        let rest = &svg[start + 7..];
        if let Some(end) = rest.find('"') {
            return rest[..end].parse().unwrap_or(800);
        }
    }
    800
}

fn parse_height(svg: &str) -> i64 {
    if let Some(start) = svg.find("height=\"") {
        let rest = &svg[start + 8..];
        if let Some(end) = rest.find('"') {
            return rest[..end].parse().unwrap_or(600);
        }
    }
    600
}

impl StdlibRegistry {
    pub fn register_canvas(&mut self) {
        let buffer: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
        let mut exports: HashMap<String, Rc<dyn Callable>> = HashMap::new();

        let b = buffer.clone();
        exports.insert("create".to_string(), Rc::new(CanvasFn { name: "create".into(), min_arity: 2, max_arity: 2, buffer: b, kind: CanvasOp::Create }));
        let b = buffer.clone();
        exports.insert("background".to_string(), Rc::new(CanvasFn { name: "background".into(), min_arity: 1, max_arity: 1, buffer: b, kind: CanvasOp::Background }));
        let b = buffer.clone();
        exports.insert("rect".to_string(), Rc::new(CanvasFn { name: "rect".into(), min_arity: 5, max_arity: 5, buffer: b, kind: CanvasOp::Rect }));
        let b = buffer.clone();
        exports.insert("circle".to_string(), Rc::new(CanvasFn { name: "circle".into(), min_arity: 4, max_arity: 4, buffer: b, kind: CanvasOp::Circle }));
        let b = buffer.clone();
        exports.insert("text".to_string(), Rc::new(CanvasFn { name: "text".into(), min_arity: 3, max_arity: 5, buffer: b, kind: CanvasOp::Text }));
        let b = buffer.clone();
        exports.insert("polygon".to_string(), Rc::new(CanvasFn { name: "polygon".into(), min_arity: 2, max_arity: 2, buffer: b, kind: CanvasOp::Polygon }));
        let b = buffer.clone();
        exports.insert("line".to_string(), Rc::new(CanvasFn { name: "line".into(), min_arity: 5, max_arity: 5, buffer: b, kind: CanvasOp::Line }));
        let b = buffer.clone();
        exports.insert("close".to_string(), Rc::new(CanvasFn { name: "close".into(), min_arity: 0, max_arity: 0, buffer: b, kind: CanvasOp::Close }));
        let b = buffer.clone();
        exports.insert("save".to_string(), Rc::new(CanvasFn { name: "save".into(), min_arity: 1, max_arity: 1, buffer: b, kind: CanvasOp::Save }));
        let b = buffer.clone();
        exports.insert("append".to_string(), Rc::new(CanvasFn { name: "append".into(), min_arity: 1, max_arity: 1, buffer: b, kind: CanvasOp::Append }));
        let b = buffer.clone();
        exports.insert("stroke".to_string(), Rc::new(CanvasFn { name: "stroke".into(), min_arity: 4, max_arity: 6, buffer: b, kind: CanvasOp::Stroke }));
        let b = buffer.clone();
        exports.insert("reset".to_string(), Rc::new(CanvasFn { name: "reset".into(), min_arity: 0, max_arity: 0, buffer: b, kind: CanvasOp::Reset }));
        let b = buffer.clone();
        exports.insert("content".to_string(), Rc::new(CanvasFn { name: "content".into(), min_arity: 0, max_arity: 0, buffer: b, kind: CanvasOp::Content }));
        exports.insert("size".to_string(), Rc::new(CanvasFn { name: "size".into(), min_arity: 2, max_arity: 2, buffer: buffer.clone(), kind: CanvasOp::Create }));

        // DSL block rendering: convert DSL blocks to SVG
        exports.insert("render_dsl".to_string(), Rc::new(StdFunction {
            name: "render_dsl".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let svg = dsl_to_svg(&args[0]);
                Ok(RuntimeValue::Str(svg))
            },
        }));

        exports.insert("render_blocks".to_string(), Rc::new(StdFunction {
            name: "render_blocks".to_string(),
            arity: 0,
            callback: |ctx, _args| {
                let env = ctx.global_env.borrow();
                let blocks = match env.lookup("_dsl_blocks") {
                    Ok(RuntimeValue::List { items, .. }) => items.borrow().clone(),
                    _ => return Ok(RuntimeValue::Str(String::new())),
                };
                let mut svg = String::new();
                for block in &blocks {
                    svg.push_str(&dsl_to_svg(block));
                }
                Ok(RuntimeValue::Str(svg))
            },
        }));

        self.register_module("std.canvas", StdlibModule {
            name: "std.canvas".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
