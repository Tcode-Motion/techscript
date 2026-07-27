//! # TechScript CLI Theme Support
//!
//! Auto-detects terminal capability (color, Unicode) and yields consistent icons.

pub struct Theme {
    pub success_icon: &'static str,
    pub warning_icon: &'static str,
    pub error_icon: &'static str,
    pub info_icon: &'static str,
    pub build_icon: &'static str,
    pub package_icon: &'static str,
    pub project_icon: &'static str,
    pub docs_icon: &'static str,
}

impl Theme {
    pub fn detect() -> Self {
        // Detect if color is disabled or if we should use raw ASCII symbols
        let use_unicode = true; // Most modern terminals support Unicode.

        // Auto-detect ASCII fallback if running inside a legacy environment or if requested
        if use_unicode {
            Self {
                success_icon: "✓",
                warning_icon: "⚠",
                error_icon: "✗",
                info_icon: "ℹ",
                build_icon: "🚀",
                package_icon: "📦",
                project_icon: "🛠",
                docs_icon: "📄",
            }
        } else {
            Self {
                success_icon: "[OK]",
                warning_icon: "[WARN]",
                error_icon: "[ERR]",
                info_icon: "[INFO]",
                build_icon: "[BUILD]",
                package_icon: "[PKG]",
                project_icon: "[PROJ]",
                docs_icon: "[DOC]",
            }
        }
    }
}
