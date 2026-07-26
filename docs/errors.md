# TechScript 2.0 Error & Warning Code Reference

This document serves as the authoritative user-facing reference for all TechScript 2.0 compile-time errors (`TSE0xxx`), runtime errors (`TSE1xxx`), parser deprecation warnings (`TSW1xxx`), semantic style warnings (`TSW2xxx`), and informational hints (`TSI3xxx`).

---

## Lexer Errors (TSE0001 – TSE0099)

Lexer errors occur during tokenization, before parsing begins.

<div id="E0001"></div>

### E0001: Unexpected character
* **Cause:** A character not supported by the TechScript 2.0 grammar was encountered (e.g. `@`, `$`, backticks outside string prefixes).
* **Example:** `x = @5`

<div id="E0010"></div>

### E0010: Trailing underscore in number
* **Cause:** A numeric literal contains an underscore grouping separator as its final character.
* **Example:** `price = 1000_`
* **Fix:** Remove the trailing underscore: `price = 1000`

<div id="E0011"></div>

### E0011: Empty numeric prefix
* **Cause:** A base-prefix (like `0x` for hex, `0b` for binary, `0o` for octal) is not followed by any digits.
* **Example:** `hex_val = 0x`

<div id="E0012"></div>

### E0012: Invalid base digit
* **Cause:** A digit within a base-prefixed numeric literal does not belong to that base.
* **Example:** `bin_val = 0b102` (binary digits must be `0` or `1`)

<div id="E0021"></div>

### E0021: Unterminated string
* **Cause:** A string literal is opened (with `"`) but never closed before a newline or end-of-file.
* **Example:** `msg = "Hello World`

---

## Parser Errors (TSE0100 – TSE0299)

Parser errors occur when the token stream does not conform to the formal grammar.

<div id="E0100"></div>

### E0100: Expected expression
* **Cause:** A statement or assignment expected an expression value but received a keyword or terminator.
* **Example:** `x =`

<div id="E0101"></div>

### E0101: Expected identifier
* **Cause:** The parser expected a variable name, function name, or class name but received a literal or keyword.
* **Example:** `do 123()`

<div id="E0104"></div>

### E0104: Expected `end` to close block
* **Cause:** A block-starting statement (like `do`, `when`, `repeat`, `for`, `class`) was not terminated with a matching `end` keyword.
* **Example:**
  ```txs
  do greet()
      say "Hello"
  # Missing 'end'
  ```

<div id="E0105"></div>

### E0105: Expected block body
* **Cause:** A block-introducing statement was not followed by an indented statement block.

<div id="E0107"></div>

### E0107: Expected statement terminator
* **Cause:** Multiple statements were placed on the same line without a newline separator. Semicolons are not allowed in TechScript 2.0.
* **Example:** `x = 5 y = 10`

<div id="E0113"></div>

### E0113: Invalid assignment target
* **Cause:** The target of an assignment is not an l-value (e.g. attempting to assign to a literal).
* **Example:** `42 = x`

---

## Semantic Errors (TSE0300 – TSE0499)

Semantic errors are identified during name resolution, scope checking, and type checking.

<div id="E0300"></div>

### E0300: Undefined variable
* **Cause:** A variable is referenced that has not been declared or assigned in the current scope or any parent scope.
* **Example:** `say unregistered_variable`

<div id="E0301"></div>

### E0301: Duplicate declaration
* **Cause:** A function, class, struct, or variable name is declared multiple times in the same scope block.
* **Example:**
  ```txs
  do compute()
  end
  do compute() # E0301
  end
  ```

<div id="E0302"></div>

### E0302: Cannot reassign `const`
* **Cause:** An assignment targets a variable declared as constant (e.g., via `const`).
* **Example:**
  ```txs
  const MAX_LIMIT = 100
  MAX_LIMIT = 200 # E0302
  ```

<div id="E0303"></div>

### E0303: Variable used before assignment
* **Cause:** A variable name is lookup-registered, but read access is attempted before it is assigned a value.

<div id="E0310"></div>

### E0310: Wrong argument count (too few)
* **Cause:** A function or method call passed fewer arguments than required by its signature.

<div id="E0311"></div>

### E0311: Wrong argument count (too many)
* **Cause:** A function or method call passed more arguments than allowed by its signature.

<div id="E0312"></div>

### E0312: `send` outside function body
* **Cause:** A `send` (return) statement appears at the top level or within module scope, outside any function.

<div id="E0313"></div>

### E0313: Mixed top-level statements with explicit main
* **Cause:** A script contains both top-level execution statements and an explicit `do main()` entrypoint function.
* **Fix:** Place top-level statements inside the `main()` function or remove the explicit `main()`.

<div id="E0320"></div>

### E0320: `self` used outside method
* **Cause:** The implicit instance reference `self` was used outside a class method context.

<div id="E0340"></div>

### E0340: Module not found
* **Cause:** A `use` statement names a module that cannot be resolved in the standard libraries or project search paths.
* **Example:** `use non_existent_module`

<div id="E0350"></div>

### E0350: Cannot export non-exportable declaration
* **Cause:** An `export` statement was applied to a declaration that cannot be exported (e.g., a local helper).

---

## DSL Validation Errors (TSE0400 – TSE0499)

DSL validation errors occur when embedded declarative blocks fail schema validation constraints.

<div id="E0400"></div>

### E0400: DSL block missing required field
* **Cause:** A required schema property is absent from a declarative DSL block.

<div id="E0401"></div>

### E0401: DSL field has wrong type
* **Cause:** A DSL property value does not conform to the expected type constraint in the schema.

<div id="E0402"></div>

### E0402: Unknown DSL directive
* **Cause:** A property name or sub-block appears inside a DSL block that is not recognized by its schema.

<div id="E0403"></div>

### E0403: Invalid nested DSL block
* **Cause:** A DSL block is nested inside a parent block where it is not allowed (e.g., placing `hero` directly inside `website` without an intervening `page` block).

---

## Runtime Errors (TSE1000 – TSE1999)

Runtime errors occur during execution inside the TechScript Virtual Machine.

<div id="E1010"></div>

### E1010: Division by zero
* **Cause:** An integer division `/`, floor division `//`, or modulo `%` operation was performed with a divisor of zero.

<div id="E1011"></div>

### E1011: Type mismatch
* **Cause:** A binary or unary operator was applied to values of incompatible types at runtime.
* **Example:** `result = "text" - 42`

<div id="E1020"></div>

### E1020: Stack overflow
* **Cause:** Unbounded recursion or excessive call stack depth exceeded the runtime recursion limit.

<div id="E1030"></div>

### E1030: Value not iterable in `for` loop
* **Cause:** The target of a `for ... in` loop is not a sequence, collection, or range at runtime.

<div id="E1041"></div>

### E1041: Field or method not found
* **Cause:** Attempted to access a member or call a method on an object that does not define it.

<div id="E1050"></div>

### E1050: Index out of bounds
* **Cause:** A list, string, or collection lookup index is outside the valid range of the sequence.

---

## Deprecation Warnings (TSW1001 – TSW1099)

Parser warnings emitted for deprecated 1.0.8 syntax.

<div id="TSW1001"></div>

### TSW1001: Deprecated variable declaration keywords
* **Cause:** Use of `make`, `let`, `var`, or `keep` to declare variables.
* **Fix:** Use plain assignment `x = 5` or `const X = 5` for constants.

<div id="TSW1002"></div>

### TSW1002: Deprecated function declaration keywords
* **Cause:** Use of `build`, `fun`, or `function` to declare a function.
* **Fix:** Replace with `do`.

<div id="TSW1003"></div>

### TSW1003: Deprecated return keyword
* **Cause:** Use of `return`.
* **Fix:** Replace with `send`.

<div id="TSW1004"></div>

### TSW1004: Deprecated error handling syntax
* **Cause:** Use of `attempt` blocks.
* **Fix:** Replace with `try`.

<div id="TSW1005"></div>

### TSW1005: Deprecated return alias
* **Cause:** Use of `give`.
* **Fix:** Replace with `send`.

<div id="TSW1006"></div>

### TSW1006: Deprecated curly braces and semicolons
* **Cause:** Use of curly braces `{}` or statement-terminating semicolons `;`.
* **Fix:** Delimit blocks using indentation + `end`, and use newlines as terminators.

<div id="TSW1007"></div>

### TSW1007: Deprecated if-statement keywords
* **Cause:** Use of `if` and `elif`.
* **Fix:** Replace with `when` and `else when`.

<div id="TSW1008"></div>

### TSW1008: Deprecated while-loop keyword
* **Cause:** Use of `while`.
* **Fix:** Replace with `repeat`.

<div id="TSW1009"></div>

### TSW1009: Deprecated import syntax
* **Cause:** Use of `import` or `from ... import`.
* **Fix:** Replace with `use`.

<div id="TSW1010"></div>

### TSW1010: Deprecated loop keyword
* **Cause:** Use of `each`.
* **Fix:** Replace with `for`.

<div id="TSW1011"></div>

### TSW1011: Deprecated null representation
* **Cause:** Use of `none`.
* **Fix:** Replace with `null`.

<div id="TSW1012"></div>

### TSW1012: Deprecated f-string syntax
* **Cause:** Use of `f"..."` format string prefix.
* **Fix:** Replace with `$"..."`.

<div id="TSW1013"></div>

### TSW1013: Deprecated model keyword
* **Cause:** Use of `model`.
* **Fix:** Replace with `class`.

<div id="TSW1014"></div>

### TSW1014: Deprecated print function calls
* **Cause:** Calling `std.io.println`.
* **Fix:** Use the implicit built-in `say` statement without parentheses.

---

## Style / Lint Warnings (TSW2001 – TSW2099)

Lint warnings emitted during semantic analysis.

<div id="TSW2001"></div>

### TSW2001: Unused variable
* **Cause:** A variable is declared or assigned but never subsequently read.

<div id="TSW2002"></div>

### TSW2002: Variable shadowing
* **Cause:** An identifier declared in an inner scope shadows a variable in a parent scope.

---

## Informational Hints (TSI3001 – TSI3099)

Hints for writing idiomatic TechScript 2.0.

<div id="TSI3001"></div>

### TSI3001: String concatenation
* **Cause:** Using `+` to concatenate strings.
* **Recommendation:** Use `$"..."` string interpolation instead.
