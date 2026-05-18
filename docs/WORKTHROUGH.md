# TechScript v1.0.6 — Runnable Workthrough

Follow these steps in order. All commands assume you are in the **repository root** (the folder that contains `runtime/` and `examples/`).

---

## Step 0 — Build the `tech` command (one time)

**Requirements:** [Rust](https://rustup.rs/) installed.

**Easiest (Windows):**

```powershell
.\run.bat build
# or
.\scripts\setup.ps1
```

**Manual:**

```powershell
cd runtime
cargo build --release --bin tech
cd ..
```

The binary is at:

`runtime\target\x86_64-pc-windows-msvc\release\tech.exe`

**Shortcut for the rest of this guide** (PowerShell):

```powershell
$tech = ".\runtime\target\x86_64-pc-windows-msvc\release\tech.exe"
& $tech version
```

You should see: `TechScript v1.0.6`

**Alternative** (no PATH needed):

```powershell
cd runtime
cargo run --release --bin tech -- version
```

---

## Step 1 — Hello world

```powershell
& $tech run examples\hello.txs
```

**Expected output:**

```
Hello, World!
Welcome to TechScript 🚀
```

---

## Step 2 — Calculator

```powershell
& $tech run examples\calc.txs
```

Prints math results from a small `build` + `match` program.

---

## Step 3 — Classes (OOP)

```powershell
& $tech run examples\classes.txs
```

Uses `model` / inheritance (`Dog`, `Cat`). First declaration uses `make` (required in v1.0.6).

---

## Step 4 — Syntax aliases

Same program logic with `class`, `try`, `loop`, `const`, `do`:

```powershell
& $tech run examples\syntax_aliases.txs
```

---

## Step 5 — Fibonacci & list methods

```powershell
& $tech run examples\fibonacci.txs
```

Shows recursion, `.sort()`, `.map()`, `.filter()`, `.reduce()`.

---

## Step 6 — FizzBuzz

```powershell
& $tech run examples\fizzbuzz.txs
```

---

## Step 7 — Language tutorials (`runtime_examples/`)

```powershell
& $tech run runtime_examples\01_basics.txs
& $tech run runtime_examples\02_math_and_logic.txs
& $tech run runtime_examples\03_control_flow.txs
& $tech run runtime_examples\04_functions.txs
& $tech run runtime_examples\05_classes.txs
& $tech run runtime_examples\06_advanced.txs
```

Skip `07_performance_test.txs` unless you want a 1M-iteration benchmark (slow).

---

## Step 8 — Web (no browser hang in tests)

**WebPage showcase** (large demo; starts server then exits quickly in test mode):

```powershell
$env:TECHSCRIPT_WEB_TEST = "1"
& $tech run examples\web_complete.txs
```

**Component + API framework:**

```powershell
& $tech run examples\web_app.txs
```

**Real server** (opens browser, runs until you stop it — remove test env var):

```powershell
Remove-Item Env:TECHSCRIPT_WEB_TEST -ErrorAction SilentlyContinue
& $tech run examples\web_app_simple.txs
# Open http://127.0.0.1:8080
```

---

## Step 9 — GUI (desktop window)

**Test mode** (no window, instant exit):

```powershell
$env:TECHSCRIPT_GUI_TEST = "1"
& $tech run examples\gui_app.txs
```

**Real window:**

```powershell
Remove-Item Env:TECHSCRIPT_GUI_TEST -ErrorAction SilentlyContinue
& $tech run examples\gui_app.txs
```

---

## Step 10 — 3D preview

```powershell
$env:TECHSCRIPT_3D_TEST = "1"
& $tech run examples\3d_scene.txs
```

Remove `TECHSCRIPT_3D_TEST` to open the 2D preview window.

---

## Step 11 — Anime timeline

```powershell
& $tech run examples\anime_demo.txs
```

Prints timeline steps (`move`, `fade`).

---

## Step 12 — Other CLI tools

```powershell
& $tech check examples\hello.txs
& $tech repl
& $tech doctor
& $tech new myapp
& $tech test .
```

**Compile bytecode:**

```powershell
& $tech build examples\hello.txs
& $tech run examples\hello.txbc
```

**Hot reload** (edit file while this runs):

```powershell
& $tech run --watch examples\hot_reload.txs
```

**Debug** (tokens + bytecode + run):

```powershell
& $tech debug examples\hello.txs
```

---

## Step 13 — Run everything (smoke)

```powershell
$env:TECHSCRIPT_WEB_TEST = "1"
$env:TECHSCRIPT_GUI_TEST = "1"
$env:TECHSCRIPT_3D_TEST = "1"
.\scripts\smoke_all.ps1
```

---

## Troubleshooting

| Problem | Fix |
|---------|-----|
| `tech` not found | Use full path to `tech.exe` or `cargo run --release --bin tech --` from `runtime/` |
| `Undefined variable` on `x = ...` | Use `make x = ...` the first time |
| Web/GUI tests hang | Set `TECHSCRIPT_WEB_TEST=1` / `TECHSCRIPT_GUI_TEST=1` |
| `use 3d` parse error | Use exactly `use 3d` (lexer splits `3` + `d`; parser handles it) |

More detail: [PARITY.md](PARITY.md), [REFERENCE.md](REFERENCE.md).
