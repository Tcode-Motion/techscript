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
   Hello, World!
   ```

---

## 3. Language Basics

TechScript is an expressive, strongly typed language. 
- **Comments**: Single-line comments start with `//`. Multi-line comments are not supported.
- **Semicolons**: Optional but recommended to terminate statements.
- **Blocks**: Scopes are delimited by curly braces `{ ... }`.

Example:
```techscript
// This is a comment
build main() {
    make message = "Welcome to TechScript!";
    say message;
}
```

---

## 4. Variables & Constants

TechScript supports dynamic and static type checking. Variables are defined using the `make` or `let` keyword, and constants using the `const` keyword.

- `make` defines a mutable variable:
  ```techscript
  make count = 10;
  count = 20; // Ok
  ```
- `const` defines an immutable constant:
  ```techscript
  const PI = 3.14159;
  PI = 3.0; // Compile-time Error!
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

Functions are declared using the `fun` keyword (or the entrypoint `build main`).

### Declaration:
```techscript
fun add(a, b) {
    return a + b;
}
```

### Closures:
Functions are first-class citizens and can capture variables from their outer scope:
```techscript
fun make_counter() {
    make count = 0;
    return fun() {
        count = count + 1;
        return count;
    };
}
```

---

## 6. Structs

Structs group related fields together. They are declared with the `struct` keyword and instantiated using `new`.

```techscript
struct Point {
    x,
    y
}

fun main() {
    make p = new Point { x: 10, y: 20 };
    say p.x; // prints 10
}
```

---

## 7. Enums

Enums represent a type that can have one of several named variants, which can optionally hold data.

```techscript
enum Status {
    Idle,
    Loading,
    Success(message),
    Failure(code)
}
```

---

## 8. Modules & Imports

Every file in TechScript acts as a module. You can export symbols using the `export` keyword and import them using `import`.

### Module file `math_helper.txs`:
```techscript
export const factor = 2;
export fun double(x) {
    return x * factor;
}
```

### Main file `main.txs`:
```techscript
import { double, factor } from "./math_helper.txs";

build main() {
    say double(10); // prints 20
}
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

## 10. Generics

Generics parameterize structs and functions over types.

```techscript
struct Box<T> {
    value: T
}

fun get_val<T>(b: Box<T>): T {
    return b.value;
}
```

---

## 11. Pattern Matching

The `when` expression provides powerful pattern matching and structural destructuring.

```techscript
fun evaluate_status(status) {
    when (status) {
        Status::Idle => say "System is idle",
        Status::Loading => say "Loading data...",
        Status::Success(msg) => say "Success: " + msg,
        Status::Failure(code) => say "Failed with code: " + code
    }
}
```

---

## 12. Error Handling

TechScript uses the `attempt` / `catch` block mechanism for error handling.

```techscript
fun read_config(path) {
    attempt {
        make content = std.fs.read_file(path);
        return content;
    } catch (err) {
        say "Could not read file: " + err;
        return null;
    }
}
```

---

## 13. Memory Model

TechScript uses a high-performance tracing Garbage Collector (GC) to manage heap-allocated values (Strings, Lists, Maps, Structs, Enums). 
Stack values (Int, Float, Bool) are copied by value. Reference-based collections are automatically collected when they are no longer reachable from root scopes.

---

## 14. Standard Library Reference

The `std` namespace is loaded automatically in every program.

### Core Modules:
- `std.io`: Console input and output (`print`, `readline`)
- `std.fs`: Filesystem read, write, and exists checks
- `std.net`: TCP listener and socket client connect
- `std.http`: Clean HTTP client GET/POST request helpers and HTTP server listeners
- `std.json` & `std.yaml`: Structured string parsing and stringification
- `std.crypto`: Safe data encryption algorithms (AES) and hashing (MD5, SHA256)
- `std.async` & `std.future`: Task spawning and futures resolution event loops
- `std.testing`: Comprehensive assertions (`assert`, `assert_eq`, `assert_ne`) and benchmarks

---

## 15. Package Manager CLI

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

## 16. CLI Reference

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

## 17. REPL Guide

Start the interactive session:
```bash
tsc repl
```
Type any valid statement to evaluate it immediately:
```
TechScript REPL v2.0
>> make x = 5
>> make y = 10
>> x * y
50
>> exit
```

---

## 18. VS Code Guide

The TechScript extension provides complete syntax coloring, error diagnostic highlights, automatic code formatting, and completion:
1. Search for "TechScript" in VS Code Extensions.
2. Install the extension.
3. Open any `.txs` file. The extension will automatically spawn the background Language Server (`techscript-lsp`).

---

## 19. Native Compilation

TechScript compiles to native machine code via its LLVM backend:
```bash
tsc build src/main.txs --output my_app.exe
```
This performs:
- Semantic Verification
- IR Lowering
- LLVM Optimization Passes (O3)
- Machine code generation to a native binary

---

## 20. Bytecode Execution

For fast developer loops, TechScript executes bytecode inside a VM:
- Compile to portable bytecode:
  ```bash
  tsc compile src/main.txs --output main.txc
  ```
- Run the compiled bytecode:
  ```bash
  tsc run main.txc --vm
  ```

---

## 21. FAQ

**Q: Is TechScript statically typed?**
A: TechScript supports hybrid typing; variables can be dynamically typed (`make`), but the optimizer resolves concrete types statically for LLVM native output where possible.

**Q: How does it compare in speed to Python?**
A: Native compiled TechScript programs run up to 10-20x faster than standard Python, and virtual machine execution outperforms Python JITs for loop operations.

---

## 22. Troubleshooting

### Windows File Lock Errors (OS Error 32)
- **Problem**: Cargo or compiler commands fail with `process cannot access the file because it is being used by another process`.
- **Solution**: Lingering LSP processes or background indexers are holding locks on compilation binaries. Run:
  ```powershell
  Stop-Process -Name "cargo", "rust-analyzer" -Force -ErrorAction SilentlyContinue
  ```

### Import Failures
- **Problem**: Symbol imported cannot be found.
- **Solution**: Ensure your imported symbols are marked with the `export` keyword in the source module file.
