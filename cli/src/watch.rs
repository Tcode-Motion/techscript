//! # TechScript Compiler Driver — File Watcher
//!
//! Polling-based file watcher that detects modifications to .txs and .ts files,
//! triggering re-execution/rebuilds automatically.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub struct FileWatcher {
    pub root: PathBuf,
    pub debounce_ms: u64,
}

impl FileWatcher {
    /// Creates a new polling watcher for the root directory.
    pub fn new(root: &Path, debounce_ms: u64) -> Self {
        Self {
            root: root.to_path_buf(),
            debounce_ms,
        }
    }

    /// Watches for file modification changes and fires the callback.
    pub fn watch<F: Fn(&[PathBuf])>(&self, on_change: F) -> anyhow::Result<()> {
        let mut file_states: HashMap<PathBuf, SystemTime> = HashMap::new();

        // 1. Initial scan
        self.scan_files(&mut file_states)?;

        println!(
            "Watching for changes in {}... (Press Ctrl+C to stop)",
            self.root.display()
        );

        loop {
            std::thread::sleep(Duration::from_millis(self.debounce_ms));

            let mut current_states = HashMap::new();
            if self.scan_files(&mut current_states).is_err() {
                continue; // Ignore brief I/O read errors during writes
            }

            let mut changed = Vec::new();

            // Detect modified or added files
            for (path, &mtime) in &current_states {
                if let Some(&old_mtime) = file_states.get(path) {
                    if mtime > old_mtime {
                        changed.push(path.clone());
                    }
                } else {
                    changed.push(path.clone()); // New file added
                }
            }

            // Detect deleted files
            for path in file_states.keys() {
                if !current_states.contains_key(path) {
                    changed.push(path.clone());
                }
            }

            if !changed.is_empty() {
                on_change(&changed);
                file_states = current_states;
            }
        }
    }

    fn scan_files(&self, states: &mut HashMap<PathBuf, SystemTime>) -> anyhow::Result<()> {
        let mut dirs = vec![self.root.clone()];
        while let Some(dir) = dirs.pop() {
            if dir.is_dir() {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            // Don't watch build or cache directories to prevent infinite loops
                            let name = path.file_name().unwrap_or_default().to_string_lossy();
                            if name != "build" && name != ".git" && name != "target" {
                                dirs.push(path);
                            }
                        } else {
                            let ext = path.extension().unwrap_or_default().to_string_lossy();
                            if ext == "txs" || ext == "ts" {
                                if let Ok(metadata) = entry.metadata() {
                                    if let Ok(modified) = metadata.modified() {
                                        states.insert(path, modified);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // Root is a single file
                let ext = self.root.extension().unwrap_or_default().to_string_lossy();
                if ext == "txs" || ext == "ts" {
                    if let Ok(metadata) = std::fs::metadata(&self.root) {
                        if let Ok(modified) = metadata.modified() {
                            states.insert(self.root.clone(), modified);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
