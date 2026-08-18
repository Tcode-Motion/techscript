# Pattern Matching in TechScript

Pattern matching provides structured multi-branch conditionals based on values or types.

---

## 🏗️ Structure
Use the `match` keyword followed by case matching clauses:

```txs
status = "success"

match status
case "success"
    say "Operation succeeded!"
case "error"
    say "An error occurred!"
case "pending"
    say "Still working..."
default
    say "Unknown status code"
end
```

---

## 🧬 Matching on Type
You can match values based on their types using the `typeof` evaluation:

```txs
do process_input(val)
    match typeof(val)
    case "int"
        say $"Processing integer: {val}"
    case "str"
        say $"Processing string: {val}"
    case "list"
        say $"Processing list with size {len(val)}"
    default
        say "Unknown data type"
    end
end

process_input(42)
process_input("hello")
```
