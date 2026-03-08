# TechScript

![TechScript Logo](assets/icons/icon-128.png)

TechScript (`.txs`) is a modern, statically-analyzable scripting language that aims to be simple, friendly, and powerful. It is built to offer a clean syntax that compiles directly to Python or evaluates on the fly.

## Why TechScript? What makes it better?

- **Zero Setup Complexity**: No heavy runtime environments to install. Download, run `setup.bat`, and you are writing code in seconds.
- **Python Ecosystem Power**: TechScript can transpile your `.txs` files directly into valid Python code, allowing you to gradually adopt it or use it as a powerful front-end for Python.
- **Friendly Syntax**: Designed to be intuitive for beginners while offering the robustness needed for serious scripting.
- **Built-in Tooling**: The single `tech` binary includes an interpreter, transpiler, syntax checker, and an interactive REPL out of the box.

## Installation

### Windows (Recommended)
1. Download this repository.
2. Double-click `setup.bat` to install. This will:
   - Copy `tech.exe` to a safe `.techscript/bin` user directory.
   - Automatically add the command to your `PATH`.
   - Install the official **VS Code extension**, providing syntax highlighting and file icons for `.txs` files.
3. Restart your terminal or command prompt!

### Manual Installation
You can manually place the `bin/tech.exe` file anywhere in your system's `PATH`. *(Note: You will need to install the VS Code extension manually from the `vscode-extension` folder if you choose this route).*

## Usage Guide

TechScript is designed to be straightforward from the command line. Use the `tech` command to interact with your code.

### 1. Run a script
Execute a script directly using the built-in interpreter:
```bash
tech run file.txs
```

### 2. Transpile to Python
Convert your TechScript code into valid Python code. You can run it immediately or save the output:
```bash
tech transpile file.txs
tech transpile file.txs -o output.py
```

### 3. Syntax Checking
Validate your code for errors without actually running it:
```bash
tech check file.txs
```

### 4. Interactive Console (REPL)
Start the interactive TechScript prompt to test out commands line-by-line:
```bash
tech repl
```

## Documentation & Learning

- **[Language Documentation](docs/)**: Explore the `docs/` folder for a deep dive into the language features, standard library, and architecture.
- **[Examples](examples/)**: Check out the `examples/` directory for ready-to-use snippets demonstrating loops, functions, variables, and more basics to get you started quickly!
