use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// =========================================================================
// 1. SemVer Structures & Parsing
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn parse(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.trim().split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid SemVer version string: {}", s));
        }
        let major = parts[0].parse()?;
        let minor = parts[1].parse()?;
        let patch = parts[2].parse()?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionConstraint {
    Any,
    Caret(Version),
    Tilde(Version),
    Compatible(Version),
}

impl VersionConstraint {
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        if s == "*" || s.is_empty() {
            return Ok(Self::Any);
        }
        if let Some(rest) = s.strip_prefix('^') {
            let ver = Version::parse(rest)?;
            Ok(Self::Caret(ver))
        } else if let Some(rest) = s.strip_prefix('~') {
            let ver = Version::parse(rest)?;
            Ok(Self::Tilde(ver))
        } else if let Some(rest) = s.strip_prefix(">=") {
            let ver = Version::parse(rest)?;
            Ok(Self::Compatible(ver))
        } else {
            let ver = Version::parse(s)?;
            Ok(Self::Compatible(ver))
        }
    }

    pub fn matches(&self, version: &Version) -> bool {
        match self {
            Self::Any => true,
            Self::Compatible(v) => version >= v,
            Self::Caret(v) => {
                if version < v {
                    return false;
                }
                if v.major > 0 {
                    version.major == v.major
                } else if v.minor > 0 {
                    version.minor == v.minor
                } else {
                    version.patch == v.patch
                }
            }
            Self::Tilde(v) => {
                if version < v {
                    return false;
                }
                version.major == v.major && version.minor == v.minor
            }
        }
    }
}

// =========================================================================
// 2. Manifest & Lockfile Configuration Formats
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub package: PackageConfig,
    pub dependencies: Option<HashMap<String, DependencyConfig>>,
    pub workspace: Option<WorkspaceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub license: Option<String>,
    pub requires_compiler: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub allow_capability_elevation: Option<Vec<String>>,
    pub network_allow: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum DependencyConfig {
    SimpleVersion(String),
    Detailed {
        version: Option<String>,
        path: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    pub package: Vec<LockedPackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
    pub source: String,
    pub checksum: String,
    pub dependencies: Option<Vec<String>>,
}

// =========================================================================
// 3. Package Registry & index Mock
// =========================================================================

#[derive(Debug, Clone)]
pub struct RegistryPackageVersion {
    pub name: String,
    pub version: Version,
    pub dependencies: HashMap<String, VersionConstraint>,
    pub required_capabilities: Vec<String>,
    pub checksum: String,
    pub signature: String,
}

pub struct Registry {
    pub packages: HashMap<String, Vec<RegistryPackageVersion>>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn register(&mut self, ver: RegistryPackageVersion) {
        self.packages.entry(ver.name.clone()).or_default().push(ver);
    }

    pub fn fetch_package_index(&mut self, url: &str) -> Result<()> {
        let response = ureq::get(url)
            .call()
            .map_err(|e| anyhow!("Failed to download registry index: {}", e))?;
        let json_str = response
            .into_string()
            .map_err(|e| anyhow!("Failed to read registry index: {}", e))?;

        #[derive(Deserialize)]
        struct RegistryIndexEntry {
            name: String,
            version: String,
            dependencies: HashMap<String, String>,
            required_capabilities: Vec<String>,
            checksum: String,
            signature: String,
        }

        let entries: Vec<RegistryIndexEntry> = serde_json::from_str(&json_str)
            .map_err(|e| anyhow!("Failed to parse registry index JSON: {}", e))?;

        for entry in entries {
            let mut resolved_deps = HashMap::new();
            for (dep_name, dep_constraint) in entry.dependencies {
                if let Ok(c) = VersionConstraint::parse(&dep_constraint) {
                    resolved_deps.insert(dep_name, c);
                }
            }
            if let Ok(ver) = Version::parse(&entry.version) {
                self.register(RegistryPackageVersion {
                    name: entry.name,
                    version: ver,
                    dependencies: resolved_deps,
                    required_capabilities: entry.required_capabilities,
                    checksum: entry.checksum,
                    signature: entry.signature,
                });
            }
        }
        Ok(())
    }
}

// =========================================================================
// 4. Dependency Solver (DFS Topological Sort & Backtracking Solver)
// =========================================================================

pub struct DependencySolver<'a> {
    pub registry: &'a Registry,
    pub offline: bool,
    pub local_cache: HashSet<String>,
}

impl<'a> DependencySolver<'a> {
    pub fn new(registry: &'a Registry, offline: bool) -> Self {
        Self {
            registry,
            offline,
            local_cache: HashSet::new(),
        }
    }

    pub fn resolve(
        &self,
        root_name: &str,
        root_version: &str,
        constraints: &HashMap<String, VersionConstraint>,
    ) -> Result<Vec<RegistryPackageVersion>> {
        let mut resolved = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        self.solve_recursive(
            root_name,
            root_version,
            constraints,
            &mut resolved,
            &mut visited,
            &mut path,
        )?;

        Ok(resolved)
    }

    fn solve_recursive(
        &self,
        node_name: &str,
        _node_version: &str,
        constraints: &HashMap<String, VersionConstraint>,
        resolved: &mut Vec<RegistryPackageVersion>,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Result<()> {
        if path.contains(&node_name.to_string()) {
            path.push(node_name.to_string());
            return Err(anyhow!(
                "Circular dependency detected: {}",
                path.join(" -> ")
            ));
        }

        path.push(node_name.to_string());

        for (dep_name, constraint) in constraints {
            if visited.contains(dep_name) {
                continue;
            }

            // Find matching versions in registry
            let versions = self
                .registry
                .packages
                .get(dep_name)
                .ok_or_else(|| anyhow!("Package not found in registry index: {}", dep_name))?;

            let mut matched = None;
            for ver in versions {
                if constraint.matches(&ver.version) {
                    matched = Some(ver);
                }
            }

            let resolved_ver = matched.ok_or_else(|| {
                anyhow!(
                    "Conflict resolved failed: No version of {} matches constraint {:?}",
                    dep_name,
                    constraint
                )
            })?;

            self.solve_recursive(
                &resolved_ver.name,
                &resolved_ver.version.to_string(),
                &resolved_ver.dependencies,
                resolved,
                visited,
                path,
            )?;

            visited.insert(dep_name.clone());
            resolved.push(resolved_ver.clone());
        }

        path.pop();
        Ok(())
    }
}

// =========================================================================
// 5. Capability Elevation Check & Sandbox Verifier
// =========================================================================

pub struct CapabilityValidator;

impl CapabilityValidator {
    pub fn validate_elevation(
        root_caps: &[String],
        dependency_caps: &[String],
        allowed_elevations: &[String],
        dep_name: &str,
    ) -> Result<()> {
        let root_set: HashSet<&String> = root_caps.iter().collect();
        for cap in dependency_caps {
            if !root_set.contains(cap) && !allowed_elevations.contains(&dep_name.to_string()) {
                return Err(anyhow!(
                    "Security validation failed: Dependency '{}' requests capability '{}' which is not granted to the parent package.",
                    dep_name,
                    cap
                ));
            }
        }
        Ok(())
    }

    pub fn validate_network_whitelist(
        root_hosts: &[String],
        dependency_hosts: &[String],
        dep_name: &str,
    ) -> Result<()> {
        let root_set: HashSet<&String> = root_hosts.iter().collect();
        let allows_all = root_set.contains(&&"*".to_string());
        for host in dependency_hosts {
            if !allows_all && !root_set.contains(host) {
                return Err(anyhow!(
                    "Security validation failed: Dependency '{}' requests network domain permission for '{}' which is not whitelisted by the parent package.",
                    dep_name,
                    host
                ));
            }
        }
        Ok(())
    }
}

// =========================================================================
// 6. Package Signing / Digital Signature Trust Verification
// =========================================================================

pub struct PackageVerifier;

impl PackageVerifier {
    pub fn verify_signature(
        package_name: &str,
        archive_hash: &str,
        signature: &str,
        public_key: &str,
    ) -> Result<()> {
        // Simulate signature verification using simple hashed identity
        let expected_sig = format!("{}:{}:{}", package_name, archive_hash, public_key);
        if signature != expected_sig {
            return Err(anyhow!(
                "Digital signature verification failed for package: {}",
                package_name
            ));
        }
        Ok(())
    }
}

// =========================================================================
// 7. AST Doc-Comment Extractor (`techdoc`)
// =========================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DocItem {
    pub name: String,
    pub doc: String,
}

pub struct DocExtractor;

impl DocExtractor {
    pub fn extract_comments(content: &str) -> Vec<DocItem> {
        let mut items = Vec::new();
        let mut active_doc = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if let Some(doc) = line.strip_prefix("///") {
                active_doc.push(doc.trim().to_string());
            } else if line.starts_with("function")
                || line.starts_with("make")
                || line.starts_with("model")
                || line.starts_with("struct")
            {
                if !active_doc.is_empty() {
                    // Extract name of symbol following keyword
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 1 {
                        let raw_name = parts[1];
                        let sym_name = raw_name
                            .split('(')
                            .next()
                            .unwrap_or("")
                            .split('{')
                            .next()
                            .unwrap_or("")
                            .split('=')
                            .next()
                            .unwrap_or("")
                            .trim();

                        if !sym_name.is_empty() {
                            items.push(DocItem {
                                name: sym_name.to_string(),
                                doc: active_doc.join("\n"),
                            });
                        }
                    }
                    active_doc.clear();
                }
            } else {
                active_doc.clear();
            }
        }
        items
    }
}
