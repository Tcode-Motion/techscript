<div align="center">

<img src="https://raw.githubusercontent.com/Tcode-Motion/techscript/main/editors/vscode/icon.png" width="128" alt="TechScript Logo" />

# TechScript 2.0 — Official VS Code Extension

**The official language support extension for TechScript 2.0**  
Syntax highlighting · IntelliSense · Diagnostics · Debugging · Formatting · Snippets

[![Version](https://img.shields.io/badge/version-2.0.0-blue.svg)](https://github.com/Tcode-Motion/techscript/releases)
[![VS Code](https://img.shields.io/badge/VS%20Code-%5E1.75.0-blue.svg)](https://code.visualstudio.com/)
[![Open VSX](https://img.shields.io/badge/Open%20VSX-Extension-purple.svg)](https://open-vsx.org/extension/Tcode-Motion/techscript)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/Tcode-Motion/techscript/blob/main/LICENSE)
[![GitHub](https://img.shields.io/badge/GitHub-Tcode--Motion%2Ftechscript-black.svg)](https://github.com/Tcode-Motion/techscript)

</div>

---

## ✨ Features

### 🎨 Syntax Highlighting
Full TextMate grammar for all TechScript 2.0 constructs — keywords, types, functions, variables, strings, comments, operators, and control flow keywords all beautifully colorized.

### 🧠 IntelliSense & Autocomplete
Context-aware completions powered by the Language Server Protocol (LSP). Get suggestions for:
- Standard library built-ins (`say`, `ask`, `len`, `type`, etc.)
- User-defined variables, functions, models, enums, and traits
- Import symbols from installed packages

### 🔴 Real-Time Diagnostics
Inline compiler errors and lint warnings as you type — no need to manually run `tsc check`. Squiggly lines, hover messages, and problem-panel integration keep your code clean.

### 🗂️ File Icon Theme
Sleek custom TechScript icons for `.txs`, `.tsx`, and `.tech` files in the Explorer sidebar. Activate it from **Preferences → File Icon Theme → TechScript Icon Theme**.

### 🐛 Debugger Integration
Launch and debug TechScript scripts directly from VS Code using the built-in Debug panel. Set breakpoints, step through code, inspect variables, and view the call stack.

### ⚡ Code Snippets
Productivity-boosting snippets for the most common TechScript patterns:

| Snippet Prefix | What it inserts |
|---|---|
| `build` | Function declaration block |
| `model` | Model / class structure |
| `when` | `when`/`else` conditional block |
| `each` | `each`/`in` loop |
| `repeat` | `repeat N` loop |
| `attempt` | Try/catch error handler |
| `say` | Print to stdout |
| `ask` | Read from stdin |
| `enum` | Enum declaration |
| `trait` | Trait declaration |
| `test` | Unit test function |
| `main` | Main entry point |
| `package` | Package descriptor block |

### 🛠️ Toolchain Commands
Run all `tsc` toolchain commands from the VS Code Command Palette (`Ctrl+Shift+P`):

| Command | Description |
|---|---|
| `TechScript: Run File` | Execute the current `.txs` file |
| `TechScript: Build Project` | Build the whole project |
| `TechScript: Check Code` | Type-check without running |
| `TechScript: Test Project` | Run all unit tests |
| `TechScript: Format File` | Auto-format with `tsfmt` |
| `TechScript: Lint File` | Analyze code with `tslint` |
| `TechScript: Open REPL` | Launch interactive TechScript shell |
| `TechScript: Generate Docs` | Generate project documentation |
| `TechScript: Package Project` | Package project for distribution |
| `TechScript: Show Compiler Version` | Show installed `tsc` version |
| `TechScript: Show AST` | Dump the Abstract Syntax Tree |
| `TechScript: Show IR` | Dump the Intermediate Representation |
| `TechScript: Show Bytecode` | Dump compiled bytecode |
| `TechScript: Restart Language Server` | Restart `techscript-lsp` |

### 📂 Sidebar Panel
A dedicated **TechScript sidebar** in the Activity Bar gives quick access to:
- **Project Explorer** — browse your `.txs` files
- **Package Manager** — manage dependencies
- **Examples** — open bundled example projects
- **Templates** — scaffold new files from templates
- **Documentation** — browse offline HTML docs

---

## 🚀 Getting Started

### Step 1 — Install the TechScript Toolchain

The extension requires the `tsc` compiler and `techscript-lsp` language server to be installed on your system.

**Windows** — Download `TechScript_Setup.exe` from [GitHub Releases](https://github.com/Tcode-Motion/techscript/releases) and run the installer.

**Linux / macOS** — Run this one-liner in your terminal:
```bash
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

**Android (Termux)** — Run these commands in Termux:
```bash
pkg update
pkg install curl
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

### Step 2 — Install This Extension

Search for **"TechScript 2.0"** in the VS Code Extensions marketplace, or install it directly from the [Marketplace listing](https://marketplace.visualstudio.com/items?itemName=tanmoy.techscript).

### Step 3 — Open a TechScript Project

```bash
tsc new my_project
cd my_project
code .
```

Open `src/main.txs` and start writing code — syntax highlighting and IntelliSense activate automatically!

---

## 🖥️ Hello World Example

Create a file `hello.txs`:

```txs
say "Hello, World! 🌍"

name = ask "What is your name? "
say $"Welcome, {name}! You are using TechScript 2.0."

prices = [100, 250, 400]
for price in prices
    discounted = price * 0.9
    say $"  Original: {price} → Discounted: {discounted}"
end
```

Press **`F5`** or click **TechScript: Run File** in the title bar to execute.

---

## ⚙️ Extension Settings

This extension contributes the following settings (configurable in `settings.json`):

| Setting | Default | Description |
|---|---|---|
| `techscript.lsp.path` | `""` | Absolute path to `techscript-lsp` if not in PATH |
| `techscript.compiler.path` | `"tsc"` | Path to the `tsc` compiler binary |
| `techscript.format.onSave` | `false` | Auto-format files on save |
| `techscript.lint.onSave` | `true` | Run linter on save |

---

## 🐛 Debugging

This extension provides a built-in debug adapter for TechScript. Add a `launch.json` entry:

```json
{
  "type": "techscript",
  "request": "launch",
  "name": "Debug TechScript File",
  "program": "${file}",
  "stopOnEntry": true
}
```

---

## 📋 Requirements

- **VS Code** `^1.75.0`
- **TechScript Toolchain** `v2.0.0` — includes `tsc` compiler and `techscript-lsp`
- The `tsc` binary must be available in your system `PATH` (or configured via extension settings)

---

## 🔗 Useful Links

| Resource | Link |
|---|---|
| 🌐 Official Website | [techscript.is-a.dev](https://techscript.is-a.dev) |
| 📦 GitHub Repository | [Tcode-Motion/techscript](https://github.com/Tcode-Motion/techscript) |
| 🖥️ VS Code Marketplace | [Visual Studio Code Marketplace](https://marketplace.visualstudio.com/items?itemName=tanmoy.techscript) |
| 🚀 Open VSX Registry | [Open VSX Registry Page](https://open-vsx.org/extension/Tcode-Motion/techscript) |
| 📥 Releases & Downloads | [GitHub Releases](https://github.com/Tcode-Motion/techscript/releases) |
| 💬 Discussions & Support | [GitHub Discussions](https://github.com/Tcode-Motion/techscript/discussions) |
| 🎬 YouTube Channel | [@tcodemotin](https://www.youtube.com/@tcodemotin) |
| 🐛 Report an Issue | [GitHub Issues](https://github.com/Tcode-Motion/techscript/issues) |

---

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](https://github.com/Tcode-Motion/techscript/blob/main/CONTRIBUTING.md) for guidelines.

---

## 📄 License

MIT License — Copyright © 2026 [Tcode-Motion](https://github.com/Tcode-Motion)

---

<div align="center">
Made with ❤️ by <a href="https://github.com/Tcode-Motion">Tcode-Motion</a>
</div>
