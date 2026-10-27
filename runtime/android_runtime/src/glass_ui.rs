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

// Compile-time assertion that UIVertex layout matches OpenGL vertex attribute pointers
const _: () = assert!(std::mem::size_of::<UIVertex>() == 60);

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

    /// Draws a modern vector Home icon (gable roof + walls + door)
    pub fn draw_icon_home(
        &mut self,
        cx: f32,
        cy: f32,
        size: f32,
        color: [f32; 4],
        bg_color: [f32; 4],
    ) {
        // Roof peak and eaves
        self.push_rect(
            cx - size * 0.44,
            cy - size * 0.16,
            size * 0.88,
            size * 0.13,
            size * 0.065,
            color,
            0.0,
        );
        self.push_rect(
            cx - size * 0.28,
            cy - size * 0.32,
            size * 0.56,
            size * 0.16,
            size * 0.08,
            color,
            0.0,
        );
        self.push_rect(
            cx - size * 0.14,
            cy - size * 0.44,
            size * 0.28,
            size * 0.15,
            size * 0.075,
            color,
            0.0,
        );
        // Base house body
        self.push_rect(
            cx - size * 0.36,
            cy - size * 0.08,
            size * 0.72,
            size * 0.50,
            size * 0.10,
            color,
            0.0,
        );
        // Inner door matches surface background
        self.push_rect(
            cx - size * 0.12,
            cy + size * 0.08,
            size * 0.24,
            size * 0.34,
            size * 0.07,
            bg_color,
            0.0,
        );
    }

    /// Draws a modern vector Explorer 4-quadrant dynamic widget icon
    pub fn draw_icon_explorer(
        &mut self,
        cx: f32,
        cy: f32,
        size: f32,
        color: [f32; 4],
        accent: [f32; 4],
    ) {
        let qs = size * 0.36;
        let gap = size * 0.08;
        // Top-left quadrant (accent cyan highlight)
        self.push_rect(
            cx - qs - gap * 0.5,
            cy - qs - gap * 0.5,
            qs,
            qs,
            size * 0.10,
            accent,
            0.0,
        );
        // Top-right quadrant
        self.push_rect(
            cx + gap * 0.5,
            cy - qs - gap * 0.5,
            qs,
            qs,
            size * 0.10,
            color,
            0.0,
        );
        // Bottom-left quadrant
        self.push_rect(
            cx - qs - gap * 0.5,
            cy + gap * 0.5,
            qs,
            qs,
            size * 0.10,
            color,
            0.0,
        );
        // Bottom-right quadrant
        self.push_rect(
            cx + gap * 0.5,
            cy + gap * 0.5,
            qs,
            qs,
            size * 0.10,
            color,
            0.0,
        );
    }

    /// Draws a modern vector Labs science flask icon
    pub fn draw_icon_labs(
        &mut self,
        cx: f32,
        cy: f32,
        size: f32,
        color: [f32; 4],
        accent: [f32; 4],
    ) {
        // Flask neck lip
        self.push_rect(
            cx - size * 0.18,
            cy - size * 0.44,
            size * 0.36,
            size * 0.10,
            size * 0.05,
            color,
            0.0,
        );
        // Flask neck
        self.push_rect(
            cx - size * 0.10,
            cy - size * 0.38,
            size * 0.20,
            size * 0.28,
            size * 0.04,
            color,
            0.0,
        );
        // Flask body base outline
        self.push_rect(
            cx - size * 0.40,
            cy - size * 0.14,
            size * 0.80,
            size * 0.56,
            size * 0.16,
            color,
            1.2,
        );
        // Liquid glow
        self.push_rect(
            cx - size * 0.32,
            cy + size * 0.06,
            size * 0.64,
            size * 0.32,
            size * 0.10,
            accent,
            0.0,
        );
        // Luminous reaction bubble
        self.push_rect(
            cx + size * 0.06,
            cy - size * 0.02,
            size * 0.10,
            size * 0.10,
            size * 0.05,
            [1.0, 1.0, 1.0, 0.9],
            0.0,
        );
    }

    /// Draws a modern vector Projects code folder icon
    pub fn draw_icon_projects(
        &mut self,
        cx: f32,
        cy: f32,
        size: f32,
        color: [f32; 4],
        accent: [f32; 4],
    ) {
        let fw = size * 0.44;
        let fh = size * 0.32;
        // Tab
        self.push_rect(
            cx - fw,
            cy - fh - size * 0.14,
            fw * 0.85,
            size * 0.18,
            size * 0.06,
            color,
            0.0,
        );
        // Folder base
        self.push_rect(
            cx - fw,
            cy - fh,
            fw * 2.0,
            fh * 2.0,
            size * 0.10,
            color,
            1.2,
        );
        // Code tag inner brackets
        self.push_rect(
            cx - size * 0.20,
            cy - size * 0.02,
            size * 0.12,
            size * 0.22,
            size * 0.04,
            accent,
            0.0,
        );
        self.push_rect(
            cx + size * 0.08,
            cy - size * 0.02,
            size * 0.12,
            size * 0.22,
            size * 0.04,
            accent,
            0.0,
        );
    }

    /// Draws a modern vector Docs document icon
    pub fn draw_icon_docs(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        let dw = size * 0.34;
        let dh = size * 0.44;
        // Document sheet
        self.push_rect(
            cx - dw,
            cy - dh,
            dw * 2.0,
            dh * 2.0,
            size * 0.10,
            color,
            1.2,
        );
        // Folded corner
        self.push_rect(
            cx + dw - size * 0.24,
            cy - dh,
            size * 0.24,
            size * 0.24,
            size * 0.04,
            color,
            0.0,
        );
        // 3 document lines
        self.push_rect(
            cx - dw * 0.65,
            cy - dh * 0.35,
            dw * 1.0,
            size * 0.07,
            size * 0.035,
            [0.133, 0.827, 0.933, 1.0],
            0.0,
        );
        self.push_rect(
            cx - dw * 0.65,
            cy + dh * 0.02,
            dw * 1.3,
            size * 0.07,
            size * 0.035,
            color,
            0.0,
        );
        self.push_rect(
            cx - dw * 0.65,
            cy + dh * 0.35,
            dw * 0.9,
            size * 0.07,
            size * 0.035,
            color,
            0.0,
        );
    }

    /// Draws a modern Sun icon for theme toggle
    pub fn draw_icon_sun(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        // Sun center core
        let r = size * 0.22;
        self.push_rect(cx - r, cy - r, r * 2.0, r * 2.0, r, color, 0.0);
        // 4 cardinal rays
        let ray_w = size * 0.08;
        let ray_len = size * 0.15;
        self.push_rect(
            cx - ray_w * 0.5,
            cy - size * 0.46,
            ray_w,
            ray_len,
            ray_w * 0.5,
            color,
            0.0,
        );
        self.push_rect(
            cx - ray_w * 0.5,
            cy + size * 0.31,
            ray_w,
            ray_len,
            ray_w * 0.5,
            color,
            0.0,
        );
        self.push_rect(
            cx - size * 0.46,
            cy - ray_w * 0.5,
            ray_len,
            ray_w,
            ray_w * 0.5,
            color,
            0.0,
        );
        self.push_rect(
            cx + size * 0.31,
            cy - ray_w * 0.5,
            ray_len,
            ray_w,
            ray_w * 0.5,
            color,
            0.0,
        );
    }

    /// Draws a modern Moon icon for theme toggle
    pub fn draw_icon_moon(
        &mut self,
        cx: f32,
        cy: f32,
        size: f32,
        color: [f32; 4],
        bg_color: [f32; 4],
    ) {
        let r = size * 0.38;
        self.push_rect(cx - r, cy - r, r * 2.0, r * 2.0, r, color, 0.0);
        // Cutout circle
        let cr = size * 0.30;
        self.push_rect(
            cx - cr + size * 0.20,
            cy - cr - size * 0.08,
            cr * 2.0,
            cr * 2.0,
            cr,
            bg_color,
            0.0,
        );
    }

    /// Draws a modern Info `i` icon
    pub fn draw_icon_info(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        let r = size * 0.44;
        self.push_rect(cx - r, cy - r, r * 2.0, r * 2.0, r, color, 1.2);
        // Dot
        self.push_rect(
            cx - size * 0.07,
            cy - size * 0.26,
            size * 0.14,
            size * 0.14,
            size * 0.07,
            color,
            0.0,
        );
        // Stem
        self.push_rect(
            cx - size * 0.07,
            cy - size * 0.04,
            size * 0.14,
            size * 0.32,
            size * 0.05,
            color,
            0.0,
        );
    }

    /// Draws the official TechScript Brand Dragon Logo
    pub fn draw_techscript_logo_mark(&mut self, cx: f32, cy: f32, size: f32) {
        let r = size * 0.50;
        let x = cx - r;
        let y = cy - r;
        let w = size;
        let h = size;

        let base_idx = self.vertices.len() as u16;
        let uv = (
            crate::typography::LOGO_UV[0],
            crate::typography::LOGO_UV[1],
            crate::typography::LOGO_UV[2],
            crate::typography::LOGO_UV[3],
        );

        self.vertices.push(UIVertex {
            pos: [x, y],
            uv: [uv.0, uv.1],
            color: [1.0, 1.0, 1.0, 1.0],
            local_pos: [0.0, 0.0],
            size: [w, h],
            radius: 0.0,
            border_width: 0.0,
            is_text: 3.0,
        });
        self.vertices.push(UIVertex {
            pos: [x + w, y],
            uv: [uv.2, uv.1],
            color: [1.0, 1.0, 1.0, 1.0],
            local_pos: [0.0, 0.0],
            size: [w, h],
            radius: 0.0,
            border_width: 0.0,
            is_text: 3.0,
        });
        self.vertices.push(UIVertex {
            pos: [x + w, y + h],
            uv: [uv.2, uv.3],
            color: [1.0, 1.0, 1.0, 1.0],
            local_pos: [0.0, 0.0],
            size: [w, h],
            radius: 0.0,
            border_width: 0.0,
            is_text: 3.0,
        });
        self.vertices.push(UIVertex {
            pos: [x, y + h],
            uv: [uv.0, uv.3],
            color: [1.0, 1.0, 1.0, 1.0],
            local_pos: [0.0, 0.0],
            size: [w, h],
            radius: 0.0,
            border_width: 0.0,
            is_text: 3.0,
        });

        self.indices.push(base_idx);
        self.indices.push(base_idx + 1);
        self.indices.push(base_idx + 2);
        self.indices.push(base_idx);
        self.indices.push(base_idx + 2);
        self.indices.push(base_idx + 3);
    }

    /// Draws the official `tsc run` Speed Runner logo (acceleration orbit + speed streaks + play runner chevron)
    pub fn draw_logo_tsc_run(&mut self, cx: f32, cy: f32, size: f32, is_dark: bool) {
        let accent = if is_dark {
            [0.0, 0.90, 1.0, 1.0]
        } else {
            [0.01, 0.52, 0.78, 1.0]
        };
        let glow_ring = if is_dark {
            [0.0, 0.90, 1.0, 0.35]
        } else {
            [0.01, 0.52, 0.78, 0.25]
        };

        // 1. Orbital velocity speed ring
        self.push_ring(cx, cy, size * 0.42, size * 0.08, glow_ring);

        // 2. Trailing speed streaks
        let streak_w = size * 0.24;
        let streak_h = size * 0.06;
        self.push_rect(
            cx - size * 0.44,
            cy - size * 0.18,
            streak_w,
            streak_h,
            streak_h * 0.5,
            accent,
            0.0,
        );
        self.push_rect(
            cx - size * 0.38,
            cy + size * 0.18,
            streak_w * 0.75,
            streak_h,
            streak_h * 0.5,
            accent,
            0.0,
        );

        // 3. Play runner triangle (sharp execution chevron)
        let pw = size * 0.46;
        let ph = size * 0.56;
        let px = cx - size * 0.05;
        self.push_rect(
            px - pw * 0.35,
            cy - ph * 0.50,
            pw * 0.25,
            ph,
            size * 0.06,
            accent,
            0.0,
        );
        self.push_rect(
            px - pw * 0.10,
            cy - ph * 0.35,
            pw * 0.25,
            ph * 0.70,
            size * 0.06,
            accent,
            0.0,
        );
        self.push_rect(
            px + pw * 0.15,
            cy - ph * 0.18,
            pw * 0.25,
            ph * 0.36,
            size * 0.06,
            accent,
            0.0,
        );
    }

    /// Draws the official `tsc app` AppForge Mobile logo (smartphone chassis + dynamic island + appforge diamond)
    pub fn draw_logo_tsc_app(&mut self, cx: f32, cy: f32, size: f32, is_dark: bool) {
        let primary = if is_dark {
            [0.65, 0.35, 0.98, 1.0]
        } else {
            [0.48, 0.23, 0.93, 1.0]
        };
        let screen_col = if is_dark {
            [0.08, 0.06, 0.22, 0.95]
        } else {
            [0.93, 0.90, 1.0, 0.95]
        };
        let accent = if is_dark {
            [0.0, 0.90, 1.0, 1.0]
        } else {
            [0.02, 0.52, 0.78, 1.0]
        };

        let phone_w = size * 0.62;
        let phone_h = size * 0.90;
        let corner = size * 0.15;

        // 1. Phone outer frame
        self.push_rect(
            cx - phone_w * 0.5,
            cy - phone_h * 0.5,
            phone_w,
            phone_h,
            corner,
            primary,
            1.4,
        );

        // 2. Inner screen glass
        let screen_w = phone_w - size * 0.12;
        let screen_h = phone_h - size * 0.14;
        self.push_rect(
            cx - screen_w * 0.5,
            cy - screen_h * 0.5,
            screen_w,
            screen_h,
            corner * 0.7,
            screen_col,
            0.0,
        );

        // 3. Top camera island notch
        self.push_rect(
            cx - size * 0.10,
            cy - screen_h * 0.5 + size * 0.03,
            size * 0.20,
            size * 0.05,
            size * 0.025,
            primary,
            0.0,
        );

        // 4. Center AppForge jewel mark (diamond grid)
        let j_sz = size * 0.18;
        self.push_rect(
            cx - j_sz * 0.5,
            cy - j_sz * 0.5,
            j_sz,
            j_sz,
            size * 0.04,
            primary,
            0.0,
        );
        self.push_rect(
            cx - j_sz * 0.25,
            cy - j_sz * 0.25,
            j_sz * 0.5,
            j_sz * 0.5,
            size * 0.02,
            accent,
            0.0,
        );

        // 5. Bottom home bar
        self.push_rect(
            cx - size * 0.12,
            cy + screen_h * 0.5 - size * 0.07,
            size * 0.24,
            size * 0.04,
            size * 0.02,
            primary,
            0.0,
        );
    }

    /// Draws the official `tsc check` AST Verification logo (compiler shield + verification checkmark + AST spark)
    pub fn draw_logo_tsc_check(&mut self, cx: f32, cy: f32, size: f32, is_dark: bool) {
        let emerald = if is_dark {
            [0.0, 0.92, 0.65, 1.0]
        } else {
            [0.02, 0.60, 0.42, 1.0]
        };
        let glow_ring = if is_dark {
            [0.0, 0.92, 0.65, 0.28]
        } else {
            [0.02, 0.60, 0.42, 0.20]
        };

        // 1. Protective shield boundary ring
        self.push_ring(cx, cy, size * 0.42, size * 0.07, glow_ring);

        // 2. Shield body outline
        let sw = size * 0.68;
        let sh = size * 0.78;
        self.push_rect(
            cx - sw * 0.5,
            cy - sh * 0.5,
            sw,
            sh * 0.65,
            size * 0.14,
            emerald,
            1.3,
        );
        self.push_rect(
            cx - sw * 0.35,
            cy + sh * 0.15 - sh * 0.5,
            sw * 0.70,
            sh * 0.50,
            size * 0.18,
            emerald,
            1.3,
        );

        // 3. Thick decisive checkmark inside shield
        let cw = size * 0.085;
        self.push_rect(
            cx - size * 0.18,
            cy + size * 0.02,
            size * 0.14,
            cw,
            cw * 0.5,
            emerald,
            0.0,
        );
        self.push_rect(
            cx - size * 0.06,
            cy - size * 0.14,
            cw,
            size * 0.32,
            cw * 0.5,
            emerald,
            0.0,
        );

        // 4. Luminous AST spark at top right of shield
        self.push_rect(
            cx + size * 0.22,
            cy - size * 0.30,
            size * 0.08,
            size * 0.08,
            size * 0.04,
            [1.0, 1.0, 1.0, 0.95],
            0.0,
        );
    }

    /// Draws a modern vector Terminal Prompt `>_` icon
    pub fn draw_icon_terminal(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        let sw = size * 0.08;
        // Chevron top leg
        self.push_rect(
            cx - size * 0.35,
            cy - size * 0.25,
            size * 0.30,
            sw,
            sw * 0.5,
            color,
            0.0,
        );
        // Chevron bottom leg
        self.push_rect(
            cx - size * 0.20,
            cy - size * 0.05,
            size * 0.25,
            sw,
            sw * 0.5,
            color,
            0.0,
        );
        // Underscore cursor
        self.push_rect(
            cx + size * 0.05,
            cy + size * 0.15,
            size * 0.32,
            sw,
            sw * 0.5,
            color,
            0.0,
        );
    }

    /// Draws a modern vector Package / Build Cube icon
    pub fn draw_icon_package(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        let r = size * 0.38;
        // Outer package squircle
        self.push_rect(cx - r, cy - r, r * 2.0, r * 2.0, size * 0.12, color, 1.2);
        // Box lid seam
        self.push_rect(
            cx - r * 0.8,
            cy - size * 0.08,
            r * 1.6,
            size * 0.06,
            size * 0.03,
            color,
            0.0,
        );
        // Box vertical tape band
        self.push_rect(
            cx - size * 0.05,
            cy - r,
            size * 0.10,
            r * 2.0,
            size * 0.03,
            [0.133, 0.827, 0.933, 0.9],
            0.0,
        );
    }

    /// Draws a modern vector Checkmark / Doctor icon
    pub fn draw_icon_check(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        let r = size * 0.38;
        self.push_rect(cx - r, cy - r, r * 2.0, r * 2.0, size * 0.12, color, 1.2);
        // Check stem
        let sw = size * 0.08;
        self.push_rect(
            cx - size * 0.22,
            cy,
            size * 0.16,
            sw,
            sw * 0.5,
            [0.10, 0.85, 0.55, 1.0],
            0.0,
        );
        self.push_rect(
            cx - size * 0.06,
            cy - size * 0.15,
            sw,
            size * 0.38,
            sw * 0.5,
            [0.10, 0.85, 0.55, 1.0],
            0.0,
        );
    }

    /// Draws a modern vector Compass icon
    pub fn draw_icon_compass(
        &mut self,
        cx: f32,
        cy: f32,
        size: f32,
        ring_color: [f32; 4],
        needle_color: [f32; 4],
    ) {
        let r = size * 0.44;
        self.push_rect(cx - r, cy - r, r * 2.0, r * 2.0, r, ring_color, 1.2);
        // Diamond needle
        self.push_rect(
            cx - size * 0.08,
            cy - size * 0.28,
            size * 0.16,
            size * 0.28,
            size * 0.05,
            needle_color,
            0.0,
        );
        self.push_rect(
            cx - size * 0.08,
            cy,
            size * 0.16,
            size * 0.28,
            size * 0.05,
            ring_color,
            0.0,
        );
        // Center pivot
        self.push_rect(
            cx - size * 0.05,
            cy - size * 0.05,
            size * 0.10,
            size * 0.10,
            size * 0.05,
            [1.0, 1.0, 1.0, 1.0],
            0.0,
        );
    }

    /// Draws a clean play arrow ▶ icon
    pub fn draw_play_icon(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        let w = size * 0.65;
        let h = size * 0.85;
        // Tri-strip play triangle
        self.push_rect(
            cx - w * 0.4,
            cy - h * 0.5,
            w * 0.25,
            h,
            size * 0.08,
            color,
            0.0,
        );
        self.push_rect(
            cx - w * 0.15,
            cy - h * 0.35,
            w * 0.25,
            h * 0.70,
            size * 0.08,
            color,
            0.0,
        );
        self.push_rect(
            cx + w * 0.10,
            cy - h * 0.20,
            w * 0.25,
            h * 0.40,
            size * 0.08,
            color,
            0.0,
        );
    }

    /// Draws a clean right chevron `>` icon
    pub fn draw_chevron_right(&mut self, cx: f32, cy: f32, size: f32, color: [f32; 4]) {
        let sw = size * 0.16;
        let arm = size * 0.42;
        // Top slant arm
        self.push_rect(
            cx - arm * 0.4,
            cy - arm * 0.6,
            arm,
            sw,
            sw * 0.5,
            color,
            0.0,
        );
        // Bottom slant arm
        self.push_rect(
            cx - arm * 0.4,
            cy + arm * 0.4,
            arm,
            sw,
            sw * 0.5,
            color,
            0.0,
        );
    }

    /// Add an antialiased circular ring / orbital halo
    pub fn push_ring(&mut self, cx: f32, cy: f32, radius: f32, thickness: f32, color: [f32; 4]) {
        let extent = radius + thickness * 2.0;
        let x = cx - extent;
        let y = cy - extent;
        let w = extent * 2.0;
        let h = extent * 2.0;
        let hw = extent;
        let hh = extent;
        let base_idx = self.vertices.len() as u16;

        self.vertices.push(UIVertex {
            pos: [x, y],
            uv: [0.0, 0.0],
            color,
            local_pos: [-hw, -hh],
            size: [radius * 2.0, radius * 2.0],
            radius,
            border_width: thickness,
            is_text: 2.0,
        });
        self.vertices.push(UIVertex {
            pos: [x + w, y],
            uv: [1.0, 0.0],
            color,
            local_pos: [hw, -hh],
            size: [radius * 2.0, radius * 2.0],
            radius,
            border_width: thickness,
            is_text: 2.0,
        });
        self.vertices.push(UIVertex {
            pos: [x + w, y + h],
            uv: [1.0, 1.0],
            color,
            local_pos: [hw, hh],
            size: [radius * 2.0, radius * 2.0],
            radius,
            border_width: thickness,
            is_text: 2.0,
        });
        self.vertices.push(UIVertex {
            pos: [x, y + h],
            uv: [0.0, 1.0],
            color,
            local_pos: [-hw, hh],
            size: [radius * 2.0, radius * 2.0],
            radius,
            border_width: thickness,
            is_text: 2.0,
        });

        self.indices.push(base_idx);
        self.indices.push(base_idx + 1);
        self.indices.push(base_idx + 2);
        self.indices.push(base_idx);
        self.indices.push(base_idx + 2);
        self.indices.push(base_idx + 3);
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
        let line_step = font_atlas.line_height * font_scale;
        let mut cur_x = x;
        let mut cur_y = y;

        for ch in text.chars() {
            if ch == '\n' {
                cur_x = x;
                cur_y += line_step;
                continue;
            }
            if let Some(m) = font_atlas.metrics.get(&ch) {
                if m.width > 0.0 && m.height > 0.0 {
                    let gx = cur_x + m.xmin * font_scale;
                    let gy = cur_y + (font_atlas.ascender - (m.ymin + m.height)) * font_scale;
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
    pub fn push_text(&mut self, x: f32, y: f32, text: &str, scale: f32, color: [f32; 4]) {
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
    if (v_is_text > 2.5) {
        // Direct full-color textured quad (Official TechScript Brand Dragon Logo)
        vec4 tex_col = texture2D(u_font_texture, v_uv);
        if (tex_col.a < 0.02) discard;
        gl_FragColor = tex_col;
    } else if (v_is_text > 1.5) {
        // Antialiased hollow circular orbital ring
        float dist = length(v_local_pos);
        float ring_r = v_radius;
        float d = abs(dist - ring_r) - v_border_width * 0.5;
        float alpha = clamp(0.5 - d, 0.0, 1.0);
        if (alpha <= 0.0) discard;
        gl_FragColor = vec4(v_color.rgb, v_color.a * alpha);
    } else if (v_is_text > 0.5) {
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

        // Vertical normalized coordinate: 0.0 at top of card, 1.0 at bottom
        float norm_y = clamp((v_local_pos.y + half_size.y) / max(v_size.y, 1.0), 0.0, 1.0);

        vec4 col = v_color;
        float lum = dot(col.rgb, vec3(0.299, 0.587, 0.114));

        if (lum > 0.65) {
            // ── Light Mode (Sky Blue / Frosted Ice Glass) ──
            // Gentle ambient lighting gradient without blinding glare
            col.rgb += vec3(0.015, 0.035, 0.070) * (1.0 - norm_y);

            // Specular frost rim along top edge (crisp crystalline glint)
            float top_edge = clamp((1.0 - norm_y * 6.0), 0.0, 1.0);
            float top_specular = pow(top_edge, 1.4) * 0.16;
            col.rgb += vec3(0.20, 0.24, 0.30) * top_specular;

            // Refined sky azure refraction border
            if (v_border_width > 0.1) {
                float border_factor = clamp((dist + v_border_width) / max(v_border_width, 0.4), 0.0, 1.0);
                vec3 border_top = vec3(0.72, 0.85, 0.99);   // Soft azure sky reflection
                vec3 border_side = vec3(0.52, 0.70, 0.92);  // Crisp cerulean edge
                vec3 border_col = mix(border_top, border_side, clamp(norm_y * 1.4, 0.0, 1.0));

                col.rgb = mix(col.rgb, border_col, border_factor * 0.85);
                col.a = max(col.a, border_factor * 0.95);
            }
        } else {
            // ── Dark Mode (Cosmic Void / Neon Cyan Glass) ──
            // Subtle vertical glass ambient lighting gradient
            col.rgb += vec3(0.04, 0.07, 0.16) * (1.0 - norm_y);

            // Specular highlight rim at top of glass surface (concentrated at top 14%)
            float top_edge = clamp((1.0 - norm_y * 7.0), 0.0, 1.0);
            float top_specular = pow(top_edge, 1.5) * 0.20;
            col.rgb += vec3(0.20, 0.70, 1.0) * top_specular;

            // Crisp luminous liquid glass border with electric cyan top and glowing royal indigo perimeter
            if (v_border_width > 0.1) {
                float border_factor = clamp((dist + v_border_width) / max(v_border_width, 0.4), 0.0, 1.0);
                
                vec3 border_top = vec3(0.25, 0.92, 1.0);     // Luminous electric cyan reflection
                vec3 border_side = vec3(0.35, 0.50, 0.90);   // Glowing electric cobalt blue
                vec3 border_col = mix(border_top, border_side, clamp(norm_y * 1.6, 0.0, 1.0));

                float border_alpha = mix(0.85, 0.55, norm_y);
                col.rgb = mix(col.rgb, border_col, border_factor * 0.82);
                col.a = max(col.a, border_factor * border_alpha);
            }
        }

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
uniform vec3 u_rim_col;
uniform vec3 u_view_pos;
uniform float u_is_wireframe;
uniform vec4 u_wire_color;

void main() {
    if (u_is_wireframe > 0.5) {
        gl_FragColor = u_wire_color;
        return;
    }

    vec3 n = normalize(v_normal);
    vec3 l = normalize(u_light_dir);
    vec3 v = normalize(u_view_pos - v_world_pos);
    vec3 h = normalize(l + v);

    // Faceted diamond jewel shading
    float diff = max(dot(n, l), 0.35);
    float spec = pow(max(dot(n, h), 0.0), 32.0);

    // Radiant Fresnel edge glow customized to the active material
    float fresnel = pow(1.0 - max(dot(n, v), 0.0), 2.4);

    vec3 rgb = v_color.rgb * (diff * 0.85 + 0.15) + u_specular_col * spec * 1.35 + u_rim_col * fresnel * 0.95;
    gl_FragColor = vec4(rgb, v_color.a);
}
"#;
