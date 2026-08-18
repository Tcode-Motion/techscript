# Exceptions and Panics in TechScript

TechScript handles errors using two complementary mechanisms: structured exception handling (`try` / `catch`) for recoverable failures, and explicit panics (`throw`) for unrecoverable logic errors.

---

## 🏗️ Recoverable Errors: `try` / `catch`
Use a `try` block to run code that might fail. If an error is thrown, execution transitions to the `catch` block:

```txs
try
    data = fs.read("non_existent_file.json")
    say data
catch error
    say $"Failed to read file: {error}"
end
```

The caught `error` object is a dictionary or error model containing details about the failure (e.g. `message`, `code`).

---

## 🧬 Throwing Exceptions
Use the `throw` keyword to manually signal a recoverable exception:

```txs
do divide(a, b)
    when b == 0
        throw "Division by zero is not allowed"
    end
    send a / b
end

try
    result = divide(10, 0)
catch err
    say $"Caught error: {err}"
end
```

---

## 🚫 Unrecoverable Errors: Panics
When the runtime encounters a fatal state (such as array out-of-bounds, out of memory, stack overflow), it triggers a panic, which immediately terminates execution:

```txs
# Triggering an array out of bounds panic
arr = [1, 2]
say arr[99] # Panics! Thread is terminated.
```
