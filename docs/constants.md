# Constants in TechScript

Constants are read-only identifiers whose values cannot change after they are defined.

---

## 🔒 Declaration
Use the `const` keyword to declare a constant. Constants must be assigned a value immediately:

```txs
const PI = 3.14159
const APP_NAME = "TechScript"
```

---

## 🚫 Reassignment Prevention
Attempting to reassign a constant or modify its reference will trigger a compilation error (`TSE0302`):

```txs
const MAX_LIMIT = 100
MAX_LIMIT = 200 # Compile Error!
```

---

## 🧬 Scope & Lifetimes
Constants exist in the scope where they are declared. It is common to declare constants at the file level (global constants) to make them available across all functions:

```txs
const DB_VERSION = 2

do check_database()
    say $"Checking version {DB_VERSION}"
end
```
