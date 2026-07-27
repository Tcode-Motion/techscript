//! # tsc install Command
//!
//! Downloads and registers dependencies declared in the project manifest.

use crate::exit_code::ExitCode;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use techscript_package_manager::{
    CapabilityValidator, DependencyConfig, DependencySolver, LockedPackage, Lockfile, Manifest,
    PackageVerifier, Registry, RegistryPackageVersion, Version, VersionConstraint,
};

pub fn execute(package: &str) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let manifest_path = current_dir.join("tech.toml");
    if !manifest_path.exists() {
        eprintln!("Error: tech.toml manifest not found in current directory.");
        return ExitCode::Failure;
    }

    let manifest_content = match fs::read_to_string(&manifest_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to read tech.toml: {}", e);
            return ExitCode::Failure;
        }
    };

    let mut manifest: Manifest = match toml::from_str(&manifest_content) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: Failed to parse tech.toml: {}", e);
            return ExitCode::Failure;
        }
    };

    println!("Resolving package '{}'...", package);

    // Setup Mock Registry index for resolving packages
    let mut registry = Registry::new();
    registry.register(RegistryPackageVersion {
        name: "log".to_string(),
        version: Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        dependencies: HashMap::new(),
        required_capabilities: vec!["FileSystem".to_string()],
        checksum: "sha_log_100".to_string(),
        signature: "log:sha_log_100:pubkey".to_string(),
    });
    registry.register(RegistryPackageVersion {
        name: "http".to_string(),
        version: Version {
            major: 2,
            minor: 1,
            patch: 0,
        },
        dependencies: HashMap::new(),
        required_capabilities: vec!["Network".to_string()],
        checksum: "sha_http_210".to_string(),
        signature: "http:sha_http_210:pubkey".to_string(),
    });

    // Parse the package constraint
    let (name, constraint_str) = if package.contains('@') {
        let parts: Vec<&str> = package.split('@').collect();
        (parts[0].to_string(), parts[1].to_string())
    } else {
        (package.to_string(), "^1.0.0".to_string())
    };

    let constraint = match VersionConstraint::parse(&constraint_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Invalid version constraint: {}", e);
            return ExitCode::Failure;
        }
    };

    // Update dependencies in manifest
    let mut deps = manifest.dependencies.unwrap_or_default();
    deps.insert(
        name.clone(),
        DependencyConfig::SimpleVersion(constraint_str.clone()),
    );
    manifest.dependencies = Some(deps.clone());

    // Resolve dependencies using package manager solver
    let solver = DependencySolver::new(&registry, false);
    let mut root_constraints = HashMap::new();
    for (k, d_config) in &deps {
        let constraint_val = match d_config {
            DependencyConfig::SimpleVersion(s) => s.as_str(),
            DependencyConfig::Detailed {
                version: Some(s), ..
            } => s.as_str(),
            _ => "*",
        };
        if let Ok(c) = VersionConstraint::parse(constraint_val) {
            root_constraints.insert(k.clone(), c);
        }
    }

    let resolved = match solver.resolve("root", "1.0.0", &root_constraints) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error resolving dependencies: {}", e);
            return ExitCode::Failure;
        }
    };

    // Perform verification checks
    let parent_caps = manifest.package.capabilities.clone().unwrap_or_default();
    let allowed_elevation = manifest
        .package
        .allow_capability_elevation
        .clone()
        .unwrap_or_default();

    for pkg in &resolved {
        // Sandboxing capability elevation checks
        if let Err(e) = CapabilityValidator::validate_elevation(
            &parent_caps,
            &pkg.required_capabilities,
            &allowed_elevation,
            &pkg.name,
        ) {
            eprintln!("Error: {}", e);
            return ExitCode::Failure;
        }

        // Digital signature verification
        if let Err(e) =
            PackageVerifier::verify_signature(&pkg.name, &pkg.checksum, &pkg.signature, "pubkey")
        {
            eprintln!("Error: {}", e);
            return ExitCode::Failure;
        }
    }

    // Install packages physically under packages/ directory
    let packages_dir = current_dir.join("packages");
    fs::create_dir_all(&packages_dir).ok();

    let mut locked_packages = Vec::new();
    for pkg in &resolved {
        let pkg_path = packages_dir.join(&pkg.name);
        fs::create_dir_all(&pkg_path).ok();

        // Write mock entry file for the dependency
        let entry_file = pkg_path.join("lib.txs");
        let mock_src = format!(
            "// Mock package: {}\npub function version() {{ return \"{}\"; }}\n",
            pkg.name, pkg.version
        );
        fs::write(entry_file, mock_src).ok();

        let pkg_manifest = pkg_path.join("tech.toml");
        let pkg_toml = format!(
            "[package]\nname = \"{}\"\nversion = \"{}\"\nentry = \"lib.txs\"\n",
            pkg.name, pkg.version
        );
        fs::write(pkg_manifest, pkg_toml).ok();

        locked_packages.push(LockedPackage {
            name: pkg.name.clone(),
            version: pkg.version.to_string(),
            source: "registry".to_string(),
            checksum: pkg.checksum.clone(),
            dependencies: Some(pkg.dependencies.keys().cloned().collect()),
        });
    }

    // Save updated tech.toml
    let updated_toml = match toml::to_string(&manifest) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error generating manifest TOML: {}", e);
            return ExitCode::Failure;
        }
    };
    if let Err(e) = fs::write(&manifest_path, updated_toml) {
        eprintln!("Error writing tech.toml: {}", e);
        return ExitCode::Failure;
    }

    // Write lockfile
    let lockfile = Lockfile {
        package: locked_packages,
    };
    let lockfile_toml = match toml::to_string(&lockfile) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error generating lockfile TOML: {}", e);
            return ExitCode::Failure;
        }
    };
    let lockfile_path = current_dir.join("tech.lock");
    if let Err(e) = fs::write(&lockfile_path, lockfile_toml) {
        eprintln!("Error writing tech.lock: {}", e);
        return ExitCode::Failure;
    }

    println!("Successfully installed and registered '{}'.", package);
    ExitCode::Success
}
