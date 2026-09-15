# TechScript Virtian for Visual Studio Code

TechScript Virtian is the official Visual Studio Code extension for TechScript 2.0. It supports the canonical `do`/`end` language syntax and integrates with the local TechScript toolchain.

## Virtian 2.0.1

Virtian updates the editor experience for the current TechScript 2.0 release:

- Canonical syntax highlighting for `do`, `send`, `when`, `loop`, `repeat`, `for`, `try`, `catch`, and `end`.
- Canonical 2.0 snippets with `end`-delimited blocks—no deprecated brace-based templates.
- Interpolated-string and hash-comment highlighting.
- Format-on-save and lint-on-save settings.
- Configurable paths for `tsc` and `techscript-lsp`.
- Explorer icon theme, task integration, command-palette commands, and language-server support.

## Requirements

- Visual Studio Code 1.75 or newer.
- TechScript 2.0.x installed locally.
- `tsc` available on `PATH`, or configured with `techscript.compiler.path`.
- `techscript-lsp` available on `PATH`, or configured with `techscript.lsp.path`, for language-server diagnostics and completions.

Install the toolchain from the [TechScript repository](https://github.com/Tcode-Motion/techscript).

## Quick start

Create a file named `hello.txs`:

```txs
do greet(name)
    send $"Hello, {name}!"
end

for name in ["Ada", "Lin"]
    say greet(name)
end
```

Open the Command Palette with `Ctrl+Shift+P`, then select **TechScript: Run File**.

## Commands

| Command | Action |
| --- | --- |
| TechScript: Run File | Run the active TechScript file. |
| TechScript: Build Project | Build the current project. |
| TechScript: Check Code | Validate the current project. |
| TechScript: Test Project | Run project tests. |
| TechScript: Format File | Format the active file. |
| TechScript: Lint File | Run the project linter. |
| TechScript: Open REPL | Open an interactive `tsc repl` terminal. |
| TechScript: Show Compiler Version | Display the installed compiler version. |
| TechScript: Show AST / IR / Bytecode | Inspect compiler output for the active file. |
| TechScript: Restart Language Server | Restart `techscript-lsp`. |

## Settings

| Setting | Default | Description |
| --- | --- | --- |
| `techscript.compiler.path` | `"tsc"` | Compiler executable or absolute path. |
| `techscript.lsp.path` | `""` | Language-server executable; empty uses `techscript-lsp` from `PATH`. |
| `techscript.format.onSave` | `false` | Format TechScript files after saving. |
| `techscript.lint.onSave` | `true` | Run the TechScript linter after saving. |

Example `settings.json`:

```json
{
  "techscript.format.onSave": true,
  "techscript.lint.onSave": true
}
```

## Install

- [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=tanmoy.techscript)
- [Open VSX Registry](https://open-vsx.org/extension/Tcode-Motion/techscript)

For a local install, download the `.vsix` file, open the Extensions view in VS Code, select **Views and More** (`...`), then select **Install from VSIX**.

## Support

- [Documentation](https://github.com/Tcode-Motion/techscript/tree/main/docs)
- [Issues](https://github.com/Tcode-Motion/techscript/issues)
- [Discussions](https://github.com/Tcode-Motion/techscript/discussions)

## License

[Apache License 2.0](../../LICENSE)
