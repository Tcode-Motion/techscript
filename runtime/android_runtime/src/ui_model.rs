// =============================================================================
// TechScript AppForge — UI Model
// runtime/android_runtime/src/ui_model.rs
//
// Represents the compiled TechScript application specification.
// Loaded from assets/app_spec.json — produced by `tsc app build android`.
// =============================================================================

use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// Top-level application specification
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSpec {
    pub app: AppMeta,
    #[serde(default)]
    pub screens: Vec<Screen>,
    #[serde(default)]
    pub routes: Vec<Route>,
    #[serde(default)]
    pub theme: ThemeConfig,
}

impl AppSpec {
    pub fn screen_by_id(&self, id: &str) -> Option<&Screen> {
        self.screens.iter().find(|s| s.id == id)
    }

    /// Default spec used when app_spec.json is missing or malformed.
    /// This matches the AppForge showcase application defined in .txs files.
    pub fn default_appforge() -> Self {
        Self {
            app: AppMeta {
                name: "TechScript AppForge".into(),
                id: "com.techscript.appforge".into(),
                version: "2.0.0".into(),
                creator: "Tanmoy Majumder".into(),
                website: "https://techscript.is-a.dev/techscript/".into(),
                repository: "https://github.com/Tcode-Motion/techscript".into(),
            },
            screens: vec![
                Screen {
                    id: "home".into(),
                    title: "Home".into(),
                    widgets: vec![
                        Widget::card(vec![
                            Widget::text("Welcome to TechScript AppForge", WidgetStyle::Headline),
                            Widget::text(
                                "Independent programming language and native application engine.\nRuntime: Rust (libts_android.so) | Entry: NativeActivity",
                                WidgetStyle::Body,
                            ),
                            Widget::Row {
                                children: vec![
                                    Widget::MetricBox { value: "v2.0.0".into(), label: "Compiler".into() },
                                    Widget::MetricBox { value: "Rust".into(),   label: "Runtime".into() },
                                ],
                            },
                        ]),
                        Widget::card(vec![
                            Widget::text("Quick Actions", WidgetStyle::Title),
                            Widget::button("Explore Documentation",  "navigate:docs",        "#4f46e5"),
                            Widget::button("Manage Projects",        "navigate:workspace",   "#0891b2"),
                            Widget::button("System Diagnostics",     "navigate:diagnostics", "#059669"),
                        ]),
                    ],
                },
                Screen {
                    id: "workspace".into(),
                    title: "Projects".into(),
                    widgets: vec![Widget::card(vec![
                        Widget::text("TechScript Projects", WidgetStyle::Headline),
                        Widget::text(
                            "Runtime: Rust NativeActivity (zero Java)\nLanguage: TechScript (.txs)\nPackage: com.techscript.appforge",
                            WidgetStyle::Mono,
                        ),
                        Widget::text(
                            "● src/main.txs\n● src/MainActivity.txs\n● src/app.txs\n● src/theme.txs\n● src/navigation.txs\n● src/screens/home.txs\n● src/screens/workspace.txs\n● src/screens/docs.txs\n● src/screens/diagnostics.txs\n● src/screens/about.txs",
                            WidgetStyle::Body,
                        ),
                    ])],
                },
                Screen {
                    id: "docs".into(),
                    title: "Docs".into(),
                    widgets: vec![Widget::card(vec![
                        Widget::text("TechScript CLI Reference", WidgetStyle::Headline),
                        Widget::text("tsc app create <name>",   WidgetStyle::Code),
                        Widget::text("Scaffold a new TechScript native application", WidgetStyle::Body),
                        Widget::Divider,
                        Widget::text("tsc app build android",   WidgetStyle::Code),
                        Widget::text("Cross-compile .txs → NDK .so → APK (zero Java)", WidgetStyle::Body),
                        Widget::Divider,
                        Widget::text("tsc app run android",     WidgetStyle::Code),
                        Widget::text("Build and install on connected device via ADB", WidgetStyle::Body),
                        Widget::Divider,
                        Widget::text("tsc check <file.txs>",   WidgetStyle::Code),
                        Widget::text("Static type and semantic verification", WidgetStyle::Body),
                    ])],
                },
                Screen {
                    id: "diagnostics".into(),
                    title: "Tools".into(),
                    widgets: vec![Widget::card(vec![
                        Widget::text("Runtime Diagnostics", WidgetStyle::Headline),
                        Widget::text(
                            "Entry Point: android_main() [Rust]\nLibrary: libts_android.so\nNDK ABI: arm64-v8a\nJava/Kotlin: none\nRenderer: EGL/ANativeWindow",
                            WidgetStyle::Mono,
                        ),
                    ])],
                },
                Screen {
                    id: "about".into(),
                    title: "About".into(),
                    widgets: vec![Widget::card(vec![
                        Widget::text("About TechScript AppForge", WidgetStyle::Headline),
                        Widget::text(
                            "TechScript is an independent, general-purpose programming language by Tanmoy Majumder (Tcode-Motion).\n\nAppForge is a native mobile application platform — applications are written entirely in TechScript (.txs) with zero Java or Kotlin.",
                            WidgetStyle::Body,
                        ),
                        Widget::button("Official Website",   "url:https://techscript.is-a.dev/techscript/",   "#4f46e5"),
                        Widget::button("GitHub Repository",  "url:https://github.com/Tcode-Motion/techscript", "#1e293b"),
                    ])],
                },
            ],
            routes: vec![
                Route { id: "home".into(),        label: "Home".into(),     screen: "home".into() },
                Route { id: "workspace".into(),   label: "Projects".into(), screen: "workspace".into() },
                Route { id: "docs".into(),        label: "Docs".into(),     screen: "docs".into() },
                Route { id: "diagnostics".into(), label: "Tools".into(),    screen: "diagnostics".into() },
                Route { id: "about".into(),       label: "About".into(),    screen: "about".into() },
            ],
            theme: ThemeConfig::default(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// App metadata
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppMeta {
    pub name: String,
    pub id: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub creator: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub repository: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub label: String,
    pub screen: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeConfig {
    #[serde(default = "ThemeConfig::default_primary")]
    pub primary: String,
    #[serde(default = "ThemeConfig::default_accent")]
    pub accent: String,
    #[serde(default = "ThemeConfig::default_mode")]
    pub mode: String,
}

impl ThemeConfig {
    fn default_primary() -> String { "#4f46e5".into() }
    fn default_accent()  -> String { "#06b6d4".into() }
    fn default_mode()    -> String { "system".into() }
}

// ─────────────────────────────────────────────────────────────────────────────
// Screen & Widget definitions
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screen {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub widgets: Vec<Widget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Widget {
    Text {
        content: String,
        #[serde(default)]
        style: WidgetStyle,
    },
    Button {
        label: String,
        action: String,
        #[serde(default)]
        color: String,
    },
    Card {
        children: Vec<Widget>,
    },
    Row {
        children: Vec<Widget>,
    },
    Column {
        children: Vec<Widget>,
    },
    MetricBox {
        value: String,
        label: String,
    },
    Spacer {
        #[serde(default = "default_spacer_height")]
        height: i32,
    },
    Divider,
}

fn default_spacer_height() -> i32 { 12 }

impl Widget {
    pub fn text(content: &str, style: WidgetStyle) -> Self {
        Widget::Text { content: content.into(), style }
    }

    pub fn button(label: &str, action: &str, color: &str) -> Self {
        Widget::Button {
            label: label.into(),
            action: action.into(),
            color: color.into(),
        }
    }

    pub fn card(children: Vec<Widget>) -> Self {
        Widget::Card { children }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WidgetStyle {
    #[default]
    Body,
    Headline,
    Title,
    Code,
    Mono,
    Muted,
}
