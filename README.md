# TechScript

TechScript is a simple, friendly programming language (.txs).

## Installation

### Windows
Double-click `setup.bat` to install. This will:
1. Copy `tech.exe` to an isolated `.techscript/bin` user directory.
2. Add the command to your `PATH`.
3. Install the VS Code extension for `.txs` files.

Alternatively, you can manually place `bin/tech.exe` anywhere in your system `PATH`.

## Features
- Interpretable `.txs` source files
- Simple syntax
- Included `tech check` and interactive `tech repl`
- Syntax checking and transpiling capabilities.

## Usage
- Run a script: `tech run file.txs`
- Transpile to python: `tech transpile file.txs`
- Syntax check only: `tech check file.txs`
- Start interactive prompt: `tech repl`

Check the `examples/` directory for syntax basics!
