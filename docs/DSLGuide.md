# Domain-Specific Language (DSL) Guide

TechScript supports inline declarative DSL blocks to define User Interfaces, HTML structures, and mock canvas templates directly within code.

---

## 🏗️ Declarative UI Blocks
DSL blocks are parsed as declarative trees. They allow you to define UI properties without writing raw HTML:

```txs
# Example UI DSL block inside WebPage definition
use web

page = WebPage("Dashboard")
page.body([
    page.h1("Sales Overview"),
    page.input({
        "name": "search_input",
        "placeholder": "Search items...",
        "type": "text"
    })
])
```

---

## 🧬 Supported DSL Properties
Common attributes available across components:

| Property | Purpose | Example |
|:---|:---|:---|
| `label` | Text label associated with the block | `"label": "Search"` |
| `placeholder`| Fallback text shown in inputs | `"placeholder": "Enter name..."` |
| `type` | Input subtype (text, number, submit) | `"type": "number"` |
| `value` | Initial value binding | `"value": "100"` |
| `required` | Form submission requirement (bool) | `"required": true` |
| `name` | Identifier name | `"name": "email_input"` |

---

## 🎨 Rendering Pipeline
At runtime, the TechScript compiler compiles DSL trees into optimized DOM nodes or platform-specific layout structures. In web environments, the engine translates these trees into clean, standard, semantic HTML5 elements.
