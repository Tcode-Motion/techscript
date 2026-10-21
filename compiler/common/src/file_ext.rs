//! TechScript File Extension Standardization & Registry.
//!
//! # Primary Design Rule
//! `.txs` is the official, default, and overwhelmingly preferred TechScript source-file extension.
//! At least 99% of ordinary TechScript code uses `.txs`.
//!
//! Specialized extensions (`.wtxs`, `.stxs`, `.atxs`, `.3txs`/`.gtxs`) and legacy compatibility (`.tech`)
//! provide domain clarity, compiler pass targeting, and backward compatibility without
//! fragmenting the language frontend.

use std::fmt;
use std::path::{Path, PathBuf};

// ── Official & Specialized Extension Constants ─────────────────────────────────

/// Canonical TechScript source file extension without leading dot.
pub const TECHSCRIPT_EXTENSION: &str = "txs";
/// Canonical TechScript source file extension with leading dot.
pub const TECHSCRIPT_DOT_EXTENSION: &str = ".txs";

/// Specialized WebForge declarative source extension.
pub const WEB_EXTENSION: &str = "wtxs";
pub const WEB_DOT_EXTENSION: &str = ".wtxs";

/// Specialized Server-side and background-task source extension.
pub const SERVER_EXTENSION: &str = "stxs";
pub const SERVER_DOT_EXTENSION: &str = ".stxs";

/// Specialized Android mobile declarative source extension.
pub const ANDROID_EXTENSION: &str = "atxs";
pub const ANDROID_DOT_EXTENSION: &str = ".atxs";

/// Specialized 3D WebGL / graphics scene source extension.
pub const SCENE3D_EXTENSION: &str = "3txs";
pub const SCENE3D_DOT_EXTENSION: &str = ".3txs";

/// Alternative graphics source extension alias.
pub const GRAPHICS_ALT_EXTENSION: &str = "gtxs";
pub const GRAPHICS_ALT_DOT_EXTENSION: &str = ".gtxs";

/// Legacy TechScript source extension (supported with migration diagnostic).
pub const LEGACY_TECH_EXTENSION: &str = "tech";
pub const LEGACY_TECH_DOT_EXTENSION: &str = ".tech";

// ── Extension Enums ────────────────────────────────────────────────────────────

/// All recognized TechScript source extensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TechScriptExtension {
    /// Canonical default TechScript source file (`.txs`).
    Txs,
    /// Web-specific declarative source for WebForge pages, components, layouts (`.wtxs`).
    Wtxs,
    /// Server-side source for backend APIs, background tasks, jobs, workers (`.stxs`).
    Stxs,
    /// Mobile Android source for declarative screens and device APIs (`.atxs`).
    Atxs,
    /// 3D scene source for WebGL, Three.js bindings, and graphics pipelines (`.3txs` or `.gtxs`).
    Scene3D,
    /// Legacy TechScript source extension (`.tech`), supported for backward compatibility.
    LegacyTech,
}

/// The architectural role and execution domain of a TechScript source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SourceRole {
    /// General-purpose application logic, functions, models, libraries, CLI tools.
    General,
    /// Declarative WebForge pages, layouts, web components, browser interactions.
    Web,
    /// Backend HTTP services, microservices, database jobs, cron tasks, daemons.
    Server,
    /// Mobile UI screens, mobile platform integrations, device sensors.
    Android,
    /// 3D scenes, viewports, meshes, camera paths, materials, animation loops.
    Scene3D,
}

/// Stability tier of a TechScript extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ExtensionStability {
    /// The official default language extension (>99% of code).
    OfficialDefault,
    /// Stable specialized extension for domain-specific clarity and tooling.
    Specialized,
    /// Experimental extension under active evolution.
    Experimental,
    /// Legacy extension supported for backward compatibility with migration recommendations.
    Legacy,
}

/// Full specification and metadata for a registered TechScript source file.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SourceFileDescriptor {
    pub extension: TechScriptExtension,
    pub primary_extension_str: &'static str,
    pub dot_extension_str: &'static str,
    pub role: SourceRole,
    pub stability: ExtensionStability,
    pub allows_entry_point: bool,
    pub allows_module_import: bool,
    pub recommended_command: &'static str,
    pub description: &'static str,
}

impl TechScriptExtension {
    /// Match an extension string (case-sensitive) to a known [`TechScriptExtension`].
    pub fn from_ext_str(ext: &str) -> Option<Self> {
        match ext {
            TECHSCRIPT_EXTENSION => Some(Self::Txs),
            WEB_EXTENSION => Some(Self::Wtxs),
            SERVER_EXTENSION => Some(Self::Stxs),
            ANDROID_EXTENSION => Some(Self::Atxs),
            SCENE3D_EXTENSION | GRAPHICS_ALT_EXTENSION => Some(Self::Scene3D),
            LEGACY_TECH_EXTENSION => Some(Self::LegacyTech),
            _ => None,
        }
    }

    /// Extract and identify the TechScript extension from a filesystem path.
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext_str = path.extension()?.to_str()?;
        Self::from_ext_str(ext_str)
    }

    /// Primary extension string without leading dot.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Txs => TECHSCRIPT_EXTENSION,
            Self::Wtxs => WEB_EXTENSION,
            Self::Stxs => SERVER_EXTENSION,
            Self::Atxs => ANDROID_EXTENSION,
            Self::Scene3D => SCENE3D_EXTENSION,
            Self::LegacyTech => LEGACY_TECH_EXTENSION,
        }
    }

    /// Extension string with leading dot.
    pub const fn dot_extension(&self) -> &'static str {
        match self {
            Self::Txs => TECHSCRIPT_DOT_EXTENSION,
            Self::Wtxs => WEB_DOT_EXTENSION,
            Self::Stxs => SERVER_DOT_EXTENSION,
            Self::Atxs => ANDROID_DOT_EXTENSION,
            Self::Scene3D => SCENE3D_DOT_EXTENSION,
            Self::LegacyTech => LEGACY_TECH_DOT_EXTENSION,
        }
    }

    /// Intended domain role of the source file.
    pub const fn role(&self) -> SourceRole {
        match self {
            Self::Txs | Self::LegacyTech => SourceRole::General,
            Self::Wtxs => SourceRole::Web,
            Self::Stxs => SourceRole::Server,
            Self::Atxs => SourceRole::Android,
            Self::Scene3D => SourceRole::Scene3D,
        }
    }

    /// Stability status of the extension.
    pub const fn stability(&self) -> ExtensionStability {
        match self {
            Self::Txs => ExtensionStability::OfficialDefault,
            Self::Wtxs | Self::Stxs | Self::Atxs => ExtensionStability::Specialized,
            Self::Scene3D => ExtensionStability::Experimental,
            Self::LegacyTech => ExtensionStability::Legacy,
        }
    }

    /// Returns `true` if this is the official canonical default extension (`.txs`).
    pub const fn is_canonical(&self) -> bool {
        matches!(self, Self::Txs)
    }

    /// Returns `true` if this is a legacy extension (`.tech`).
    pub const fn is_legacy(&self) -> bool {
        matches!(self, Self::LegacyTech)
    }

    /// Returns `true` if this is an experimental extension (`.3txs`).
    pub const fn is_experimental(&self) -> bool {
        matches!(self, Self::Scene3D)
    }

    /// Returns an optional migration or guidance note for developer tooling.
    pub fn migration_hint(&self) -> Option<&'static str> {
        match self {
            Self::LegacyTech => Some(
                "Notice: '.tech' is a legacy extension. TechScript 2.0 uses '.txs' as canonical default. Run 'tsc migrate' to update.",
            ),
            Self::Wtxs => Some(
                "Tip: '.wtxs' files contain declarative WebForge components and pages. Build or run with 'tsc dev' or 'tsc build web'.",
            ),
            Self::Stxs => Some(
                "Tip: '.stxs' files define server-side routes and background workers. Run with 'tsc dev' or 'tsc run'.",
            ),
            Self::Scene3D => Some(
                "Experimental: '.3txs' provides dedicated 3D scene declarations. Embed in WebForge pages or build via 'tsc build web'.",
            ),
            _ => None,
        }
    }

    /// Get complete descriptor metadata.
    pub fn descriptor(&self) -> SourceFileDescriptor {
        match self {
            Self::Txs => SourceFileDescriptor {
                extension: Self::Txs,
                primary_extension_str: TECHSCRIPT_EXTENSION,
                dot_extension_str: TECHSCRIPT_DOT_EXTENSION,
                role: SourceRole::General,
                stability: ExtensionStability::OfficialDefault,
                allows_entry_point: true,
                allows_module_import: true,
                recommended_command: "tsc run",
                description: "Canonical TechScript source for general logic, functions, modules, and ordinary application code.",
            },
            Self::Wtxs => SourceFileDescriptor {
                extension: Self::Wtxs,
                primary_extension_str: WEB_EXTENSION,
                dot_extension_str: WEB_DOT_EXTENSION,
                role: SourceRole::Web,
                stability: ExtensionStability::Specialized,
                allows_entry_point: true,
                allows_module_import: true,
                recommended_command: "tsc dev / tsc build web",
                description: "WebForge declarative web source for pages, components, layouts, and interactive client UI.",
            },
            Self::Stxs => SourceFileDescriptor {
                extension: Self::Stxs,
                primary_extension_str: SERVER_EXTENSION,
                dot_extension_str: SERVER_DOT_EXTENSION,
                role: SourceRole::Server,
                stability: ExtensionStability::Specialized,
                allows_entry_point: true,
                allows_module_import: true,
                recommended_command: "tsc dev / tsc run",
                description: "Server-side and background-task source for REST handlers, database workers, cron jobs, and daemons.",
            },
            Self::Atxs => SourceFileDescriptor {
                extension: Self::Atxs,
                primary_extension_str: ANDROID_EXTENSION,
                dot_extension_str: ANDROID_DOT_EXTENSION,
                role: SourceRole::Android,
                stability: ExtensionStability::Specialized,
                allows_entry_point: true,
                allows_module_import: true,
                recommended_command: "tsc build android",
                description: "Android mobile source for declarative activities, views, and mobile sensor integrations.",
            },
            Self::Scene3D => SourceFileDescriptor {
                extension: Self::Scene3D,
                primary_extension_str: SCENE3D_EXTENSION,
                dot_extension_str: SCENE3D_DOT_EXTENSION,
                role: SourceRole::Scene3D,
                stability: ExtensionStability::Experimental,
                allows_entry_point: false,
                allows_module_import: true,
                recommended_command: "tsc build web",
                description: "Experimental 3D scene source for WebGL viewports, Three.js bindings, meshes, and animation loops.",
            },
            Self::LegacyTech => SourceFileDescriptor {
                extension: Self::LegacyTech,
                primary_extension_str: LEGACY_TECH_EXTENSION,
                dot_extension_str: LEGACY_TECH_DOT_EXTENSION,
                role: SourceRole::General,
                stability: ExtensionStability::Legacy,
                allows_entry_point: true,
                allows_module_import: true,
                recommended_command: "tsc migrate",
                description: "Legacy TechScript source extension supported for full backward compatibility.",
            },
        }
    }

    /// List of all supported source extensions in resolution order.
    pub const fn all_supported() -> &'static [TechScriptExtension] {
        &[
            Self::Txs,
            Self::Wtxs,
            Self::Stxs,
            Self::Atxs,
            Self::Scene3D,
            Self::LegacyTech,
        ]
    }
}

// ── Validation and Resolution Functions ────────────────────────────────────────

/// Returns `true` if the given path has the canonical `.txs` extension.
///
/// This function enforces the frozen design decision that `.txs` is the official default.
/// Case-sensitive: `.TXS`, `.Txs`, and other variants are rejected.
#[inline]
pub fn is_techscript_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext == TECHSCRIPT_EXTENSION)
}

/// Returns `true` if the given path has any officially supported TechScript source extension
/// (`.txs`, `.wtxs`, `.stxs`, `.atxs`, `.3txs`, `.gtxs`, `.tech`).
#[inline]
pub fn is_supported_source_file(path: &Path) -> bool {
    TechScriptExtension::from_path(path).is_some()
}

/// Resolution error types for source and module file resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceResolutionError {
    /// File not found in the searched locations.
    NotFound {
        module: String,
        searched_paths: Vec<PathBuf>,
    },
    /// Multiple files with different extensions match the same module name.
    AmbiguousImport {
        module: String,
        candidates: Vec<PathBuf>,
    },
    /// Security violation: attempt to traverse outside the allowed directory.
    PathTraversalDetected { path: String },
    /// Unrecognized file extension.
    UnsupportedExtension {
        path: String,
        extension: String,
    },
}

impl fmt::Display for SourceResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { module, searched_paths } => {
                write!(
                    f,
                    "Module '{}' not found. Searched locations: {:?}",
                    module, searched_paths
                )
            }
            Self::AmbiguousImport { module, candidates } => {
                write!(
                    f,
                    "Ambiguous module import for '{}'. Multiple matching files found: {:?}",
                    module, candidates
                )
            }
            Self::PathTraversalDetected { path } => {
                write!(
                    f,
                    "Security violation: path traversal outside project root is forbidden: '{}'",
                    path
                )
            }
            Self::UnsupportedExtension { path, extension } => {
                write!(
                    f,
                    "Unsupported file extension '.{}' for '{}'. Supported extensions: .txs (default), .wtxs, .stxs, .atxs, .3txs, .tech",
                    extension, path
                )
            }
        }
    }
}

impl std::error::Error for SourceResolutionError {}

/// Resolves a module reference relative to `base_dir` deterministically.
///
/// # Rules
/// 1. Prohibits directory traversal escaping `base_dir`.
/// 2. If `module_path` already includes a recognized extension (e.g. `pages/home.wtxs` or `lib.txs`),
///    resolves that exact file.
/// 3. If extension is omitted:
///    - Checks `.txs` FIRST (canonical default). If found, returns it immediately without ambiguity.
///    - If no `.txs` exists, checks specialized and legacy extensions (`.wtxs`, `.stxs`, `.atxs`, `.3txs`, `.tech`).
///    - If exactly one matches, returns it.
///    - If multiple match, returns [`SourceResolutionError::AmbiguousImport`].
///    - If none match, returns [`SourceResolutionError::NotFound`].
pub fn resolve_source_module(base_dir: &Path, module_path: &str) -> Result<PathBuf, SourceResolutionError> {
    // Check path traversal and directory escapes
    let normalized = module_path.replace('\\', "/");
    if normalized.starts_with('/') || (normalized.len() >= 2 && normalized.as_bytes()[1] == b':') {
        return Err(SourceResolutionError::PathTraversalDetected {
            path: module_path.to_string(),
        });
    }

    if normalized.contains("..") {
        let mut depth: isize = 0;
        for comp in normalized.split('/') {
            if comp == ".." {
                depth -= 1;
                if depth < 0 {
                    return Err(SourceResolutionError::PathTraversalDetected {
                        path: module_path.to_string(),
                    });
                }
            } else if !comp.is_empty() && comp != "." {
                depth += 1;
            }
        }
    }

    let rel_path = PathBuf::from(module_path.replace('\\', "/"));

    // Case 1: Extension explicitly specified
    if let Some(ext) = rel_path.extension().and_then(|e| e.to_str()) {
        if let Some(_known_ext) = TechScriptExtension::from_ext_str(ext) {
            let full_path = base_dir.join(&rel_path);
            if full_path.exists() {
                return Ok(full_path);
            } else {
                return Err(SourceResolutionError::NotFound {
                    module: module_path.to_string(),
                    searched_paths: vec![full_path],
                });
            }
        } else {
            return Err(SourceResolutionError::UnsupportedExtension {
                path: module_path.to_string(),
                extension: ext.to_string(),
            });
        }
    }

    // Case 2: Extension omitted. Rule: .txs is checked FIRST!
    let txs_candidate = base_dir.join(format!("{}.txs", module_path));
    if txs_candidate.exists() {
        return Ok(txs_candidate);
    }

    // Check specialized extensions
    let specialized_extensions = [
        WEB_EXTENSION,
        SERVER_EXTENSION,
        ANDROID_EXTENSION,
        SCENE3D_EXTENSION,
        GRAPHICS_ALT_EXTENSION,
        LEGACY_TECH_EXTENSION,
    ];

    let mut found_candidates = Vec::new();
    let mut searched = vec![txs_candidate];

    for ext in specialized_extensions {
        let cand = base_dir.join(format!("{}.{}", module_path, ext));
        if cand.exists() {
            found_candidates.push(cand);
        } else {
            searched.push(cand);
        }
    }

    match found_candidates.len() {
        0 => Err(SourceResolutionError::NotFound {
            module: module_path.to_string(),
            searched_paths: searched,
        }),
        1 => Ok(found_candidates.into_iter().next().unwrap()),
        _ => Err(SourceResolutionError::AmbiguousImport {
            module: module_path.to_string(),
            candidates: found_candidates,
        }),
    }
}

// ── CommonError Integration ────────────────────────────────────────────────────

/// Errors originating from the `techscript_common` crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommonError {
    /// The provided file does not have a supported TechScript extension.
    InvalidExtension {
        path: String,
        message: String,
    },
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommonError::InvalidExtension { path, message } => {
                write!(f, "{message}: '{path}'")
            }
        }
    }
}

impl std::error::Error for CommonError {}

/// Validates that the given path has the canonical `.txs` extension.
///
/// Preserves backward compatibility with existing tests that assert strictly on `.txs`.
pub fn validate_extension(path: &Path) -> Result<(), CommonError> {
    if is_techscript_file(path) {
        Ok(())
    } else {
        Err(CommonError::InvalidExtension {
            path: path.display().to_string(),
            message: format!(
                "TechScript source files must use the '{TECHSCRIPT_DOT_EXTENSION}' extension"
            ),
        })
    }
}

/// Validates that the given path has a supported TechScript extension and returns its descriptor.
pub fn validate_source_file(path: &Path) -> Result<SourceFileDescriptor, CommonError> {
    match TechScriptExtension::from_path(path) {
        Some(ext) => Ok(ext.descriptor()),
        None => Err(CommonError::InvalidExtension {
            path: path.display().to_string(),
            message: format!(
                "Unsupported file extension. TechScript source files must use '{}' (default), '{}', '{}', '{}', or '{}'",
                TECHSCRIPT_DOT_EXTENSION,
                WEB_DOT_EXTENSION,
                SERVER_DOT_EXTENSION,
                ANDROID_DOT_EXTENSION,
                SCENE3D_DOT_EXTENSION
            ),
        }),
    }
}
