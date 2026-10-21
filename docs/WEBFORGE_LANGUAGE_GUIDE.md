# TechScript WebForge: Language & Syntax Guide
**The Beginner-Friendly, English-Like Full-Stack Programming Language**

---

## 1. Design Philosophy

TechScript is designed to make full-stack development accessible to anyone who understands basic English, while preserving formal grammar rules that compile deterministically.

Instead of writing separate HTML, CSS, JavaScript, SQL, and backend server routing code, TechScript combines them into one unified, readable syntax.

---

## 2. Core Grammar Rules

### A. Website Declaration
Every WebForge program begins with a website declaration:

```techscript
make a website called MyApp
```

And finishes with:

```techscript
start website
```

### B. Defining Pages
Pages are structured blocks containing visual components:

```techscript
page Home:
    show heading "Welcome to My Application"
    show text "Built with TechScript WebForge"
```

You can define multiple pages within a single application:

```techscript
page About:
    show heading "About Us"
    show text "We believe programming should be as easy as reading English."
```

### C. Visual UI Elements
Within any `page` block, the `show` keyword introduces UI elements:

| Element Syntax | Generated HTML / Role | Example |
| :--- | :--- | :--- |
| `show heading "<Text>"` | `<h1>` or `<h2>` header | `show heading "Overview"` |
| `show text "<Text>"` | `<p>` paragraph | `show text "This is live text."` |
| `show button "<Label>" called <ID>` | `<button id="<ID>">` | `show button "Submit" called SubmitBtn` |
| `show input "<Placeholder>" called <ID>` | `<input placeholder="..." id="<ID>">` | `show input "Enter email" called EmailInput` |
| `show card "<Title>":` | `<div class="card">` | Encapsulates sub-elements inside a card container |

### D. Event Handling & Client Actions
Interactive actions are triggered using `when <ID> is clicked:` blocks:

```techscript
when SubmitBtn is clicked:
    show message "Form submitted successfully!"
```

To send input values to an API endpoint:

```techscript
when SaveBtn is clicked:
    send NoteInput to /api/notes
    show message "Note saved to database!"
```

### E. Declarative Database Definitions
To declare a database table with columns:

```techscript
database AppDB:
    table notes:
        title is text
        content is text
```

Supported column types include:
- `text`: Strings and textual descriptions.
- `number`: Floating-point and integer numbers.
- `boolean`: True or false flags.
- `datetime`: Timestamp records.

### F. Interactive 3D Scenes
Embed WebGL graphics using Three.js directly in TechScript:

```techscript
show 3D scene called World:
    add cube:
        color is "blue"
        rotation speed is 2
    enable mouse controls
```

Supported shapes:
- `cube`
- `sphere`
- `plane`

Properties:
- `color is "<hex or color name>"`
- `rotation speed is <number>`
- `enable mouse controls` (enables drag-to-orbit)

---

## 3. Specialized Source Extensions

While **`.txs`** is the standard, canonical extension for over 99% of TechScript source files, WebForge optionally recognizes specialized extensions for modular multi-file architectures:

```text
src/
├── main.txs        # Primary entry point (Full-stack or orchestrator)
├── pages/
│   └── docs.wtxs   # Web frontend page view (.wtxs)
├── server/
│   └── api.stxs    # Server endpoints & database workers (.stxs)
└── scenes/
    └── logo.3txs   # 3D Three.js WebGL scenes (.3txs)
```

The WebForge compiler automatically resolves and links these modular files into a single unified AST during `techscript dev` and `techscript build web`.

---

## 4. Complete Working Example

```techscript
make a website called NotesApp

database Notes:
    table notes:
        title is text
        content is text

page Home:
    show heading "TechScript Notes"
    show text "Store notes safely in SQLite."
    show input "Enter note title" called NoteTitle
    show input "Enter note content" called NoteContent
    show button "Save Note" called SaveButton

when SaveButton is clicked:
    send NoteTitle to /api/notes
    show message "Saved to SQLite!"

start website
```
