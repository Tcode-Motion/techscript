# TechScript Language Support for VS Code

Bring the friendly, expressive power of **TechScript** directly into your favorite editor! This extension provides official language support for TechScript (`.txs`), equipping you with a premium, robust, and beautiful programming workspace.

Developed and maintained with passion by **[Tanmoy Majumder](https://github.com/Tcode-Motion)**.

---

## 🌟 Key Features

* **🌈 Premium Syntax Highlighting**: Custom, rich lexical tokenizer rules mapping high-contrast vibrant styles to TechScript’s friendly keywords (`say`, `make`, `be`, `if`, `else`, `while`, `fun`, `return`, `and`, `or`, `not`).
* **⚡ Production Snippets**: Type a shortcut and press Tab to scaffold statements instantly. Pre-baked snippets include `say` statements, variables, `if/else` conditionals, loops, functions, and standard web/GUI structures.
* **📂 Workspace File Icon Recognition**: Adds the official custom TechScript file icon to `.txs` and `.tx` files inside your Explorer sidebar.
* **📟 VS Code Commands Integration**: Trigger compilation, REPLs, and the native Cyberpunk Studio IDE directly from the VS Code Command Palette:
  * `TechScript: Run Current File` - Instantly compiles and executes your active file.
  * `TechScript: Open Interactive REPL` - Launches the interactive command line prompt.
  * `TechScript: Launch Studio IDE` - Runs the high-fidelity stateful `tech_studio.exe` docking dashboard.

---

## 🚀 What's New in v1.0.6

* **Stateful Docking Workspace support**: Perfectly optimized syntax configurations matching the new Cyberpunk IDE's resizable split containers (`egui_dock`).
* **Explorer Double-Click Integration support**: Fully registered `--double-click` command options mapping to ensure your script host console windows stay active until you press Enter.
* **Improved Web Module bindings**: Full language support highlighting for the new built-in plain-English web backend bindings (`start server at port`).

---

## ⌨️ Code Snippets & Shortcuts

Boost your developer speed with these instant shortcuts:

| Shortcut Prefix | Code Snippet Generated |
| :--- | :--- |
| `say` | `say "message"` |
| `make` | `make variable be value` |
| `if` | `if expression then ... else ...` |
| `while` | `while expression do ...` |
| `fun` | `fun name(arguments) do ... end` |
| `web` | `start server at port 8080` |

---

## ⚙️ Extension Commands

Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS) to bring up the Command Palette, and type `TechScript` to run the following integrated actions:

1. **TechScript: Run Current File**
   * *Under the hood*: Invokes `tech run <active_file_path>` in a new integrated terminal pane.
2. **TechScript: Open Interactive REPL**
   * *Under the hood*: Launches `tech repl` in your terminal for immediate command testing.
3. **TechScript: Launch Studio IDE**
   * *Under the hood*: Launches the compiled graphical TechScript Studio executable, letting you code inside the premium Cyberpunk theme workspace.

---

## 🛠️ Step-by-Step Marketplace Publishing Guide

To compile and package the extension locally into a `.vsix` file ready for upload to the **Visual Studio Marketplace** or **Open VSX Registry**:

### 1. Pre-requisites
Ensure you have Node.js installed, then install the official publishing tool globally:
```bash
npm install -g @vscode/vsce
```

### 2. Package the Extension
Open a terminal inside the `vscode-extension` directory and run:
```bash
vsce package
```
This command compiles the files, validates the package properties, and builds the deployable archive: `techscript-1.0.6.vsix`.

### 3. Upload to the Marketplace
1. Sign in to the [Visual Studio Marketplace Management Portal](https://marketplace.visualstudio.com/manage).
2. Create or select your publisher (e.g., `techscript-team` or a personalized profile).
3. Click the **New Extension** button and select **Visual Studio Code**.
4. Drag and drop the generated `techscript-1.0.6.vsix` file.
5. Click **Upload**! Your extension will go live within minutes and become searchable directly inside VS Code and Cursor extensions view!

---

## 🧑‍💻 Creator & Maintainer
Created with love by **[Tanmoy Majumder](https://github.com/Tcode-Motion)**.
* **GitHub**: [@Tcode-Motion](https://github.com/Tcode-Motion)
* **Official Repository**: [Tcode-Motion/techscript](https://github.com/Tcode-Motion/techscript)

---

## 📄 License
This extension is released under the **MIT License**. Feel free to modify, extend, and distribute!
