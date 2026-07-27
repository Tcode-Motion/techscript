# TechScript 2.0 Migration Guide

This guide describes how to update legacy TechScript 1.0.8 projects to use the clean canonical syntax of TechScript 2.0.

---

## 📐 Overview of Key Syntax Changes

TechScript 2.0 simplifies syntax by eliminating legacy variable keywords (`make`, `keep`) and aligns control loop commands with standard conventions:

| TechScript 1.0.8 Legacy | TechScript 2.0 Canonical | Description |
|:---|:---|:---|
| `make name = "Alice"` | `name = "Alice"` | Variable keyword removed. |
| `make name be "Alice"`| `name = "Alice"` | `be` keyword deprecated. |
| `keep PI = 3.14` | `const PI = 3.14` | Use `const` for immutables. |
| `model Dog ... end` | `class Dog ... end` | Use `class` instead of `model`. |
| `attempt ... end` | `try ... end` | Use standard `try`/`catch`. |
| `stop` | `break` | Terminate loop execution. |
| `skip` | `continue` | Skip to next loop iteration. |
| `each item in list` | `for item in list` | Enforce `for` loops. |

---

## 🧬 Code Migration Examples

### Variables & Constants
```txs
# ❌ Legacy 1.0.8 Style (generates TSW1001 warnings)
make username = "Bob"
keep MAX_ITEMS = 50

# ✅ Modern 2.0.0 Style
username = "Bob"
const MAX_ITEMS = 50
```

### Conditional Blocks
```txs
# ❌ Legacy 1.0.8 Style
when x equals y then
    say "Equal"
end

# ✅ Modern 2.0.0 Style
when x == y
    say "Equal"
end
```

---

## 🛠️ Automated Migration Command
You can auto-migrate old scripts using the built-in linter and formatter:
```bash
tech migrate .
```
This automatically updates legacy keywords to their modern counterparts.
