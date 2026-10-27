# Exceptions and Panics in TechScript

TechScript handles errors using two complementary mechanisms: structured exception handling (`try` / `catch`) for recoverable failures, and explicit panics (`throw`) for unrecoverable errors.

---

## 🏗️ Recoverable Errors: `try` / `catch`
Use a `try` block to run code that might fail. If an error is raised, execution transitions to the `catch` block:

```txs
try:
    data is fs.read("non_existent_file.json")
    say "{data}"
catch error:
    say "Failed to read file: {error}"
```

The caught `error` object provides structured failure details.

---

## 🧬 Throwing Exceptions
Use the `throw` keyword to manually signal a recoverable exception:

```txs
to divide(a, b):
    if b == 0:
        throw "Division by zero is not allowed"
    return a / b

try:
    result is divide(10, 0)
catch err:
    say "Caught error: {err}"
```

---

## 🚫 Unrecoverable Errors: Panics
When the runtime encounters a fatal state (such as out-of-bounds or stack overflow), it triggers a panic, which terminates execution safely.
