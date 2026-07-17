# TechScript 2.0 Lexer Token Specification

This document lists every token kind recognized by the TechScript 2.0 Lexer (`techscript_lexer`) and defined in the Unified Token Registry (`techscript_syntax`).

---

## 1. Keywords

### 1.1 Canonical Keywords
These are the preferred keywords in TechScript 2.0. The formatter and linter will automatically rewrite aliases to these canonical forms.

| Token Kind | Lexeme | Description |
|---|---|---|
| `Make` | `make` | Mutable variable declaration |
| `Const` | `const` | Immutable constant declaration |
| `Say` | `say` | Print to standard output (with newline) |
| `Ask` | `ask` | Read line from standard input |
| `Build` | `build` | Function or method definition |
| `Return` | `return` | Return from function |
| `Model` | `model` | Class/Object declaration |
| `SelfKw` | `self` | Instance self-reference |
| `New` | `new` | Instantiate a model |
| `If` | `if` | Conditional branch start |
| `Elif` | `elif` | Else-if conditional branch |
| `Else` | `else` | Default conditional branch |
| `For` | `for` | For-in loop iterator |
| `In` | `in` | In iterator boundary / membership operator |
| `While` | `while` | Loop while condition is true |
| `Repeat` | `repeat` | Loop N times |
| `Break` | `break` | Terminate loop execution |
| `Continue` | `continue` | Skip to next loop iteration |
| `Try` | `try` | Exception block start |
| `Catch` | `catch` | Exception error handler block |
| `Throw` | `throw` | Raise/throw an exception |
| `Import` | `import` | Import module |
| `From` | `from` | Selective import module source |
| `Export` | `export` | Export declaration |
| `True` | `true` | Boolean true literal |
| `False` | `false` | Boolean false literal |
| `Null` | `null` | Canonical null/none literal |

### 1.2 Alias Keywords (Backward Compatibility)
These are supported to prevent compile errors on legacy code.

| Token Kind | Lexeme | Canonical Target |
|---|---|---|
| `Let` | `let` | `make` |
| `Var` | `var` | `make` |
| `Fun` | `fun` | `build` |
| `Function` | `function` | `build` |
| `When` | `when` | `if` |
| `Attempt` | `attempt` | `try` |
| `None` | `none` | `null` |
| `Class` | `class` | `model` |

### 1.3 Future Reserved Keywords
Reserved for future static typing, concurrency, and pattern-matching features.

| Token Kind | Lexeme | Description |
|---|---|---|
| `Async` | `async` | Asynchronous function modifier |
| `Await` | `await` | Wait for asynchronous coroutine |
| `Type` | `type` | Type alias declaration |
| `Interface`| `interface` | Interface contract definition |
| `Match` | `match` | Pattern matching switch |
| `Switch` | `switch` | Standard switch-case block |
| `Case` | `case` | Switch pattern branch |
| `Enum` | `enum` | Algebraic data type declaration |
| `Struct` | `struct` | Data-only record declaration |
| `Trait` | `trait` | Interface/trait declaration |
| `Yield` | `yield` | Coroutine yield generator |
| `Spawn` | `spawn` | Thread/concurrency spawn |
| `Pub` | `pub` | Public visibility modifier |
| `Mut` | `mut` | Explicit mutability modifier |

---

## 2. Literals and Identifiers

| Token Kind | Example Lexemes | Description |
|---|---|---|
| `Identifier` | `foo`, `_bar`, `name_123` | Variable/function/type names |
| `IntLiteral` | `42`, `0xFF`, `0b101`, `1_000` | Integer numbers |
| `FloatLiteral`| `3.14`, `1.0e10`, `2.5E-3` | Floating-point numbers |
| `StringLiteral`| `"hello world"`, `"escaped\n"` | Double-quoted strings |
| `FStringStart` | `f"` | Start of an interpolated string |
| `FStringText` | `segment ` | Constant string portion of f-string |
| `FStringExprStart` | `{` | Start of an expression within f-string |
| `FStringExprEnd` | `}` | End of an expression within f-string |
| `FStringEnd` | `"` | End of an interpolated string |

---

## 3. Operators and Punctuation

### 3.1 Arithmetic
| Token Kind | Lexeme | Description |
|---|---|---|
| `Plus` | `+` | Addition / Unary Plus |
| `Minus` | `-` | Subtraction / Unary Minus |
| `Star` | `*` | Multiplication |
| `Slash` | `/` | Division |
| `DoubleSlash`| `//` | Integer Division |
| `Percent` | `%` | Modulo |
| `DoubleStar` | `**` | Exponentiation |

### 3.2 Comparison and Logical
| Token Kind | Lexeme | Description |
|---|---|---|
| `EqualEqual` | `==` | Loose Equality |
| `BangEqual` | `!=` | Loose Inequality |
| `TripleEqual`| `===` | Strict Equality |
| `BangEqualEqual`| `!==` | Strict Inequality |
| `Less` | `<` | Less Than / Generic Left Bracket |
| `Greater` | `>` | Greater Than / Generic Right Bracket |
| `LessEqual` | `<=` | Less or Equal |
| `GreaterEqual`| `>=` | Greater or Equal |
| `And` | `&&`, `and` | Logical AND (both symbol and word matched to same kind) |
| `Or` | `\|\|`, `or` | Logical OR (both symbol and word matched to same kind) |
| `Not` | `!`, `not` | Logical NOT (both symbol and word matched to same kind) |
| `Is` | `is` | Identity check |

### 3.3 Assignment
| Token Kind | Lexeme | Description |
|---|---|---|
| `Equal` | `=` | Variable assignment |
| `PlusEqual` | `+=` | Add-assign |
| `MinusEqual` | `-=` | Subtract-assign |
| `StarEqual` | `*=` | Multiply-assign |
| `SlashEqual` | `/=` | Divide-assign |
| `PercentEqual`| `%=` | Modulo-assign |

### 3.4 Ranges and Navigation
| Token Kind | Lexeme | Description |
|---|---|---|
| `DotDot` | `..` | Range (exclusive) |
| `DotDotEqual`| `..=` | Range (inclusive) |
| `QuestionDot`| `?.` | Optional Chaining |
| `QuestionQuestion`| `??` | Null Coalescing |
| `Arrow` | `->` | Function return type specifier |

### 3.5 Delimiters and Separators
| Token Kind | Lexeme | Description |
|---|---|---|
| `LeftParen` | `(` | Left Parenthesis |
| `RightParen` | `)` | Right Parenthesis |
| `LeftBrace` | `{` | Left Curly Brace |
| `RightBrace` | `}` | Right Curly Brace |
| `LeftBracket`| `[` | Left Square Bracket |
| `RightBracket`| `]` | Right Square Bracket |
| `Comma` | `,` | Separator |
| `Dot` | `.` | Member access |
| `Colon` | `:` | Type annotation / map separator |
| `Semicolon` | `;` | Optional statement terminator |

---

## 4. Special and Control

| Token Kind | Lexeme | Description |
|---|---|---|
| `Newline` | `\n`, `\r\n` | Significant statement boundary |
| `Eof` | `\0` | End of source file |
| `Error` | N/A | Malformed token / unrecognized character |
