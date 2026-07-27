use crate::{StdFunction, StdlibModule, StdlibRegistry};
use image::{Rgba, RgbaImage};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    context::Capability, error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue,
};

impl StdlibRegistry {
    pub fn register_graphics(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "create_canvas".to_string(),
            Rc::new(StdFunction {
                name: "create_canvas".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    let w = args[0].try_into_int()? as u32;
                    let h = args[1].try_into_int()? as u32;

                    let img = RgbaImage::new(w, h);
                    let handle = ctx.resources.borrow_mut().insert(RefCell::new(img));
                    Ok(RuntimeValue::Int(handle as i64))
                },
            }),
        );

        exports.insert(
            "draw_rect".to_string(),
            Rc::new(StdFunction {
                name: "draw_rect".to_string(),
                arity: 6,
                callback: |ctx, args| {
                    let handle = args[0].try_into_int()? as u32;
                    let x = args[1].try_into_int()?;
                    let y = args[2].try_into_int()?;
                    let w = args[3].try_into_int()?;
                    let h = args[4].try_into_int()?;
                    let color_hex = args[5].try_into_string()?;

                    let color = parse_color(&color_hex);

                    let resources = ctx.resources.borrow();
                    let img_cell =
                        resources.get::<RefCell<RgbaImage>>(handle).ok_or_else(|| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "Invalid canvas resource handle: {}",
                                    handle
                                )),
                                None,
                                None,
                            )
                        })?;

                    let mut img = img_cell.borrow_mut();
                    for px in x..(x + w) {
                        for py in y..(y + h) {
                            if px >= 0
                                && px < img.width() as i64
                                && py >= 0
                                && py < img.height() as i64
                            {
                                img.put_pixel(px as u32, py as u32, color);
                            }
                        }
                    }

                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "draw_circle".to_string(),
            Rc::new(StdFunction {
                name: "draw_circle".to_string(),
                arity: 5,
                callback: |ctx, args| {
                    let handle = args[0].try_into_int()? as u32;
                    let cx = args[1].try_into_int()?;
                    let cy = args[2].try_into_int()?;
                    let r = args[3].try_into_int()?;
                    let color_hex = args[4].try_into_string()?;

                    let color = parse_color(&color_hex);

                    let resources = ctx.resources.borrow();
                    let img_cell =
                        resources.get::<RefCell<RgbaImage>>(handle).ok_or_else(|| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "Invalid canvas resource handle: {}",
                                    handle
                                )),
                                None,
                                None,
                            )
                        })?;

                    let mut img = img_cell.borrow_mut();
                    for px in (cx - r)..(cx + r) {
                        for py in (cy - r)..(cy + r) {
                            let dx = px - cx;
                            let dy = py - cy;
                            if dx * dx + dy * dy <= r * r {
                                if px >= 0
                                    && px < img.width() as i64
                                    && py >= 0
                                    && py < img.height() as i64
                                {
                                    img.put_pixel(px as u32, py as u32, color);
                                }
                            }
                        }
                    }

                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "draw_line".to_string(),
            Rc::new(StdFunction {
                name: "draw_line".to_string(),
                arity: 6,
                callback: |ctx, args| {
                    let handle = args[0].try_into_int()? as u32;
                    let x1 = args[1].try_into_int()?;
                    let y1 = args[2].try_into_int()?;
                    let x2 = args[3].try_into_int()?;
                    let y2 = args[4].try_into_int()?;
                    let color_hex = args[5].try_into_string()?;

                    let color = parse_color(&color_hex);

                    let resources = ctx.resources.borrow();
                    let img_cell =
                        resources.get::<RefCell<RgbaImage>>(handle).ok_or_else(|| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "Invalid canvas resource handle: {}",
                                    handle
                                )),
                                None,
                                None,
                            )
                        })?;

                    let mut img = img_cell.borrow_mut();
                    let dx = (x2 - x1).abs();
                    let dy = (y2 - y1).abs();
                    let sx = if x1 < x2 { 1 } else { -1 };
                    let sy = if y1 < y2 { 1 } else { -1 };
                    let mut err = dx - dy;

                    let mut cx = x1;
                    let mut cy = y1;

                    loop {
                        if cx >= 0 && cx < img.width() as i64 && cy >= 0 && cy < img.height() as i64
                        {
                            img.put_pixel(cx as u32, cy as u32, color);
                        }
                        if cx == x2 && cy == y2 {
                            break;
                        }
                        let e2 = 2 * err;
                        if e2 > -dy {
                            err -= dy;
                            cx += sx;
                        }
                        if e2 < dx {
                            err += dx;
                            cy += sy;
                        }
                    }

                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "save_png".to_string(),
            Rc::new(StdFunction {
                name: "save_png".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let handle = args[0].try_into_int()? as u32;
                    let path = args[1].try_into_string()?;

                    let resources = ctx.resources.borrow();
                    let img_cell =
                        resources.get::<RefCell<RgbaImage>>(handle).ok_or_else(|| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "Invalid canvas resource handle: {}",
                                    handle
                                )),
                                None,
                                None,
                            )
                        })?;

                    let img = img_cell.borrow();
                    img.save_with_format(&path, image::ImageFormat::Png)
                        .map_err(|e| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "Failed to save PNG: {}",
                                    e
                                )),
                                None,
                                None,
                            )
                        })?;

                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "save_jpeg".to_string(),
            Rc::new(StdFunction {
                name: "save_jpeg".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let handle = args[0].try_into_int()? as u32;
                    let path = args[1].try_into_string()?;

                    let resources = ctx.resources.borrow();
                    let img_cell =
                        resources.get::<RefCell<RgbaImage>>(handle).ok_or_else(|| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "Invalid canvas resource handle: {}",
                                    handle
                                )),
                                None,
                                None,
                            )
                        })?;

                    let img = img_cell.borrow();
                    img.save_with_format(&path, image::ImageFormat::Jpeg)
                        .map_err(|e| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "Failed to save JPEG: {}",
                                    e
                                )),
                                None,
                                None,
                            )
                        })?;

                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.graphics",
            StdlibModule {
                name: "std.graphics".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}

fn parse_color(hex: &str) -> Rgba<u8> {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 {
        return Rgba([0, 0, 0, 255]);
    }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    let a = if hex.len() >= 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
    } else {
        255
    };
    Rgba([r, g, b, a])
}
