# TechScript 2.0 Language Syntax Specification

> **Status**: Official Reference Design Specification
> **Version**: 2.0.0 (Syntax Frozen)
> **Last Updated**: 2026-07-16
> **Authoritative Target**: All Compiler Frontends (`techscript_lexer`, `techscript_parser`, `techscript_semantic`)

---

## 1. Language Philosophy

TechScript 2.0 is designed to bridge the gap between beginner-friendly scripting and production-grade software engineering. It is governed by a strict hierarchy of design priorities:

1. **Readability**: Code should feel like reading natural English. Keyword-based control flow and delimiters are preferred over symbolic clutter.
2. **Simplicity**: Language concepts should have a single, obvious way to write them. Boilerplate is systematically minimized.
3. **Consistency**: Syntax and semantics must remain consistent across different paradigms (functional, object-oriented, structured).
4. **Parser Friendliness**: A deterministic, unambiguous grammar is preferred over complex, context-sensitive syntax. **Readability, simplicity, consistency, and parser friendliness always take priority over adding features.**

---

## 2. Lexical Grammar

### 2.1 Source Encoding and Whitespace
- **Encoding**: Source files must be valid UTF-8.
- **Whitespace**: Space (`U+0020`), tab (`U+0009`), and carriage returns (`\r`) are treated as whitespace.
- **Statement Termination**: Semicolons (`;`) are optional. Statements are terminated by a newline (`\n` or `\r\n`) or the end of a block (`}`). A newline does not terminate a statement if it follows an unclosed delimiter (`(`, `[`, `{`) or an infix operator.

### 2.2 Comments
TechScript 2.0 supports three comment styles to maximize familiarity:
1. **Single-line Hash**: `# comment` (Python style)
2. **Single-line Slash**: `// comment` (C++/Rust/Dart style)
3. **Multi-line Block**: `/* comment */` (C/Rust style). Block comments **can be nested** arbitrarily.

### 2.3 Identifiers
Identifiers are case-sensitive. They must begin with an ASCII letter (`a-z`, `A-Z`) or an underscore (`_`), followed by any number of letters, digits (`0-9`), or underscores.
- **Unicode Support**: Identifiers may also contain non-ASCII Unicode letters, allowing localized variable and function naming.
- **Reserved Identifier Prefix**: Identifiers starting with double underscores (`__`) are reserved for internal compiler/runtime use and trigger warning `W0001`.

---

## 3. Keywords and Policy

To maintain both clean, modern readability and backward compatibility, TechScript 2.0 defines canonical keywords and their backward-compatibility aliases.

### 3.1 Keyword Policy
- **Active Keywords**: Recognized and compiled.
- **Canonical Keywords**: The preferred representation of a language construct. The formatter (`tech fmt`) and linter (`tech lint --fix`) will **automatically rewrite** alias keywords to their canonical equivalents.
- **Alias Keywords**: Tolerated and successfully parsed to ensure backward compatibility with Version 1.0.

### 3.2 Canonical vs. Alias Keywords

| Construct | Canonical Keyword | Alias Keyword(s) | Behavior |
|---|---|---|---|
| **Variable** | `make` | `let`, `var` | Formatter rewrites `let`/`var` to `make` |
| **Function** | `build` | `fun`, `function` | Formatter rewrites `fun`/`function` to `build` |
| **Condition** | `if` | `when` | Formatter rewrites `when` to `if` |
| **Error Block** | `try` | `attempt` | Formatter rewrites `attempt` to `try` |
| **Null/None** | `null` | `none` | Formatter rewrites `none` to `null` |
| **Class** | `model` | `class` | Formatter rewrites `class` to `model` |

### 3.3 Reserved Keywords Table
Distinguishes active keywords from future-reserved keywords to ensure forward compatibility:

| Type | Keywords | Description |
|---|---|---|
| **Active (Canonical)** | `make`, `const`, `say`, `ask`, `build`, `return`, `model`, `self`, `new`, `if`, `else`, `elif`, `for`, `in`, `while`, `repeat`, `break`, `continue`, `try`, `catch`, `throw`, `import`, `from`, `export`, `true`, `false`, `null` | Primary keywords forming the core language. |
| **Active (Aliases)** | `let`, `var`, `fun`, `function`, `when`, `attempt`, `none`, `class` | Aliases for backward compatibility. |
| **Future Reserved** | `async`, `await`, `type`, `interface`, `match`, `switch`, `case`, `enum`, `struct`, `trait`, `yield`, `spawn`, `pub`, `mut` | Reserved for future compiler phases. Cannot be used as identifiers. |

### 3.4 Literals
- **Integers**:
  - Decimal: `12345` (can use underscores: `1_000_000`)
  - Hexadecimal: `0x1A2B` (prefix `0x` or `0X`)
  - Binary: `0b1010` (prefix `0b` or `0B`)
  - Octal: `0o755` (prefix `0o` or `0O`)
- **Floats**:
  - Standard decimal notation: `3.14159`
  - Scientific notation: `6.022e23` or `1.0E-9`
- **Booleans**: `true` and `false` (strictly lowercase).
- **Null Values**: `null` (canonical) and `none` (alias).
- **Strings**: Double-quoted UTF-8 literals `"..."`.
  - **Escape Sequences**: `\n` (newline), `\t` (tab), `\r` (carriage return), `\"` (double quote), `\\` (backslash), `\0` (null byte), and `\u{HEX}` (Unicode scalar value, e.g., `\u{1F600}`).
- **F-Strings (Interpolated Strings)**: Sourced with `f"..."`. Expressions inside `{}` are evaluated at runtime.
  - *Example*: `f"Total: {price + tax}"`

---

## 4. Numeric Type Behavior

TechScript 2.0 defines explicit behaviors for numeric calculations to provide a fixed contract for semantic analysis and code generation.

### 4.1 Underlying Type Sizes
- **Integers**: Signed 64-bit integers (`i64` in Rust).
- **Floating-point**: Double precision 64-bit floats (`f64` in Rust) conforming to the IEEE-754 standard.

### 4.2 Arithmetic and Division
- **Standard Division (`/`)**: Division of any two numbers (integers or floats) always yields a `Float`.
  - *Example*: `5 / 2` yields `2.5`.
- **Integer Division (`//`)**: Truncating division of any two numbers. Returns an `Int`. Truncates towards zero.
  - *Example*: `5 // 2` yields `2`. `-5 // 2` yields `-2`.
- **Modulo (`%`)**: Standard remainder. For floats, it calculates the remainder of division.

### 4.3 Overflow and Coercion Rules
- **Integer Overflow**: In standard mode, integer overflow behaves like Rust's default release mode (wraps around using two's complement arithmetic). In `strict mode`, integer overflow is a checked runtime exception.
- **Implicit Conversion (Coercion)**: Implicit coercion only occurs from `Int` to `Float` when an integer and a float are mixed in binary operations (e.g. `+`, `-`, `*`, `/`, `**`).
  - *Example*: `2 + 1.5` implicitly coerces `2` to `2.0` and returns `3.5`.
  - Coerced comparisons: Comparing an `Int` and a `Float` via `==` is loose; `===` is strict and evaluates to `false` if types differ.
- **Literal Inference**: Numeric literals without a decimal point or exponent are inferred as `Int`. Literals containing a dot or exponent are inferred as `Float`.

---

## 5. Type System and Generics

TechScript 2.0 is dynamically typed at runtime, but supports optional type annotations for compile-time checking.

### 5.1 Compilation vs. Runtime Type Semantics
- **Type Erasure**: All type annotations and generic definitions are entirely erased during compilation. They impose **zero runtime performance overhead**.
- **Compile-Time Checking**:
  - In **Standard Mode**, type mismatch flags a warning at compile time but does not block execution.
  - In **Strict Mode**, type mismatch triggers a compile-time compiler error.

### 5.2 Collection Types
- **Lists**: Ordered, dynamic arrays. Generic type: `List<T>`.
- **Maps**: Insertion-ordered associative tables. Generic type: `Map<K, V>`.
- **Tuples**: Fixed-size immutable sequences. Syntax: `(Int, Str)`.

### 5.3 Optional Type Annotations
Postfix colon syntax is used for all annotations:
```txs
make age: Int = 30
const name: Str = "Alice"
```
If annotations are omitted, the compiler infers the type based on the initial value.

### 5.4 Structs and Enums
- **Structs**: Lightweight data containers with named fields, no methods, and value semantics.
  ```txs
  struct Point {
      x: Float
      y: Float
  }
  ```
- **Enums**: Algebraic data types.
  ```txs
  enum ConnectionState {
      Disconnected
      Connecting
      Connected(Str) // supports payload values
  }
  ```

### 5.5 Generics
Generics use industry-standard angle brackets (`<...>`) to align with Rust, TypeScript, and Dart.
- **Usage**:
  ```txs
  build getFirst<T>(list: List<T>) -> T {
      return list[0]
  }
  ```

---

## 6. Variables and Constants

### 6.1 Declarations
- **Mutable**: `make` or `let` / `var`. All three are valid.
- **Immutable**: `const`. Must be initialized immediately.
- **Block Scope**: Variables are lexically scoped to the enclosing `{}` block.

### 6.2 Shadowing
Redeclaring a variable in an inner scope is allowed, but triggers warning `W0010`. Redeclaring a variable in the *same* scope triggers error `E0301`.

### 6.3 Destructuring
Allows unpacking lists, maps, structs, and tuples:
```txs
make (x, y) = (10, 20)          // Tuple destructuring
make [first, second] = list    // List destructuring
make { x, y } = point          // Struct/Map destructuring
```

---

## 7. Expressions and Operators

### 7.1 Precedence and Associativity Reference Table
The table below lists every operator, defining its precedence level and associativity. This represents the stable parser binding rules.

| Level | Operator(s) | Description | Associativity |
|---|---|---|---|
| **1** | `?.` `.` `[]` `()` | Optional Chaining, Member Access, Indexing, Function Call | Left |
| **2** | `**` | Exponentiation | Right |
| **3** | `+` `-` `!` `not` | Unary Plus, Unary Minus, Logical NOT | Right |
| **4** | `*` `/` `//` `%` | Multiplication, Division, Integer Division, Modulo | Left |
| **5** | `+` `-` | Addition, Subtraction | Left |
| **6** | `<<` `>>` | Bitwise Shifts | Left |
| **7** | `&` | Bitwise AND | Left |
| **8** | `^` | Bitwise XOR | Left |
| **9** | `\|` | Bitwise OR | Left |
| **10**| `<` `>` `<=` `>=` `is` `in` | Comparisons, Identity, Membership | Left |
| **11**| `==` `!=` `===` `!==` | Equality (loose and strict) | Left |
| **12**| `&&` `and` | Logical AND | Left |
| **13**| `\|\|` `or` | Logical OR | Left |
| **14**| `??` | Null Coalescing | Right |
| **15**| `=` `+=` `-=` `*=` `/=` | Assignment (Lowest Precedence) | Right |

### 7.2 Special Operators
- **Assignment as Expression**: Assignments behave as right-associative expressions, returning the assigned value. This permits chained assignments.
  - *Example*: `make x = y = 10` parses as `make x = (y = 10)`.
- **Strict Equality (`===` / `!==`)**: Performs value comparison without coercion.
- **Loose Equality (`==` / `!=`)**: Standard comparison (implicitly coerces Float to Int if values match).
- **Null-Coalescing (`??`)**: Returns the right-hand operand if the left-hand is `null`/`none`.
- **Optional Chaining (`?.`)**: Safely navigates nested objects; evaluates to `null` if any reference is null.

---

## 8. Statements and Control Flow

### 8.1 Conditionals
Conditions do not require surrounding parentheses.
- **Canonical `if`**:
  ```txs
  if x > 10 {
      say "Large"
  } elif x > 5 {
      say "Medium"
  } else {
      say "Small"
  }
  ```
- **Alias `when`**: Behaves identically to `if`.

### 8.2 Loops
- **For-In Loop**:
  ```txs
  for i in 1..10 { say i } // exclusive range (1 to 9)
  for i in 1..=10 { say i } // inclusive range (1 to 10)
  ```
- **Each Loop** (backward compatibility alias):
  ```txs
  each item in list { say item }
  ```
- **While Loop**:
  ```txs
  while condition { tick() }
  ```
- **Repeat Loop**:
  ```txs
  repeat 5 { say "Hello" } // executes block exactly 5 times
  ```

### 8.3 Match (Pattern Matching)
Uses structurally matched patterns:
```txs
match state {
    ConnectionState.Disconnected => say "Offline"
    ConnectionState.Connected(user) => say f"Online as {user}"
    _ => say "Unknown state"
}
```

### 8.4 Error Handling
Supports both legacy `attempt`/`catch` and modern `try`/`catch`:
```txs
try {
    throw "Oops"
} catch err {
    say err
}
```

---

## 9. Modular System

### 9.1 Module Imports
Modules correspond directly to source files.
- **Simple Import**: `import math`
- **Selective Import**: `from math import sin, cos`

### 9.2 Exports
Exports public symbols:
```txs
export build add(a, b) {
    return a + b
}
```

### 9.3 Standard Entry Point
A TechScript file can be executed directly. The compiler/interpreter starts execution at the top-level statements. If an explicit `main` function is defined, it is automatically called at the end of top-level statement execution.
```txs
build main() {
    say "Entry Point Executed"
}
```

---

## 10. Minimum v2.0 Standard Library

The following built-in functions are pre-registered in the global compiler/interpreter scope:

| Signature | Description |
|---|---|
| `say(value: Any) -> None` | Prints value and a newline to stdout. |
| `ask(prompt: Str) -> Str` | Prints prompt to stdout, reads a line from stdin. |
| `len(collection: Any) -> Int` | Returns the length of list, map, string, or tuple. |
| `type_of(value: Any) -> Str` | Returns the primitive type name of the value. |
| `range(start: Int, end: Int) -> List<Int>` | Generates an exclusive list of integers. |
| `to_int(value: Any) -> Int` | Coerces string or float to Int. |
| `to_float(value: Any) -> Float` | Coerces string or int to Float. |
| `to_str(value: Any) -> Str` | Formats value to String. |
| `to_bool(value: Any) -> Bool` | Resolves truthiness of the value. |
| `exit(code: Int) -> None` | Terminates process with code. |
| `assert(condition: Bool) -> None` | Aborts execution with error if condition is false. |

---

## 11. Parser Implications & Ambiguities

### 11.1 Angle Bracket Generics vs Comparison Operators
Using `<` and `>` for generics introduces potential parsing ambiguities (e.g., is `x < y > z` a comparison chain or a generic instantiation?).
- **Disambiguation Rule**: TechScript 2.0 adopts a **context-aware tokenization** rule. In type annotation contexts (following a `:` or `->` token) or declaration signatures (struct, class, build declarations), `<` and `>` are lexed as type brackets. In standard expression contexts, they are parsed as comparisons. To instantiate generic structures in expressions, type arguments can be explicitly passed using a dot-prefix if necessary, or resolved during semantic resolution if the symbol resolves to a generic type definition.

### 11.2 Optional Semicolons
To support optional semicolons without JavaScript-style automatic semicolon insertion (ASI) bugs:
- **Termination Rule**: A statement is terminated by a newline if the next non-whitespace character cannot syntactically continue the current statement. If the next line begins with a binary operator (like `+` or `&&`), it is automatically treated as a statement continuation.

### 10.3 Interpolated Strings (F-Strings)
The `{` and `}` braces inside f-strings could conflict with block brackets.
- **Lexer Stack**: The lexer maintains a mode stack. When `f"` is encountered, the lexer enters f-string mode. Upon encountering `{`, it pushes a standard expression mode onto the stack, resuming normal tokenization. Upon matching the closing `}`, it pops the expression mode and returns to f-string mode.

---

## 12. Future Extension Points & Strict Mode

### 12.1 Strict Mode
To enable long-term evolution without breaking backward compatibility, developers can opt-in to strict compiler checking:
```txs
strict mode // must be the first line of the file
```
**Strict Mode Enforcements**:
- Deprecated keywords (like `fun` and `when`) trigger hard compiler errors instead of warnings.
- Explicit type annotations are mandatory for all function parameters and struct/class fields (no raw dynamic typing allowed).

### 12.2 Future Roadmaps
- **v2.2**: Static type-checking checks.
- **v3.0**: Native code generation optimizations via an LLVM (`inkwell`) backend.

---

## 13. Language Freeze (v2.0)

> [!IMPORTANT]
> **TechScript 2.0 Syntax v2.0.0 is declared frozen as of 2026-07-16.**
>
> Any future modifications, additions, or deprecations to the keywords, operators, delimiters, EBNF grammar, or core execution semantics require a formal RFC proposal. This constraint guarantees compiler frontend stability during subsequent implementation milestones.

---

## 14. EBNF Grammar

```ebnf
program = { statement } EOF ;

statement = declaration
          | expression_statement
          | assignment_statement
          | say_statement
          | return_statement
          | throw_statement
          | break_statement
          | continue_statement
          | if_statement
          | for_statement
          | while_statement
          | repeat_statement
          | try_catch_statement
          | import_statement
          | block ;

block = "{" { statement } "}" ;

declaration = variable_declaration
            | constant_declaration
            | function_declaration
            | struct_declaration
            | enum_declaration
            | model_declaration
            | export_declaration ;

variable_declaration = ( "make" | "let" | "var" ) pattern [ ":" type_spec ] "=" expression TERMINATOR ;
constant_declaration = "const" pattern [ ":" type_spec ] "=" expression TERMINATOR ;

pattern = IDENTIFIER
        | "(" IDENTIFIER { "," IDENTIFIER } ")"
        | "[" IDENTIFIER { "," IDENTIFIER } "]"
        | "{" IDENTIFIER { "," IDENTIFIER } "}" ;

function_declaration = [ "async" ] ( "build" | "fun" | "function" ) IDENTIFIER [ generic_params ] "(" [ parameter_list ] ")" [ "->" type_spec ] block ;

generic_params = "<" IDENTIFIER { "," IDENTIFIER } ">" ;

parameter_list = parameter { "," parameter } ;
parameter = IDENTIFIER [ ":" type_spec ] [ "=" expression ] ;

struct_declaration = "struct" IDENTIFIER "{" { IDENTIFIER ":" type_spec TERMINATOR } "}" ;

enum_declaration = "enum" IDENTIFIER "{" { enum_variant } "}" ;
enum_variant = IDENTIFIER [ "(" type_list ")" ] TERMINATOR ;

model_declaration = ( "model" | "class" ) IDENTIFIER [ "extends" IDENTIFIER ] "{" { model_member } "}" ;
model_member = variable_declaration | constant_declaration | function_declaration ;

export_declaration = "export" ( function_declaration | constant_declaration | model_declaration | struct_declaration | enum_declaration ) ;

expression_statement = expression TERMINATOR ;
assignment_statement = assignment_target assignment_operator expression TERMINATOR ;

assignment_target = IDENTIFIER | member_access | index_access ;
assignment_operator = "=" | "+=" | "-=" | "*=" | "/=" | "%=" ;

say_statement = "say" expression TERMINATOR ;
return_statement = "return" [ expression ] TERMINATOR ;
throw_statement = "throw" expression TERMINATOR ;
break_statement = "break" TERMINATOR ;
continue_statement = "continue" TERMINATOR ;

TERMINATOR = NEWLINE | ";" ;

if_statement = ( "if" | "when" ) expression block
               { ( "elif" | "else" "if" | "else" "when" ) expression block }
               [ "else" block ] ;

for_statement = ( "for" | "each" ) IDENTIFIER "in" expression block ;
while_statement = "while" expression block ;
repeat_statement = "repeat" expression block ;

try_catch_statement = ( "try" | "attempt" ) block "catch" IDENTIFIER block ;

import_statement = "import" module_path TERMINATOR
                 | "from" module_path "import" import_list TERMINATOR ;

module_path = IDENTIFIER { "." IDENTIFIER } ;
import_list = IDENTIFIER { "," IDENTIFIER } ;

type_spec = IDENTIFIER [ generic_args ] ;
generic_args = "<" type_spec { "," type_spec } ">" ;
type_list = type_spec { "," type_spec } ;

expression = logic_or ;
logic_or = logic_and { ( "or" | "||" ) logic_and } ;
logic_and = logic_not { ( "and" | "&&" ) logic_not } ;
logic_not = [ "not" | "!" ] comparison ;
comparison = additive_expression [ comp_operator additive_expression ] ;
comp_operator = "==" | "!=" | "===" | "!==" | "<" | ">" | "<=" | ">=" | "is" | "in" ;

additive_expression = multiplicative_expression { ( "+" | "-" ) multiplicative_expression } ;
multiplicative_expression = exponent_expression { ( "*" | "/" | "//" | "%" ) exponent_expression } ;
exponent_expression = unary_expression [ "**" exponent_expression ] ;
unary_expression = ( "-" | "not" | "!" | "+" ) unary_expression
                 | null_coalescing ;

null_coalescing = optional_chaining { "??" optional_chaining } ;
optional_chaining = postfix_expression { "?." postfix_expression } ;

postfix_expression = primary { postfix_operator } ;
postfix_operator = call_operator | member_operator | index_operator ;

call_operator = "(" [ argument_list ] ")" ;
member_operator = "." IDENTIFIER ;
index_operator = "[" expression "]" ;

argument_list = expression { "," expression } ;

primary = INT_LITERAL
        | FLOAT_LITERAL
        | STRING_LITERAL
        | FSTRING_LITERAL
        | "true"
        | "false"
        | "none"
        | "null"
        | IDENTIFIER
        | list_literal
        | map_literal
        | "(" expression ")"
        | "ask" expression
        | "new" IDENTIFIER [ generic_args ] "(" [ argument_list ] ")" ;

list_literal = "[" [ expression { "," expression } ] "]" ;
map_literal = "{" [ map_entry { "," map_entry } ] "}" ;
map_entry = expression ":" expression ;
```
