# TechScript 2.0 VS Code Extension

Official extension providing syntax highlighting and Language Server integration for the TechScript 2.0 language ecosystem.

## Features

- **Syntax Highlighting**: Full token colorizer for keywords, literals, and comments.
- **IntelliSense & Autocomplete**: Scoped lookup of primitives, variables, and models.
- **Diagnostics**: Real-time error feedback in the editor.
- **Formatting**: Integrated document formatting powered by `tsc fmt`.

## Requirements

Ensure that `tsc` and `techscript-lsp` are installed on your system. By default, the extension resolves `techscript-lsp` from your system PATH.

## Extension Settings

This extension contributes the following settings:

* `techscript.lsp.path`: Absolute path to the `techscript-lsp` executable (if not in PATH).
