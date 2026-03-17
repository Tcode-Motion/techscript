This folder is for **prebuilt `techscript-lsp` binaries** shipped with the VS Code extension.

Layout (recommended):

- `server/win32-x64/techscript-lsp.exe`
- `server/linux-x64/techscript-lsp`
- `server/linux-arm64/techscript-lsp`
- `server/darwin-x64/techscript-lsp`
- `server/darwin-arm64/techscript-lsp`

If the binary for the current platform is missing, the extension falls back to launching `techscript-lsp` from `PATH`.

