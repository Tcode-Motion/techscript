# Command Line Interface (CLI)

The `tech` command-line tool is your portal for executing, compiling, formatting, and analyzing TechScript code.

---

## 🛠️ Global Flags

* `-V`, `--version`: Print version information and exit.
* `-h`, `--help`: Print help details.

---

## 💻 Commands

### `run`
Executes a TechScript source file or compiled bytecode file.
```bash
tech run main.txs
tech run main.txc
```
**Flags:**
* `--debug`: Run VM with debugging telemetry enabled (shows instruction pipeline, stack pushes/pops).

### `build`
Compiles a `.txs` source file into binary bytecode `.txc`.
```bash
tech build main.txs -o main.txc
```

### `repl`
Launches the interactive shell (Read-Eval-Print Loop).
```bash
tech repl
```

### `fmt`
Auto-formats TechScript source code files following style rules.
```bash
tech fmt main.txs
tech fmt .
```

### `lint`
Analyzes code for deprecated syntax, potential performance traps, and variable errors.
```bash
tech lint main.txs
```

### `test`
Runs all unit tests in the current package or directory.
```bash
tech test
```

### `studio`
Launches TechScript Studio IDE.
```bash
tech studio
```

### `eval`
Runs an inline snippet of code.
```bash
tech eval "say 5 + 10"
```
