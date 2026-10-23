// =============================================================================
// TechScript AppForge — 2D Liquid Glass Shader & Batch Renderer
// runtime/android_runtime/src/glass_ui.rs
//
// Hardware-accelerated OpenGL ES 2.0 rendering of liquid glass panels,
// frosted cards, specular highlights, vector icons, and font text quads.
// =============================================================================

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UIVertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
    pub local_pos: [f32; 2],
    pub size: [f32; 2],
    pub radius: f32,
    pub border_width: f32,
    pub is_text: f32,
}

pub struct GlassBatch {
    pub vertices: Vec<UIVertex>,
    pub indices: Vec<u16>,
}

impl GlassBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::with_capacity(4096),
            indices: Vec::with_capacity(6144),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    /// Add a rounded rectangle (liquid glass panel, button, card, or pill)
    pub fn push_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: [f32; 4],
        border_width: f32,
    ) {
        let base_idx = self.vertices.len() as u16;
        let hw = w * 0.5;
        let hh = h * 0.5;

        // 4 quad vertices centered around (0,0) locally for SDF
        self.vertices.push(UIVertex {
            pos: [x, y],
            uv: [0.0, 0.0],
            color,
            local_pos: [-hw, -hh],
            size: [w, h],
            radius,
            border_width,
            is_text: 0.0,
        });
        self.vertices.push(UIVertex {
            pos: [x + w, y],
            uv: [1.0, 0.0],
            color,
            local_pos: [hw, -hh],
            size: [w, h],
            radius,
            border_width,
            is_text: 0.0,
        });
        self.vertices.push(UIVertex {
            pos: [x + w, y + h],
            uv: [1.0, 1.0],
            color,
            local_pos: [hw, hh],
            size: [w, h],
            radius,
            border_width,
            is_text: 0.0,
        });
        self.vertices.push(UIVertex {
            pos: [x, y + h],
            uv: [0.0, 1.0],
            color,
            local_pos: [-hw, hh],
            size: [w, h],
            radius,
            border_width,
            is_text: 0.0,
        });

        self.indices.push(base_idx);
        self.indices.push(base_idx + 1);
        self.indices.push(base_idx + 2);
        self.indices.push(base_idx);
        self.indices.push(base_idx + 2);
        self.indices.push(base_idx + 3);
    }

    /// Draws a modern vector Home icon (roof + walls + door)
    pub fn draw_icon_home(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        let hw = size * 0.42;
        let hh = size * 0.35;
        // Roof cap
        self.push_rect(cx - hw, cy - hh - size * 0.10, hw * 2.0, size * 0.28, size * 0.08, color, 0.0);
        // Base walls
        self.push_rect(cx - hw * 0.85, cy - hh + size * 0.05, hw * 1.7, hh * 2.0, size * 0.10, color, 0.0);
        // Inner door
        self.push_rect(cx - size * 0.14, cy + size * 0.08, size * 0.28, size * 0.38, size * 0.06, [0.06, 0.08, 0.14, 0.95], 0.0);
    }

    /// Draws a modern vector Explorer compass/grid icon
    pub fn draw_icon_explorer(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4], accent: [f32; 4]) {
        let r = size * 0.44;
        // Outer ring
        self.push_rect(cx - r, cy - r, r * 2.0, r * 2.0, r, [0.0, 0.0, 0.0, 0.0], 1.5);
        // North needle (primary)
        self.push_rect(cx - size * 0.09, cy - size * 0.36, size * 0.18, size * 0.36, size * 0.06, color, 0.0);
        // South needle (accent cyan)
        self.push_rect(cx - size * 0.09, cy, size * 0.18, size * 0.36, size * 0.06, accent, 0.0);
        // Center pivot
        self.push_rect(cx - size * 0.10, cy - size * 0.10, size * 0.20, size * 0.20, size * 0.10, [1.0, 1.0, 1.0, 1.0], 0.0);
    }

    /// Draws a modern vector Labs 3D wireframe / prism icon
    pub fn draw_icon_labs(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4], accent: [f32; 4]) {
        let s = size * 0.40;
        // Outer prism
        self.push_rect(cx - s, cy - s, s * 2.0, s * 2.0, size * 0.15, color, 1.6);
        // Inner core
        self.push_rect(cx - s * 0.45, cy - s * 0.45, s * 0.9, s * 0.9, size * 0.08, accent, 0.0);
    }

    /// Draws a modern vector Projects code folder icon
    pub fn draw_icon_projects(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4], accent: [f32; 4]) {
        let fw = size * 0.44;
        let fh = size * 0.32;
        // Tab
        self.push_rect(cx - fw, cy - fh - size * 0.12, fw * 0.9, size * 0.20, size * 0.06, color, 0.0);
        // Folder base
        self.push_rect(cx - fw, cy - fh, fw * 2.0, fh * 2.0, size * 0.10, color, 0.0);
        // Code tag inner mark
        self.push_rect(cx - size * 0.18, cy - size * 0.05, size * 0.36, size * 0.16, size * 0.06, accent, 0.0);
    }

    /// Draws a modern vector Docs document icon
    pub fn draw_icon_docs(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        let dw = size * 0.34;
        let dh = size * 0.44;
        // Sheet
        self.push_rect(cx - dw, cy - dh, dw * 2.0, dh * 2.0, size * 0.10, color, 1.5);
        // 3 document lines
        self.push_rect(cx - dw * 0.6, cy - dh * 0.4, dw * 1.2, size * 0.07, size * 0.035, color, 0.0);
        self.push_rect(cx - dw * 0.6, cy - dh * 0.05, dw * 1.2, size * 0.07, size * 0.035, color, 0.0);
        self.push_rect(cx - dw * 0.6, cy + dh * 0.3, dw * 0.8, size * 0.07, size * 0.035, color, 0.0);
    }

    /// Draws the official TechScript Brand Logo Mark
    pub fn draw_techscript_logo_mark(&mut self, cx: f32, cy: f32, size: f32) {
        let r = size * 0.50;
        // Outer glowing indigo diamond
        self.push_rect(cx - r, cy - r, r * 2.0, r * 2.0, size * 0.26, [0.388, 0.400, 0.945, 0.95], 1.5);
        // Inner cyan facet
        self.push_rect(cx - r * 0.62, cy - r * 0.62, r * 1.24, r * 1.24, size * 0.14, [0.133, 0.827, 0.933, 0.85], 0.0);
        // Center code slash
        self.push_rect(cx - size * 0.05, cy - size * 0.22, size * 0.10, size * 0.44, size * 0.05, [1.0, 1.0, 1.0, 1.0], 0.0);
    }

    /// Add a single text glyph quad
    pub fn push_glyph(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        uv: (f32, f32, f32, f32),
        color: [f32; 4],
    ) {
        let base_idx = self.vertices.len() as u16;

        self.vertices.push(UIVertex {
            pos: [x, y],
            uv: [uv.0, uv.1],
            color,
            local_pos: [0.0, 0.0],
            size: [w, h],
            radius: 0.0,
            border_width: 0.0,
            is_text: 1.0,
        });
        self.vertices.push(UIVertex {
            pos: [x + w, y],
            uv: [uv.2, uv.1],
            color,
            local_pos: [0.0, 0.0],
            size: [w, h],
            radius: 0.0,
            border_width: 0.0,
            is_text: 1.0,
        });
        self.vertices.push(UIVertex {
            pos: [x + w, y + h],
            uv: [uv.2, uv.3],
            color,
            local_pos: [0.0, 0.0],
            size: [w, h],
            radius: 0.0,
            border_width: 0.0,
            is_text: 1.0,
        });
        self.vertices.push(UIVertex {
            pos: [x, y + h],
            uv: [uv.0, uv.3],
            color,
            local_pos: [0.0, 0.0],
            size: [w, h],
            radius: 0.0,
            border_width: 0.0,
            is_text: 1.0,
        });

        self.indices.push(base_idx);
        self.indices.push(base_idx + 1);
        self.indices.push(base_idx + 2);
        self.indices.push(base_idx);
        self.indices.push(base_idx + 2);
        self.indices.push(base_idx + 3);
    }

    /// Render high-DPI TrueType text using the dynamic font atlas with proportional advance
    pub fn push_text_ttf(
        &mut self,
        font_atlas: &crate::typography::FontAtlas,
        x: f32,
        y: f32,
        text: &str,
        font_size: f32,
        color: [f32; 4],
    ) -> f32 {
        let font_scale = font_size / crate::typography::BASE_FONT_SIZE;
        let mut cur_x = x;

        for ch in text.chars() {
            if ch == '\n' {
                continue;
            }
            if let Some(m) = font_atlas.metrics.get(&ch) {
                if m.width > 0.0 && m.height > 0.0 {
                    let gx = cur_x + m.xmin * font_scale;
                    let gy = y + (font_atlas.ascender - (m.ymin + m.height)) * font_scale;
                    let gw = m.width * font_scale;
                    let gh = m.height * font_scale;
                    self.push_glyph(gx, gy, gw, gh, (m.uv[0], m.uv[1], m.uv[2], m.uv[3]), color);
                }
                cur_x += m.advance_width * font_scale;
            } else if ch == ' ' {
                cur_x += (crate::typography::BASE_FONT_SIZE * 0.30) * font_scale;
            } else {
                cur_x += (crate::typography::BASE_FONT_SIZE * 0.45) * font_scale;
            }
        }
        cur_x - x
    }

    /// Render a string of text at (x, y) with a given scale multiplier (fallback)
    pub fn push_text(
        &mut self,
        x: f32,
        y: f32,
        text: &str,
        scale: f32,
        color: [f32; 4],
    ) {
        let gw = 8.0 * scale;
        let gh = 8.0 * scale;
        let line_height = gh + 4.0 * scale;
        let mut cur_x = x;
        let mut cur_y = y;

        for ch in text.chars() {
            if ch == '\n' {
                cur_x = x;
                cur_y += line_height;
                continue;
            }
            let uv = crate::font8x8::get_char_uv(ch);
            self.push_glyph(cur_x, cur_y, gw, gh, uv, color);
            cur_x += gw;
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GLES 2.0 Shader Source Code
// ─────────────────────────────────────────────────────────────────────────────

pub const UI_VS_SOURCE: &str = r#"
attribute vec2 a_pos;
attribute vec2 a_uv;
attribute vec4 a_color;
attribute vec2 a_local_pos;
attribute vec2 a_size;
attribute float a_radius;
attribute float a_border_width;
attribute float a_is_text;

uniform vec2 u_screen_size;

varying vec2 v_uv;
varying vec4 v_color;
varying vec2 v_local_pos;
varying vec2 v_size;
varying float v_radius;
varying float v_border_width;
varying float v_is_text;

void main() {
    // Transform screen coords (0..w, 0..h) to OpenGL clip space (-1..1, 1..-1)
    vec2 ndc = (a_pos / u_screen_size) * 2.0 - 1.0;
    ndc.y = -ndc.y; // Flip Y so (0,0) is top-left
    gl_Position = vec4(ndc, 0.0, 1.0);

    v_uv = a_uv;
    v_color = a_color;
    v_local_pos = a_local_pos;
    v_size = a_size;
    v_radius = a_radius;
    v_border_width = a_border_width;
    v_is_text = a_is_text;
}
"#;

pub const UI_FS_SOURCE: &str = r#"
precision mediump float;

varying vec2 v_uv;
varying vec4 v_color;
varying vec2 v_local_pos;
varying vec2 v_size;
varying float v_radius;
varying float v_border_width;
varying float v_is_text;

uniform sampler2D u_font_texture;

void main() {
    if (v_is_text > 0.5) {
        // High-DPI TrueType font rendering with subpixel antialiasing
        float a = texture2D(u_font_texture, v_uv).a;
        if (a < 0.03) discard;
        gl_FragColor = vec4(v_color.rgb, v_color.a * a);
    } else {
        // Liquid glass rounded card with SDF
        vec2 half_size = v_size * 0.5;
        vec2 d = abs(v_local_pos) - half_size + vec2(v_radius);
        float dist = length(max(d, 0.0)) + min(max(d.x, d.y), 0.0) - v_radius;

        // Antialiased border cut
        float alpha = clamp(0.5 - dist, 0.0, 1.0);
        if (alpha <= 0.0) discard;

        // Specular highlight rim at top of glass surface
        float top_rim = clamp((-v_local_pos.y + half_size.y) / max(v_size.y * 0.35, 1.0), 0.0, 1.0);
        float highlight = pow(top_rim, 2.5) * 0.30;

        // Luminous border outline with subtle specular refraction
        vec4 col = v_color;
        if (v_border_width > 0.1) {
            float b_dist = dist + v_border_width;
            float is_border = clamp(b_dist, 0.0, 1.0);
            col = mix(col, vec4(col.rgb + vec3(0.28, 0.32, 0.40), clamp(col.a + 0.40, 0.0, 1.0)), is_border * 0.6);
        }

        col.rgb += vec3(highlight);
        gl_FragColor = vec4(col.rgb, col.a * alpha);
    }
}
"#;

pub const MESH3D_VS_SOURCE: &str = r#"
attribute vec3 a_pos;
attribute vec3 a_normal;
attribute vec4 a_color;

uniform mat4 u_mvp;
uniform mat4 u_model;

varying vec3 v_normal;
varying vec4 v_color;
varying vec3 v_world_pos;

void main() {
    gl_Position = u_mvp * vec4(a_pos, 1.0);
    v_world_pos = (u_model * vec4(a_pos, 1.0)).xyz;
    v_normal = normalize((u_model * vec4(a_normal, 0.0)).xyz);
    v_color = a_color;
}
"#;

pub const MESH3D_FS_SOURCE: &str = r#"
precision mediump float;

varying vec3 v_normal;
varying vec4 v_color;
varying vec3 v_world_pos;

uniform vec3 u_light_dir;
uniform vec3 u_specular_col;
uniform vec3 u_view_pos;

void main() {
    vec3 n = normalize(v_normal);
    vec3 l = normalize(u_light_dir);
    vec3 v = normalize(u_view_pos - v_world_pos);
    vec3 h = normalize(l + v);

    // Ambient + Diffuse (Faceted jewel shading)
    float diff = max(dot(n, l), 0.38);
    float spec = pow(max(dot(n, h), 0.0), 28.0);

    // Fresnel glass reflection rim
    float fresnel = pow(1.0 - max(dot(n, v), 0.0), 2.2) * 0.45;

    vec3 rgb = v_color.rgb * diff + u_specular_col * spec * 0.85 + vec3(0.85, 0.92, 1.0) * fresnel;
    gl_FragColor = vec4(rgb, 1.0);
}
"#;
