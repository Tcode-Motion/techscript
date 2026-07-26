# TechScript 2.0 Best Practices Guide

> **Status**: Frozen Specification — 2.0.0 Stable
> **Last Updated**: 2026-07-26

Guidelines for writing safe, performant, and readable TechScript 2.0 code.

---

## 1. Variable and Constant Declarations

- **Use plain assignments for variables**: Do not write `make`, `let`, or `var`. First assignment declares a variable in the local scope.
- **Use `const` for read-only values**: Declare configuration constants, mathematical bounds, and static structures with `const` to enforce immutability at compile time.
- **Avoid shadowing where possible**: Shadowing is allowed, but it emits `TSW2002` style warnings. Choose unique names for variables in nested scopes to improve code clarity.

```txs
# Good
const PI = 3.14159
radius = 5
area = PI * radius * radius

# Avoid (triggers shadowing warning TSW2002)
x = 10
do outer()
    x = 20    # TSW2002
end
```

---

## 2. String Management

- **Prefer `$"..."` string interpolation**: Use dollar-prefix string interpolation instead of `+` concatenation. It is cleaner and performs fewer allocation operations.
- **Do not use `f"..."`**: F-strings are deprecated and emit `TSW1012`.

```txs
# Good
say $"Welcome, {username}!"

# Bad (unnecessary allocations, triggers hint TSI3001)
say "Welcome, " + username + "!"

# Deprecated (triggers TSW1012)
say f"Welcome, {username}!"
```

---

## 3. Control Flow

- **Always choose the correct loop primitive**:
  - Use `loop N` when you want to repeat execution a fixed number of times. It is optimized to avoid induction variable checks.
  - Use `repeat cond` for while-loops.
  - Use `for item in list` for list, map, and range iterations.
- **Prefer `match` over nested `when` blocks**: For checking a single variable against multiple potential constants, `match`/`case` blocks generate faster jump tables than multiple `when`/`else when` branches.

```txs
# Good
loop 5
    say "Processing item..."
end

# Good
match status
case "success"
    say "Done"
case "error"
    say "Failed"
default
    say "Pending"
end
```

---

## 4. Standard Library Calls

- **Only use implicit style for language built-ins**: The built-ins `say`, `ask`, `env`, and `file` are part of the core language syntax. Never call them with parentheses or module prefixes.
- **Import and qualify all other standard library modules**: Keep standard calls qualified to prevent name collisions in larger codebases.

```txs
# Good
use math
result = math.abs(-10)
say result

# Bad
result = abs(-10)    # Error: Undefined variable 'abs'
```

---

## 5. Async Event Loops

- **Cooperative async execution**: Long-running synchronous blocks inside `async do` functions block the entire runtime event thread. Ensure tasks yield execution regularly.
- **Use `parallel` blocks for independent requests**: When fetching independent resources, wrap them in a `parallel` block to execute them concurrently instead of executing them sequentially with sequential `await` calls.

```txs
# Good
parallel
    users = fetch_users()
    orders = fetch_orders()
end

# Avoid (sequential blocking)
users = await fetch_users()
orders = await fetch_orders()
```
