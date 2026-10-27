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
        if is_dark {
            Self::dark()
        } else {
            Self::light()
        }
    }

    pub fn dark() -> Self {
        Self {
            is_dark: true,
            bg: argb(255, 3, 7, 18),            // #030712 Deep Midnight Navy
            surface: argb(240, 8, 16, 38),      // #081026 Rich Navy Surface
            card: argb(255, 10, 17, 40),        // #0A1128 Solid Dark Navy Card
            border: argb(255, 30, 46, 93),      // #1E2E5D Crisp Electric Blue Border
            highlight: argb(120, 0, 229, 255),  // Luminous Cyan Reflection
            primary: argb(255, 99, 102, 241),   // #6366F1 Electric Indigo
            violet: argb(255, 168, 85, 247),    // #A855F7 Vibrant Violet
            accent: argb(255, 0, 229, 255),     // #00E5FF Electric Neon Cyan
            magenta: argb(255, 217, 70, 239),   // #D946EF Magenta
            secondary: argb(255, 168, 85, 247), // Violet
            success: argb(255, 0, 229, 160),    // #00E5A0 Emerald Green
            text: argb(255, 244, 247, 255),     // #F4F7FF Crisp Pure White
            text_muted: argb(255, 142, 158, 194), // #8E9EC2 Blue-Gray Secondary
            dock_bg: argb(255, 8, 15, 38),      // #080F26 Midnight Dock
            dock_active: argb(255, 26, 36, 84), // #1A2454 Luminous Active Tab Pill
        }
    }

    pub fn light() -> Self {
        Self {
            is_dark: false,
            bg: argb(255, 218, 233, 248), // #DAE9F8 Soothing Eye-Comfort Sky Blue Background
            surface: argb(240, 235, 245, 255), // #EBF5FF Soft Translucent Sky Surface
            card: argb(245, 242, 249, 255), // #F2F9FF Frosted Liquid Ice Glass Card
            border: argb(255, 172, 202, 236), // #ACC8EC Refined Azure Glass Refraction Border
            highlight: argb(220, 255, 255, 255), // Frosted Specular Glint
            primary: argb(255, 30, 64, 175), // #1E40AF Deep Royal Indigo
            violet: argb(255, 109, 40, 217), // #6D28D9 Royal Violet
            accent: argb(255, 2, 132, 199), // #0284C7 Sky Azure Cyan
            magenta: argb(255, 162, 28, 175), // #A21CAF Rich Berry Magenta
            secondary: argb(255, 91, 33, 182), // Deep Violet
            success: argb(255, 4, 120, 87), // #047857 Deep Emerald Green
            text: argb(255, 15, 23, 42),  // #0F172A Crisp Deep Navy Slate Text
            text_muted: argb(255, 71, 85, 105), // #475569 High-Contrast Slate Text
            dock_bg: argb(245, 228, 241, 253), // #E4F1FD Frosted Sky Navigation Dock
            dock_active: argb(255, 196, 222, 250), // #C4DEF8 Luminous Sky Blue Active Tab Pill
        }
    }

    pub fn to_f32_color(color: Color) -> [f32; 4] {
        [
            ((color >> 16) & 0xFF) as f32 / 255.0,
            ((color >> 8) & 0xFF) as f32 / 255.0,
            ((color) & 0xFF) as f32 / 255.0,
            ((color >> 24) & 0xFF) as f32 / 255.0,
        ]
    }
}

#[inline]
pub fn argb(a: u8, r: u8, g: u8, b: u8) -> Color {
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
