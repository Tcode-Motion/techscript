use std::collections::HashMap;
use techscript_package_manager::{
    CapabilityValidator, DependencySolver, DocExtractor, Manifest, PackageVerifier, Registry,
    RegistryPackageVersion, Version, VersionConstraint,
};

#[test]
fn test_manifest_lockfile_serialization() {
    let manifest_toml = r#"
[package]
name = "my_pkg"
version = "1.2.0"
entry = "src/main.ts"
capabilities = ["FileSystem"]
"#;

    let manifest: Manifest = toml::from_str(manifest_toml).unwrap();
    assert_eq!(manifest.package.name, "my_pkg");
    assert_eq!(manifest.package.version, "1.2.0");
    assert_eq!(manifest.package.entry, "src/main.ts");
    assert_eq!(
        manifest.package.capabilities.unwrap(),
        vec!["FileSystem".to_string()]
    );
}

#[test]
fn test_semver_constraint_matching() {
    let constraint_caret = VersionConstraint::parse("^1.2.3").unwrap();
    assert!(constraint_caret.matches(&Version {
        major: 1,
        minor: 3,
        patch: 0
    }));
    assert!(!constraint_caret.matches(&Version {
        major: 2,
        minor: 0,
        patch: 0
    }));

    let constraint_tilde = VersionConstraint::parse("~1.2.3").unwrap();
    assert!(constraint_tilde.matches(&Version {
        major: 1,
        minor: 2,
        patch: 5
    }));
    assert!(!constraint_tilde.matches(&Version {
        major: 1,
        minor: 3,
        patch: 0
    }));
}

#[test]
fn test_dependency_solving_and_cycle_detection() {
    let mut registry = Registry::new();

    // Register package A version 1.0.0
    registry.register(RegistryPackageVersion {
        name: "A".to_string(),
        version: Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        dependencies: HashMap::new(),
        required_capabilities: Vec::new(),
        checksum: "sha_A".to_string(),
        signature: "sig_A".to_string(),
    });

    // Register package B version 1.2.0 depending on A ^1.0.0
    let mut b_deps = HashMap::new();
    b_deps.insert("A".to_string(), VersionConstraint::parse("^1.0.0").unwrap());
    registry.register(RegistryPackageVersion {
        name: "B".to_string(),
        version: Version {
            major: 1,
            minor: 2,
            patch: 0,
        },
        dependencies: b_deps,
        required_capabilities: Vec::new(),
        checksum: "sha_B".to_string(),
        signature: "sig_B".to_string(),
    });

    let solver = DependencySolver::new(&registry, false);

    let mut root_deps = HashMap::new();
    root_deps.insert("B".to_string(), VersionConstraint::parse("^1.0.0").unwrap());
    let resolved = solver.resolve("root", "0.1.0", &root_deps).unwrap();

    // Check resolve order
    assert_eq!(resolved.len(), 2);
    assert_eq!(resolved[0].name, "A");
    assert_eq!(resolved[1].name, "B");

    // Test cycle detection
    let mut cyclic_registry = Registry::new();
    let mut dep_on_y = HashMap::new();
    dep_on_y.insert("Y".to_string(), VersionConstraint::Any);
    cyclic_registry.register(RegistryPackageVersion {
        name: "X".to_string(),
        version: Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        dependencies: dep_on_y,
        required_capabilities: Vec::new(),
        checksum: "sha_X".to_string(),
        signature: "sig_X".to_string(),
    });

    let mut dep_on_x = HashMap::new();
    dep_on_x.insert("X".to_string(), VersionConstraint::Any);
    cyclic_registry.register(RegistryPackageVersion {
        name: "Y".to_string(),
        version: Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        dependencies: dep_on_x,
        required_capabilities: Vec::new(),
        checksum: "sha_Y".to_string(),
        signature: "sig_Y".to_string(),
    });

    let cyclic_solver = DependencySolver::new(&cyclic_registry, false);
    let mut cyclic_deps = HashMap::new();
    cyclic_deps.insert("X".to_string(), VersionConstraint::Any);
    let res = cyclic_solver.resolve("root", "0.1.0", &cyclic_deps);
    assert!(res.is_err());
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Circular dependency detected"));
}

#[test]
fn test_capability_validation() {
    let root_caps = vec!["FileSystem".to_string()];
    let dep_caps = vec!["FileSystem".to_string(), "Process".to_string()];
    let allowed_elevations = vec![];

    let res = CapabilityValidator::validate_elevation(
        &root_caps,
        &dep_caps,
        &allowed_elevations,
        "unsafe_dep",
    );
    assert!(res.is_err());
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("requests capability 'Process' which is not granted"));

    // Verify allowance under elevation whitelist
    let allowed_res = CapabilityValidator::validate_elevation(
        &root_caps,
        &dep_caps,
        &["unsafe_dep".to_string()],
        "unsafe_dep",
    );
    assert!(allowed_res.is_ok());
}

#[test]
fn test_package_verification() {
    let key = "my_public_key";
    let hash = "hash_xyz";
    let signature = format!("{}:{}:{}", "my_pkg", hash, key);

    let res = PackageVerifier::verify_signature("my_pkg", hash, &signature, key);
    assert!(res.is_ok());

    let bad_res = PackageVerifier::verify_signature("my_pkg", hash, "bad_signature", key);
    assert!(bad_res.is_err());
}

#[test]
fn test_doc_extraction() {
    let source_code = r#"
/// This is a test function.
/// It performs a math operation.
function abs(x) {
    return x;
}

make y = 42;
"#;

    let docs = DocExtractor::extract_comments(source_code);
    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0].name, "abs");
    assert_eq!(
        docs[0].doc,
        "This is a test function.\nIt performs a math operation."
    );
}
