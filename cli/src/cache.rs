//! # TechScript Compiler Driver — Incremental Cache
//!
//! Tracks file modification timestamps, source hashing, and dependency hashes
//! to avoid recompilation of unchanged modules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use techscript_optimizer::OptimizationLevel;

/// Incremental compilation fingerprint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Fingerprint {
    pub source_hash: u64,
    pub dependency_hash: u64,
    pub compiler_version: String,
    pub optimization_level: String,
}

/// Persistent on-disk cache database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheDatabase {
    pub fingerprints: HashMap<String, Fingerprint>,
}

pub struct BuildCache {
    pub cache_dir: PathBuf,
    pub db_path: PathBuf,
    pub db: CacheDatabase,
}

impl BuildCache {
    /// Loads the cache database from the build directory.
    pub fn load(cache_dir: &Path) -> anyhow::Result<Self> {
        let db_path = cache_dir.join("fingerprints.json");
        let db = if let Ok(content) = std::fs::read_to_string(&db_path) {
            serde_json::from_str(&content).unwrap_or_else(|_| CacheDatabase {
                fingerprints: HashMap::new(),
            })
        } else {
            CacheDatabase {
                fingerprints: HashMap::new(),
            }
        };

        Ok(Self {
            cache_dir: cache_dir.to_path_buf(),
            db_path,
            db,
        })
    }

    /// Saves the database to disk.
    pub fn save(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.cache_dir)?;
        let content = serde_json::to_string_pretty(&self.db)?;
        std::fs::write(&self.db_path, content)?;
        Ok(())
    }

    /// Checks if a file needs to be rebuilt.
    pub fn needs_rebuild(&self, path: &Path, current: &Fingerprint) -> bool {
        let key = path.to_string_lossy().to_string();
        if let Some(cached) = self.db.fingerprints.get(&key) {
            cached != current
        } else {
            true
        }
    }

    /// Updates the fingerprint for a compiled module path.
    pub fn update_fingerprint(&mut self, path: &Path, fingerprint: Fingerprint) {
        let key = path.to_string_lossy().to_string();
        self.db.fingerprints.insert(key, fingerprint);
    }

    /// Stores a lowered IR module in the cache.
    pub fn store_ir(&self, path: &Path, module: &techscript_ir::Module) -> anyhow::Result<()> {
        let ir_dir = self.cache_dir.join("ir");
        std::fs::create_dir_all(&ir_dir)?;
        let cache_file = ir_dir.join(hash_path_filename(path) + ".ir");
        let data = bincode::serialize(module)
            .map_err(|e| anyhow::anyhow!("Serialization failed: {}", e))?;
        std::fs::write(cache_file, data)?;
        Ok(())
    }

    /// Loads a cached IR module.
    pub fn load_ir(&self, path: &Path) -> anyhow::Result<Option<techscript_ir::Module>> {
        let cache_file = self
            .cache_dir
            .join("ir")
            .join(hash_path_filename(path) + ".ir");
        if let Ok(data) = std::fs::read(cache_file) {
            let module = bincode::deserialize(&data)
                .map_err(|e| anyhow::anyhow!("Deserialization failed: {}", e))?;
            Ok(Some(module))
        } else {
            Ok(None)
        }
    }

    /// Stores a compiled bytecode module in the cache.
    pub fn store_bytecode(
        &self,
        path: &Path,
        module: &techscript_bytecode::BytecodeModule,
    ) -> anyhow::Result<()> {
        let bc_dir = self.cache_dir.join("bytecode");
        std::fs::create_dir_all(&bc_dir)?;
        let cache_file = bc_dir.join(hash_path_filename(path) + ".tsb");
        let data = techscript_bytecode::BytecodeSerializer::serialize(module)
            .map_err(|e| anyhow::anyhow!("Bytecode serialization failed: {}", e))?;
        std::fs::write(cache_file, data)?;
        Ok(())
    }

    /// Loads a cached bytecode module.
    pub fn load_bytecode(
        &self,
        path: &Path,
    ) -> anyhow::Result<Option<techscript_bytecode::BytecodeModule>> {
        let cache_file = self
            .cache_dir
            .join("bytecode")
            .join(hash_path_filename(path) + ".tsb");
        if let Ok(data) = std::fs::read(cache_file) {
            let module = techscript_bytecode::BytecodeSerializer::deserialize(&data)
                .map_err(|e| anyhow::anyhow!("Bytecode deserialization failed: {}", e))?;
            Ok(Some(module))
        } else {
            Ok(None)
        }
    }

    /// Clears the cache directory.
    pub fn clear(&self) -> anyhow::Result<()> {
        if self.cache_dir.exists() {
            std::fs::remove_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }
}

/// Helper to generate a unique filename for a cached path.
pub fn hash_path_filename(path: &Path) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Helper to calculate the source text hash.
pub fn compute_source_hash(source: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
}

/// Helper to compute combined fingerprint for a unit.
pub fn compute_fingerprint(
    source_hash: u64,
    dependency_hash: u64,
    opt_level: OptimizationLevel,
) -> Fingerprint {
    Fingerprint {
        source_hash,
        dependency_hash,
        compiler_version: techscript_common::TECHSCRIPT_VERSION.to_string(),
        optimization_level: format!("{:?}", opt_level),
    }
}
