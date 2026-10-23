// =============================================================================
// TechScript AppForge — Native TrueType Typography & Layout Engine
// runtime/android_runtime/src/typography.rs
//
// High-DPI subpixel-antialiased typography powered by fontdue and Android's
// native system fonts (/system/fonts/Roboto-Regular.ttf).
//
// Provides:
//   - Proportional TrueType font rasterization into a 512x512 GPU atlas
//   - Subpixel glyph metrics (advance, bearing, bounds, baseline)
//   - Accurate string measurement
//   - Responsive word-wrapping engine
// =============================================================================

use std::collections::HashMap;
use std::fs;

pub const ATLAS_WIDTH: usize = 512;
pub const ATLAS_HEIGHT: usize = 512;
pub const BASE_FONT_SIZE: f32 = 32.0;

#[derive(Clone, Copy, Debug)]
pub struct GlyphMetric {
    pub uv: [f32; 4],        // u0, v0, u1, v1 in atlas
    pub width: f32,          // pixel width in atlas
    pub height: f32,         // pixel height in atlas
    pub xmin: f32,           // horizontal bearing
    pub ymin: f32,           // vertical bearing
    pub advance_width: f32,  // horizontal advance
}

pub struct FontAtlas {
    pub atlas_rgba: Vec<u8>,
    pub metrics: HashMap<char, GlyphMetric>,
    pub line_height: f32,
    pub ascender: f32,
}

impl FontAtlas {
    /// Loads the system font and generates a 512x512 RGBA texture atlas with proportional glyphs.
    pub fn create_system_atlas() -> Self {
        // Attempt to load standard Android TrueType fonts in order of preference
        let font_paths = [
            "/system/fonts/Roboto-Regular.ttf",
            "/system/fonts/NotoSans-Regular.ttf",
            "/system/fonts/DroidSans.ttf",
            "/system/fonts/Roboto.ttf",
        ];

        let font_bytes = font_paths
            .iter()
            .find_map(|path| fs::read(path).ok())
            .unwrap_or_else(|| {
                log::warn!("[TechScript Font] System fonts not accessible, using built-in minimal fallback");
                Vec::new()
            });

        if font_bytes.is_empty() {
            return Self::create_fallback_atlas();
        }

        let font = match fontdue::Font::from_bytes(font_bytes, fontdue::FontSettings::default()) {
            Ok(f) => f,
            Err(e) => {
                log::error!("[TechScript Font] Failed to parse TrueType font: {e}");
                return Self::create_fallback_atlas();
            }
        };

        let mut atlas_rgba = vec![0u8; ATLAS_WIDTH * ATLAS_HEIGHT * 4];
        let mut metrics = HashMap::new();

        // Standard printable ASCII characters
        let mut cur_x = 2usize;
        let mut cur_y = 2usize;
        let mut row_max_h = 0usize;

        let chars_to_bake: Vec<char> = (32u8..=126u8).map(|b| b as char).collect();

        for ch in chars_to_bake {
            let (metric, bitmap) = font.rasterize(ch, BASE_FONT_SIZE);
            let w = metric.width;
            let h = metric.height;

            // Move to next row if overflowing horizontally
            if cur_x + w + 2 >= ATLAS_WIDTH {
                cur_x = 2;
                cur_y += row_max_h + 2;
                row_max_h = 0;
            }

            if cur_y + h + 2 >= ATLAS_HEIGHT {
                log::warn!("[TechScript Font] Atlas full, skipping further glyphs");
                break;
            }

            // Blit bitmap into atlas RGBA
            for row in 0..h {
                for col in 0..w {
                    let alpha = bitmap[row * w + col];
                    let dst_idx = ((cur_y + row) * ATLAS_WIDTH + (cur_x + col)) * 4;
                    atlas_rgba[dst_idx]     = 255;
                    atlas_rgba[dst_idx + 1] = 255;
                    atlas_rgba[dst_idx + 2] = 255;
                    atlas_rgba[dst_idx + 3] = alpha;
                }
            }

            let u0 = cur_x as f32 / ATLAS_WIDTH as f32;
            let v0 = cur_y as f32 / ATLAS_HEIGHT as f32;
            let u1 = (cur_x + w) as f32 / ATLAS_WIDTH as f32;
            let v1 = (cur_y + h) as f32 / ATLAS_HEIGHT as f32;

            metrics.insert(
                ch,
                GlyphMetric {
                    uv: [u0, v0, u1, v1],
                    width: w as f32,
                    height: h as f32,
                    xmin: metric.xmin as f32,
                    ymin: metric.ymin as f32,
                    advance_width: metric.advance_width,
                },
            );

            cur_x += w + 2;
            if h > row_max_h {
                row_max_h = h;
            }
        }

        let line_metrics = font.horizontal_line_metrics(BASE_FONT_SIZE);
        let (ascender, line_height) = match line_metrics {
            Some(m) => (m.ascent, m.new_line_size),
            None => (BASE_FONT_SIZE * 0.85, BASE_FONT_SIZE * 1.25),
        };

        log::info!("[TechScript Font] Successfully generated 512x512 TrueType atlas ({} glyphs, ascender={ascender:.1})", metrics.len());

        Self {
            atlas_rgba,
            metrics,
            line_height,
            ascender,
        }
    }

    /// Fallback font table in case system TTF is not accessible
    fn create_fallback_atlas() -> Self {
        let atlas_rgba = crate::font8x8::generate_font_atlas_rgba();
        let mut metrics = HashMap::new();

        for ch in 32u8..=126u8 {
            let c = ch as char;
            let (u0, v0, u1, v1) = crate::font8x8::get_char_uv(c);
            metrics.insert(
                c,
                GlyphMetric {
                    uv: [u0, v0, u1, v1],
                    width: 8.0,
                    height: 8.0,
                    xmin: 0.0,
                    ymin: 0.0,
                    advance_width: 8.0,
                },
            );
        }

        Self {
            atlas_rgba,
            metrics,
            line_height: 10.0,
            ascender: 8.0,
        }
    }

    /// Measures the exact pixel width of a string at a specified target font size.
    pub fn measure_text(&self, text: &str, target_size: f32) -> f32 {
        let scale = target_size / BASE_FONT_SIZE;
        let mut total_w = 0.0;
        for ch in text.chars() {
            if let Some(m) = self.metrics.get(&ch) {
                total_w += m.advance_width * scale;
            } else if ch == ' ' {
                total_w += (BASE_FONT_SIZE * 0.35) * scale;
            } else {
                total_w += (BASE_FONT_SIZE * 0.5) * scale;
            }
        }
        total_w
    }

    /// Splits text into lines that each fit within max_width_px at target_size.
    pub fn wrap_text(&self, text: &str, max_width_px: f32, target_size: f32) -> Vec<String> {
        let mut lines = Vec::new();
        for paragraph in text.split('\n') {
            if paragraph.is_empty() {
                lines.push(String::new());
                continue;
            }

            let words: Vec<&str> = paragraph.split(' ').collect();
            let mut current_line = String::new();

            for word in words {
                let candidate = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{current_line} {word}")
                };

                if self.measure_text(&candidate, target_size) <= max_width_px {
                    current_line = candidate;
                } else {
                    if !current_line.is_empty() {
                        lines.push(current_line);
                    }
                    current_line = word.to_string();
                }
            }

            if !current_line.is_empty() {
                lines.push(current_line);
            }
        }
        lines
    }
}
