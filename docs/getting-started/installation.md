# Installing TechScript 2.0.0

TechScript is designed to install cleanly across desktop, server, and mobile developer environments with zero external runtime dependencies.

---

## 📋 System Requirements & Support Matrix

| Platform | Architecture | Binary Package | Status | Verification |
| :--- | :--- | :--- | :--- | :--- |
| **Windows 10/11** | `x86_64` | `techscript-2.0.0-windows-x86_64.zip` | Supported | Verified Native CI |
| **Linux (glibc 2.31+)** | `x86_64` | `techscript-2.0.0-linux-x86_64.tar.gz` | Supported | Verified Native CI |
| **Linux (ARM64)** | `aarch64` | `techscript-2.0.0-linux-aarch64.tar.gz` | Supported | Cross-Compiled & Tested |
| **macOS (Apple Silicon)**| `aarch64` | `techscript-2.0.0-macos-aarch64.tar.gz` | Supported | Verified Native CI |
| **macOS (Intel)** | `x86_64` | `techscript-2.0.0-macos-x86_64.tar.gz` | Supported | Verified Native CI |
| **Android Termux** | `aarch64` | `techscript-2.0.0-termux-aarch64.tar.gz` | Supported | Verified on Termux aarch64 |

---

## 🪟 Windows Installation

### Option 1: Pre-built Archive (Recommended)

1. Download [`techscript-2.0.0-windows-x86_64.zip`](https://github.com/Tcode-Motion/techscript/releases/download/v2.0.0/techscript-2.0.0-windows-x86_64.zip).
2. Extract the archive into a permanent folder (e.g. `C:\Program Files\TechScript` or `%LOCALAPPDATA%\TechScript`).
3. Add the extracted directory to your system `PATH`:
   - Open PowerShell as Administrator:
     ```powershell
     [Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:LOCALAPPDATA\TechScript", [EnvironmentVariableTarget]::User)
     ```
4. Open a fresh terminal and verify:
   ```powershell
   techscript version
   webforge version
   ```

### Option 2: Build from Source with Cargo

Prerequisites: Rust 1.80+ and MSVC Build Tools.
```powershell
git clone https://github.com/Tcode-Motion/techscript.git
cd techscript
cargo build --release --workspace
# Binaries are placed in target\release\ (techscript.exe, webforge.exe, tsc.exe, tech.exe)
```

---

## 🐧 Linux Installation

### Option 1: Automated Shell Installer

Run the universal installer script:
```bash
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh -o install.sh && bash install.sh
```

### Option 2: Manual Tarball Installation

```bash
# For x86_64:
curl -LO https://github.com/Tcode-Motion/techscript/releases/download/v2.0.0/techscript-2.0.0-linux-x86_64.tar.gz
tar -xzf techscript-2.0.0-linux-x86_64.tar.gz
sudo install -m 755 techscript /usr/local/bin/
sudo install -m 755 webforge /usr/local/bin/
sudo install -m 755 tsc /usr/local/bin/
sudo install -m 755 tech /usr/local/bin/

# Verify
techscript version
```

---

## 🍎 macOS Installation

### Option 1: Manual Tarball Installation

Download the archive matching your Mac's hardware:
- **Apple Silicon (M1/M2/M3/M4):**
  ```bash
  curl -LO https://github.com/Tcode-Motion/techscript/releases/download/v2.0.0/techscript-2.0.0-macos-aarch64.tar.gz
  tar -xzf techscript-2.0.0-macos-aarch64.tar.gz
  sudo install -m 755 techscript /usr/local/bin/
  sudo install -m 755 webforge /usr/local/bin/
  sudo install -m 755 tsc /usr/local/bin/
  sudo install -m 755 tech /usr/local/bin/
  ```

- **Intel:**
  ```bash
  curl -LO https://github.com/Tcode-Motion/techscript/releases/download/v2.0.0/techscript-2.0.0-macos-x86_64.tar.gz
  tar -xzf techscript-2.0.0-macos-x86_64.tar.gz
  sudo install -m 755 techscript /usr/local/bin/
  sudo install -m 755 webforge /usr/local/bin/
  sudo install -m 755 tsc /usr/local/bin/
  sudo install -m 755 tech /usr/local/bin/
  ```

> [!NOTE]
> **macOS Gatekeeper**: Unsigned development binaries can be authorized by removing the quarantine attribute:
> ```bash
> xattr -d com.apple.quarantine /usr/local/bin/techscript
> xattr -d com.apple.quarantine /usr/local/bin/webforge
> ```

---

## 📱 Android Termux Installation

TechScript provides first-class support for 64-bit ARM Android devices running inside Termux.

### Prerequisites

Open Termux and install system build essentials:
```bash
pkg update && pkg upgrade -y
pkg install -y git rust clang binutils openssl
```

### Option 1: Direct Termux Package

```bash
curl -LO https://github.com/Tcode-Motion/techscript/releases/download/v2.0.0/techscript-2.0.0-termux-aarch64.tar.gz
tar -xzf techscript-2.0.0-termux-aarch64.tar.gz -C $PREFIX/bin/
chmod +x $PREFIX/bin/techscript $PREFIX/bin/webforge $PREFIX/bin/tsc $PREFIX/bin/tech

# Verify
techscript version
```

### Option 2: Compiling inside Termux

```bash
git clone https://github.com/Tcode-Motion/techscript.git
cd techscript
cargo build --release -p techscript_cli
cp target/release/techscript target/release/webforge $PREFIX/bin/
```

---

## 🧪 Post-Installation Smoke Test

1. Verify the toolchain versions:
   ```bash
   techscript version
   webforge version
   ```
   *Expected output: `TechScript 2.0.0` and `WebForge 2.0.0`.*

2. Create and run a new TechScript project:
   ```bash
   techscript new my_project
   cd my_project
   techscript run src/main.txs
   ```

3. Launch a full-stack WebForge application:
   ```bash
   webforge dev
   ```
   *Visit `http://localhost:3000` in your web browser to test real-time hot reloading, 3D WebGL rendering, and SQLite persistence.*

---

## 🛠️ Uninstallation

- **Windows**: Delete the extracted directory and remove it from your User `PATH` environment variable.
- **Linux / macOS**:
  ```bash
  sudo rm -f /usr/local/bin/techscript /usr/local/bin/webforge /usr/local/bin/tsc /usr/local/bin/tech
  ```
- **Android Termux**:
  ```bash
  rm -f $PREFIX/bin/techscript $PREFIX/bin/webforge $PREFIX/bin/tsc $PREFIX/bin/tech
  ```
