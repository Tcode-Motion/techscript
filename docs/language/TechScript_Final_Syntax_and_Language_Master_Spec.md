# TechScript: Final Canonical Syntax, Standard Library, Webforge, AppForge, and Agent Rules

**Purpose:** This document is the authoritative target specification and implementation prompt for the existing TechScript repository. It defines one simple, human-readable syntax for normal TechScript programs and shows how the same language supports libraries, Webforge, and AppForge.

**Repository:** `C:\Users\Tanmoy\OneDrive\Documents\techscript`  
**Primary source extension:** `.txs`

> **Important implementation rule:** This document defines the target syntax. It does not claim that every construct is already supported by the current compiler. The coding agent must audit the real repository, implement the specification through the real compiler/runtime, test every example, and mark any unfinished feature honestly. Do not make examples appear to work by hardcoding their output.

---

# PART I — THE GOAL AND NON-NEGOTIABLE PRINCIPLES

## 1. What TechScript is

TechScript is a general-purpose programming language. It is not a UI language, a liquid-glass language, an AppForge-only language, or a collection of hardcoded demos.

The intended mental model is:

1. **TechScript** provides the language syntax and core programming rules.
2. **The compiler and runtime** parse, validate, compile, and execute `.txs` code.
3. **The standard library** provides common functions for text, collections, numbers, files, data, networking, errors, and other general tasks.
4. **Native integrations** expose operating-system, hardware, graphics, and external-library capabilities.
5. **Libraries and frameworks** provide reusable tools such as Webforge and AppForge.
6. **Applications** combine these capabilities. Their behavior must come from actual `.txs` code and libraries, not hardcoded Rust screens.

The language must be short and learnable by someone who knows everyday English. Keep common operations concise. Use explicit, consistent rules instead of unrestricted natural-language parsing.

## 2. Non-negotiable principles

- The primary file extension is `.txs`.
- Use one canonical syntax for each operation.
- Use indentation and `:` to introduce ordinary code blocks. Do not require curly braces for normal control-flow or function bodies.
- Do not require `let`, `var`, `fn`, `function`, or semicolons for ordinary code.
- Use `is` for creating a variable, and `becomes` for changing an existing variable.
- Use `to` to define a function.
- Use `say` to display text in beginner-facing examples.
- Use standard mathematical comparison symbols for comparisons and `and`, `or`, `not` for logic.
- Use parentheses for function calls and square brackets for lists and indexing.
- Use indentation-based blocks consistently across the language and the framework DSLs.
- Keep the core language separate from Webforge and AppForge APIs.
- Do not add a new language keyword for each UI component, animation, material, or visual effect.
- Do not switch back to conventional syntax in later code, docs, generated examples, or AI responses.
- A proposed feature is not an implemented feature until it compiles, runs, and has tests.
- If existing code conflicts with this target, report the conflict, migrate it carefully, and update tests and documentation together.

## 3. Canonical examples at a glance

```text
name is "TechScript"
count is 0

say "Hello, {name}!"

count becomes count + 1

if count < 10:
    say "Count is {count}"

repeat 3 times:
    say "Hello again!"

to greet(person):
    return "Hello, {person}!"

say greet("Tanmoy")
```

These examples define the target style. The implementation agent must make the actual compiler and runtime support them rather than copying them into documentation only.

---

# PART II — CANONICAL LANGUAGE SYNTAX

## 4. Source files and layout

- Source files use `.txs`.
- Use four spaces for each indentation level. Tabs are not allowed in indentation in the canonical formatter output.
- A colon (`:`) starts a block.
- A block ends when indentation returns to the previous level.
- Statements normally end at the end of a line. Semicolons are not required and are not part of canonical style.
- Empty lines may separate logical sections.
- Comments begin with `#` and continue to the end of the line.
- Use UTF-8 source files.
- The compiler must report inconsistent indentation, unexpected indentation, missing block bodies, and invalid continuation lines with clear source locations.

Example:

```text
# A short TechScript program
name is "Tanmoy"

if name == "Tanmoy":
    say "Welcome back!"
else:
    say "Hello!"
```

Do not use curly braces to delimit ordinary function, `if`, `repeat`, `for`, `while`, `try`, or framework layout blocks. Curly braces are used only for object/map literals and interpolation escaping as defined below.

## 5. Names and reserved words

Names begin with a letter or `_`, followed by letters, digits, or `_`. Canonical names use `snake_case` for variables and functions, and readable names for types. Names are case-sensitive.

Core reserved words and forms:

`is`, `becomes`, `constant`, `to`, `return`, `if`, `else`, `repeat`, `times`, `for`, `in`, `while`, `stop`, `skip`, `and`, `or`, `not`, `true`, `false`, `nothing`, `use`, `as`, `try`, `catch`, `finally`, `raise`, `async`, `await`, `type`, `test`, `expect`, `say`, `ask`.

The actual implementation may classify built-ins such as `say` and `ask` as standard-library functions instead of reserved tokens, but their source behavior must match this specification. Avoid reserving words that are not necessary.

## 5.1 Normative grammar skeleton

The parser must implement the following grammar model. This is a required language contract, not an invitation to invent a different syntax. The complete grammar must be checked into the repository and kept in sync with the parser.

```ebnf
program          = { statement, newline } ;
statement        = variable_decl | constant_decl | assignment
                 | function_decl | type_decl | enum_decl | protocol_decl
                 | if_stmt | repeat_stmt | for_stmt | while_stmt
                 | return_stmt | use_stmt | try_stmt | raise_stmt
                 | say_stmt | ask_expr | expression_stmt | test_decl ;

variable_decl    = name, "is", expression ;
constant_decl    = "constant", name, [ ":", type_expr ], "is", expression ;
assignment       = assignable, "becomes", expression ;
assignable       = name, { ".", name | "[", expression, "]" } ;

function_decl    = "to", name, "(", [ parameters ], ")", [ "->", type_expr ], block ;
parameters       = parameter, { ",", parameter } ;
parameter        = name, [ ":", type_expr ], [ "=", expression ] ;
call             = expression, "(", [ arguments ], ")" ;
arguments        = argument, { ",", argument } ;
argument         = [ name, ":" ], expression ;
block_call       = call, block ;
block            = ":", newline, indent, { statement, newline }, dedent ;

if_stmt          = "if", expression, block,
                   { "else if", expression, block },
                   [ "else", block ] ;
repeat_stmt      = "repeat", expression, "times", block
                 | "repeat", name, "from", expression, "to", expression, block ;
for_stmt         = "for", name, "in", expression, block ;
while_stmt       = "while", expression, block ;

list_literal     = "[", [ expression, { ",", expression } ], "]" ;
object_literal   = "{", [ pair, { ",", pair } ], "}" ;
pair             = ( name | string ), ":", expression ;
interpolation    = "{", expression, [ ":", format_spec ], "}" ;
expression       = literal | name | list_literal | object_literal
                 | call | property_access | index_access
                 | unary_expr | binary_expr | lambda_expr | grouped_expr ;
lambda_expr      = name, "=>", expression
                 | "(", [ name, { ",", name } ], ")", "=>", expression ;
```

This skeleton must be completed with precedence rules, lexical rules, type expressions, async forms, exception forms, enum/protocol/type bodies, multiline string handling, named arguments, and framework builder blocks. Framework blocks use the same generic `block_call` grammar; the core parser must not contain a special syntax rule for each individual framework widget.

`is` is reserved for declaration. `becomes` is reserved for reassignment. Equality uses `==`. All blocks use indentation and colons. Normal code blocks do not use braces.

## 6. Variables, reassignment, constants, and types

### 6.1 Create a variable

Use `is` for first assignment:

```text
name is "TechScript"
count is 0
active is true
nothing_here is nothing
```

The compiler infers a type from the initial value when it can. A variable name must not be silently redeclared in the same scope. If shadowing is allowed in nested scopes, document and test the rule.

### 6.2 Change an existing variable

Use `becomes` for reassignment:

```text
count becomes count + 1
name becomes "AppForge"
active becomes false
```

`becomes` is a statement, not an expression. It must fail with a useful diagnostic if the name is unknown, immutable, or assigned an incompatible value.

Do not use `name = value`, `let name = value`, or `var name = value` as canonical TechScript examples.

### 6.3 Constants

```text
constant app_name is "TechScript"
constant max_attempts is 5
constant tax_rate is 0.08
```

A constant cannot be reassigned. Compile-time evaluation may be used when safe and supported, but the language must not claim a value is compile-time constant when it depends on runtime work.

### 6.4 Optional explicit type annotations

Most beginner code should rely on inference. Explicit annotations are available where useful:

```text
age: whole is 20
price: decimal is 19.95
username: text is "tcode"
items: list is ["one", "two"]
```

Canonical primitive names:

- `whole`: whole-number values.
- `decimal`: fractional numeric values.
- `number`: a general numeric type where the implementation supports it.
- `text`: Unicode text.
- `boolean`: `true` or `false`.
- `nothing`: absence of a value.
- `list[T]`: ordered collection.
- `map[K, V]`: key/value collection.
- `bytes`: byte data.

Do not create multiple aliases for the same type without a compelling compatibility reason. The compiler must define integer overflow, decimal precision, numeric conversion, and type compatibility clearly.

## 7. Values and expressions

### 7.1 Literals

```text
message is "Hello"
whole_number is 42
fraction is 3.14159
is_ready is true
is_empty is nothing
letters is ["a", "b", "c"]
point is {x: 10, y: 20}
```

### 7.2 Arithmetic operators

Canonical arithmetic operators:

- `+` addition and text concatenation where defined.
- `-` subtraction.
- `*` multiplication.
- `/` division.
- `//` integer/floor division, with exact behavior documented and tested.
- `%` remainder.
- `**` exponentiation. It is part of the canonical operator set and must be implemented and tested.

Parentheses may group expressions:

```text
total is (price * quantity) + delivery_fee
```

The grammar must document operator precedence, left/right associativity, numeric conversion, divide-by-zero behavior, and overflow behavior. Do not leave these as accidental consequences of the parser.

### 7.3 Comparison operators

Use:

`==`, `!=`, `<`, `<=`, `>`, `>=`

Examples:

```text
if count >= 10:
    say "Full"

if name != "" and active:
    say "Ready"
```

Do not use assignment/declaration syntax as an equality test. `is` begins a variable declaration; `becomes` changes an existing variable.

### 7.4 Boolean operators

Use `and`, `or`, and `not`:

```text
if age >= 18 and has_permission:
    say "Allowed"

if not is_ready:
    say "Please wait"
```

Define short-circuit behavior and require boolean conditions. If truthy/falsy conversion is supported, document it explicitly rather than allowing inconsistent behavior.

## 8. Strings, formatting, interpolation, and conversion

String handling must be a first-class, well-tested part of TechScript.

### 8.1 String literals

Use double quotes for ordinary text:

```text
name is "Tanmoy"
message is "Welcome to TechScript"
```

Support standard escape sequences including `\\`, `\"`, `\n`, `\r`, and `\t`. Invalid escapes must generate useful diagnostics or follow a clearly documented preservation rule.

Triple-quoted multiline strings are part of the canonical specification:

```text
help_text is """TechScript is simple.
It can span multiple lines.
Keep the original line breaks."""
```

### 8.2 String interpolation

Inside a double-quoted string, `{expression}` inserts the value of an expression:

```text
name is "Tanmoy"
age is 20
say "My name is {name} and I am {age}."
say "Next year I will be {age + 1}."
```

Interpolation must support variable names, property access, list indexing, and safe expressions that the parser permits. Define escaping for literal braces: `{{` produces `{` and `}}` produces `}`. Do not evaluate arbitrary code through an unsafe string-eval mechanism; interpolation expressions must use the normal parser and execution rules.

### 8.3 Common text operations

Expose consistent text helpers through the standard library. Canonical illustrative APIs:

```text
text is "  TechScript  "
clean is text.trim()
upper is text.upper()
lower is text.lower()
parts is "red,green,blue".split(",")
joined is parts.join(" / ")
contains is text.contains("Script")
length is text.length()
```

Use the method form shown here as the canonical public API for text values. Add compatibility wrappers only if the existing public API requires them, and document those wrappers as legacy.

### 8.4 Number and value formatting

Provide one stable formatter API and interpolation format specifications. Required goals:

```text
price is 19.5
count is 12000
say "Price: {price:.2f}"
say "Count: {count:,}"
```

The canonical formatter must support these format forms:

- General text conversion.
- Whole-number and decimal formatting.
- Precision, such as `.2f`.
- Thousands grouping, such as `,`.
- Uppercase/lowercase text transforms where appropriate.
- Date/time formatting through a documented date/time API.
- Safe padding/alignment where useful.

Also provide a standard function such as `format(value, pattern)` for formatting values outside a string template. Define exact behavior and errors. Do not claim every formatting code exists until it is implemented and tested.

### 8.5 Convert values to text

Provide:

```text
text_value is to_text(123)
number_value is to_number("123")
```

Conversions must have documented failure behavior. Do not silently turn invalid text into zero. Prefer a result/error or a clear exception.

## 9. Lists, maps, objects, and structured data

### 9.1 Lists

Use square brackets:

```text
colors is ["cyan", "indigo", "violet"]
first_color is colors[0]
colors.add("blue")
colors.remove("cyan")
```

Indices are zero-based unless the repository has a demonstrated, documented compatibility requirement that makes that impossible. Index-out-of-range behavior must be consistent and tested.

Iteration:

```text
for color in colors:
    say color
```

### 9.2 Object/map literals

Use a compact object/map literal with named keys and commas:

```text
person is {
    name: "Tanmoy",
    age: 20,
    active: true
}

say person.name
person.age becomes 21
```

The parser must distinguish an object literal from a block by context. Object literals can contain nested lists and objects:

```text
profile is {
    name: "Tanmoy",
    links: ["https://example.com", "https://github.com"],
    preferences: {
        theme: "dark",
        motion: true
    }
}
```

Define duplicate-key behavior, key types, lookup failure behavior, mutability, and property assignment. Do not conflate a dynamic map with a statically defined record/type.

### 9.3 Named types and records

Support clear user-defined structured types as the language matures. Target syntax:

```text
type Person:
    name: text
    age: whole

person is Person(name: "Tanmoy", age: 20)
say person.name
```

If existing compiler architecture cannot implement type declarations in the first migration, keep the syntax in the versioned specification with a clear `planned` status and prioritize implementing it. Do not advertise it as working until tests pass. The eventual syntax must remain consistent with indentation blocks and the `is` declaration rule.

### 9.4 Formatting objects and collections

Provide stable ways to inspect and serialize values:

```text
person is {name: "Tanmoy", age: 20}
say inspect(person)
json_text is to_json(person, pretty: true)
copy is from_json(json_text)
```

Requirements:

- `inspect(value)` produces a useful developer-readable representation.
- `to_json(value, pretty: true)` serializes supported values to valid JSON.
- `from_json(text)` parses valid JSON into TechScript values.
- Define handling for unsupported values, cycles, invalid JSON, keys, Unicode, numbers, and errors.
- Stable ordering of object keys should be documented if promised.
- Separate debug inspection from JSON serialization; do not present debug output as valid JSON.

## 10. Conditions and branching

Canonical form:

```text
if score >= 90:
    say "Excellent"
else if score >= 70:
    say "Good"
else:
    say "Keep practising"
```

The parser must support the literal two-word sequence `else if` as a branch in the same conditional chain. Conditions are boolean expressions. Use this exact source form in all documentation and tests.

## 11. Repetition and loops

### 11.1 Repeat a fixed number of times

```text
repeat 3 times:
    say "Hello"
```

### 11.2 Iterate through a collection

```text
for item in ["one", "two", "three"]:
    say item
```

### 11.3 Repeat over an inclusive numeric range

```text
repeat index from 1 to 5:
    say "Item {index}"
```

Define that this form includes both endpoints. If a descending range is supported, define its step and termination behavior. A zero or negative repeat count must have an explicit, tested rule.

### 11.4 While loop

```text
while count < 5:
    say count
    count becomes count + 1
```

### 11.5 Loop controls

Use `stop` to exit the nearest loop and `skip` to proceed to its next iteration. If nested-loop labels or labelled breaks are needed later, specify them separately. Prevent infinite-loop tests from hanging the test runner.

## 12. Functions and returns

### 12.1 Function declaration

Use `to`:

```text
to greet(person):
    return "Hello, {person}!"

to add(first, second):
    return first + second

say greet("Tanmoy")
say add(2, 3)
```

No `fn`, `function`, or braces are required for a normal function body. The function name and parameters must follow the same naming rules as other code.

### 12.2 Optional parameter and return types

```text
to add(first: number, second: number) -> number:
    return first + second
```

The `-> type` form is the only canonical optional return-type annotation. Implement it consistently in the parser, type checker, documentation, and tests.

### 12.3 Default and named arguments

Target default parameter syntax:

```text
to greet(person = "friend"):
    say "Hello, {person}!"

greet()
greet("Tanmoy")
```

Target named call syntax:

```text
greet(person: "Tanmoy")
```

Define the relationship between `:` in named arguments and `:` beginning blocks so the parser remains unambiguous. Implement optional/default/named arguments according to the repository's capabilities; mark unsupported features clearly until done.

### 12.4 Scope and closures

Define local, module-level, and nested-function scope. If closures are supported, define capture and lifetime behavior. Avoid accidental global variables and silent creation of unknown names.

## 13. Modules and imports

Use `use` to import a module:

```text
use math
use text_tools

root is math.sqrt(81)
say root
```

Selective imports may use:

```text
use math as m
say m.sqrt(81)
```

Prefer qualified module names over hidden global imports. Define how files map to modules, how package paths resolve, how dependency versions are managed, and how import cycles are handled.

A module imported by `.txs` code must contain actual executable code or native bindings. Do not silently stub missing imports.

## 14. Errors and safe failure handling

Canonical target form:

```text
try:
    contents is files.read_text("notes.txt")
    say contents
catch error:
    say "Could not read the file: {error}"
finally:
    say "Finished"
```

Use `raise` to signal an error where appropriate:

```text
if age < 0:
    raise "Age cannot be negative"
```

Define error values, propagation, catching rules, stack traces, and the difference between recoverable errors and process-terminating failures. The implementation agent must align the exact form with the current parser and document any required migration. Never swallow errors silently.

## 15. Async and concurrency

Target syntax for supported asynchronous operations:

```text
async to load_profile(user_id):
    response is await http.get("https://example.com/users/{user_id}")
    return response.json()
```

Concurrency requires a real runtime design. Do not implement `async` and `await` as syntax-only placeholders. Define cancellation, error propagation, task scheduling, shared state, and platform constraints before marking them stable.

For native Android APIs, permissions, lifecycle callbacks, and event-driven UI, expose documented library interfaces backed by actual Rust/C/C++ native integration. Do not hardcode these operations into the language parser.

## 16. Standard library design

The standard library must be consistent, documented, testable, and general-purpose. Organize it in modules rather than exposing hundreds of unrelated global functions.

Target module families:

- `text`: formatting, splitting, joining, trimming, matching, Unicode-safe operations.
- `math`: common arithmetic and mathematical functions.
- `collections`: lists, maps, sets, sorting, filtering, mapping, searching.
- `json`: parse and encode JSON.
- `files`: read/write text and bytes, list paths, metadata.
- `path`: safe path manipulation.
- `time`: dates, times, durations, formatting.
- `http`: HTTP requests and responses when networking is available.
- `system`: environment, arguments, process information where allowed.
- `random`: seeded and system randomness with clear semantics.
- `log`: structured logging with levels.
- `test`: assertions and test helpers.
- `graphics`: reusable graphics primitives when a real graphics backend exists.

Provide canonical methods or functions for each capability and keep naming consistent. Clearly separate platform-dependent functionality from portable core functionality.

### 16.1 Common collection operations

The standard library should support reusable transformations such as map, filter, sort, and search, with clear typing and error behavior. Example target style:

```text
numbers is [1, 2, 3, 4, 5]
evens is numbers.filter(item => item % 2 == 0)
doubled is numbers.map(item => item * 2)
```

The canonical anonymous-function form is `parameter => expression` for one parameter and `(first, second) => expression` for multiple parameters. Implement these forms in the parser and runtime. Do not introduce a second lambda syntax.

### 16.2 Formatter requirements

Provide documented formatting for:

- Plain text and Unicode.
- Numeric precision and grouping.
- Lists and objects for developer inspection.
- Valid JSON serialization and parsing.
- Dates, times, and durations when the time module is supported.
- Errors and diagnostic messages.

The formatter must not execute arbitrary code or silently discard invalid values.

## 17. Tests and test-runner syntax

Provide a repeatable test workflow. Target test syntax:

```text
test "addition works":
    expect add(2, 3) == 5

test "text interpolation works":
    name is "TechScript"
    expect "Hello, {name}!" == "Hello, TechScript!"
```

If `test` and `expect` are implemented as library APIs rather than language constructs, that is acceptable only if this exact source is supported and documented. Tests must actually execute and fail when an expectation fails.

---

# PART III — FRAMEWORKS BUILT ON THE SAME LANGUAGE

## 18. Frameworks must not redefine TechScript

Webforge and AppForge are libraries/frameworks on top of TechScript. They may provide domain-specific builder APIs, components, route declarations, lifecycle hooks, and rendering functionality. They must not change the core meaning of variables, functions, loops, strings, types, modules, or errors.

A Webforge program and an AppForge program must share the same general language syntax. Only their imported framework APIs and host capabilities should differ.

Do not add special compiler keywords for `button`, `glass`, `3d`, `route`, `page`, or every other framework feature. These names belong to framework libraries and DSL APIs, not the core grammar, unless a minimal generic block mechanism requires compiler support.

### 18.1 Generic builder blocks

If framework DSLs need indented child blocks, implement one general, well-defined trailing-block or builder mechanism rather than a parser branch for every UI component. Define the mechanism, its AST representation, evaluation order, and error diagnostics. Frameworks should be able to create new components without editing the compiler.

The following framework examples specify the canonical target public style. Implement the framework APIs to support them and test them end-to-end. Do not report them working until they build and run.

## 19. Webforge: target example

```text
use webforge

site("Tcode Motion"):
    route("/"):
        page(title: "Home"):
            column():
                heading("Build ideas with TechScript")
                text("One language, simple syntax.")
                button("Explore examples"):
                    on("click"):
                        navigate("/examples")

    route("/about"):
        page(title: "About"):
            heading("About this project")
            text("A website built with Webforge and TechScript.")
```

Webforge capabilities should be supplied by real libraries and compiler/runtime-supported builders:

- Routes and navigation.
- Page layouts and reusable components.
- Forms and validation.
- Static assets, fonts, images, and icons.
- Responsive design and accessibility.
- Network/data integration.
- Animation and optional graphics effects.
- SEO, metadata, deployment, and build output where supported.

The same effects must be implementable using reusable style and graphics APIs. Liquid glass is a library/example, not a language primitive.

## 20. AppForge: target example

```text
use appforge

app("Counter"):
    count is state(0)

    screen("Home"):
        column():
            heading("Counter")
            text("Count: {count}")

            button("Add one"):
                on("click"):
                    count becomes count + 1

            button("Reset"):
                on("click"):
                    count becomes 0
```

AppForge must provide actual native UI, layout, state, event, navigation, lifecycle, storage, platform APIs, accessibility, and graphics integrations. The framework must render the `.txs` program's real declarative output. It must not select a hardcoded Rust screen regardless of the source code.

For Android, preserve the native Rust/C/C++ execution model where appropriate. The target is zero manually authored Java/Kotlin application logic. Do not claim that Android platform integration is entirely eliminated if a platform limitation remains; report actual requirements precisely.

## 21. Libraries and custom visual effects

Developers must be able to create reusable libraries rather than asking the compiler to recognize every new visual effect.

For example, a liquid-glass library should compose supported graphics primitives, colors, geometry, material settings, clipping, compositing, blur, shaders, and animation APIs. A 3D library should compose real geometry, transformations, cameras, materials, lighting, and rendering APIs.

Core-language examples should remain ordinary TechScript code. For example:

```text
use graphics
use effects.glass

panel_style is glass.create(
    tint: "indigo",
    opacity: 0.12,
    blur: 18,
    border: 0.5
)
```

This is the target library API. Implement it as reusable library code backed by native graphics APIs. The compiler must not hardcode the visual result. Developers must be able to replace the library, build an entirely different visual style, or use no UI framework at all.

---

# PART IV — CANONICAL COMPLETE EXAMPLES

## 22. Hello world

```text
say "Hello, world!"
```

## 23. Variables and reassignment

```text
name is "TechScript"
version is 3
say "Welcome to {name} {version}"
version becomes version + 1
say "Next version number: {version}"
```

## 24. Functions

```text
to greet(person):
    return "Hello, {person}!"

say greet("Tanmoy")
say greet("TechScript user")
```

## 25. Conditions and repetition

```text
score is 85

if score >= 90:
    say "Excellent"
else if score >= 70:
    say "Good work"
else:
    say "Keep learning"

repeat 3 times:
    say "Practise a little every day"
```

## 26. Lists and objects

```text
person is {
    name: "Tanmoy",
    age: 20,
    skills: ["TechScript", "Rust", "AppForge"]
}

say "Name: {person.name}"
say "First skill: {person.skills[0]}"
person.age becomes person.age + 1
say inspect(person)
```

## 27. Data processing

```text
numbers is [1, 2, 3, 4, 5]
total is 0

for number in numbers:
    total becomes total + number

say "Total: {total}"
```

## 28. Formatting and JSON

```text
price is 1234.5
say "Price: {price:,.2f}"

user is {
    name: "Tanmoy",
    active: true,
    score: 99.5
}

json_text is to_json(user, pretty: true)
say json_text
user_again is from_json(json_text)
say "User: {user_again.name}"
```

The exact formatting and JSON APIs must be implemented and tested; if a formatting spec is unsupported, do not include it in stable documentation until it works.

## 29. Error handling

```text
try:
    data is files.read_text("settings.txt")
    say data
catch error:
    say "Could not read settings: {error}"
```

## 30. Modules

```text
use math
use json

value is math.sqrt(144)
say value
```

## 31. Native app example

```text
use appforge

app("Greeting App"):
    screen("Home"):
        column():
            heading("Hello")
            text("Built using TechScript")
            button("Continue"):
                on("click"):
                    navigate("/next")
```

## 32. Website example

```text
use webforge

site("Starter"):
    route("/"):
        page(title: "Welcome"):
            heading("My first TechScript website")
            text("This page is built with Webforge.")
```

---

# PART V — IMPLEMENTATION PLAN FOR THE CODING AI

## 33. Audit the existing repository before changing it

Repository root:

`C:\Users\Tanmoy\OneDrive\Documents\techscript`

Inspect and preserve the existing Git history and unrelated user changes. Determine how the current lexer, parser, AST, semantic analysis, interpreter, VM, standard library, CLI, and framework integrations actually work. Do not assume any crate or feature works solely because it exists.

Produce a concise capability matrix with these states:

- Implemented and tested.
- Implemented but not adequately tested.
- Partially implemented.
- Documented but missing.
- Broken.
- Not implemented.

## 34. Reconcile the spec and implementation

Treat this document as the canonical target. If an existing, verified design conflicts with this document, do not silently combine the two or invent a third syntax. Identify the conflict, assess the cost and compatibility implications, and implement the defined canonical form unless doing so would demonstrably break a critical, working feature. If a change is required, update this document, its examples, grammar, tests, and memory files together in the same change.

Do not keep old and new syntax both as canonical. If a migration compatibility mode is required, label it explicitly as legacy, document its removal path, and never use legacy syntax in new examples.

## 35. Implement in small verified phases

Recommended order:

1. Canonical grammar and parser fixtures.
2. Variables, constants, reassignment, literals, and expression precedence.
3. Strings, interpolation, collections, object literals, and formatting.
4. Conditions, loops, functions, return values, and scope.
5. Modules, errors, standard-library contracts, and CLI diagnostics.
6. Interpreter/VM parity and regression suite.
7. Formatter, linting, and beginner examples.
8. Generic framework builder mechanism, only if necessary for Webforge/AppForge syntax.
9. Webforge and AppForge integration examples driven by actual `.txs` code.
10. Repository documentation and agent-rule enforcement.

Run relevant tests after each phase. Avoid changing unrelated UI or applications during core syntax work.

## 36. Required regression tests

Positive tests must compile and run every canonical example in this document once the feature is marked implemented. Negative tests must verify clear errors for:

- Redeclaring a variable where disallowed.
- Reassigning an unknown or constant value.
- Invalid indentation and missing block bodies.
- Missing function arguments or wrong types when applicable.
- Invalid interpolation expressions.
- Invalid object/map entries and duplicate keys according to the chosen rule.
- Unsupported operators or syntax.
- Missing modules and invalid standard-library calls.

Where both an interpreter and VM exist, ensure supported programs yield matching output and errors. Do not make the VM pass by hardcoding example outputs.

## 37. Create permanent project rules and syntax memory

Inspect the repository's existing instructions first. Update existing files rather than creating contradictory instruction layers.

Create or update repository-level `AGENTS.md` and, where supported by this workspace, the appropriate Antigravity/Gemini project rules. Include these mandatory rules:

1. Always read the canonical TechScript syntax specification before writing `.txs` code.
2. Always use the exact syntax defined in this specification.
3. Never invent syntax based on Python, Kotlin, C, Rust, Go, or any other language.
4. Never reintroduce `let`, `fn`, braces for ordinary blocks, or other legacy forms as canonical syntax.
5. Every syntax change requires an update to the specification, grammar, compiler, tests, examples, and syntax memory in the same task.
6. Test actual `.txs` programs with the real compiler/runtime; never rely only on static code inspection.
7. Do not document a feature as implemented until tests pass.
8. Keep the language core general-purpose. UI effects and framework capabilities belong in libraries/frameworks, not one-off compiler keywords.
9. Do not work on AppForge visual polish or unrelated framework enhancements until core syntax and runtime milestones are completed.
10. If a conflict or missing feature is found, report it and fix it deliberately. Do not silently change the canonical syntax.

Create `docs/language/TECHSCRIPT_SYNTAX_MEMORY.md` or update the repository's existing equivalent. It must record the canonical variable declaration, reassignment, function, block, condition, loop, interpolation, object, module, and error syntax; authoritative spec path; implemented status; tested examples; legacy forms; and instructions for future modifications.

Memory must be a version-controlled file, not just a claim that the AI remembers it. Add an automated documentation consistency check and syntax regression tests.

## 38. Deliverables

Create/update the following artifacts, adapting paths to the existing repository structure:

- One authoritative TechScript language specification.
- A grammar or equivalent parser specification.
- Lexer/parser/semantic/runtime implementation of the canonical syntax.
- Executable `.txs` examples for every stable feature.
- Standard-library reference for text, formatting, objects, collections, JSON, and other implemented modules.
- Webforge/AppForge integration examples that use common TechScript syntax.
- Automated positive and negative syntax tests.
- Updated `AGENTS.md` and relevant Antigravity rules.
- Persistent syntax memory file.
- Migration guidance from old syntax.

Do not create duplicate competing specifications. If equivalent files already exist, update and consolidate them instead.

## 39. Final report requirements

At completion, report:

- The exact files created and changed.
- The canonical syntax that is now supported.
- Which example files were compiled and executed.
- Test commands actually run and results.
- Interpreter/VM parity results, if applicable.
- Implemented, partial, and planned features.
- Any remaining grammar or compatibility risks.
- Confirmation that the permanent agent rules and syntax memory were written.

Never claim the work is complete if only documentation changed. Never claim examples pass unless they were executed. Never hide a mismatch between documented syntax and the compiler.

---

# FINAL ACCEPTANCE CONDITION

TechScript has one short, consistent, human-readable canonical syntax; a real compiler/runtime that implements it; clear string, object, list, and value formatting; a useful standard library; real runnable examples; and permanent repository-level rules that stop future AI sessions from inventing a different syntax. Webforge and AppForge use the same core language and remain separate frameworks. Core-language work is completed and verified before unrelated application redesign begins.
