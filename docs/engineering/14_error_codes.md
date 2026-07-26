# 14 — TechScript 2.0 Error Code Specification

> **Status:** Authoritative Specification
> **Version:** 2.0.0
> **Last Updated:** 2026-07-26

---

## Table of Contents

1. [Error Code Namespaces](#error-code-namespaces)
2. [Lexer Errors — TSE0001–TSE0099](#lexer-errors-tse0001--tse0099)
3. [Parser Errors — TSE0100–TSE0299](#parser-errors-tse0100--tse0299)
4. [Semantic Errors — TSE0300–TSE0499](#semantic-errors-tse0300--tse0499)
5. [DSL Validation Errors — TSE0400–TSE0499](#dsl-validation-errors-tse0400--tse0499)
6. [Runtime Errors — TSE1000–TSE1999](#runtime-errors-tse1000--tse1999)
7. [Deprecation Warnings — Parser Phase — TSW1001–TSW1099](#deprecation-warnings--parser-phase-tsw1001--tsw1099)
8. [Style / Lint Warnings — Semantic Phase — TSW2001–TSW2099](#style--lint-warnings--semantic-phase-tsw2001--tsw2099)
9. [Informational Hints — TSI3001–TSI3099](#informational-hints-tsi3001--tsi3099)
10. [Migration](#migration)

---

## Error Code Namespaces

All diagnostic codes follow the pattern `<prefix><4-digit-code>`. The prefix encodes both the severity and the compiler phase that emits the diagnostic.

| Prefix    | Range       | Severity    | Phase             | Description                       |
| --------- | ----------- | ----------- | ----------------- | --------------------------------- |
| `TSE0xxx` | 0001 – 0999 | Error       | Compile-time      | Hard errors that halt compilation |
| `TSE1xxx` | 1000 – 1999 | Error       | Runtime           | Errors raised during execution    |
| `TSW1xxx` | 1001 – 1099 | Warning     | Parser            | Deprecated syntax warnings        |
| `TSW2xxx` | 2001 – 2099 | Warning     | Semantic / Lint   | Style and semantic lint warnings  |
| `TSI3xxx` | 3001 – 3099 | Hint / Info | Semantic          | Non-blocking improvement hints    |

> **Note:** Errors (`TSE`) always halt the current compilation unit. Warnings (`TSW`) and hints (`TSI`) are emitted as diagnostics but do not prevent output generation unless `--strict` mode is enabled.

---

## Lexer Errors — TSE0001–TSE0099

Lexer errors occur during tokenisation, before any parsing takes place.

| Code    | Message                            | Cause                                                                          | Example       |
| ------- | ---------------------------------- | ------------------------------------------------------------------------------ | ------------- |
| `E0001` | Unexpected character               | A character that is not part of the TechScript 2.0 grammar was found           | `x = @5`      |
| `E0010` | Trailing underscore in number      | A numeric literal ends with an underscore separator                            | `1_000_`      |
| `E0011` | Empty numeric prefix               | A base prefix (`0x`, `0b`, `0o`) is not followed by any digit                 | `0x`          |
| `E0012` | Invalid base digit                 | A digit does not belong to the declared numeric base                           | `0b102`       |
| `E0021` | Unterminated string                | A string literal is opened but never closed before end-of-line or end-of-file | `x = "hello`  |

### Diagnostic Format

```
[TSE0001] file.ts:3:8 — Unexpected character '@'
         x = @5
             ^
```

---

## Parser Errors — TSE0100–TSE0299

Parser errors occur when the token stream does not conform to the TechScript 2.0 grammar.

| Code    | Message                                         | Cause                                                                                                           |
| ------- | ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `E0100` | Expected expression                             | A position that requires an expression received a non-expression token                                          |
| `E0101` | Expected identifier                             | A position that requires a name (variable, function, class) received another token                              |
| `E0104` | Expected `end` to close block                   | A block was opened (by `do`, `when`, `repeat`, `for`, `class`, etc.) but `end` was not found before the next top-level token |
| `E0105` | Expected block body                             | A block-introducing keyword was not followed by an indented body                                                |
| `E0107` | Expected statement terminator (missing newline) | Two statements appear on the same line without a valid separator                                                |
| `E0113` | Invalid assignment target                       | The left-hand side of `=` is not a valid l-value (e.g., a literal or expression)                               |

### E0104 Detail

TechScript 2.0 uses indentation and the `end` keyword to delimit blocks. Curly-brace (`{` / `}`) delimiters are **not** part of the language. This error fires when the parser expects `end` but encounters something else.

**Incorrect (deprecated brace style):**

```
do greet(name)
{
    say $"Hello {name}"
}
```

**Correct (canonical 2.0):**

```
do greet(name)
    say $"Hello {name}"
end
```

---

## Semantic Errors — TSE0300–TSE0499

Semantic errors are detected after parsing, during name resolution and type analysis.

### Name & Scope Errors

| Code    | Message                                 | Cause                                                                   |
| ------- | --------------------------------------- | ----------------------------------------------------------------------- |
| `E0300` | Undefined variable `<name>`             | A name is referenced that has not been declared in any accessible scope |
| `E0301` | Duplicate declaration `<name>` in scope | A name is declared more than once in the same scope level               |
| `E0302` | Cannot reassign `const` `<name>`        | An assignment targets a name declared with `const`                      |
| `E0303` | Variable `<name>` used before assignment | A variable is read before it has been given a value                    |

### Call & Signature Errors

| Code    | Message                                     | Cause                                                               |
| ------- | ------------------------------------------- | ------------------------------------------------------------------- |
| `E0310` | Wrong argument count: too few for `<name>`  | A function call provides fewer arguments than the function requires |
| `E0311` | Wrong argument count: too many for `<name>` | A function call provides more arguments than the function accepts   |

### Control-Flow Errors

| Code    | Message                                                | Cause                                                                                           |
| ------- | ------------------------------------------------------ | ----------------------------------------------------------------------------------------------- |
| `E0312` | `send` outside function body                           | A `send` statement appears at module/top level, outside any `do` block                         |
| `E0313` | Mixed top-level statements with explicit main          | Top-level executable statements exist alongside an explicitly declared `main` function          |
| `E0320` | `self` used outside method                             | The implicit `self` reference appears in a function that is not a class method                 |

### Module Errors

| Code    | Message                                       | Cause                                                                                                      |
| ------- | --------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `E0340` | Module not found: `<path>`                    | A `use` statement names a module that cannot be resolved                                                   |
| `E0350` | Cannot export non-exportable declaration      | An `export` modifier is applied to a declaration that is not allowed to be exported (e.g., a local variable) |

---

## DSL Validation Errors — TSE0400–TSE0499

DSL validation errors are raised when embedded domain-specific language blocks fail structural validation.

| Code    | Message                                    | Cause                                                             |
| ------- | ------------------------------------------ | ----------------------------------------------------------------- |
| `E0400` | DSL block missing required field `<field>` | A mandatory field is absent from a DSL block definition           |
| `E0401` | DSL field `<field>` has wrong type         | A DSL field value does not match the expected type for that field |
| `E0402` | Unknown DSL directive `<directive>`        | An unrecognised directive keyword appears inside a DSL block      |
| `E0403` | DSL block nesting depth exceeded           | A DSL block is nested beyond the maximum allowed depth            |

---

## Runtime Errors — TSE1000–TSE1999

Runtime errors are raised by the TechScript 2.0 virtual machine during program execution. They may be caught with `try` / `catch` unless marked **uncatchable**.

| Code    | Message                               | Catchable | Cause                                                                                   |
| ------- | ------------------------------------- | :-------: | --------------------------------------------------------------------------------------- |
| `E1010` | Division by zero                      | ✅        | An integer or float division or modulo operation with a zero divisor                    |
| `E1011` | Type mismatch                         | ✅        | An operation received a value of an incompatible type                                   |
| `E1020` | Stack overflow                        | ❌        | Unbounded or excessively deep recursion exhausted the call stack                        |
| `E1030` | Value not iterable in `for` loop      | ✅        | The target of `for x in y` is not a sequence, range, or iterator                       |
| `E1041` | Field or method `<name>` not found    | ✅        | A field access or method call targets a name that does not exist on the object          |
| `E1050` | Index out of bounds                   | ✅        | A list or string subscript is outside the valid index range                             |

### Catching Runtime Errors

```
try
    result = items[idx]
catch e
    say $"Caught: {e.code} — {e.message}"
end
```

---

## Deprecation Warnings — Parser Phase — TSW1001–TSW1099

These warnings are emitted when the parser encounters syntax from TechScript 1.x that has been superseded in 2.0. The code will still compile, but the deprecated form should be migrated to its canonical replacement.

| Code      | Deprecated Syntax                                   | Canonical Replacement (2.0)               |
| --------- | --------------------------------------------------- | ----------------------------------------- |
| `TSW1001` | `make x = 5`, `let x = 5`, `var x = 5`             | Plain assignment: `x = 5`                 |
| `TSW1002` | `build fn()`, `fun fn()`, `function fn()`           | `do fn()`                                 |
| `TSW1003` | `return x`                                          | `send x`                                  |
| `TSW1004` | `attempt { … }`                                     | `try … end`                               |
| `TSW1005` | `give x`                                            | `send x`                                  |
| `TSW1006` | `{ }` block delimiters, `;` statement terminators   | `end`, newlines                           |
| `TSW1007` | `if cond`, `elif cond`                              | `when cond`, `else when cond`             |
| `TSW1008` | `while cond`                                        | `repeat cond`                             |
| `TSW1009` | `import mod`, `from mod import x`                   | `use mod`                                 |
| `TSW1010` | `each x in y`                                       | `for x in y`                              |
| `TSW1011` | `none`                                              | `null`                                    |
| `TSW1012` | `f"…"` f-strings                                    | `$"…"` interpolated strings               |
| `TSW1013` | `model Name`                                        | `class Name`                              |
| `TSW1014` | `std.io.println(x)`                                 | Built-in `say x`                          |

### Example Warning Output

```
[TSW1002] file.ts:12:1 — Deprecated: 'build' is a TechScript 1.x keyword.
  Use 'do' for function declarations.
  12 | build greet(name)
       ^^^^^
  Canonical: do greet(name)
  Run: tsc migrate file.ts  to apply automatic fix.
```

---

## Style / Lint Warnings — Semantic Phase — TSW2001–TSW2099

Lint warnings are raised by the semantic analyser to flag code that is syntactically valid but stylistically problematic or potentially erroneous.

| Code      | Message                                       | Cause                                                                                                          |
| --------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `TSW2001` | Variable `<name>` declared but never used     | A variable is assigned a value that is never subsequently read                                                 |
| `TSW2002` | Variable `<name>` shadows outer scope         | A declaration in an inner scope has the same name as one in an outer scope, hiding the outer binding           |

---

## Informational Hints — TSI3001–TSI3099

Hints are purely informational suggestions. They do not indicate incorrect code; they point toward idiomatic TechScript 2.0 style. Hints are suppressed by default and enabled with `--hints` or in editor integrations.

| Code      | Message                                                                              |
| --------- | ------------------------------------------------------------------------------------ |
| `TSI3001` | Consider using `$"…"` string interpolation instead of string concatenation with `+`  |

### Example

```
# Triggers TSI3001
greeting = "Hello, " + name + "!"

# Idiomatic 2.0
greeting = $"Hello, {name}!"
```

---

## Migration

### Automatic Migration Tool

Run the following command to automatically rewrite all `TSW1001`–`TSW1013` deprecated-syntax patterns to their canonical 2.0 equivalents:

```
tsc migrate <file-or-directory>
```

The migrator performs in-place rewrites with a `.bak` backup of the original. After migration, re-run the compiler to confirm that no warnings remain.

### Manual Mapping Reference

For manual migration, use the [Deprecation Warnings table](#deprecation-warnings--parser-phase-tsw1001--tsw1099) above as a find-and-replace reference. The most common migrations are:

| Pattern                           | Replace with         |
| --------------------------------- | -------------------- |
| `build` / `fun` / `function`      | `do`                 |
| `return`                          | `send`               |
| `if` / `elif`                     | `when` / `else when` |
| `while`                           | `repeat`             |
| `import` / `from … import`        | `use`                |
| `none`                            | `null`               |
| `f"…"`                            | `$"…"`               |
| `model`                           | `class`              |
| `{ … }` block delimiters          | indent + `end`       |
| `;` statement terminators         | newline              |

> **Note:** `TSW1014` (`std.io.println` → `say`) is excluded from automatic migration because the migrator cannot always distinguish qualified `std.io.println` calls from user-defined functions with the same name. Verify these manually.

---

*End of specification — TechScript 2.0 Error Code Reference v2.0.0*
