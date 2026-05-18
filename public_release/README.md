<p align="center">
  <img src="techscript-logo.png" alt="TechScript Logo" width="128" height="128" />
</p>

<h1 align="center">TechScript v1.0.6 Official Public Release</h1>

<p align="center">
  <strong>A friendly, plain-English programming language and high-performance developer workspace built entirely in Rust.</strong>
</p>

<p align="center">
  <a href="https://github.com/Tcode-Motion"><img src="https://img.shields.io/badge/Developer-Tanmoy%20Majumder-E0F2FE?style=flat-square&logo=github&logoColor=0B0F19" alt="Developer" /></a>
  <a href="https://github.com/Tcode-Motion/techscript"><img src="https://img.shields.io/badge/Repository-techscript-0284C7?style=flat-square&logo=git&logoColor=ffffff" alt="Repository" /></a>
  <a href="https://github.com/Tcode-Motion/techscript/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-059669?style=flat-square" alt="License" /></a>
</p>

---

## 🌟 Welcome to TechScript

**TechScript** is an elegant, human-centric, high-performance programming language designed to replace confusing syntaxes with friendly, expressive plain-English instructions. Underneath its natural language surface lies a lightning-fast native compiler and Virtual Machine written entirely in pure **Rust**, delivering high stability, robust error handling, and memory efficiency with zero runtime runtime bottlenecks.

This directory contains the **official standalone production release (v1.0.6)**, which includes the full command-line compiler, native VM, and the state-of-the-art **TechScript Studio IDE**.

---

## 🚀 Version 1.0.6 Feature Highlights

### 1. The Redesigned TechScript Studio IDE
* **Resizable Docking Layout (`egui_dock`)**: A state-of-the-art, responsive multi-pane workspace layout. Drag, split, and dock layouts to match your workflow:
  * 📝 **Editor**: High-performance editor with customized line-number gutter column.
  * 📂 **Workspace Explorer**: Browse, load, and edit `.txs` scripts instantly.
  * 📟 **Native Multi-Channel Terminal**: Real-time standard output and diagnostic feedback logs.
  * 🔍 **AST & Bytecode Inspector**: Inspect the live parsed Abstract Syntax Tree and virtual machine execution instructions side-by-side.
* **Premium Cyberpunk Aesthetics**: A sleek geometric dark theme tailored with vibrant HSL cyberpunk accents:
  * Custom tabs for `📟 Stdout` (Emerald green `#0DF28B`), `⚙ Compiler` (Electric blue `#00A3FF`), and `🐞 VM Debugger` (Lavender purple `#D8B4FE`).
  * Direct inline controls including `🧹 Clear Logs`, `📋 Copy Output`, and a glowing `▶ Re-run Script` action button.
* **High-Fidelity Official Branding**: Features the official high-resolution **TechScript dragon logo** embedded both in the system title bar and scaled beautifully at `22x22` in the top left header menu.

### 2. Smart Double-Click Explorer Execution
* Full Windows Shell integration: Double-clicking any `.txs` script inside Windows Explorer launches a native command-line host that executes the code and stays open, printing:
  ```text
  [Process completed. Press Enter to exit...]
  ```
  This ensures your output terminals do not instantly disappear, allowing you to review logs, compile states, or runtime outputs at your own pace!

### 3. Professional Setup Maintenance Manager
* Features a single, unified installer `TechScript_v1.0.6_x64.exe` that supports advanced maintenance configurations:
  * ⚙ **Modify**: Customise path environment variables, shortcut folders, and script association options.
  * 🔧 **Repair**: Safely checks for missing components, restores broken registry keys, and reinstalls release binaries.
  * 🗑 **Uninstall**: Cleanly purges path settings, file configurations, and folders in a single click.

---

## 📦 What's Inside This Release Folder?

| Asset | Description |
| :--- | :--- |
| **`TechScript_v1.0.6_x64.exe`** | The complete graphical installer containing the compiler, virtual machine, and IDE workspace. |
| **`techscript-logo.png`** | High-resolution branding logo for visual integration. |
| **`README.md`** | This setup manual. |

---

## 🛠️ Step-by-Step Installation Guide

1. **Launch Setup**: Double-click the `TechScript_v1.0.6_x64.exe` installer executable in this folder.
2. **Configure Options**:
   * Tick **Add TechScript to PATH** to run `tech` CLI commands from standard command prompts.
   * Tick **Associate .txs Files** to enable explorer double-click execution.
3. **Complete Installation**: Click Install to deploy the binaries.
4. **Run a Test**: Double-click any of the example scripts inside the `examples/` directory of the repository to confirm successful installation!

---

## 💻 Writing Your First Script

Create a file named `hello.txs` and write the following plain-English code:

```techscript
// Say hello to the world!
say "Hello World!"

// Perform calculations naturally
make x be 10
make y be 20
make sum be x + y

say "The sum is:"
say sum
```

Run it instantly inside the **Studio IDE** or via command-line:
```bash
tech run hello.txs
```

---

## 🧑‍💻 Creator & Author
Developed and maintained with passion by **[Tanmoy Majumder](https://github.com/Tcode-Motion)**.
* **GitHub Profile**: [@Tcode-Motion](https://github.com/Tcode-Motion)
* **Official Repository**: [Tcode-Motion/techscript](https://github.com/Tcode-Motion/techscript)

For bug reports, feature requests, or contributions, please open an issue on the official GitHub repository!
