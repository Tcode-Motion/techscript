# TechScript for Visual Studio Code

The official Visual Studio Code and Open VSX extension for **TechScript 2.0**.

Provides rich language support, real-time Language Server Protocol (LSP) diagnostics, intelligent code completion, semantic syntax highlighting, colon-based auto-indentation, and canonical snippets.

---

## Features

- **Semantic Syntax Highlighting**: Full grammar support for TechScript v2.0 keywords (`is`, `becomes`, `constant`, `to`, `say`, `return`, `repeat`, `for`, `while`, `class`), double-quoted interpolated strings (`"Hello, {name}!"`), and comments (`#`).
- **Canonical Snippets**: Instant snippets for functions (`to`), loops (`repeat N times:`, `for item in collection:`, `while`), conditionals (`if`/`else if`/`else`), classes (`class`), and standard declarations.
- **Language Server Protocol (LSP)**: Integrated with `techscript-lsp` for real-time error checking, hover documentation, symbol navigation, and autocomplete.
- **Auto-Indentation**: Indentation rules enforcing the 4-space block standard following a colon (`:`).
- **Format & Lint Integration**: Native format-on-save (`tsfmt` / `tsc fmt`) and lint-on-save (`tslint` / `tsc lint`) capabilities.
- **One-Click Execution**: Integrated command palette actions to run, build, check, and test TechScript files directly from the editor.

---

## Quick Start

Create a new file named `hello.txs`:

```techscript
constant app_name is "TechScript"

to greet(user_name):
    say "Hello, {user_name}! Welcome to {app_name}."
    return "done"

names is ["Ada", "Alan", "Linus"]

for person in names:
    greet(person)

repeat 2 times:
    say "TechScript executes natively with zero overhead."
```

Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`), then select:
- **TechScript: Run File** (`techscript.run`)

---

## Extension Commands

| Command | Identifier | Description |
| :--- | :--- | :--- |
| **TechScript: Run File** | `techscript.run` | Run the active TechScript file through the VM. |
| **TechScript: Build Project** | `techscript.build` | Compile the current project. |
| **TechScript: Check Code** | `techscript.check` | Run fast AST and semantic validation. |
| **TechScript: Test Project** | `techscript.test` | Run the built-in test suite. |
| **TechScript: Format File** | `techscript.format` | Format the current file using `tsc fmt`. |
| **TechScript: Lint File** | `techscript.lint` | Run the TechScript linter. |
| **TechScript: Open REPL** | `techscript.openRepl` | Launch an interactive TechScript REPL session. |
| **TechScript: Restart Language Server** | `techscript.restartLsp` | Restart the background `techscript-lsp` process. |

---

## Configuration Settings

| Setting | Default | Description |
| :--- | :--- | :--- |
| `techscript.compiler.path` | `"tsc"` | Path to the TechScript compiler binary (`tsc` or `tsc.exe`). |
| `techscript.lsp.path` | `""` | Path to `techscript-lsp` (defaults to auto-detecting on `PATH`). |
| `techscript.format.onSave` | `false` | Automatically format files on save using `tsc fmt`. |
| `techscript.lint.onSave` | `true` | Automatically run linter on save. |

---

## Installation

- **VS Code Marketplace**: [marketplace.visualstudio.com/items?itemName=tanmoy.techscript](https://marketplace.visualstudio.com/items?itemName=tanmoy.techscript)
- **Open VSX Registry**: [open-vsx.org/extension/tanmoy/techscript](https://open-vsx.org/extension/tanmoy/techscript)
- **Manual Installation**:
  Download `TechScript.vsix` from the [Releases](https://github.com/Tcode-Motion/techscript/releases), open VS Code, go to the Extensions view (`Ctrl+Shift+X`), click the **`...`** (Views and More) menu in the top right, and select **Install from VSIX...**.

---

## Documentation & Community

- [Official Website](https://techscript.is-a.dev/)
- [WebForge Live Example](https://techscript.is-a.dev/techscript/)
- [Documentation](https://github.com/Tcode-Motion/techscript/tree/main/docs)
- [GitHub Repository](https://github.com/Tcode-Motion/techscript)
- [Issue Tracker](https://github.com/Tcode-Motion/techscript/issues)

---

## License

Licensed under the [Apache License 2.0](https://github.com/Tcode-Motion/techscript/blob/main/LICENSE).
