use std::fs;
use std::path::Path;
use techscript_common::{
    is_supported_source_file, is_techscript_file, resolve_source_module, validate_source_file,
    ExtensionStability, SourceResolutionError, SourceRole, TechScriptExtension,
};

#[test]
fn test_canonical_txs_file_extension() {
    let canonical = Path::new("src/main.txs");
    assert!(is_techscript_file(canonical));
    assert!(is_supported_source_file(canonical));

    let ext = TechScriptExtension::from_path(canonical).expect("should resolve txs");
    assert_eq!(ext, TechScriptExtension::Txs);
    assert_eq!(ext.as_str(), "txs");
    assert_eq!(ext.role(), SourceRole::General);
    assert_eq!(ext.stability(), ExtensionStability::OfficialDefault);
    assert!(ext.is_canonical());
    assert!(!ext.is_legacy());
    assert_eq!(ext.migration_hint(), None);

    assert!(validate_source_file(canonical).is_ok());
}

#[test]
fn test_specialized_file_extensions_descriptors() {
    // .wtxs (WebForge web page / component)
    let web_path = Path::new("src/pages/dashboard.wtxs");
    assert!(!is_techscript_file(web_path)); // Canonical check is strictly .txs
    assert!(is_supported_source_file(web_path));
    let web_ext = TechScriptExtension::from_path(web_path).unwrap();
    assert_eq!(web_ext, TechScriptExtension::Wtxs);
    assert_eq!(web_ext.role(), SourceRole::Web);
    assert_eq!(web_ext.stability(), ExtensionStability::Specialized);

    // .stxs (Server task / endpoint / background worker)
    let srv_path = Path::new("src/server/auth_worker.stxs");
    assert!(is_supported_source_file(srv_path));
    let srv_ext = TechScriptExtension::from_path(srv_path).unwrap();
    assert_eq!(srv_ext, TechScriptExtension::Stxs);
    assert_eq!(srv_ext.role(), SourceRole::Server);
    assert_eq!(srv_ext.stability(), ExtensionStability::Specialized);

    // .atxs (Android declarative UI)
    let and_path = Path::new("src/android/login_screen.atxs");
    assert!(is_supported_source_file(and_path));
    let and_ext = TechScriptExtension::from_path(and_path).unwrap();
    assert_eq!(and_ext, TechScriptExtension::Atxs);
    assert_eq!(and_ext.role(), SourceRole::Android);
    assert_eq!(and_ext.stability(), ExtensionStability::Specialized);

    // .3txs and .gtxs (3D scene / WebGL canvas)
    let scene_path = Path::new("src/scenes/hero_viewport.3txs");
    assert!(is_supported_source_file(scene_path));
    let scene_ext = TechScriptExtension::from_path(scene_path).unwrap();
    assert_eq!(scene_ext, TechScriptExtension::Scene3D);
    assert_eq!(scene_ext.role(), SourceRole::Scene3D);
    assert_eq!(scene_ext.stability(), ExtensionStability::Experimental);

    let gtxs_path = Path::new("src/scenes/shader_mesh.gtxs");
    let gtxs_ext = TechScriptExtension::from_path(gtxs_path).unwrap();
    assert_eq!(gtxs_ext, TechScriptExtension::Scene3D);
    assert_eq!(gtxs_ext.role(), SourceRole::Scene3D);
}

#[test]
fn test_legacy_tech_extension_compatibility_and_hint() {
    let legacy_path = Path::new("legacy_app.tech");
    assert!(is_supported_source_file(legacy_path));
    let legacy_ext = TechScriptExtension::from_path(legacy_path).unwrap();
    assert_eq!(legacy_ext, TechScriptExtension::LegacyTech);
    assert!(legacy_ext.is_legacy());
    assert!(!legacy_ext.is_canonical());

    let hint = legacy_ext.migration_hint().expect("legacy must have migration hint");
    assert!(hint.contains(".txs"));
    assert!(hint.contains("tsc migrate"));
}

#[test]
fn test_deterministic_resolution_txs_precedence() {
    let temp_dir = std::env::temp_dir().join(format!("txs_test_prec_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    // Create both user/service.txs and user/service.wtxs
    let user_dir = temp_dir.join("user");
    fs::create_dir_all(&user_dir).unwrap();
    let txs_file = user_dir.join("service.txs");
    let wtxs_file = user_dir.join("service.wtxs");
    fs::write(&txs_file, "export function hello() {}").unwrap();
    fs::write(&wtxs_file, "page Service: show heading \"Service\"").unwrap();

    // Canonical .txs MUST be preferred deterministically over specialized extensions
    let resolved = resolve_source_module(&temp_dir, "user/service").unwrap();
    assert_eq!(resolved, txs_file);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_resolution_ambiguous_import_detection() {
    let temp_dir = std::env::temp_dir().join(format!("txs_test_ambig_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    // Create user/worker.wtxs and user/worker.stxs (WITHOUT user/worker.txs)
    let user_dir = temp_dir.join("user");
    fs::create_dir_all(&user_dir).unwrap();
    fs::write(user_dir.join("worker.wtxs"), "page Worker:").unwrap();
    fs::write(user_dir.join("worker.stxs"), "task Worker:").unwrap();

    // Resolution must fail deterministically with AmbiguousImport error detailing both candidates
    let err = resolve_source_module(&temp_dir, "user/worker").unwrap_err();
    match err {
        SourceResolutionError::AmbiguousImport { module, candidates } => {
            assert_eq!(module, "user/worker");
            assert_eq!(candidates.len(), 2);
        }
        other => panic!("Expected AmbiguousImport, got: {:?}", other),
    }

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_resolution_path_traversal_protection() {
    let temp_dir = std::env::temp_dir().join(format!("txs_test_trav_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let err = resolve_source_module(&temp_dir, "../../../etc/passwd").unwrap_err();
    match err {
        SourceResolutionError::PathTraversalDetected { path } => {
            assert!(path.contains(".."));
        }
        other => panic!("Expected PathTraversalDetected, got: {:?}", other),
    }

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_unsupported_extension_validation() {
    assert!(validate_source_file(Path::new("script.py")).is_err());
    assert!(validate_source_file(Path::new("app.js")).is_err());
    assert!(validate_source_file(Path::new("main.rs")).is_err());
    assert!(validate_source_file(Path::new("no_extension")).is_err());
}

#[test]
fn test_tsc_migrate_tooling_renames_legacy_tech() {
    let temp_dir = std::env::temp_dir().join(format!("txs_test_mig_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let tech_file = temp_dir.join("app.tech");
    fs::write(
        &tech_file,
        "import \"./helper.tech\";\nfunction test() { return 1; }\n",
    )
    .unwrap();

    let toml_file = temp_dir.join("tech.toml");
    fs::write(
        &toml_file,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nentry = \"app.tech\"\n",
    )
    .unwrap();

    // Execute tsc migrate on temp_dir
    let exit_code = techscript_cli::commands::migrate::execute(Some(temp_dir.to_str().unwrap()));
    assert_eq!(exit_code, techscript_cli::exit_code::ExitCode::Success);

    // Verify .tech was renamed to .txs
    assert!(!tech_file.exists(), "app.tech should be renamed");
    let txs_file = temp_dir.join("app.txs");
    assert!(txs_file.exists(), "app.txs should exist after migration");

    // Verify tech.toml entry was updated
    let updated_toml = fs::read_to_string(&toml_file).unwrap();
    assert!(updated_toml.contains("entry = \"app.txs\""));

    // Verify explicit import was modernized
    let updated_src = fs::read_to_string(&txs_file).unwrap();
    assert!(updated_src.contains("./helper.txs"));

    let _ = fs::remove_dir_all(&temp_dir);
}
