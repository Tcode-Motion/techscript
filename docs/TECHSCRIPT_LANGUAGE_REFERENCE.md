# TechScript 2.0 Language Reference

> **Official Authoritative Syntax & Specification**  
> Edition: TechScript 2.0 LTS | Host Target: Multi-Platform | File Extension: `.txs`

---

## 1. Introduction & Philosophy

TechScript is an expressive, memory-safe, general-purpose programming language designed for full-stack software engineering. It unifies general-purpose algorithmic programming, reactive web applications via **Webforge**, and cross-platform native mobile and desktop applications via **Appforge** under one concise, readable grammar.

---

## 2. Source Files & Lexical Grammar

### 2.1 File Extensions & Source Roles
- **`.txs`**: Canonical TechScript source code for all modules, scripts, applications, and frameworks.
- **`.wtxs` / `.stxs` / `.3txs`**: Specialized Webforge descriptors (Web, Server, and 3D scenes).
- **`.tech`**: Legacy format automatically recognized and migratable via `tsc migrate`.

### 2.2 Comments
```techscript
# Single-line comment (preferred)
// C-style single-line comment (supported)
/* Multi-line block comment */
/// Doc-comment for documentation extraction
```

### 2.3 Identifiers & Keywords
Identifiers start with an ASCII letter or underscore, followed by letters, digits, or underscores (`[a-zA-Z_][a-zA-Z0-9_]*`).

---

## 3. Variables & Declarations

TechScript supports dynamic and static type declaration styles:

```techscript
# Direct assignment
name = "TechScript"
count = 42
price = 19.99
active = true

# Explicit keywords
let user = "Alice"           # Mutable variable
const MAX_USERS = 100        # Immutable constant
keep PI = 3.14159265         # Strict compile-time constant
```

---

## 4. Types & Data Structures

### 4.1 Primitives
- **Integer**: 64-bit signed integer (`42`, `-7`, `0xFF`, `0b1010`)
- **Float**: 64-bit IEEE 754 floating point (`3.14159`, `-0.001`)
- **Boolean**: `true` or `false`
- **String**: UTF-8 encoded text (`"Hello, World!"`, `$"Interpolated: {value}"`)
- **Null**: `null` or `none`

### 4.2 Collections
```techscript
# Lists (0-indexed dynamic arrays)
numbers = [1, 2, 3, 4, 5]
say numbers[0]              # Prints 1
say len(numbers)            # Prints 5

# Dictionaries / Maps (key-value pairs)
person = {
    "name": "Bob",
    "age": 30,
    "active": true
}
say person["name"]          # Bracket access
say person.age              # Member access notation
```

---

## 5. Control Flow

### 5.1 Conditionals (`when` / `if`)
TechScript uses `when` (modern) or `if` terminated by `end`:

```techscript
score = 85

when score >= 90
    grade = "A"
else when score >= 80
    grade = "B"
else
    grade = "C"
end

say $"Grade: {grade}"
```

### 5.2 Loops (`for`, `while`, `repeat`)
```techscript
# Inclusive range loop
for i in 0..4
    say $"Index: {i}"       # Prints 0, 1, 2, 3, 4
end

# While loop
count = 3
while count > 0
    say $"Countdown: {count}"
    count = count - 1
end

# Repeat loop
repeat 3
    say "Processing..."
end
```

---

## 6. Functions & Methods

Functions are declared using `do` (preferred) or `fun`:

```techscript
do add(a, b)
    send a + b              # 'send' returns the value
end

# Calling functions
result = add(10, 25)
say $"Result: {result}"     # Prints: Result: 35
```

### 6.1 Main Program Entrypoint
If a file declares `do main()`, the TechScript VM and runtime automatically invoke it upon execution:

```techscript
do main()
    say "Application initialized successfully!"
end
```

---

## 7. Error Handling

TechScript uses `attempt` / `try` with `catch`:

```techscript
attempt
    when denominator == 0
        throw "Division by zero attempted"
    end
    result = numerator / denominator
catch error
    say $"Caught error: {error.message}"
end
```

---

## 8. Classes & Object-Oriented Programming

```techscript
class User
    do constructor(name, email)
        self.name = name
        self.email = email
    end

    do display()
        say $"User: {self.name} <{self.email}>"
    end
end

user = User("Alice", "alice@example.com")
user.display()
```

---

## 9. Modules & Standard Library

Import standard library modules using `use <module>`:

```techscript
use math
use collections
use json

val = math.sqrt(144)
say $"Square root: {val}"    # Prints 12
```

### Core Standard Library Modules
| Module | Responsibilities | Key Functions |
| :--- | :--- | :--- |
| `math` | Arithmetic, geometry, statistics | `sqrt`, `sin`, `cos`, `floor`, `ceil`, `pow`, `abs` |
| `collections` | List and map manipulation | `push`, `pop`, `contains`, `keys`, `values`, `filter`, `map` |
| `file` / `io` | File system operations | `read_text`, `write_text`, `exists`, `remove`, `read_lines` |
| `json` | Serialization & parsing | `parse`, `stringify` |
| `http` | Network client | `get`, `post`, `put`, `delete` |
| `os` / `system` | Host OS interactions | `args`, `env`, `exit`, `platform` |
| `time` / `datetime` | Timers & timestamps | `now`, `epoch_ms`, `sleep` |
| `sqlite` / `database`| SQL database access | `connect`, `execute`, `query` |

---

## 10. Webforge Declarations (`use web`)

Webforge provides reactive, declarative syntax for full-stack web applications:

```techscript
use web

website "My Company":
    theme:
        primary is "#4f46e5"
        dark_mode is true

    page "/":
        navbar:
            title is "TechScript"

        hero:
            heading "Build for the Future"
            subheading "Next-generation web development"
            button "Get Started" -> "/docs"

        card:
            heading "High Performance"
            text "Compiles to modern optimized HTML/CSS/JS."
end
```

---

## 11. Appforge Declarations (`use app`)

Appforge provides declarative UI specifications for cross-platform Android, Windows, and desktop applications:

```techscript
app "TaskManager":
    theme:
        mode is "dark"
        primary is "#4f46e5"
        background is "#0f172a"

    screen Home:
        navbar:
            title is "My Tasks"

        column:
            padding is 20
            spacing is 16

            card:
                heading "Daily Standup"
                text "Review sprint backlog at 10:00 AM."

            button "Add Task" called AddBtn:
                on_click:
                    say "Add task clicked!"
end
```

---

## 12. Foreign Language Interoperability

TechScript integrates with foreign toolchains through explicit bindings:

- **Kotlin & Java**: Integrated in Appforge via Android Gradle Plugin host scaffolding and `@JavascriptInterface` bridge.
- **Rust**: Integrated via Cargo library modules (`tsc native add rust`).
- **C & C++**: Integrated via native shared objects and Android NDK CMake bindings.
- **HTML/CSS/JavaScript**: Handled via Webforge compilation pipeline (`tsc web build`).
- **SQL**: Handled via SQLite stdlib module (`use sqlite`).

---

## 13. Compiler Support Matrix

| Language Feature | Interpreter | Bytecode VM | LLVM Backend |
| :--- | :--- | :--- | :--- |
| Variables & Primitives | Supported | Supported | Supported |
| Control Flow (`when`, `for`, `while`) | Supported | Supported | Supported |
| Functions & Recursion | Supported | Supported | Supported |
| Modules & Standard Library | Supported | Supported | Partial |
| Webforge Compilation | Supported | N/A (Source AST) | N/A |
| Appforge Android Build | Supported | Supported | Planned |
| Appforge Windows Build | Supported | Supported | Supported |
