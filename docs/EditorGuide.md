# Editor & IDE Integration Guide — TechScript 2.0

TechScript 2.0 provides developer experience utilities comparable to modern languages. Follow this guide to configure and get started.

## Project Structure

A typical TechScript project contains:
- `package.json` or `package` description block.
- `main.txs` (the primary entry point).
- `.agents/` (custom workspace rules).

## Setting Up Your Editor

### VS Code
1. Compile the extension package:
   ```bash
   npx vsce package
   ```
2. Install the `.vsix` file:
   ```bash
   code --install-extension techscript-2.0.0.vsix
   ```
3. Ensure `techscript-lsp` is on your system PATH or configure the setting `techscript.lsp.path` in your workspace Settings.

### Other Editors (Sublime Text, Vim, Helix, Neovim)
1. Point your editor's LSP client configuration to the executable `techscript-lsp`.
2. Configure file type detection to associate `.txs` and `.tsx` with `techscript`.
