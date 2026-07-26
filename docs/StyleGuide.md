# TechScript 2.0 Style Guide

> The canonical formatting standard for TechScript 2.0. `tsc fmt` enforces these rules automatically.
> Status: Frozen 2.0.0 | Last Updated: 2026-07-26

---

## 1. Indentation

- 4 spaces (NEVER tabs)
- 2-space indentation is never accepted
- Nested blocks: each level adds 4 spaces

```txs
do outer()
    do_something()
    when condition
        do_inner()
    end
end
```

---

## 2. Line Length

- Maximum: 100 characters
- Prefer shorter when readable
- Break long expressions across multiple lines at operator boundaries

---

## 3. Statement Terminators

- Never use semicolons
- Each statement on its own line
- Newlines are the canonical terminator

```txs
# Correct
x = 10
y = 20

# Wrong — tsc fmt will reject this
x = 10; y = 20
```

---

## 4. Block Style

ALWAYS use indentation + `end`. Never use `{}` braces for blocks.

```txs
do greet(name)
    say "Hello " + name
end
```

> [!IMPORTANT]
> Curly braces `{}` are not valid TechScript 2.0 syntax. `tsc fmt` will error on any `{` or `}` used as block delimiters.

---

## 5. Blank Lines

- 1 blank line between top-level declarations
- 0 blank lines inside a short block (< 6 lines)
- 1 blank line before/after body of a long block (>= 6 lines)
- 2 blank lines between major sections in large files

---

## 6. Import / Use Statements

- All `use` statements at the very top of the file
- Sorted alphabetically
- One `use` per line

```txs
use http
use json
use math
```

> [!NOTE]
> `import` and `from` are deprecated. Always use `use`.

---

## 7. Naming Conventions

| Kind                          | Style                  | Example        |
|-------------------------------|------------------------|----------------|
| Variables, functions, fields  | `snake_case`           | `user_name`    |
| Classes, structs, enums       | `PascalCase`           | `UserAccount`  |
| Constants                     | `SCREAMING_SNAKE_CASE` | `MAX_RETRIES`  |
| Module names                  | `snake_case`           | `http`, `json` |

```txs
const MAX_RETRIES = 3

class UserAccount
    user_name = ""
    account_id = 0
end

do calculate_total(price, tax)
    send price + tax
end
```

---

## 8. Strings

- Always use double quotes (`"`)
- Use `$"..."` interpolation instead of `+` concatenation
- Single quotes are not valid in TechScript 2.0

```txs
# Preferred
greeting = $"Hello {name}!"

# Avoid
greeting = "Hello " + name + "!"
```

> [!WARNING]
> `f"..."` (Python-style) is deprecated. Use `$"..."` exclusively.

---

## 9. Comments

- Line comments: `#` with one space after `#`
- Section dividers: `# --` followed by section name
- No `//` or `/* */` — these are not valid TechScript 2.0 syntax

```txs
# This is a comment

# -- Section Name --

# Good: one space after hash
#Bad: no space after hash — tsc fmt will flag this
```

---

## 10. Say (print) — No Parentheses

Always use `say` without parentheses:

```txs
# Correct
say "Hello"
say result

# Wrong — tsc fmt will remove parens
say("Hello")
```

---

## 11. Ask (input) — No Parentheses

```txs
name = ask "What is your name?"
age  = ask "How old are you?"
```

---

## 12. Built-in Calls (Implicit Style)

`say`, `ask`, `env`, `file` — never use parentheses:

```txs
say "output"
name    = ask "Enter name: "
path    = env "PATH"
content = file "readme.txt"
```

> [!IMPORTANT]
> These are built-ins with implicit call style. Adding parentheses is a syntax error that `tsc fmt` will reject.

---

## 13. Stdlib Calls (Qualified, With Parens)

Standard library functions use qualified dot-notation and always require parentheses:

```txs
result   = math.abs(-42)
parsed   = json.parse(data)
response = http.get(url)
encoded  = json.stringify(obj)
```

---

## 14. DSL Blocks

Properties are aligned at the same indent level. No commas between properties:

```txs
use web

page "/"
    title "My App"
    hero
        title "Welcome"
        subtitle "Simple as English"
        button "Get Started"
    end
end

start
```

---

## 15. Trailing Whitespace

- Never allowed on any line
- `tsc fmt` strips all trailing spaces and tabs automatically

---

## 16. Final Newline

- Every file ends with exactly one newline character
- `tsc fmt` enforces this — it will add a missing newline or collapse multiple trailing newlines

---

## 17. Function Parameters

- No space before `(`
- One space after `,` in argument lists
- No space inside parentheses

```txs
do add(a, b)
    send a + b
end

result = add(3, 7)
```

```txs
# Correct
do connect(host, port, timeout)
    send http.get($"{host}:{port}")
end

# Wrong
do connect( host,port,timeout )
```

---

## 18. Operator Spacing

- One space around all binary operators (`+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `>`, etc.)
- No space between a unary operator and its operand

```txs
total    = price + tax
negative = -value
result   = a * b + c / d
flag     = not active
```

---

## 19. Max Blank Lines

- Maximum 2 consecutive blank lines anywhere in a file
- `tsc fmt` collapses 3 or more consecutive blank lines down to 2

---

## 20. Quick Reference Table

| Rule          | Correct                           | Wrong                                  |
|---------------|-----------------------------------|----------------------------------------|
| Indentation   | 4 spaces                          | tabs or 2 spaces                       |
| Blocks        | `end`                             | `{ }`                                  |
| Semicolons    | none                              | `;`                                    |
| Strings       | double quotes `"`                 | single quotes `'`                      |
| Comments      | `#`                               | `//` or `/* */`                        |
| Print         | `say x`                           | `say(x)` or `print(x)`                |
| Null          | `null`                            | `none`                                 |
| Interpolation | `$"Hello {name}"`                 | `f"Hello {name}"`                      |
| Declare       | `x = 5`                           | `make x = 5` or `let x = 5`           |
| Function      | `do fn(a, b)`                     | `function fn(a, b)` or `fun fn(a, b)` |
| Return        | `send value`                      | `return value` or `give value`         |
| Conditional   | `when` / `else when` / `else`     | `if` / `elif` / `else`                |
| While loop    | `repeat cond`                     | `while cond`                           |
| For-each      | `for x in y`                      | `each x in y`                          |
| Import        | `use mod`                         | `import mod` or `from mod import`      |
| Class         | `class Name`                      | `model Name`                           |
| Constant      | `const X = val`                   | `keep X = val`                         |
