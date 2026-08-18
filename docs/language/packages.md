# Packages in TechScript

Packages bundle modules together along with metadata for distribution and installation.

---

## 🏗️ The `package.toml` File
A TechScript package is defined by a `package.toml` file located in the root of the project directory.

Example `package.toml`:
```toml
[package]
name = "http_helper"
version = "0.1.0"
authors = ["Tanmoy Majumder <tanmoy@example.com>"]
description = "A simple HTTP helper utility wrapper for TechScript"

[dependencies]
json_parser = "1.2.0"
```

---

## 🧬 Package Structure
A standard package follows this layout:

```
my_package/
├── package.toml
├── LICENSE
├── README.md
├── src/
│   ├── index.txs
│   └── helper.txs
└── tests/
    └── test_helper.txs
```

The entry point of the package is always `src/index.txs`. When another project depends on this package, importing `use my_package` loads the definitions from `src/index.txs`.
For details about managing packages, see the [Package Manager Guide](package-manager.md).
