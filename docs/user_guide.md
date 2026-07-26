# TechScript 2.0 Complete User Manual

Welcome to the official TechScript 2.0 documentation. This manual provides a first-class guide for installing, writing, compiling, testing, and debugging TechScript applications.

---

## 1. Installation Guide

### Windows Installation
1. Download the latest `TechScript_Setup.exe` installer from the releases page.
2. Run the installer. It will install the compiler driver `tsc.exe` and the standard library modules to `%USERPROFILE%\.techscript\bin`.
3. The installer will automatically add `tsc` to your user `PATH` environment variable. If installing manually, add the installation directory to your PATH.
4. Open a new terminal window and verify the installation:
   ```cmd
   tsc --version
   ```

### Portable Installation
1. Extract `TechScript_Portable.zip` to a folder of your choice (e.g. `C:\tools\techscript`).
2. Add the directory to your system environment variables `PATH`.
3. Confirm installation with `tsc --help`.

---

## 2. Quick Start

Let's build a simple TechScript console application.

1. **Scaffold a new project**:
   ```bash
   tsc init hello_world --template Console
   ```
   This creates a folder structure:
   - `hello_world/tech.toml` (Project manifest)
   - `hello_world/src/main.txs` (Entrypoint)

2. **Run the program**:
   ```bash
   cd hello_world
   tsc run src/main.txs
   ```
   Output:
   ```
   Hello, TechScript!
   ```

---

## 3. Language Basics

TechScript is an expressive, dynamically typed language.
- **Comments**: Single-line comments start with `#`. Multi-line comments are not supported.
- **Semicolons**: Semicolons are not used. Statements are terminated by newlines.
- **Blocks**: Scopes open with a statement and close with the `end` keyword. Indentation is 4 spaces.

Example:
```txs
# This is a comment
do main()
    message = "Welcome to TechScript!"
    say message
end
```

---

## 4. Variables & Constants

TechScript variables do not require declaration keywords. First assignment declares the variable in the current lexical scope. Constants are declared using the `const` keyword.

- Variable declaration and assignment:
  ```txs
  count = 10
  count = 20      # Reassignment is allowed
  ```
- Constant declaration:
  ```txs
  const PI = 3.14159
  PI = 3.0        # Compile-time Error (TSE0302)!
  ```

### Supported Core Types:
- **Int**: 64-bit signed integer (`42`)
- **Float**: 64-bit floating-point (`3.14`)
- **Bool**: Boolean values (`true`, `false`)
- **Str**: UTF-8 string (`"hello"`)
- **List**: Mutable array (`[1, 2, 3]`)
- **Map**: Key-value map (`{"a": 1, "b": 2}`)
- **Null**: Represents absence of value (`null`)

---

## 5. Functions & Closures

Functions are declared using the `do` keyword and return values with `send`.

### Declaration:
```txs
do add(a, b)
    send a + b
end
```

### Closures:
Functions can capture variables from their enclosing lexical scopes:
```txs
do make_counter()
    count = 0
    send do()
        count = count + 1
        send count
    end
end
```

---

## 6. Structs

Structs group fields together. They are declared with the `struct` keyword and instantiated using `new`.

```txs
struct Point
    x
    y
end

do main()
    p = new Point()
    p.x = 10
    p.y = 20
    say p.x    # prints 10
end
```

---

## 7. Enums

Enums represent a type that can have one of several named variants.

```txs
enum Status
    Idle
    Loading
    Success
    Failure
end
```

---

## 8. Modules & Imports

Every file in TechScript acts as a module. You can export symbols using the `export` keyword and import them using the `use` keyword.

### Module file `math_helper.txs`:
```txs
export const factor = 2

export do double(x)
    send x * factor
end
```

### Main file `main.txs`:
```txs
use math_helper

do main()
    say math_helper.double(10)    # prints 20
end
```

---

## 9. Packages & Manifests

Packages are defined using a `tech.toml` file at the root.

```toml
[package]
name = "my_app"
version = "0.1.0"
description = "A clean TechScript app"
authors = ["Developer"]

[dependencies]
log = "^1.0.0"
```

Running `tsc install` resolves dependencies and creates a `tech.lock` lockfile.

---

## 10. Pattern Matching

The `match` expression provides pattern matching.

```txs
do evaluate_status(status)
    match status
    case Status.Idle
        say "System is idle"
    case Status.Loading
        say "Loading data..."
    case Status.Success
        say "Success"
    default
        say "Unknown status"
    end
end
```

---

## 11. Error Handling

TechScript uses the `try` / `catch` block mechanism for error handling. Use `throw` to raise errors.

```txs
use fs

do read_config(path)
    try
        content = fs.read_file(path)
        send content
    catch err
        say $"Could not read file: {err}"
        send null
    end
end
```

---

## 12. Memory Model

TechScript uses a tracing Garbage Collector (GC) to manage heap-allocated values (Strings, Lists, Maps, Structs, Enums). Stack values (Int, Float, Bool) are copied by value.

---

## 13. Standard Library Reference

True language built-ins (`say`, `ask`, `env`, `file`, `len`, `typeof`, `assert`, `panic`, `exit`, `sleep`, `json`, `time`) are available implicitly. Standard library modules require the `use` keyword and qualified calls.

### Core Modules:
- `math`: Mathematical operations and constants (`math.abs`, `math.sqrt`)
- `json`: JSON parsing and serialization (`json.parse`, `json.stringify`)
- `http`: HTTP client GET/POST requests and HTTP server hosting
- `crypto`: Data hashing (`crypto.sha256`) and encryption
- `testing`: Unit assertions (`testing.assert_eq`) and benchmarks

---

## 14. Package Manager CLI

Manage dependencies in your project using the compiler driver:

- **Install Dependencies**: Resolve packages in `tech.toml` and download them:
  ```bash
  tsc install
  ```
- **Uninstall a Package**:
  ```bash
  tsc uninstall log
  ```
- **Publish a Package**: Package and sign code to registry:
  ```bash
  tsc publish
  ```
- **Update Lockfile**:
  ```bash
  tsc update
  ```

---

## 15. CLI Reference

```
TechScript 2.0 CLI compiler driver (tsc)

Usage: tsc <COMMAND> [OPTIONS]

Commands:
  run <FILE>      Execute a script file directly
  build <FILE>    Compile source files to native LLVM executables
  test            Run unit and integration tests in the current project
  fmt             Format source files in-place
  lint            Run style and bug analysis rules
  repl            Start interactive command-line session
  install         Resolve and fetch packages in tech.toml
  update          Update package lockfile constraints
  publish         Publish local package to registry index
```

---

## 16. REPL Guide

Start the interactive session:
```bash
tsc repl
```
Type any valid statement to evaluate it immediately:
```
TechScript REPL v2.0
>> x = 5
>> y = 10
>> x * y
50
>> exit
```

---

## 17. VS Code Guide

The TechScript extension provides complete syntax coloring, error diagnostic highlights, automatic formatting, and completion:
1. Search for "TechScript" in VS Code Extensions.
2. Install the extension.
3. Open any `.txs` file. The extension will automatically spawn the background Language Server (`techscript-lsp`).

---

## 18. Native Compilation

TechScript compiles to native machine code via its LLVM backend:
```bash
tsc build src/main.txs --output my_app.exe
```

---

## 19. FAQ

**Q: Is TechScript statically typed?**
A: TechScript 2.0 is dynamically typed, but the optimizer resolves concrete types statically for LLVM native output where possible. Optional static type annotations are scheduled for future v2.2 releases.

**Q: How does it compare in speed to Python?**
A: Native compiled TechScript programs run up to 10-20x faster than standard Python, and virtual machine execution outperforms Python JITs for loop operations.

---

## 20. Troubleshooting

### Windows File Lock Errors (OS Error 32)
- **Problem**: Cargo or compiler commands fail with `process cannot access the file because it is being used by another process`.
- **Solution**: Lingering LSP processes or background indexers are holding locks on compilation binaries. Run:
  ```powershell
  Stop-Process -Name "cargo", "rust-analyzer" -Force -ErrorAction SilentlyContinue
  ```

### Import Failures
- **Problem**: Symbol imported cannot be found.
- **Solution**: Ensure your imported symbols are marked with the `export` keyword in the source module file.
