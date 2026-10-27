---
name: techscript
description: >-
  Authoritative developer skill for the TechScript programming language (v2.0.2).
  Use this skill EVERY time the user wants to write, compile, debug, format, lint,
  run, test, or analyze TechScript (.txs) source code, build WebForge reactive web apps,
  develop AppForge cross-platform mobile/desktop apps, work with the TechScript CLI (tsc),
  or troubleshoot compiler and runtime errors. Always read this skill before writing any TechScript code.
---

# TechScript Developer Skill & Operational Runbook

TechScript is a high-performance, human-readable programming language featuring a native Rust compiler pipeline, an optimized register-based Virtual Machine, an LLVM native code generator, the WebForge web framework, and the AppForge cross-platform mobile engine.

Untrained AI models habitually generate Python, JavaScript, Rust, or Go syntax when attempting to write TechScript. **This skill exists to completely eliminate syntax hallucination and enforce 100% canonical, compiler-verified TechScript.**

---

## 🚨 Non-Negotiable Rules (Apply to EVERY TechScript File)

1. **NEVER use foreign assignment keywords.**
   - Initial binding: `is` (e.g., `count is 0`, `name is "TechScript"`).
   - Reassignment / Mutation: `becomes` (e.g., `count becomes count + 1`).
   - Constants: `constant` + `is` (e.g., `constant max_connections is 100`).
   - Forbidden: `let`, `var`, `val`, `def`, `:=`, `=`, `const`, `final`.
2. **NEVER use foreign function keywords.**
   - Function definition: `to function_name(param1, param2):`
   - Return statement: `return value`
   - Forbidden: `fn`, `def`, `func`, `function`, `sub`, `void`, `send`, `give`, `->`.
3. **NEVER use curly braces `{}` or `begin/end` for code blocks.**
   - All blocks (functions, conditionals, loops, classes) MUST use a colon `:` followed by exactly 4 spaces of indentation.
   - Forbidden: `{ ... }`, `end`, `done`, `fi`, `esac`.
4. **NEVER use foreign print statements.**
   - Standard output: `say "text"`
   - Forbidden: `print()`, `println!()`, `console.log()`, `fmt.Println()`, `echo`.
5. **Double-quoted string interpolation is automatic with `{expression}`.**
   - Canonical: `"Hello, {user_name}! You have {unread_count} new messages."`
   - NEVER prefix strings with `f`, `$`, or `@`.
   - NEVER use string concatenation (`"Hello " + user_name`) when interpolation expresses the intent cleanly.
6. **Conditionals strictly use `if`, `else if`, and `else` with colons.**
   - Canonical:
     ```txs
     if score >= 90:
         grade is "A"
     else if score >= 80:
         grade is "B"
     else:
         grade is "F"
     ```
   - Forbidden: `elif`, `elsif`, `when`, parenthesis around conditions `if (score >= 90)`.
7. **Repetition strictly uses canonical loop constructs.**
   - Counted iterations: `repeat N times:`
   - Sequence traversal: `for item in collection:`
   - Conditional iteration: `while condition:`
   - Forbidden: C-style loops `for (int i=0; i<n; i++)`, `loop`, `repeat ... until`, `do ... while`.
8. **Object-Oriented constructs use `class` and `to init(...):`.**
   - Canonical:
     ```txs
     class Vector2D:
         to init(x, y):
             this.x is x
             this.y is y

         to magnitude():
             return (this.x * this.x + this.y * this.y) ** 0.5
     ```
9. **Never document an unsupported or fictitious feature as implemented.**
   - The authoritative reference for standard library modules is [references/stdlib_api.md](./references/stdlib_api.md).
   - Only import supported standard modules: `text`, `math`, `collections`, `json`, `files`, `time`, `http`, `system`, `random`, `log`, `test`.
10. **Verify before delivering.**
    - Every generated `.txs` file must be verified using `tsc check <file>` or `tsc run <file>` whenever CLI tools are accessible.

---

## 🛠️ Step-by-Step Developer Workflow

When given any TechScript task, execute these steps systematically:

### Step 1 — Scope & Target Selection
Determine the target execution layer:
- **Core CLI / Script**: Standalone logic, algorithms, file processing, or automation.
- **WebForge Web Application**: Reactive web client rendering via WebAssembly / HTML canvas (`web:` components).
- **AppForge Mobile Application**: Declarative cross-platform mobile UI targeting Android APK or desktop (`app:` components).

### Step 2 — Architecture & Canonical Authoring
Draft the code adhering strictly to the 10 Non-Negotiable Rules.
- Organize code into clean functions (`to ...:`).
- Keep state mutations explicit with `becomes`.
- Use standard modules via `use <module>`:
  ```txs
  use math
  use json
  use files
  ```

### Step 3 — Semantic Validation & Check
Validate the AST, symbol resolution, and type rules without executing side effects:
```bash
tsc check src/main.txs
# Or via cargo in repository workspace:
cargo run -p techscript_cli -- check src/main.txs
```

### Step 4 — Execution & Verification
Run the program through the TechScript Virtual Machine:
```bash
tsc run src/main.txs
# Or via cargo:
cargo run -p techscript_cli -- run src/main.txs
```

### Step 5 — Code Formatting & Linting
Format according to standard style guidelines (4-space indentation, normalized spacing):
```bash
tsc fmt src/main.txs
tsc lint --fix src/main.txs
```

### Step 6 — Self-Testing Toolchain
Run the automated test runner to ensure the local environment is fully operational:
- **Windows PowerShell**:
  ```powershell
  powershell -File skills/techscript/scripts/test_techscript.ps1
  ```
- **Linux / macOS / Termux**:
  ```bash
  bash skills/techscript/scripts/test_techscript.sh
  ```

---

## 📐 Production Blueprints & Code Recipes

### Blueprint 1: Core CLI & Data Processing Application

```txs
# ==============================================================================
# TechScript v2.0.2 - Production Core Data Processor
# ==============================================================================

constant app_version is "2.0.2"
constant max_retries is 3

class Transaction:
    to init(id, amount, category):
        this.id is id
        this.amount is amount
        this.category is category
        this.processed is false

    to apply_discount(percent):
        discount is this.amount * (percent / 100.0)
        this.amount becomes this.amount - discount
        return this.amount

to analyze_transactions(records):
    total_volume is 0.0
    processed_count is 0

    for item in records:
        if item.amount > 0:
            total_volume becomes total_volume + item.amount
            item.processed becomes true
            processed_count becomes processed_count + 1
        else:
            say "Skipping invalid transaction: {item.id}"

    return {
        "count": processed_count,
        "volume": total_volume
    }

# Main entry point
say "Initializing Transaction Engine v{app_version}..."

tx1 is Transaction(101, 450.0, "Infrastructure")
tx2 is Transaction(102, 120.5, "Software")
tx3 is Transaction(103, 89.0, "Services")

tx1.apply_discount(10.0)

batch is [tx1, tx2, tx3]
summary is analyze_transactions(batch)

say "Processed {summary['count']} transactions."
say "Total settlement volume: ${summary['volume']}"

attempt is 1
while attempt <= max_retries:
    say "Health check ping #{attempt}: OK"
    attempt becomes attempt + 1

repeat 2 times:
    say "Audit record finalized."
```

---

### Blueprint 2: Standard Library Power User

```txs
# Utilizing canonical standard library modules
use math
use text
use json
use files
use time

to process_configuration(config_path):
    # Read configuration file
    raw_content is files.read_text(config_path)
    config is json.parse(raw_content)

    host is config["host"]
    port is config["port"]
    say "Connecting to {host}:{port}..."

    # Timestamp and math operations
    started_at is time.now_ms()
    radius is 14.5
    circle_area is math.pi * math.pow(radius, 2)

    say "Computed operational zone: {circle_area} sq units."
    elapsed is time.now_ms() - started_at
    say "Completed processing in {elapsed}ms."

    # Write status receipt
    receipt is json.stringify({
        "status": "success",
        "processed_at": time.iso8601()
    })
    files.write_text("status.json", receipt)
    return true
```

---

### Blueprint 3: WebForge Reactive Web Component

```txs
# ==============================================================================
# WebForge Reactive Component
# ==============================================================================

component CounterWidget:
    state count is 0

    to increment():
        this.count becomes this.count + 1

    to decrement():
        if this.count > 0:
            this.count becomes this.count - 1

    render:
        div(class: "counter-card"):
            h1("Current Count: {this.count}")
            button(onclick: this.decrement, class: "btn-secondary"): "Decrease"
            button(onclick: this.increment, class: "btn-primary"): "Increase"
```

---

### Blueprint 4: AppForge Cross-Platform Mobile Screen

```txs
# ==============================================================================
# AppForge Mobile View (Android & Desktop)
# ==============================================================================

screen ProfileScreen:
    state user_name is "Alex Rivera"
    state is_notifications_enabled is true

    to toggle_notifications():
        this.is_notifications_enabled becomes not this.is_notifications_enabled

    render:
        Column(padding: 24, spacing: 16):
            Image(source: "assets/avatar.png", width: 96, height: 96, shape: "circle")
            Text("User Profile", style: "headlineMedium", weight: "bold")
            Text("Logged in as {this.user_name}", style: "bodyMedium")
            Row(alignment: "spaceBetween"):
                Text("Enable Push Notifications")
                Switch(checked: this.is_notifications_enabled, on_change: this.toggle_notifications)
            Button(label: "Save Changes", style: "primary", on_click: to():
                say "Saved preferences for {this.user_name}"
            )
```

---

## 🔍 Compiler & Runtime Error Diagnostics

| Diagnostic Code | Typical Cause | Authoritative Fix |
| :--- | :--- | :--- |
| **`TSE0201: Invalid Token`** | Using Python/JS keywords like `let`, `var`, `def`, `fn`, `:=`, or `=`. | Replace with `is` for binding or `becomes` for mutation. |
| **`TSE0202: Unexpected Delimiter`** | Using curly braces `{}` or `end` to close a block. | Remove braces; terminate the header line with `:` and indent body by 4 spaces. |
| **`TSE0301: Immutable Reassignment`** | Attempting to mutate an existing variable using `is`. | Use `becomes`: `x becomes x + 1`. |
| **`TSE0302: Undefined Identifier`** | Typo in variable name or using variable before declaration. | Declare variable using `name is value` before accessing. |
| **`TSE0304: Constant Mutation`** | Attempting to reassign a `constant` with `becomes`. | Constants are immutable. Use standard `is` variable if mutation is required. |
| **`TSE0401: String Prefix Error`** | Prepended `f"..."` or `$"..."` before string literal. | Remove prefix. TechScript natively interpolates all double-quoted strings. |
| **`TSE0404: Indentation Error`** | Inconsistent tabs and spaces, or tab width != 4 spaces. | Convert all indents to 4 spaces using `tsc fmt <file>`. |

---

## ⚡ Agent One-Line Skill Installation Matrix

This skill can be installed in seconds into any AI coding environment:

### Linux / macOS / Claude Code / Cursor / OpenClaw / Codex:
```bash
curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/skills/techscript/install.sh -o install.sh
bash install.sh
```

### Windows PowerShell / Antigravity IDE / Gemini CLI / VS Code:
```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/Tcode-Motion/techscript/main/skills/techscript/install.ps1 -OutFile install.ps1
.\install.ps1
```

---

## 📚 Technical Reference Guides

For comprehensive technical documentation, refer to the bundled specifications in this skill:
- **[Syntax Specification](./references/syntax_specification.md)** — Complete EBNF grammar, token lexer rules, control structures, and memory models.
- **[Standard Library API](./references/stdlib_api.md)** — Complete function signatures and documentation for all standard modules.
- **[CLI Toolchain Reference](./references/cli_toolchain.md)** — Full command-line options for `tsc` (`run`, `build`, `check`, `fmt`, `lint`, `test`, `lsp`).
