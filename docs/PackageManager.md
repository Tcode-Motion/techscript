# TechScript 2.0 Package Manager Guide

TechScript's build driver integrates package management features.

## Manifest Layout (`tech.toml`)

```toml
[package]
name = "my_pkg"
version = "0.1.0"
entry = "src/main.txs"
capabilities = ["FileSystem"]

[dependencies]
log = "^1.0.0"
```

## CLI Commands

### 1. `tsc install <package>`
Resolves and installs a dependency from the index, performing digital signature verification and sandbox capability checking.

### 2. `tsc uninstall <package>`
Unregisters the package from `tech.toml` and deletes the folder.

### 3. `tsc publish`
Signs package archive using private keys and uploads metadata.

### 4. `tsc update`
Resolves constraints and updates all package dependencies.
