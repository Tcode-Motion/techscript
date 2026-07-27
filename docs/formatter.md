# Code Formatter

The TechScript Formatter (`tools/formatter`) enforces a uniform style layout across all codebases.

---

## 🏗️ Usage
Format a single file:
```bash
tech fmt main.txs
```

Format all files recursively in the current workspace:
```bash
tech fmt .
```

Verify formatting on CI without editing files:
```bash
tech fmt --check .
```

---

## 🧬 Formatter Styles & Rules
The formatter applies the following styling rules automatically:
* **Indentation**: Inserts 4 spaces per indentation level. Tabs are converted to spaces.
* **Block Terminators**: Ensures matching `end` markers are aligned with their initiating keyword.
* **Whitespace**:
  * Inserts spaces around binary operators: `x = a + b` instead of `x=a+b`.
  * Removes trailing whitespaces from lines.
  * Adds a single space after commas: `do add(a, b)` instead of `do add(a,b)`.
* **String Prefixes**: Enforces the canonical `$` prefix for interpolated strings (e.g. converting deprecated `f"..."` to `$"..."`).
