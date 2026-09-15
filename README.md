# TechScript

TechScript is a human-readable programming language with a Rust compiler, a bytecode virtual machine, native compilation support, and an integrated developer toolchain.

The language uses explicit English-like keywords for blocks and control flow. The toolchain includes a compiler driver, formatter, linter, language server, package manager, testing support, and standard-library modules for common application tasks.

**Current line:** TechScript 2.0.x
**License:** [Apache License 2.0](LICENSE)
**Maintainer:** [Tcode-Motion](https://github.com/Tcode-Motion)

[Releases](https://github.com/Tcode-Motion/techscript/releases) | [Documentation](docs/index.md) | [Issue tracker](https://github.com/Tcode-Motion/techscript/issues) | [Discussions](https://github.com/Tcode-Motion/techscript/discussions)

## Contents

- [Overview](#overview)
- [Language at a glance](#language-at-a-glance)
- [Why TechScript](#why-techscript)
- [Syntax comparison](#syntax-comparison)
- [How the toolchain works](#how-the-toolchain-works)
- [Installation](#installation)
- [First program](#first-program)
- [CLI](#cli)
- [Standard library](#standard-library)
- [Examples](#examples)
- [Editor support](#editor-support)
- [Documentation](#documentation)
- [Roadmap](#roadmap)
- [Repository layout](#repository-layout)
- [Development](#development)
- [Project policies](#project-policies)
- [License](#license)

## Overview

TechScript is designed for readable scripts and structured applications. Its source files use the `.txs` extension and are processed by the `tsc` command-line driver.

The 2.0 language line provides:

- English-like block keywords such as `do`, `when`, `loop`, `repeat`, `try`, `catch`, and `end`.
- Functions, classes, structs, enums, traits, interfaces, modules, pattern matching, and generics.
- Synchronous, asynchronous, and parallel execution constructs.
- A standard library for files, JSON, HTTP, databases, collections, cryptography, graphics, web workflows, and more.
- A formatter, linter, migration tool, REPL, package manager, language server, and test runner.

## Why TechScript

TechScript is intended for developers who want source code to read like a structured description of the work it performs. It replaces punctuation-heavy block syntax with explicit keywords while keeping familiar programming concepts: functions, modules, data structures, error handling, concurrency, and native execution.

| Design goal | TechScript approach |
| --- | --- |
| Readable control flow | `do` and `end` delimit blocks; `when`, `else`, `loop`, and `repeat` describe control flow directly. |
| Small toolchain surface | The `tsc` executable provides compilation, execution, formatting, linting, testing, project creation, and package operations. |
| Native performance | Rust compiler components, an optimized bytecode VM, and an LLVM backend provide multiple execution targets. |
| Practical ecosystem | The repository includes a standard library, language server, VS Code extension, examples, package tooling, and platform installers. |
| Safe evolution | Deprecated 1.x aliases remain migratable during the 2.x line, while the canonical 2.0 syntax is documented separately. |

## Syntax comparison

The following examples show the same basic constructs in TechScript, JavaScript, and Python.

| Construct | TechScript | JavaScript | Python |
| --- | --- | --- | --- |
| Variable | `count = 10` | `let count = 10;` | `count = 10` |
| Constant | `const limit = 10` | `const limit = 10;` | `LIMIT = 10` by convention |
| Function | `do greet(name)`<br>`    send "Hi " + name`<br>`end` | `function greet(name) {`<br>`  return "Hi " + name;`<br>`}` | `def greet(name):`<br>`    return "Hi " + name` |
| Condition | `when count > 5`<br>`    say "large"`<br>`else`<br>`    say "small"`<br>`end` | `if (count > 5) { ... } else { ... }` | `if count > 5:`<br>`    print("large")` |
| Collection loop | `for item in items`<br>`    say item`<br>`end` | `for (const item of items) { ... }` | `for item in items:`<br>`    print(item)` |
| Error handling | `try`<br>`    work()`<br>`catch error`<br>`    say error`<br>`end` | `try { work(); } catch (error) { ... }` | `try:`<br>`    work()`<br>`except Exception as error:` |

TechScript 2.0 also provides `class`, `struct`, `enum`, `trait`, `interface`, `match`, `async`, `await`, `parallel`, `use`, `export`, and generics. See the [language overview](docs/language/overview.md) and [syntax guide](docs/SyntaxGuide.md) for the complete grammar.

## Language at a glance

```txs
const limit = 5

do classify(value)
    when value > limit
        send "large"
    else
        send "small"
    end
end

for value in [1, 3, 8]
    say classify(value)
end
```

Canonical 2.0 syntax uses `null`, `$"..."` interpolation, `loop` for counted loops, and `repeat` for condition-controlled loops. Legacy aliases remain available during the 2.x line and can be migrated with `tsc migrate`.

## How the toolchain works

The following diagram describes the complete path from a `.txs` source file to execution or a native executable. Each stage has a defined responsibility: source text is tokenized, parsed into an AST, validated semantically, optimized, lowered to IR, and then sent to the selected backend.

```mermaid
flowchart TB
    subgraph INPUT[1. Source and command layer]
        source["TechScript source<br/>.txs files"]
        project["Project manifest<br/>package.toml"]
        cli["tsc compiler driver<br/>run, build, check, test"]
        source --> cli
        project --> cli
    end

    subgraph FRONTEND[2. Frontend]
        lexer["Lexer<br/>tokens, keywords, spans"]
        parser["Parser<br/>expressions and statements"]
        ast["Abstract syntax tree<br/>structured program model"]
        diagnostics["Diagnostics<br/>source locations and error codes"]
        lexer --> parser --> ast
        parser -. syntax errors .-> diagnostics
    end

    subgraph ANALYSIS[3. Analysis and optimization]
        semantic["Semantic analysis<br/>names, scopes, types, capabilities"]
        resolver["Module resolver<br/>imports, exports, dependencies"]
        optimizer["Optimizer<br/>constant folding and simplification"]
        semantic --> resolver --> optimizer
        semantic -. semantic errors .-> diagnostics
    end

    subgraph LOWERING[4. Lowering]
        ir["Intermediate representation<br/>control flow and instructions"]
        bytecode["Bytecode compiler<br/>portable VM instructions"]
        llvm["LLVM backend<br/>native code generation"]
        optimizer --> ir
        ir --> bytecode
        ir --> llvm
    end

    subgraph EXECUTION[5. Execution]
        artifact["Bytecode artifact<br/>.txc"]
        vm["Stack virtual machine<br/>frames, values, instructions"]
        runtime["Runtime services<br/>builtins, GC, standard library"]
        native["Native executable<br/>platform binary"]
        output["Program output<br/>files, services, or console"]
        bytecode --> artifact --> vm --> runtime --> output
        llvm --> native --> runtime
    end

    subgraph TOOLS[Developer tools]
        fmt["Formatter"]
        lint["Linter and migration"]
        lsp["Language server"]
        repl["REPL and test runner"]
        packages["Package manager"]
    end

    cli --> lexer
    ast -. diagnostics .-> diagnostics
    cli -.-> fmt
    cli -.-> lint
    cli -.-> lsp
    cli -.-> repl
    cli -.-> packages
```

The repository also contains a tree-walking interpreter for development and compatibility workflows. The VM and native backend are the primary execution targets exposed by the compiler architecture.

## Installation

### Windows

Download the latest Windows package from the [GitHub Releases](https://github.com/Tcode-Motion/techscript/releases) page. For detailed setup steps, see the [installation guide](docs/getting-started/installation.md).

The Windows installer configures the `tsc` executable, PATH integration, `.txs` file associations, and optional editor integration. Portable archives are available when an installer is not appropriate.

### Linux and macOS

Use the installer script:

```bash
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

Or build from source:

```bash
git clone https://github.com/Tcode-Motion/techscript.git
cd techscript
cargo build --release
```

### Python installer

The optional Python wrapper downloads the matching native release package:

```bash
pip install techscript-lang
techscript install
```

The packages [`techscript`](https://pypi.org/project/techscript/) and [`techscript-lang`](https://pypi.org/project/techscript-lang/) are lightweight bootstrap installers; they download the native compiler rather than embedding compiler binaries. They support Python 3.8 and newer.

### Android and Termux

```bash
pkg update
pkg install curl
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
```

The Python installer is also available in Termux when Python is installed. Package-management restrictions may require the environment-specific `--break-system-packages` option.

After installation, verify the CLI:

```bash
tsc version
```

## First program

Create `hello.txs`:

```txs
say "Hello, TechScript"
```

Run it with:

```bash
tsc run hello.txs
```

For a guided introduction, read [Getting Started](docs/getting-started/getting-started.md) and the [language overview](docs/language/overview.md).

## CLI

Run `tsc --help` for the complete command list. The main commands are:

| Command | Purpose |
| --- | --- |
| `tsc run` | Compile and execute a source file. |
| `tsc build` | Build a project or package. |
| `tsc check` | Validate source without executing it. |
| `tsc fmt` | Format TechScript source. |
| `tsc lint` | Report lint and compatibility issues. |
| `tsc lint --fix` | Apply safe automated lint fixes. |
| `tsc migrate` | Migrate legacy 1.x syntax to the 2.0 form. |
| `tsc test` | Run project tests. |
| `tsc repl` | Start the interactive REPL. |
| `tsc new` | Create a new project. |
| `tsc install` | Install a package dependency. |
| `tsc publish` | Publish a package. |
| `tsc doctor` | Inspect the local toolchain and project environment. |
| `tsc clean` | Remove compiled target caches and logs. |
| `tsc uninstall` | Remove an installed package dependency. |
| `tsc update` | Update workspace packages within their constraints. |
| `tsc dump-ast` | Inspect the parsed AST. |
| `tsc dump-ir` | Inspect lowered intermediate representation. |
| `tsc dump-bytecode` | Inspect generated VM bytecode. |
| `tsc emit-llvm` | Emit LLVM intermediate representation. |
| `tsc emit-asm` | Emit assembly output through the native backend. |
| `tsc benchmark` | Run runtime and compiler benchmarks. |

## Standard library

The standard library is implemented in the `stdlib/` workspace crate and exposed through modules including:

| Area | Modules |
| --- | --- |
| Core | `collections`, `math`, `random`, `datetime`, `path`, `env` |
| Data | `json`, `csv`, `xml`, `yaml`, `ini`, `encoding`, `compress` |
| Files and systems | `file`, `os`, `process`, `io`, `logging`, `config` |
| Network and services | `http`, `net`, `dns`, `ftp`, `oauth`, `email` |
| Databases | `sqlite`, `database`, `mysql`, `postgres`, `mongodb`, `redis` |
| Applications | `canvas`, `graphics`, `charts`, `pdf`, `excel`, `word`, `web` |
| Development | `testing`, `debug`, `benchmark`, `profiler`, `mock` |

See the [standard library reference](docs/reference/StdlibReference.md), [web guide](docs/WebGuide.md), and [canvas guide](docs/CanvasGuide.md) for module details.

## Examples

Runnable examples are organized under [`examples/`](examples). Useful starting points include:

- [Hello World](examples/hello_world)
- [Calculator](examples/calculator)
- [Collections](examples/collections)
- [Database](examples/database)
- [Error handling](examples/error_handling)
- [JSON parser](examples/json_parser)
- [Web API](examples/web_api)
- [Todo CLI](examples/todo_cli)
- [Testing](examples/testing)

| Example | Purpose |
| --- | --- |
| [AI](examples/ai) | Use the AI standard-library integration. |
| [Async](examples/async) | Work with asynchronous functions and `await`. |
| [Canvas](examples/canvas) | Draw shapes and text in a canvas. |
| [Enums](examples/enums) | Declare and match enum values. |
| [File reader](examples/file_reader) | Read, write, and remove files. |
| [Generics](examples/generics) | Use generic and structured data examples. |
| [Guess number](examples/guess_number) | Follow a complete interactive loop. |
| [HTTP server](examples/http_server) | Explore web service structure. |
| [Modules](examples/modules) | Import standard and project modules. |
| [Object-oriented examples](examples/oop) | Explore classes and object structure. |
| [Threads](examples/threads) | Spawn and join operating-system threads. |
| [Todo CLI](examples/todo_cli) | Build a multi-command collection workflow. |
| [Web API](examples/web_api) | Make an HTTP request from TechScript. |

The repository includes additional examples for AI integrations, asynchronous code, canvas drawing, enums, file access, generics, HTTP services, modules, object-oriented patterns, threads, and package management. Browse the complete [examples directory](examples) or read the [examples guide](docs/guides/ExamplesGuide.md).

Run an example from the repository root:

```bash
tsc run examples/hello_world/hello.txs
```

The [examples guide](docs/guides/ExamplesGuide.md) explains how examples are structured and verified.

## Editor support

The official [TechScript Virtian VS Code extension](https://marketplace.visualstudio.com/items?itemName=tanmoy.techscript) provides canonical 2.0 syntax highlighting, language-server integration, debugger support, configurable save-time formatting and linting, and editor commands. The extension is also available through [Open VSX](https://open-vsx.org/extension/Tcode-Motion/techscript).

The language server is implemented in [`tools/lsp/`](tools/lsp). Formatter and linter implementation is in [`tools/formatter/`](tools/formatter) and [`tools/linter/`](tools/linter).

## Documentation

- [Documentation portal](docs/index.md)
- [Installation guide](docs/getting-started/installation.md)
- [Getting Started guide](docs/getting-started/getting-started.md)
- [Language overview](docs/language/overview.md)
- [Syntax guide](docs/SyntaxGuide.md)
- [Compiler architecture](docs/compiler/architecture.md)
- [Virtual machine specification](docs/compiler/vm.md)
- [CLI reference](docs/tooling/cli.md)
- [Formatter reference](docs/tooling/formatter.md)
- [Package manager reference](docs/tooling/package-manager.md)
- [Standard library reference](docs/reference/StdlibReference.md)
- [Migration guide](docs/MigrationGuide.md)
- [Release notes](docs/ReleaseNotes.md)
- [Roadmap](docs/Roadmap.md)
- [Supported versions](docs/SUPPORTED_VERSIONS.md)

Additional references include [FAQ](docs/faq.md), [best practices](docs/guides/BestPractices.md), [DSL guide](docs/DSLGuide.md), [performance reference](docs/Performance.md), [Web guide](docs/WebGuide.md), [API reference](docs/reference/APIReference.md), [memory model](docs/specification/memory-model.md), and [FFI specification](docs/specification/ffi.md).

The engineering specifications are also available in [`docs/engineering/`](docs/engineering), including the language freeze, grammar, AST, semantic analysis, runtime, CLI, testing, and coding standards documents.

## Roadmap

Completed foundations include the Pratt parser, canonical 2.0 syntax, the bytecode pipeline, formatter, linter, test runner, asynchronous constructs, and standard-library modules. Planned work is tracked in the [roadmap](docs/Roadmap.md) and includes:

- Completing and expanding LLVM native code generation.
- Adding debugger and memory-tracing workflows to the standard tools.
- Expanding package registry and cross-platform distribution support.
- Strengthening standard-library verification and documentation.

## Repository layout

```text
compiler/       Lexer, parser, AST, semantic analysis, IR, optimizer, and backends
runtime/        Interpreter, VM, runtime services, garbage collection, and builtins
stdlib/         Standard-library modules
cli/            The tsc compiler driver
tools/          Formatter, linter, LSP, package manager, and packager
docs/           User guides, specifications, and project documentation
examples/       Runnable TechScript programs
editors/        Editor integrations
installer/      Installer configuration
scripts/        Installation and maintenance scripts
benchmarks/     Performance benchmarks and benchmark reports
assets/         Branding and editor assets
templates/      New project templates
third_party/    Vendored or extracted third-party source material
```

## Development

Prerequisites are Rust stable, Git, and the platform dependencies required by optional LLVM features. Build and test the workspace with:

```bash
cargo fmt --all -- --check
cargo build --workspace
cargo test --workspace
```

Before opening a pull request, read the [contribution guide](CONTRIBUTING.md), follow the [code of conduct](CODE_OF_CONDUCT.md), and review the [security policy](SECURITY.md).

## Project policies

- [Contributing](CONTRIBUTING.md): development setup, branch rules, testing, commits, and pull requests.
- [Code of Conduct](CODE_OF_CONDUCT.md): community standards and reporting process.
- [Security Policy](SECURITY.md): supported versions and private vulnerability reporting.
- [License](LICENSE): Apache License 2.0 terms.
- [Issue tracker](https://github.com/Tcode-Motion/techscript/issues): bug reports and actionable tasks.
- [Feature requests](https://github.com/Tcode-Motion/techscript/issues/new?template=feature_request.md): proposals for new functionality.
- [Bug reports](https://github.com/Tcode-Motion/techscript/issues/new?template=bug_report.md): reproducible defects.
- [Questions](https://github.com/Tcode-Motion/techscript/issues/new?template=question.md): usage and documentation questions.
- [Discussions](https://github.com/Tcode-Motion/techscript/discussions): design and community conversations.

## License

TechScript is distributed under the [Apache License 2.0](LICENSE). Third-party components remain under their own licenses and are listed in [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).
