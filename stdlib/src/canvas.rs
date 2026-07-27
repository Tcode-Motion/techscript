use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    context::RuntimeContext, error::RuntimeError, function::Callable, value::RuntimeValue,
};

/// Convert a DslBlockValue to SVG string.
fn dsl_to_svg(val: &RuntimeValue, is_dragon: bool) -> String {
    match val {
        RuntimeValue::DslBlock(dsl) => {
            let mut svg = String::new();
            if is_dragon {
                match dsl.kind.as_str() {
                    "logo" => {
                        let text = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "text")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "Logo".to_string());
                        let color = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "color")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "#00d4ff".to_string());
                        let size = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "size")
                            .and_then(|p| p.value.as_ref())
                            .and_then(|v| v.try_into_int().ok())
                            .unwrap_or(48);
                        svg.push_str(&format!(
                            r#"<text x="250" y="440" text-anchor="middle" dominant-baseline="central" font-size="{}" font-weight="800" fill="{}" font-family="system-ui, -apple-system, sans-serif" letter-spacing="3">{}</text>"#,
                            size, color, text
                        ));
                    }
                    "rings" => {
                        // BLUE FLAME / WING (Bottom-Left Swirl)
                        svg.push_str(
                            r##"<path d="M 142 240 L 133 248 L 124 260 L 124 262 L 122 264 L 120 270 L 118 272 L 118 275 L 116 278 L 115 284 L 115 308 L 119 323 L 122 328 L 122 330 L 128 342 L 134 351 L 142 361 L 154 373 L 179 390 L 181 390 L 190 395 L 195 396 L 198 398 L 207 401 L 224 405 L 233 406 L 260 406 L 266 405 L 257 404 L 238 398 L 224 390 L 215 382 L 209 374 L 203 361 L 200 347 L 200 342 L 202 337 L 202 332 L 205 321 L 207 319 L 207 317 L 209 315 L 212 309 L 215 307 L 216 304 L 220 301 L 224 296 L 242 284 L 244 284 L 252 279 L 254 279 L 266 273 L 268 273 L 270 271 L 277 269 L 279 267 L 281 267 L 289 262 L 299 253 L 301 250 L 302 245 L 297 248 L 290 250 L 260 252 L 254 251 L 247 253 L 240 253 L 229 255 L 213 260 L 192 272 L 181 284 L 176 292 L 176 294 L 174 296 L 173 300 L 171 301 L 167 294 L 164 284 L 163 267 L 165 256 L 169 247 L 169 245 L 173 237 L 182 224 L 178 224 L 162 229 L 157 232 L 155 232 L 150 236 L 148 236 L 145 239 Z" fill="url(#blueFlame)" />"##
                        );
                    }
                    "emblem" => {
                        // ORANGE-RED TAIL (Bottom-Right Swirl)
                        svg.push_str(
                            r##"<path d="M 389 251 L 388 255 L 386 257 L 384 256 L 382 254 L 381 262 L 379 260 L 379 258 L 378 256 L 378 260 L 377 267 L 378 271 L 377 273 L 379 278 L 380 306 L 379 313 L 377 318 L 378 320 L 376 328 L 374 331 L 374 335 L 376 333 L 376 331 L 379 326 L 379 324 L 383 317 L 385 311 L 385 308 L 387 305 L 390 292 L 391 283 L 390 281 L 392 273 L 391 270 L 391 266 L 392 262 L 391 265 L 389 260 Z" fill="url(#orangeFlame)" />
                            <path d="M 372 252 L 372 254 L 370 257 L 368 258 L 366 256 L 353 257 L 349 255 L 344 257 L 339 257 L 336 254 L 333 253 L 333 251 L 332 253 L 334 255 L 337 256 L 335 258 L 330 258 L 328 261 L 328 265 L 326 265 L 310 281 L 307 282 L 302 286 L 290 292 L 288 292 L 286 294 L 281 295 L 275 299 L 273 299 L 265 303 L 263 305 L 260 306 L 247 317 L 244 321 L 242 326 L 239 330 L 237 337 L 236 347 L 239 361 L 241 365 L 243 367 L 244 370 L 251 378 L 254 379 L 258 383 L 260 383 L 267 387 L 269 387 L 272 389 L 289 392 L 298 391 L 300 392 L 308 391 L 322 387 L 327 384 L 329 384 L 339 377 L 341 377 L 350 370 L 358 361 L 356 361 L 351 364 L 345 366 L 335 368 L 320 368 L 316 367 L 312 365 L 316 361 L 319 360 L 330 351 L 335 345 L 338 343 L 345 334 L 353 322 L 353 320 L 355 318 L 359 310 L 359 308 L 363 301 L 368 285 L 370 276 L 371 267 L 370 265 L 370 263 L 372 262 L 372 260 L 370 259 L 372 258 L 373 256 Z" fill="url(#orangeFlame)" />"##
                        );
                    }
                    "letter" => {
                        // GOLDEN DRAGON HEAD (Top-Center Crest)
                        svg.push_str(
                            r##"<path d="M 118 196 L 120 205 L 127 214 L 137 220 L 150 222 L 146 216 L 147 213 L 165 205 L 180 204 L 188 206 L 190 205 L 197 208 L 198 212 L 191 232 L 191 240 L 201 230 L 221 217 L 244 208 L 255 207 L 259 205 L 275 207 L 285 211 L 298 222 L 302 230 L 304 227 L 304 213 L 299 201 L 290 191 L 282 185 L 261 176 L 274 172 L 292 172 L 301 175 L 316 184 L 331 202 L 337 221 L 336 223 L 338 225 L 338 235 L 335 249 L 330 260 L 322 269 L 326 265 L 328 266 L 333 262 L 335 265 L 337 263 L 366 263 L 368 260 L 371 266 L 372 252 L 376 261 L 378 276 L 379 271 L 382 267 L 390 263 L 391 273 L 392 261 L 389 239 L 384 222 L 377 207 L 367 194 L 367 192 L 355 180 L 336 167 L 346 165 L 357 166 L 383 173 L 367 157 L 344 144 L 325 138 L 303 134 L 315 127 L 330 112 L 338 99 L 342 87 L 342 82 L 330 95 L 309 109 L 285 120 L 275 122 L 273 124 L 267 123 L 276 115 L 285 103 L 292 86 L 294 70 L 282 87 L 273 96 L 251 113 L 215 133 L 182 147 L 174 156 L 170 164 L 160 174 Z" fill="url(#dragonGold)" />
                            <!-- DRAGON EYE (Hole/Slit) -->
                            <path d="M 216 157 L 216 158 L 215 159 L 214 159 L 214 160 L 212 162 L 212 163 L 211 164 L 211 165 L 210 166 L 210 167 L 206 171 L 205 171 L 205 172 L 203 174 L 202 174 L 201 175 L 200 175 L 199 176 L 196 176 L 195 177 L 192 177 L 191 176 L 190 176 L 189 177 L 185 177 L 183 175 L 183 174 L 187 170 L 187 169 L 188 168 L 189 168 L 191 166 L 192 166 L 194 164 L 196 164 L 197 163 L 198 163 L 199 162 L 200 162 L 201 161 L 203 161 L 204 160 L 205 160 L 206 159 L 210 159 L 212 157 L 214 157 L 215 156 Z" fill="#030408" />"##
                        );
                    }
                    "circuits" => {
                        // Subtle circular grids / tech lines background
                        svg.push_str(
                            r##"<circle cx="250" cy="250" r="200" fill="none" stroke="#ffffff" stroke-width="1" opacity="0.03" stroke-dasharray="10 10"/>
                            <circle cx="250" cy="250" r="150" fill="none" stroke="#ffffff" stroke-width="1" opacity="0.02"/>"##
                        );
                    }
                    _ => {}
                }
            } else {
                // RENDER OLD GEOMETRIC LOGO
                match dsl.kind.as_str() {
                    "logo" => {
                        let text = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "text")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "Logo".to_string());
                        let color = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "color")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "#00d4ff".to_string());
                        let size = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "size")
                            .and_then(|p| p.value.as_ref())
                            .and_then(|v| v.try_into_int().ok())
                            .unwrap_or(48);
                        svg.push_str(&format!(
                            r#"<text x="250" y="380" text-anchor="middle" dominant-baseline="central" font-size="{}" font-weight="800" fill="{}" font-family="system-ui, -apple-system, sans-serif" letter-spacing="3">{}</text>"#,
                            size, color, text
                        ));
                    }
                    "rings" => {
                        let count = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "count")
                            .and_then(|p| p.value.as_ref())
                            .and_then(|v| v.try_into_int().ok())
                            .unwrap_or(3);
                        let color = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "color")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "#00d4ff".to_string());
                        let size = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "size")
                            .and_then(|p| p.value.as_ref())
                            .and_then(|v| v.try_into_int().ok())
                            .unwrap_or(40);
                        let thickness = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "thickness")
                            .and_then(|p| p.value.as_ref())
                            .and_then(|v| v.try_into_int().ok())
                            .unwrap_or(3);
                        for i in 0..count {
                            let r = 80 + i as i64 * (size / 2);
                            let opacity = 0.4 - (i as f32 * 0.08);
                            svg.push_str(&format!(
                                r#"<circle cx="250" cy="180" r="{}" fill="none" stroke="{}" stroke-width="{}" opacity="{}"/>"#,
                                r, color, thickness, opacity
                            ));
                        }
                    }
                    "emblem" => {
                        let color = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "color")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "#0088cc".to_string());
                        let size = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "size")
                            .and_then(|p| p.value.as_ref())
                            .and_then(|v| v.try_into_int().ok())
                            .unwrap_or(120);
                        let x = 250 - size / 2;
                        let y = 180 - size / 2;
                        svg.push_str(&format!(
                            r#"<rect x="{}" y="{}" width="{}" height="{}" rx="{}" fill="{}" transform="rotate(45 250 180)" filter="url(#glow)"/>"#,
                            x, y, size, size, size / 4, color
                        ));
                    }
                    "letter" => {
                        let ch = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "char")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "T".to_string());
                        let color = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "color")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "#0a0e27".to_string());
                        let size = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "size")
                            .and_then(|p| p.value.as_ref())
                            .and_then(|v| v.try_into_int().ok())
                            .unwrap_or(32);
                        svg.push_str(&format!(
                            r#"<text x="250" y="180" text-anchor="middle" dominant-baseline="central" font-size="{}" font-weight="900" fill="{}" font-family="system-ui, -apple-system, sans-serif">{}</text>"#,
                            size, color, ch
                        ));
                    }
                    "core" => {
                        let color = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "color")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "#66e0ff".to_string());
                        let size = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "size")
                            .and_then(|p| p.value.as_ref())
                            .and_then(|v| v.try_into_int().ok())
                            .unwrap_or(40);
                        svg.push_str(&format!(
                            r#"<circle cx="250" cy="180" r="{}" fill="{}" opacity="0.8"/>"#,
                            size / 2,
                            color
                        ));
                    }
                    "circuits" => {
                        let color = dsl
                            .properties
                            .iter()
                            .find(|p| p.name == "color")
                            .and_then(|p| p.value.as_ref())
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "#00d4ff".to_string());
                        svg.push_str(&format!(
                            r#"<line x1="50" y1="180" x2="450" y2="180" stroke="{}" stroke-width="1.5" stroke-dasharray="5 5" opacity="0.6"/>
                            <line x1="250" y1="30" x2="250" y2="330" stroke="{}" stroke-width="1.5" stroke-dasharray="5 5" opacity="0.6"/>
                            <circle cx="50" cy="180" r="4" fill="{}"/>
                            <circle cx="450" cy="180" r="4" fill="{}"/>
                            <circle cx="250" cy="30" r="4" fill="{}"/>
                            <circle cx="250" cy="330" r="4" fill="{}"/>"#,
                            color, color, color, color, color, color
                        ));
                    }
                    _ => {}
                }
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
    Create,
    Background,
    Rect,
    Circle,
    Text,
    Polygon,
    Line,
    Close,
    Save,
    Reset,
    Content,
    Append,
    Stroke,
}

impl Callable for CanvasFn {
    fn name(&self) -> &str {
        &self.name
    }
    fn arity(&self) -> usize {
        self.min_arity
    }
    fn accepts_arity(&self, count: usize) -> bool {
        count >= self.min_arity && count <= self.max_arity
    }
    fn call(
        &self,
        _ctx: &mut RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        let mut buf = self.buffer.borrow_mut();
        match self.kind {
            CanvasOp::Create => {
                let w = args[0].try_into_int().unwrap_or(800);
                let h = args[1].try_into_int().unwrap_or(600);
                *buf = format!(
                    r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
                    w, h, w, h
                );
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Background => {
                let w = parse_width(&buf);
                let h = parse_height(&buf);
                let fill = args[0].to_string();
                buf.push_str(&format!(
                    r#"<rect x="0" y="0" width="{}" height="{}" fill="{}"/>"#,
                    w, h, fill
                ));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Rect => {
                let x = args[0].try_into_int().unwrap_or(0);
                let y = args[1].try_into_int().unwrap_or(0);
                let w = args[2].try_into_int().unwrap_or(100);
                let h = args[3].try_into_int().unwrap_or(100);
                let fill = args[4].to_string();
                buf.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
                    x, y, w, h, fill
                ));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Circle => {
                let cx = args[0].try_into_int().unwrap_or(50);
                let cy = args[1].try_into_int().unwrap_or(50);
                let r = args[2].try_into_int().unwrap_or(40);
                let fill = args[3].to_string();
                buf.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="{}" fill="{}"/>"#,
                    cx, cy, r, fill
                ));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Text => {
                let x = args[0].try_into_int().unwrap_or(10);
                let y = args[1].try_into_int().unwrap_or(30);
                let txt = args[2].to_string();
                let size = if args.len() > 3 {
                    args[3].try_into_int().unwrap_or(16)
                } else {
                    16
                };
                let color = if args.len() > 4 {
                    args[4].to_string()
                } else {
                    "black".to_string()
                };
                buf.push_str(&format!(r#"<text x="{}" y="{}" font-size="{}" font-family="Arial,sans-serif" fill="{}">{}</text>"#, x, y, size, color, txt));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Polygon => {
                let points = args[0].to_string();
                let fill = args[1].to_string();
                buf.push_str(&format!(
                    r#"<polygon points="{}" fill="{}"/>"#,
                    points, fill
                ));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Line => {
                let x1 = args[0].try_into_int().unwrap_or(0);
                let y1 = args[1].try_into_int().unwrap_or(0);
                let x2 = args[2].try_into_int().unwrap_or(100);
                let y2 = args[3].try_into_int().unwrap_or(100);
                let stroke = args[4].to_string();
                buf.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="2"/>"#,
                    x1, y1, x2, y2, stroke
                ));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Close => {
                buf.push_str("</svg>");
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Save => {
                let path = args[0].to_string();
                let content = buf.clone();
                std::fs::write(&path, &content).map_err(|e| {
                    RuntimeError::new(
                        techscript_runtime::error::RuntimeErrorKind::InvalidOperation(
                            e.to_string(),
                        ),
                        None,
                        None,
                    )
                })?;
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
                let width = if args.len() > 4 {
                    args[4].to_string()
                } else {
                    "2".to_string()
                };
                let opacity = if args.len() > 5 {
                    args[5].to_string()
                } else {
                    "1".to_string()
                };
                buf.push_str(&format!(r#"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="{}" stroke-width="{}" opacity="{}"/>"#, cx, cy, r, color, width, opacity));
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Reset => {
                *buf = String::new();
                Ok(RuntimeValue::Null)
            }
            CanvasOp::Content => Ok(RuntimeValue::Str(buf.clone())),
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
        exports.insert(
            "create".to_string(),
            Rc::new(CanvasFn {
                name: "create".into(),
                min_arity: 2,
                max_arity: 2,
                buffer: b,
                kind: CanvasOp::Create,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "background".to_string(),
            Rc::new(CanvasFn {
                name: "background".into(),
                min_arity: 1,
                max_arity: 1,
                buffer: b,
                kind: CanvasOp::Background,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "rect".to_string(),
            Rc::new(CanvasFn {
                name: "rect".into(),
                min_arity: 5,
                max_arity: 5,
                buffer: b,
                kind: CanvasOp::Rect,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "circle".to_string(),
            Rc::new(CanvasFn {
                name: "circle".into(),
                min_arity: 4,
                max_arity: 4,
                buffer: b,
                kind: CanvasOp::Circle,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "text".to_string(),
            Rc::new(CanvasFn {
                name: "text".into(),
                min_arity: 3,
                max_arity: 5,
                buffer: b,
                kind: CanvasOp::Text,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "polygon".to_string(),
            Rc::new(CanvasFn {
                name: "polygon".into(),
                min_arity: 2,
                max_arity: 2,
                buffer: b,
                kind: CanvasOp::Polygon,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "line".to_string(),
            Rc::new(CanvasFn {
                name: "line".into(),
                min_arity: 5,
                max_arity: 5,
                buffer: b,
                kind: CanvasOp::Line,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "close".to_string(),
            Rc::new(CanvasFn {
                name: "close".into(),
                min_arity: 0,
                max_arity: 0,
                buffer: b,
                kind: CanvasOp::Close,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "save".to_string(),
            Rc::new(CanvasFn {
                name: "save".into(),
                min_arity: 1,
                max_arity: 1,
                buffer: b,
                kind: CanvasOp::Save,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "append".to_string(),
            Rc::new(CanvasFn {
                name: "append".into(),
                min_arity: 1,
                max_arity: 1,
                buffer: b,
                kind: CanvasOp::Append,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "stroke".to_string(),
            Rc::new(CanvasFn {
                name: "stroke".into(),
                min_arity: 4,
                max_arity: 6,
                buffer: b,
                kind: CanvasOp::Stroke,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "reset".to_string(),
            Rc::new(CanvasFn {
                name: "reset".into(),
                min_arity: 0,
                max_arity: 0,
                buffer: b,
                kind: CanvasOp::Reset,
            }),
        );
        let b = buffer.clone();
        exports.insert(
            "content".to_string(),
            Rc::new(CanvasFn {
                name: "content".into(),
                min_arity: 0,
                max_arity: 0,
                buffer: b,
                kind: CanvasOp::Content,
            }),
        );
        exports.insert(
            "size".to_string(),
            Rc::new(CanvasFn {
                name: "size".into(),
                min_arity: 2,
                max_arity: 2,
                buffer: buffer.clone(),
                kind: CanvasOp::Create,
            }),
        );

        // DSL block rendering: convert DSL blocks to SVG
        exports.insert(
            "render_dsl".to_string(),
            Rc::new(StdFunction {
                name: "render_dsl".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let svg = dsl_to_svg(&args[0], false);
                    Ok(RuntimeValue::Str(svg))
                },
            }),
        );

        exports.insert("render_blocks".to_string(), Rc::new(StdFunction {
            name: "render_blocks".to_string(),
            arity: 0,
            callback: |ctx, _args| {
                let env = ctx.global_env.borrow();
                let blocks = match env.lookup("_dsl_blocks") {
                    Ok(RuntimeValue::List { items, .. }) => items.borrow().clone(),
                    _ => return Ok(RuntimeValue::Str(String::new())),
                };
                let is_dragon = blocks.iter().any(|block| {
                    if let RuntimeValue::DslBlock(dsl) = block {
                        if dsl.kind == "logo" {
                            if let Some(arg) = dsl.args.first() {
                                if arg.to_string() == "dragon" {
                                    return true;
                                }
                            }
                        }
                    }
                    false
                });
                let bg_color = if is_dragon { "#030408" } else { "#0a0e27" };

                let mut svg = String::new();
                svg.push_str(&format!(r##"<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500" viewBox="0 0 500 500">
  <defs>
    <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
      <feGaussianBlur stdDeviation="6" result="blur" />
      <feComposite in="SourceGraphic" in2="blur" operator="over" />
    </filter>
    <linearGradient id="dragonGold" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#FFF000" />
      <stop offset="40%" stop-color="#FFB800" />
      <stop offset="100%" stop-color="#FF5C00" />
    </linearGradient>
    <linearGradient id="blueFlame" x1="0%" y1="100%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#0055FF" />
      <stop offset="70%" stop-color="#00D4FF" />
      <stop offset="100%" stop-color="#80F3FF" />
    </linearGradient>
    <linearGradient id="orangeFlame" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#FF8A00" />
      <stop offset="60%" stop-color="#FF3D00" />
      <stop offset="100%" stop-color="#E50000" />
    </linearGradient>
  </defs>
  <rect width="100%" height="100%" fill="{}"/>
"##, bg_color));
                for block in &blocks {
                    svg.push_str(&dsl_to_svg(block, is_dragon));
                }
                svg.push_str("</svg>");
                Ok(RuntimeValue::Str(svg))
            },
        }));

        self.register_module(
            "std.canvas",
            StdlibModule {
                name: "std.canvas".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
