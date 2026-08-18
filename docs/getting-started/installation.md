# Installing TechScript

TechScript is designed to install cleanly on multiple platforms with zero external dependencies.

---

## 🪟 Windows Setup

### Option 1: Setup Wizard (Recommended)
1. Go to the [Releases](https://github.com/Tcode-Motion/techscript/releases) page on GitHub.
2. Download **`TechScript_v2.0.0_x64.exe`** (or the latest setup file).
3. Double-click to execute the installer. The wizard will automatically:
   * Install the native Rust compiler and VM binaries.
   * Add the `tech` command-line executable to your system environment `PATH` variables.
   * Associate `.txs` source files with the console runner host.
   * Offer to configure the VS Code Syntax Highlighter extension.

### Option 2: Python package wrapper (Cross-platform)
```powershell
pip install techscript-lang
```

---

## 🐧 Linux Setup

Use the official one-line installation shell script:
```bash
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

Alternatively, build from source using Cargo:
```bash
git clone https://github.com/Tcode-Motion/techscript.git
cd techscript
cargo build --release
sudo cp target/release/tech /usr/local/bin/
```

---

## 🍎 macOS Setup

Download and install using Homebrew:
```bash
brew tap tcode-motion/techscript
brew install techscript
```

Or run the standard installer script:
```bash
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

---

## 🚀 Post-Installation Check
Verify that the `tech` executable is correctly configured in your `PATH` by running:
```bash
tech version
```
This should output the current active release version (e.g. `TechScript v2.0.0`).
