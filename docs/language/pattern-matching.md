# Pattern Matching in TechScript

Pattern matching provides structured multi-branch conditionals based on values or types.

---

## 🏗️ Structure
Use the `match` keyword followed by case matching clauses:

```txs
status is "success"

match status:
    case "success":
        say "Operation succeeded!"
    case "error":
        say "An error occurred!"
    case "pending":
        say "Still working..."
    default:
        say "Unknown status code"
```

---

## 🧬 Matching on Type
You can match values based on their types using `typeof(val)`:

```txs
to process_input(val):
    match typeof(val):
        case "int":
            say "Processing integer: {val}"
        case "str":
            say "Processing string: {val}"
        case "list":
            say "Processing list with size {len(val)}"
        default:
            say "Unknown data type"

process_input(42)
process_input("hello")
```
