# Package Manager Example

This example demonstrates the structured layout of a `package.toml` configuration to manage third-party library dependencies and metadata in TechScript.

## Code (`package.toml`)
```toml
[package]
name = "math_demo"
version = "1.0.0"
authors = ["Tanmoy Majumder <tanmoy@example.com>"]
description = "A math library demo package"

[dependencies]
http_helper = "1.2.0"
```

## Initializing dependencies
```bash
tech package install
```
This fetches the required dependencies matching the semver specifications and stores them in the local package compiler cache directory.
