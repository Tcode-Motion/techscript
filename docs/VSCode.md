# VS Code Extension Guide — TechScript 2.0

The TechScript VS Code extension provides a fully integrated development environment for writing, checking, running, debugging, formatting, and linting TechScript (`.txs`, `.tsx`) source files.

## Features

- **Syntax Highlighting**: Complete semantic highlighting and TM grammar tokenization.
- **Diagnostics**: Real-time error/warning diagnostics provided by the compiler pipeline.
- **Intellisense**: Full autocomplete, signature help, hover information, and outline support.
- **Debugger**: Interactive debugger with step execution, breakpoints, local scope variable view, and call stack frames.
- **Tasks**: Configured build task provider running `tsc build` directly inside VS Code.
- **Commands**:
  - `TechScript: Run File` (invokes `tsc run`)
  - `TechScript: Build Project` (invokes `tsc build`)
  - `TechScript: Check Code` (invokes `tsc check`)
  - `TechScript: Format File` (invokes `tsc fmt`)
  - `TechScript: Lint File` (invokes `tsc lint`)
  - `TechScript: Restart Language Server`
  - `TechScript: Show AST`
  - `TechScript: Show IR`
  - `TechScript: Show Bytecode`
  - `TechScript: Open Documentation`

## Debugging Configuration

Add a `techscript` configuration to your `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "techscript",
      "request": "launch",
      "name": "Debug TechScript File",
      "program": "${file}",
      "stopOnEntry": true
    }
  ]
}
```
