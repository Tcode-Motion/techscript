# TechScript 2.0 VS Code Extension

Official extension providing syntax highlighting, autocomplete, real-time diagnostics, and Language Server (LSP) integration for the TechScript 2.0 language ecosystem.

## Features

- ✨ **Syntax Highlighting**: Comprehensive token colorizer for keywords, classes, functions, variables, and control flow.
- 🔍 **IntelliSense & Autocomplete**: Context-aware suggestions for standard library builtins and user variables.
- ⚙️ **Diagnostics & Lints**: Live compiler error diagnostics with inline error notes and suggestions.
- 🗃️ **File Icon Theme**: Sleek custom TechScript logos for `.txs` and `.tsx` files inside the Explorer tree.

## Enabling Custom File Icons

To display the custom TechScript file icons in the VS Code sidebar:
1. Open the VS Code Command Palette (`Ctrl + Shift + P` or `Cmd + Shift + P` on macOS).
2. Type and select: **Preferences: File Icon Theme**.
3. Choose **TechScript Icon Theme** from the dropdown list.

## Requirements

Ensure that the TechScript toolchain compiler driver (`tsc`) and language server (`techscript-lsp`) are installed and registered in your system PATH.

## Settings

This extension contributes the following settings:

* `techscript.lsp.path`: Absolute file path to the `techscript-lsp` executable if not available in your system PATH.
