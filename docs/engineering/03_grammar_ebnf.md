# 03 — TechScript 2.0 EBNF Grammar

> **Status**: Authoritative Specification — FROZEN 2.0.0
> **Version**: 2.0.0 Stable
> **Last Updated**: 2026-07-26
> **Related Documents**: [01 Language Spec](./01_language_spec_v1.md) · [05 AST Design](./05_ast_design.md) · [07 Parser Design](./07_parser_design.md)

---

## Notation Conventions

| Notation | Meaning |
|---|---|
| `=` | Definition |
| `;` | End of production |
| `\|` | Alternative |
| `( )` | Grouping |
| `[ ]` | Optional (0 or 1) |
| `{ }` | Repetition (0 or more) |
| `" "` | Terminal string |
| `UPPER_CASE` | Terminal token (from lexer) |
| `lower_case` | Non-terminal (grammar rule) |
| `(* *)` | Comment |

---

## 1. Program Structure

```ebnf
program = { top_level_statement } EOF ;

top_level_statement = use_statement
                    | const_declaration
                    | function_declaration
                    | class_declaration
                    | struct_declaration
                    | enum_declaration
                    | statement ;

(* Blocks use indentation + 'end', never curly braces *)
block = NEWLINE INDENT { statement } DEDENT "end" NEWLINE ;
```

---

## 2. Import / Module System

```ebnf
use_statement = "use" module_path NEWLINE ;

module_path = IDENTIFIER { "." IDENTIFIER } ;
```

**Examples:**
```txs
use math
use http
use json
```

*Legacy aliases (deprecated, emit TSW1009):*
```
import math              -> use math
from json import parse   -> use json
```

---

## 3. Declarations

```ebnf
declaration = const_declaration
            | function_declaration
            | class_declaration
            | struct_declaration
            | enum_declaration ;

(* Constants — immutable, must be top-level or at block scope *)
const_declaration = "const" IDENTIFIER "=" expression NEWLINE ;

(* Functions — 'do' is the canonical keyword *)
function_declaration = [ "async" ] "do" IDENTIFIER "(" [ param_list ] ")" block ;

param_list = param { "," param } ;
param      = IDENTIFIER [ ":" type_expr ] [ "=" expression ] ;

(* Typed parameter — explicit form (type annotation optional, v2.0) *)
typed_param = IDENTIFIER ":" type_expr [ "=" expression ] ;

(* Lambdas — single-line and multi-line forms *)
lambda_expression = "do" "(" [ param_list ] ")" "->" expression         (* single-line *)
                  | "do" "(" [ param_list ] ")" block ;                 (* multi-line  *)

(* Classes *)
class_declaration = "class" IDENTIFIER [ "(" IDENTIFIER ")" ] NEWLINE
                    { class_member }
                    "end" NEWLINE ;

class_member = IDENTIFIER "=" expression NEWLINE    (* field *)
             | function_declaration ;                (* method *)

(* Structs *)
struct_declaration = "struct" IDENTIFIER NEWLINE
                     { IDENTIFIER [ ":" type_expr ] NEWLINE }
                     "end" NEWLINE ;

(* Enums *)
enum_declaration = "enum" IDENTIFIER NEWLINE
                   { IDENTIFIER [ "=" expression ] NEWLINE }
                   "end" NEWLINE ;

(* Exports *)
export_declaration = "export" ( function_declaration
                               | const_declaration
                               | class_declaration ) ;
```

---

## 4. Statements

```ebnf
statement = assignment_statement
          | expression_statement
          | send_statement
          | say_statement
          | throw_statement
          | break_statement
          | continue_statement
          | when_statement
          | for_statement
          | loop_statement
          | repeat_statement
          | try_statement
          | match_statement
          | async_statement
          | parallel_statement
          | raw_block
          | dsl_block ;

(* Variable assignment — first assignment declares the variable *)
assignment_statement = assignment_target assignment_operator expression NEWLINE ;

assignment_target    = IDENTIFIER
                     | member_access
                     | index_access ;

assignment_operator  = "=" | "+=" | "-=" | "*=" | "/=" | "%=" ;

expression_statement = expression NEWLINE ;

(* Return from function *)
send_statement = "send" [ expression ] NEWLINE ;

(* Print to stdout *)
say_statement  = "say" expression NEWLINE ;

throw_statement    = "throw" expression NEWLINE ;
break_statement    = "break" NEWLINE ;
continue_statement = "continue" NEWLINE ;
```

---

## 5. Control Flow

```ebnf
(* Conditional — 'when' is canonical, 'if'/'elif' are deprecated aliases *)
when_statement = "when" expression block
                 { "else" "when" expression block }
                 [ "else" block ] ;

(* For-each iteration — 'each' is a deprecated alias *)
for_statement = "for" IDENTIFIER "in" expression block ;

(* Count loop: runs exactly N times — no deprecated alias *)
loop_statement = "loop" expression block ;

(* Condition loop: repeats while condition is true — 'while' is deprecated alias *)
repeat_statement = "repeat" expression block ;

(* Error handling — 'attempt' is a deprecated alias for 'try' *)
try_statement = "try" block "catch" IDENTIFIER block
                [ "finally" block ] ;

(* Pattern matching — 'switch' is a deprecated alias *)
match_statement = "match" expression NEWLINE
                  { "case" expression block }
                  [ "default" block ]
                  "end" NEWLINE ;
```

**Examples:**
```txs
when x > 5
    say "big"
else when x > 0
    say "small"
else
    say "zero"
end

for item in list
    say item
end

loop 10
    say "Hello"
end

repeat running
    update()
end

try
    data = file.read("config.txt")
catch error
    say "File not found"
end

match status
case "ok"
    say "success"
case "error"
    say "failed"
default
    say "unknown"
end
```

---

## 6. Async & Parallel

```ebnf
async_statement    = "async" block ;

await_expression   = "await" expression ;

parallel_statement = "parallel" block ;
```

**Examples:**
```txs
async
    result = await http.get("https://example.com")
    say result
end

parallel
    task1()
    task2()
    task3()
end
```

---

## 7. Raw Escape Hatch

```ebnf
raw_block = "raw" NEWLINE { RAW_LINE } "end" NEWLINE ;

RAW_LINE = ANY_CONTENT_EXCEPT_END NEWLINE ;
```

Used to embed content that must not be interpreted by the TechScript parser.

---

## 8. DSL Blocks

Every DSL module (`web`, `canvas`, `gui`, `game`, `ai`, etc.) uses the same grammar:

```ebnf
(* Uniform DSL grammar — frozen rule *)
dsl_block    = IDENTIFIER [ STRING_LITERAL ] NEWLINE
               { dsl_member }
               "end" NEWLINE ;

dsl_member   = dsl_property
             | dsl_block ;           (* nesting is recursive *)

dsl_property = IDENTIFIER { expression } NEWLINE ;

(* DSL terminator keywords (module-specific) *)
dsl_start    = "start" NEWLINE      (* web *)
             | "show" NEWLINE       (* gui *)
             | "run" NEWLINE        (* game *)
             | "export" NEWLINE ;   (* canvas, graphics *)
```

**Examples — uniform across all modules:**
```txs
use web

page "/"
    title "My App"
    hero
        heading "Welcome"
        subtitle "TechScript 2.0"
        button "Get Started"
    end
end

start
```

```txs
use canvas

logo
    size 512 512
    rings 3
    core "#4A90E2"
    title "TechScript"
    export "logo.png"
end
```

```txs
use gui

window
    title "My App"
    size 800 600
    button "OK"
    textbox placeholder "Enter text"
end

show
```

---

## 9. Expressions

### 9.1 Precedence (lowest to highest)

| Level | Operators | Associativity | Notes |
|---|---|---|---|
| 0 | `do(...) ->` | — | Lambda / closure (lowest) |
| 1 | `=` `+=` `-=` `*=` `/=` `%=` | Right | Assignment |
| 2 | `or` | Left | Logical OR |
| 3 | `and` | Left | Logical AND |
| 4 | `not` | Right (unary) | Logical NOT |
| 5 | `==` `!=` `<` `>` `<=` `>=` `is` | Left, non-associative | Comparison |
| 6 | `..` `..=` | Left | Range |
| 7 | `\|` `&` `^` | Left | Bitwise OR / AND / XOR |
| 8 | `+` `-` | Left | Additive |
| 9 | `*` `/` `//` `%` | Left | Multiplicative |
| 10 | `**` | Right | Exponentiation |
| 11 | `-` `not` (unary) | Right | Unary prefix |
| 12 | `()` `.` `[]` | Left | Postfix / call / member / index |

```ebnf
(* expression admits lambda at top level as well as assignment *)
expression = lambda_expression
           | assignment_expression ;

assignment_expression = [ assignment_target assignment_operator ] or_expression ;
assignment_operator   = "=" | "+=" | "-=" | "*=" | "/=" | "%=" ;

or_expression   = and_expression { "or" and_expression } ;
and_expression  = not_expression { "and" not_expression } ;
not_expression  = "not" not_expression | comparison_expression ;

comparison_expression = range_expression { cmp_op range_expression } ;
cmp_op = "==" | "!=" | "<" | ">" | "<=" | ">=" | "is" ;

range_expression = bitwise_expression [ ( ".." | "..=" ) bitwise_expression ] ;

bitwise_expression = additive_expression { ( "|" | "&" | "^" ) additive_expression } ;

additive_expression = multiplicative_expression { ( "+" | "-" ) multiplicative_expression } ;

multiplicative_expression = exponent_expression { ( "*" | "/" | "//" | "%" ) exponent_expression } ;

(* "**" is the canonical exponentiation operator — right-associative *)
exponent_expression = unary_expression [ "**" exponent_expression ] ;

unary_expression = ( "-" | "not" ) unary_expression
                 | postfix_expression ;

postfix_expression = primary { postfix_op } ;

postfix_op = call_op
           | member_access
           | index_op ;

call_op       = "(" [ arg_list ] ")" ;
arg_list      = expression { "," expression } ;
member_access = "." IDENTIFIER ;
index_op      = "[" expression "]" ;
```

### 9.2 Implicit Built-in Calls

The following built-ins are called without parentheses:

```ebnf
implicit_call = ( "say" | "ask" | "env" | "file" | "panic" ) expression ;
```

All other calls — including all stdlib module calls — require `module.function(args)`.

### 9.3 Primary Expressions

```ebnf
primary = INT_LITERAL
        | FLOAT_LITERAL
        | STRING_LITERAL
        | DSTRING_LITERAL          (* $"Hello {name}" — canonical interpolation *)
        | "true"
        | "false"
        | "null"
        | IDENTIFIER
        | list_literal
        | map_literal
        | tuple_literal
        | grouped_expression
        | new_expression
        | await_expression
        | lambda_expression ;

list_literal    = "[" [ expression { "," expression } [ "," ] ] "]" ;

map_literal     = "{" [ map_entry { "," map_entry } [ "," ] ] "}" ;
map_entry       = expression ":" expression ;

tuple_literal   = "(" expression "," expression { "," expression } ")" ;

grouped_expression = "(" expression ")" ;

new_expression  = "new" IDENTIFIER "(" [ arg_list ] ")" ;
```

---

## 10. Lexical Grammar

### 10.1 Keywords (Canonical — Frozen 2.0.0)

```
do        send      when      else      for       in
loop      repeat    match     case      default   try
catch     throw     finally   async     await     parallel
class     struct    enum      trait     interface const
use       new       export    end       raw       break
continue  say       ask       true      false     null
self      is        and       or        not       typeof
```

### 10.2 Deprecated Aliases (emit TSW100x warning)

```
build   -> do        make    -> (assignment)   let     -> (assignment)
var     -> (assignment)       return  -> send          give    -> send
attempt -> try       if      -> when           elif    -> else when
while   -> repeat    each    -> for            import  -> use
from    -> use       model   -> class          fun     -> do
fn      -> do        function -> do            keep    -> const
stop    -> break     skip    -> continue       none    -> null
```

### 10.3 Identifiers

```ebnf
IDENTIFIER   = IDENT_START { IDENT_CONTINUE } ;
IDENT_START  = LETTER | "_" ;
IDENT_CONTINUE = LETTER | DIGIT | "_" ;
LETTER       = "a".."z" | "A".."Z" | UNICODE_LETTER ;
```

### 10.4 Numeric Literals

```ebnf
INT_LITERAL  = DECIMAL_INT | HEX_INT | BINARY_INT | OCTAL_INT ;
DECIMAL_INT  = DIGIT { DIGIT | "_" } ;
HEX_INT      = "0" ( "x" | "X" ) HEX_DIGIT { HEX_DIGIT | "_" } ;
BINARY_INT   = "0" ( "b" | "B" ) BIN_DIGIT { BIN_DIGIT | "_" } ;
OCTAL_INT    = "0" ( "o" | "O" ) OCT_DIGIT { OCT_DIGIT | "_" } ;
FLOAT_LITERAL = DIGIT { DIGIT | "_" } "." DIGIT { DIGIT | "_" } [ EXPONENT ]
              | DIGIT { DIGIT | "_" } EXPONENT ;
EXPONENT     = ( "e" | "E" ) [ "+" | "-" ] DIGIT { DIGIT } ;
```

### 10.5 String Literals

```ebnf
STRING_LITERAL  = '"' { STRING_CHAR } '"' ;

(* Canonical interpolated string — $"..." prefix *)
DSTRING_LITERAL = '$"' { DSTRING_PART } '"' ;
DSTRING_PART    = STRING_CHAR | "{" expression "}" ;

(* Deprecated alias — f"..." emits TSW1012 *)
FSTRING_LITERAL = 'f"' { FSTRING_PART } '"' ;
FSTRING_PART    = STRING_CHAR | "{" expression "}" ;

STRING_CHAR     = ESCAPE_SEQUENCE | ANY_CHAR_EXCEPT_QUOTE ;
ESCAPE_SEQUENCE = "\" ( "n" | "t" | "\" | '"' | "r" | "0" | UNICODE_ESCAPE ) ;
UNICODE_ESCAPE  = "u" "{" HEX_DIGIT { HEX_DIGIT } "}" ;
```

> **NOTE:** `$"Hello {name}"` is the **canonical** interpolation syntax in TechScript 2.0.
> The `f"..."` prefix is a **deprecated alias** and emits warning **TSW1012** at compile time.
> Use `tsc migrate` to auto-convert all `f"..."` occurrences to `$"..."`.

### 10.6 Comments

```ebnf
LINE_COMMENT  = "#" { ANY_CHAR_EXCEPT_NEWLINE } NEWLINE ;
BLOCK_COMMENT = "/*" { BLOCK_COMMENT_BODY } "*/" ;
```

> **NOTE:** `//` line comments are not part of the TechScript grammar. Only `#` is canonical.
> `/* */` block comments are parsed but emit **TSW1007** — prefer `#` comments per style guide.

### 10.7 Whitespace & Terminators

```ebnf
WHITESPACE = " " | "\t" | "\r" ;
NEWLINE    = "\n" | "\r\n" ;
```

Statements are terminated by newlines. Semicolons `;` are accepted but deprecated (TSW1006).

---

## 11. Type Annotations (Optional — v2.0)

Type annotations are optional and do not affect runtime behavior in 2.0.
They are parsed and stored in the AST for future static analysis (v2.2+).

```ebnf
type_expr = simple_type
          | generic_type
          | function_type
          | tuple_type ;

simple_type   = IDENTIFIER ;                                    (* Int, String, Bool *)
generic_type  = IDENTIFIER "[" type_expr { "," type_expr } "]" ; (* List[Int] *)
function_type = "(" type_expr { "," type_expr } ")" "->" type_expr ;
tuple_type    = "(" type_expr "," type_expr { "," type_expr } ")" ;
```

**Usage in declarations:**
```txs
# Typed parameters
do add(x: Int, y: Int)
    send x + y
end

# Typed constant
const PI = 3.14159

# Struct with types
struct Point
    x: Float
    y: Float
end
```

---

## 12. Compatibility & Evolution

### 12.1 Compatibility Rules

- All deprecated keywords parse successfully and emit `TSW100x` warnings.
- No deprecated keyword is a hard error in the 2.x series.
- `tsc migrate` converts deprecated syntax to canonical automatically.
- Legacy code compiles identically to canonical code — no behavior change.

### 12.2 Freeze Declaration

> **TechScript 2.0 syntax is frozen as of 2026-07-20.**
> No syntax changes are permitted in the 2.x series without a TechScript 3.0 major version.
> All new modules (`web`, `canvas`, `gui`, `game`, `mobile`, `3d`, `ai`, etc.)
> must implement the uniform DSL grammar defined in section 8.

### 12.3 Future Roadmap

- **v2.2**: Static type checking based on optional annotations.
- **v2.5**: Pattern matching with destructuring.
- **v3.0**: Native codegen; possible syntax additions under new major version.

---

## 13. Deprecation Notes

All deprecated syntax is accepted by the compiler and silently migrated at runtime, but emits
a compile-time warning with the TSW code listed below. Use `tsc migrate` to auto-convert an
entire project.

### 13.1 Complete Deprecation Table

| Deprecated Syntax | Canonical Replacement | TSW Code | Notes |
|---|---|---|---|
| `build fn_name(...)` | `do fn_name(...)` | TSW1001 | Old function keyword |
| `fun fn_name(...)` | `do fn_name(...)` | TSW1001 | Short alias |
| `fn fn_name(...)` | `do fn_name(...)` | TSW1001 | Short alias |
| `function fn_name(...)` | `do fn_name(...)` | TSW1001 | Long alias |
| `return expr` | `send expr` | TSW1002 | Return statement |
| `give expr` | `send expr` | TSW1002 | Return alias |
| `model ClassName` | `class ClassName` | TSW1003 | Class keyword |
| `if cond` | `when cond` | TSW1004 | Conditional |
| `elif cond` | `else when cond` | TSW1004 | Else-if branch |
| `while cond` | `repeat cond` | TSW1005 | While loop |
| `;` (semicolons) | *(newline terminator)* | TSW1006 | Statement terminator |
| `// comment` | `# comment` | TSW1007 | Line comment style |
| `/* comment */` | `# comment` | TSW1007 | Block comment style |
| `import mod` | `use mod` | TSW1009 | Module import |
| `from mod import x` | `use mod` | TSW1009 | Selective import |
| `each x in y` | `for x in y` | TSW1010 | For-each keyword |
| `keep X = val` | `const X = val` | TSW1011 | Constant declaration |
| `f"Hello {name}"` | `$"Hello {name}"` | TSW1012 | String interpolation prefix |
| `let x = val` | `x = val` | TSW1013 | Variable declaration |
| `var x = val` | `x = val` | TSW1013 | Variable declaration |
| `make x = val` | `x = val` | TSW1013 | Variable declaration |
| `attempt` | `try` | TSW1014 | Error handling keyword |
| `none` | `null` | TSW1015 | Null literal |
| `stop` | `break` | TSW1016 | Loop break |
| `skip` | `continue` | TSW1017 | Loop continue |
| `switch expr` | `match expr` | TSW1018 | Pattern matching |

> **WARNING:** TSW codes TSW1001-TSW1018 are warnings, **not errors**, in TechScript 2.x.
> They become **hard errors** in TechScript 3.0. Migrate early using `tsc migrate`.

### 13.2 Migration Tool

```
tsc migrate ./src            # migrate all .txs files in ./src
tsc migrate ./src/app.txs    # migrate a single file
tsc migrate --dry-run ./src  # preview changes without writing
```

### 13.3 TSW Code Reference Summary

| Range | Category |
|---|---|
| TSW1001 | Function keyword variants (`build`, `fun`, `fn`, `function`) |
| TSW1002 | Return keyword variants (`return`, `give`) |
| TSW1003 | Class keyword variants (`model`) |
| TSW1004 | Conditional keyword variants (`if`, `elif`) |
| TSW1005 | Loop keyword variants (`while`) |
| TSW1006 | Semicolon terminator |
| TSW1007 | Comment style (`//`, `/* */`) |
| TSW1009 | Import keyword variants (`import`, `from`) |
| TSW1010 | For-each keyword variants (`each`) |
| TSW1011 | Constant keyword variants (`keep`) |
| TSW1012 | String interpolation prefix (`f"..."`) |
| TSW1013 | Variable declaration keywords (`let`, `var`, `make`) |
| TSW1014 | Error handling keyword variants (`attempt`) |
| TSW1015 | Null literal variants (`none`) |
| TSW1016 | Break keyword variants (`stop`) |
| TSW1017 | Continue keyword variants (`skip`) |
| TSW1018 | Match/switch keyword variants (`switch`) |
