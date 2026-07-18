//! # TechScript Compiler Driver — 4-Tier Project Build Graph
//!
//! Organizes compilation into Workspace → Package → Module → CompilationUnit.
//! Discovers workspaces, packages, compiles dependencies, and performs cycle checking.

use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use techscript_common::{FileId, SourceManager};
use techscript_package_manager::{LockedPackage, Lockfile, Manifest};

/// Represents a single source module.
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub file_id: FileId,
    pub path: PathBuf,
    pub imports: Vec<Vec<String>>, // Import paths as list of path components (e.g. ["std", "math"])
}

/// Represents a Package (single tech.toml).
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub root: PathBuf,
    pub manifest: Manifest,
    pub entry_file: PathBuf,
    pub entry_file_id: Option<FileId>,
    pub modules: HashMap<PathBuf, Module>,
}

/// Compilation Status for a Unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationStatus {
    Pending,
    Compiling,
    Compiled,
    Failed,
    Cached,
}

/// Lowest level representation of build scheduler tasks.
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    pub file_id: FileId,
    pub path: PathBuf,
    pub package_name: String,
    pub source_hash: u64,
    pub dependency_hash: u64,
    pub status: CompilationStatus,
}

/// Multi-package workspace.
#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
    pub packages: HashMap<String, Package>,
    pub members: Vec<String>,
}

/// The overall Project build graph.
#[derive(Debug, Clone)]
pub struct ProjectBuildGraph {
    pub workspace: Workspace,
    pub units: IndexMap<FileId, CompilationUnit>,
    pub build_order: Vec<FileId>,
    pub adjacency: HashMap<FileId, Vec<FileId>>,
}

impl ProjectBuildGraph {
    /// Discovers workspace/packages from the given directory root.
    pub fn discover(root: &Path) -> anyhow::Result<Self> {
        let manifest_path = root.join("tech.toml");
        let mut packages = HashMap::new();
        let mut members = Vec::new();

        if manifest_path.exists() {
            let content = std::fs::read_to_string(&manifest_path)?;
            let manifest: Manifest = toml::from_str(&content)?;

            if let Some(workspace_cfg) = &manifest.workspace {
                // Workspace mode
                members = workspace_cfg.members.clone();
                for member_glob in &members {
                    // Simulating directory lookup for member folders
                    let member_dir = root.join(member_glob);
                    if member_dir.exists() {
                        let pkg = load_package(&member_dir)?;
                        packages.insert(pkg.name.clone(), pkg);
                    }
                }
            } else {
                // Single package mode
                let pkg = load_package(root)?;
                packages.insert(pkg.name.clone(), pkg);
            }
        } else {
            // No tech.toml, single file virtual package mode
            let entry = root.to_path_buf();
            let parent = entry.parent().unwrap_or(root).to_path_buf();
            let pkg = Package {
                name: "virtual_pkg".to_string(),
                version: "0.1.0".to_string(),
                root: parent,
                manifest: Manifest {
                    package: techscript_package_manager::PackageConfig {
                        name: "virtual_pkg".to_string(),
                        version: "0.1.0".to_string(),
                        entry: entry
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                        license: None,
                        requires_compiler: None,
                        capabilities: None,
                        allow_capability_elevation: None,
                        network_allow: None,
                    },
                    dependencies: None,
                    workspace: None,
                },
                entry_file: entry,
                entry_file_id: None,
                modules: HashMap::new(),
            };
            packages.insert(pkg.name.clone(), pkg);
        }

        let workspace = Workspace {
            root: root.to_path_buf(),
            packages,
            members,
        };

        Ok(Self {
            workspace,
            units: IndexMap::new(),
            build_order: Vec::new(),
            adjacency: HashMap::new(),
        })
    }

    /// Resolves dependencies by parsing each module's import statements.
    pub fn resolve_dependencies(&mut self, source_mgr: &mut SourceManager) -> anyhow::Result<()> {
        let mut to_resolve = Vec::new();

        // 1. Add all entry points to compilation list
        for pkg in self.workspace.packages.values_mut() {
            if pkg.entry_file.exists() {
                if let Ok(source) = std::fs::read_to_string(&pkg.entry_file) {
                    let fid = source_mgr.add_file(pkg.entry_file.clone(), source.clone());
                    pkg.entry_file_id = Some(fid);
                    to_resolve.push((fid, pkg.entry_file.clone(), pkg.name.clone()));
                }
            }
        }

        // 2. Resolve imports recursively
        let mut visited = HashSet::new();
        while let Some((fid, path, pkg_name)) = to_resolve.pop() {
            if visited.contains(&fid) {
                continue;
            }
            visited.insert(fid);

            let source_file = source_mgr
                .get_file(fid)
                .ok_or_else(|| anyhow::anyhow!("File not found in source manager: {:?}", fid))?;

            // Simple scanner of imports (tokens)
            let mut reporter = techscript_errors::DiagnosticReporter::new();
            let mut lexer = techscript_lexer::Lexer::new(source_file.source());
            let tokens = lexer.lex(&mut reporter).unwrap_or_default();

            let mut imports = Vec::new();
            let mut parser = techscript_parser::Parser::new(&tokens);
            let mut parse_reporter = techscript_errors::DiagnosticReporter::new();
            if let Ok(program) = parser.parse(&mut parse_reporter) {
                for stmt in &program.statements {
                    if let techscript_ast::Statement::Import(import_stmt) = stmt {
                        let path_vec: Vec<String> = import_stmt
                            .path
                            .iter()
                            .map(|ident| ident.name.clone())
                            .collect();
                        imports.push(path_vec);
                    }
                }
            }

            let source_hash = crate::cache::compute_source_hash(source_file.source());

            // Build Module representation
            let module = Module {
                name: path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                file_id: fid,
                path: path.clone(),
                imports: imports.clone(),
            };

            if let Some(pkg) = self.workspace.packages.get_mut(&pkg_name) {
                pkg.modules.insert(path.clone(), module);
            }

            let mut unit = CompilationUnit {
                file_id: fid,
                path: path.clone(),
                package_name: pkg_name.clone(),
                source_hash,
                dependency_hash: 0,
                status: CompilationStatus::Pending,
            };

            // Process resolved imports and queue them
            let parent_dir = path.parent().unwrap_or(&path);
            let mut dep_fids = Vec::new();

            for imp in imports {
                if imp.is_empty() {
                    continue;
                }

                // standard library imports like "std.math" are skipped during incremental local build graphs
                if imp[0] == "std" {
                    continue;
                }

                // Resolve user import path (e.g. import utils -> utils.txs or utils.ts)
                let mut relative_path = parent_dir.to_path_buf();
                for part in &imp {
                    relative_path.push(part);
                }
                let mut import_file = relative_path.with_extension("txs");
                if !import_file.exists() {
                    import_file = relative_path.with_extension("ts");
                }

                if !import_file.exists() {
                    let mut pkg_path = self.workspace.root.join("packages");
                    for part in &imp {
                        pkg_path.push(part);
                    }
                    let mut try_file = pkg_path.with_extension("txs");
                    if !try_file.exists() {
                        try_file = pkg_path.with_extension("ts");
                    }
                    if !try_file.exists() {
                        let manifest_toml = pkg_path.join("tech.toml");
                        if manifest_toml.exists() {
                            if let Ok(toml_content) = std::fs::read_to_string(&manifest_toml) {
                                if let Ok(manifest) = toml::from_str::<techscript_package_manager::Manifest>(&toml_content) {
                                    try_file = pkg_path.join(manifest.package.entry);
                                }
                            }
                        }
                    }
                    if try_file.exists() {
                        import_file = try_file;
                    }
                }

                if import_file.exists() {
                    if let Ok(source) = std::fs::read_to_string(&import_file) {
                        let dep_fid = source_mgr.add_file(import_file.clone(), source);
                        dep_fids.push(dep_fid);
                        to_resolve.push((dep_fid, import_file, pkg_name.clone()));
                    }
                }
            }

            self.adjacency.insert(fid, dep_fids);
            self.units.insert(fid, unit);
        }

        Ok(())
    }

    /// Computes the topological build order. Reports cycles.
    pub fn compute_build_order(&mut self) -> anyhow::Result<()> {
        let mut visited = HashSet::new();
        let mut temp = HashSet::new();
        let mut order = Vec::new();

        for &fid in self.units.keys() {
            if !visited.contains(&fid) {
                self.dfs(fid, &mut visited, &mut temp, &mut order)?;
            }
        }

        self.build_order = order;
        Ok(())
    }

    fn dfs(
        &self,
        fid: FileId,
        visited: &mut HashSet<FileId>,
        temp: &mut HashSet<FileId>,
        order: &mut Vec<FileId>,
    ) -> anyhow::Result<()> {
        if temp.contains(&fid) {
            return Err(anyhow::anyhow!(
                "Circular dependency detected containing file ID {:?}",
                fid
            ));
        }

        if !visited.contains(&fid) {
            temp.insert(fid);
            if let Some(deps) = self.adjacency.get(&fid) {
                for &dep in deps {
                    self.dfs(dep, visited, temp, order)?;
                }
            }
            temp.remove(&fid);
            visited.insert(fid);
            order.push(fid);
        }

        Ok(())
    }

    /// Returns a list of units that have no pending dependencies.
    pub fn ready_units(&self) -> Vec<FileId> {
        let mut ready = Vec::new();
        for (&fid, unit) in &self.units {
            if unit.status != CompilationStatus::Pending {
                continue;
            }
            let mut deps_ready = true;
            if let Some(deps) = self.adjacency.get(&fid) {
                for &dep in deps {
                    if let Some(dep_unit) = self.units.get(&dep) {
                        if dep_unit.status != CompilationStatus::Compiled
                            && dep_unit.status != CompilationStatus::Cached
                        {
                            deps_ready = false;
                            break;
                        }
                    }
                }
            }
            if deps_ready {
                ready.push(fid);
            }
        }
        ready
    }

    /// Marks a compilation unit as compiled.
    pub fn mark_compiled(&mut self, fid: FileId) {
        if let Some(unit) = self.units.get_mut(&fid) {
            unit.status = CompilationStatus::Compiled;
        }
    }
}

fn load_package(dir: &Path) -> anyhow::Result<Package> {
    let manifest_path = dir.join("tech.toml");
    let content = std::fs::read_to_string(&manifest_path)?;
    let manifest: Manifest = toml::from_str(&content)?;
    let entry_file = dir.join(&manifest.package.entry);
    Ok(Package {
        name: manifest.package.name.clone(),
        version: manifest.package.version.clone(),
        root: dir.to_path_buf(),
        manifest,
        entry_file,
        entry_file_id: None,
        modules: HashMap::new(),
    })
}
