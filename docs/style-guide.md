# TechScript Style Guide

This style guide defines the formatting standards and naming conventions for TechScript code. Code formatters enforce these rules automatically.

---

## 📐 Indentation & Layout
* **Indentation**: Use 4 spaces per nesting level. Never use tabs.
* **Line Length**: Limit lines to a maximum of 100 characters.
* **Braces**: Never use `{}` curly braces for control blocks. Every block must end with the `end` keyword.
* **Semicolons**: Never use semicolons. Each statement must reside on its own line.

---

## 🏷️ Naming Conventions

Follow these casing rules:

| Element | Casing Style | Example |
|:---|:---|:---|
| Variables & Fields | `snake_case` | `user_id`, `item_count` |
| Functions & Methods | `snake_case` | `fetch_data`, `calculate_sum` |
| Classes, Structs, Enums | `PascalCase` | `DatabaseConnection`, `HttpServer` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_LIMIT`, `DEFAULT_TIMEOUT` |
| Modules & Packages | `snake_case` | `math_helpers`, `json_parser` |

---

## 🔄 Imports Organization
Place all `use` statements at the very top of the file, ordered alphabetically:

```txs
use crypto
use fs
use json
```

---

## 💬 Comments
Use `#` with a leading space for comments. Keep comments concise and informative:

```txs
# Correct comment style
x = 100 # Inline comment
```
