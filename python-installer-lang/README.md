# TechScript — Python Installer

<div align="center">

[![PyPI](https://img.shields.io/pypi/v/techscript?style=for-the-badge&color=0DF28B)](https://pypi.org/project/techscript/)
[![Python](https://img.shields.io/pypi/pyversions/techscript?style=for-the-badge)](https://pypi.org/project/techscript/)
[![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](../LICENSE)
[![GitHub](https://img.shields.io/badge/GitHub-Tcode--Motion%2Ftechscript-black?style=for-the-badge)](https://github.com/Tcode-Motion/techscript)

**The official Python-based installer for the TechScript 2.0 programming language.**

</div>

---

## What This Package Does

This is a **lightweight bootstrapper only** — it contains no compiler binaries.

When you run `techscript install`, it:
1. Detects your OS and CPU architecture
2. Fetches the correct native binary from [GitHub Releases](https://github.com/Tcode-Motion/techscript/releases/latest)
3. Installs it into your PATH
4. Verifies the installation by running `tsc version`

> **The actual TechScript compiler remains distributed via GitHub Releases.**  
> This PyPI package is under 200 KB and contains only Python code.

---

## Supported Platforms

| Platform | Architecture | Status |
|---|---|---|
| Windows | x64 | ✅ Supported |
| Windows | ARM64 | ✅ Supported |
| Linux | x64 | ✅ Supported |
| Linux | ARM64 | ✅ Supported |
| macOS | Intel (x64) | ✅ Supported |
| macOS | Apple Silicon (ARM64) | ✅ Supported |
| Android Termux | ARM64 | ✅ Supported |
| Android Termux | ARMv7 | 🔄 Optional |

---

## Installation

### Via pip (Recommended)

```bash
pip install techscript
techscript install
```

### Via curl — Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

### Via installer.exe — Windows

Download `TechScript_Setup.exe` from [GitHub Releases](https://github.com/Tcode-Motion/techscript/releases/latest).

### Via Termux — Android

```bash
pkg update
pkg install curl python
# Option 1: Python installer
pip install techscript
techscript install

# Option 2: Shell installer
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

### Via Homebrew — macOS *(coming soon)*

```bash
brew install tcode-motion/tap/techscript
```

### Via Winget — Windows *(coming soon)*

```powershell
winget install Tcode-Motion.TechScript
```

### Via Scoop — Windows *(coming soon)*

```powershell
scoop install techscript
```

---

## Usage

### Install the compiler

```bash
techscript install
```

### Force reinstall / update

```bash
techscript install --force
```

### Check installed version

```bash
techscript version
```

### Check for updates

```bash
techscript check
```

### Uninstall

```bash
techscript uninstall
```

### Forward commands to tsc

```bash
# These are equivalent:
techscript run myfile.txs
tsc run myfile.txs
```

---

## Quick Start

After installing:

```bash
# Create a new project
tsc new hello_world
cd hello_world

# Run it
tsc run
```

Your first `src/main.txs`:

```txs
say "Hello, World! 🌍"
name = ask "What is your name? "
say $"Welcome, {name}! You are running TechScript 2.0."
```

---

## Install Locations

| Platform | Location |
|---|---|
| Windows | `%LOCALAPPDATA%\TechScript\bin\tsc.exe` |
| Linux / macOS | `~/.local/bin/tsc` |
| Termux | `$PREFIX/bin/tsc` |

---

## Debug Mode

Enable verbose output and Python tracebacks:

```bash
techscript install --debug
```

---

## Requirements

- Python 3.8 or newer
- Internet access (to download the compiler binary)
- The `requests` library (installed automatically)

---

## Development

```bash
git clone https://github.com/Tcode-Motion/techscript
cd techscript/python-installer

# Install in editable mode
pip install -e .

# Run tests
python -m pytest tests/ -v

# Build wheel and sdist
python -m build
```

---

## Links

| Resource | Link |
|---|---|
| 🌐 Website | [techscript.is-a.dev](https://techscript.is-a.dev) |
| 📦 GitHub | [Tcode-Motion/techscript](https://github.com/Tcode-Motion/techscript) |
| 🐍 PyPI | [pypi.org/project/techscript](https://pypi.org/project/techscript/) |
| 💻 VS Code Extension | [marketplace.visualstudio.com](https://marketplace.visualstudio.com/items?itemName=tanmoy.techscript) |
| 🐛 Issues | [GitHub Issues](https://github.com/Tcode-Motion/techscript/issues) |
| 💬 Discussions | [GitHub Discussions](https://github.com/Tcode-Motion/techscript/discussions) |

---

## License

MIT License — Copyright © 2026 [Tcode-Motion](https://github.com/Tcode-Motion)
