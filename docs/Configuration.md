# Configuration Reference — TechScript 2.0

Configure extension and LSP options under your project or workspace configuration settings.

## VS Code Settings

Configure workspace options in your `.vscode/settings.json`:

```json
{
  "techscript.lsp.path": "techscript-lsp",
  "techscript.formatting.indentSize": 4,
  "techscript.linter.enabled": true,
  "techscript.compiler.optLevel": "default",
  "techscript.compiler.target": "vm"
}
```

## Settings Schema

- **`techscript.lsp.path`**: Path to the `techscript-lsp` binary.
- **`techscript.formatting.indentSize`**: Indentation size for auto-formatting.
- **`techscript.linter.enabled`**: Toggles background lint diagnostics.
- **`techscript.compiler.optLevel`**: Optimization levels passed to optimizer (`none`, `less`, `default`, `aggressive`).
- **`techscript.compiler.target`**: Compilation target backend (`vm` or `native`).
