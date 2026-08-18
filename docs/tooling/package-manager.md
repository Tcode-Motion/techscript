# Package Manager

TechScript's built-in package manager allows you to manage dependencies, fetch external packages, and publish packages to the registry.

---

## 🏗️ Basic Commands

### `init`
Creates a new package structure with a template `package.toml` in the current directory:
```bash
tech package init my_app
```

### `install`
Downloads and installs all dependencies listed in the `package.toml` file:
```bash
tech package install
```
To install a specific package and add it to `package.toml`:
```bash
tech package install http_helper@1.0.0
```

### `update`
Updates all dependencies to their newest allowed versions based on semver rules:
```bash
tech package update
```

### `publish`
Publishes your package to the official TechScript Package Registry:
```bash
tech package publish
```

---

## 📂 Cache Directory
All downloaded packages are stored globally in a central cache directory:
* **Windows**: `C:\Users\<user>\.techscript\cache`
* **Linux/macOS**: `~/.techscript/cache`

During compilation, the compiler resolves imports by checking the local cache first before searching internet sources.
See [Packages Guide](packages.md) for details on metadata.
