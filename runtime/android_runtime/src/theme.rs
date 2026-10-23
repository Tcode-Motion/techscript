// =============================================================================
// TechScript AppForge — Premium Liquid Glass Theme System
// runtime/android_runtime/src/theme.rs
//
// Official Palette:
//   - Background:        #080A12 (Deep Cosmic Void)
//   - Secondary surface: #111525 (Deep Indigo Slate)
//   - Glass surface:     RGBA(255, 255, 255, 0.08)
//   - Primary Violet:    #8B5CF6
//   - Electric Indigo:   #6366F1
//   - Cyan:              #22D3EE
//   - Magenta:           #D946EF
//   - Primary text:      #F8FAFC
//   - Secondary text:    #A7B0C0
// =============================================================================

use android_activity::AndroidApp;

pub type Color = u32;

#[derive(Debug, Clone)]
pub struct Theme {
    pub is_dark: bool,
    pub bg: Color,
    pub surface: Color,
    pub card: Color,
    pub border: Color,
    pub highlight: Color,
    pub primary: Color,
    pub violet: Color,
    pub accent: Color,
    pub magenta: Color,
    pub secondary: Color,
    pub success: Color,
    pub text: Color,
    pub text_muted: Color,
    pub dock_bg: Color,
    pub dock_active: Color,
}

impl Theme {
    pub fn resolve_system(app: &AndroidApp) -> Self {
        let is_dark = app.config().ui_mode_night() == ndk::configuration::UiModeNight::Yes;
        if is_dark { Self::dark() } else { Self::light() }
    }

    pub fn dark() -> Self {
        Self {
            is_dark: true,
            bg:          argb(255, 8,   10,  18),  // #080A12
            surface:     argb(210, 17,  21,  37),  // #111525
            card:        argb(165, 24,  28,  48),  // Luminous Frosted Glass
            border:      argb(50,  255, 255, 255), // Translucent Specular Rim
            highlight:   argb(90,  255, 255, 255), // Top Rim Reflection
            primary:     argb(255, 99,  102, 241), // #6366F1 Electric Indigo
            violet:      argb(255, 139, 92,  246), // #8B5CF6 Primary Violet
            accent:      argb(255, 34,  211, 238), // #22D3EE Cyan
            magenta:     argb(255, 217, 70,  239), // #D946EF Magenta
            secondary:   argb(255, 139, 92,  246), // Violet
            success:     argb(255, 16,  185, 129), // #10B981 Emerald
            text:        argb(255, 248, 250, 252), // #F8FAFC
            text_muted:  argb(255, 167, 176, 192), // #A7B0C0
            dock_bg:     argb(235, 17,  21,  37),  // #111525 Floating Liquid Dock
            dock_active: argb(180, 99,  102, 241),
        }
    }

    pub fn light() -> Self {
        Self {
            is_dark: false,
            bg:          argb(255, 246, 248, 252), // #F6F8FC
            surface:     argb(230, 255, 255, 255), // Crisp White Frosted
            card:        argb(240, 255, 255, 255),
            border:      argb(35,  99,  102, 241), // Subtle Indigo Rim
            highlight:   argb(255, 255, 255, 255),
            primary:     argb(255, 99,  102, 241), // #6366F1
            violet:      argb(255, 124, 58,  237), // #7C3AED
            accent:      argb(255, 14,  165, 233), // #0EA5E9
            magenta:     argb(255, 192, 38,  211), // #C026D3
            secondary:   argb(255, 124, 58,  237),
            success:     argb(255, 16,  185, 129),
            text:        argb(255, 15,  23,  42),  // #0F172A
            text_muted:  argb(255, 100, 116, 139), // #64748B
            dock_bg:     argb(240, 255, 255, 255),
            dock_active: argb(60,  99,  102, 241),
        }
    }

    pub fn to_f32_color(color: Color) -> [f32; 4] {
        [
            ((color >> 16) & 0xFF) as f32 / 255.0,
            ((color >>  8) & 0xFF) as f32 / 255.0,
            ((color      ) & 0xFF) as f32 / 255.0,
            ((color >> 24) & 0xFF) as f32 / 255.0,
        ]
    }
}

#[inline]
pub fn argb(a: u8, r: u8, g: u8, b: u8) -> Color {
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
