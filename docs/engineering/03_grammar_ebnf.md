# 03 — TechScript 2.0 EBNF Grammar

> **Status**: Authoritative Specification
> **Version**: 2.0.0
> **Last Updated**: 2026-07-15
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
| `'  '` | Terminal string (alternate) |
| `(* *)` | Comment |
| `UPPER_CASE` | Terminal token (from lexer) |
| `lower_case` | Non-terminal (grammar rule) |

---

## 1. Program Structure

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
          | when_statement
          | each_statement
          | repeat_statement
          | while_statement
          | attempt_statement
          | import_statement
          | block ;

block = "{" { statement } "}" ;
```

---

## 2. Declarations

```ebnf
declaration = variable_declaration
            | constant_declaration
            | function_declaration
            | model_declaration
            | export_declaration ;

variable_declaration = "make" IDENTIFIER "=" expression TERMINATOR ;

constant_declaration = "const" IDENTIFIER "=" expression TERMINATOR ;

function_declaration = "build" IDENTIFIER "(" [ parameter_list ] ")" block ;

parameter_list = parameter { "," parameter } ;

parameter = IDENTIFIER [ "=" expression ] ;

model_declaration = "model" IDENTIFIER "{" { model_member } "}" ;

model_member = field_declaration
             | method_declaration ;

field_declaration = "make" IDENTIFIER "=" expression TERMINATOR ;

method_declaration = ( "build" | "fun" ) IDENTIFIER "(" [ parameter_list ] ")" block ;
(* Note: "fun" is a deprecated alias inside model declarations *)

export_declaration = "export" ( function_declaration | constant_declaration | model_declaration ) ;
```

---

## 3. Statements

```ebnf
expression_statement = expression TERMINATOR ;

assignment_statement = assignment_target assignment_operator expression TERMINATOR ;

assignment_target = IDENTIFIER
                  | member_access
                  | index_access ;

assignment_operator = "=" | "+=" | "-=" | "*=" | "/=" | "%=" ;

say_statement = "say" expression TERMINATOR ;

return_statement = "return" [ expression ] TERMINATOR ;

throw_statement = "throw" expression TERMINATOR ;

break_statement = "break" TERMINATOR ;

continue_statement = "continue" TERMINATOR ;

TERMINATOR = NEWLINE | ";" ;
```

---

## 4. Control Flow

```ebnf
when_statement = "when" expression block
                 { "else" "when" expression block }
                 [ "else" block ] ;

each_statement = "each" IDENTIFIER "in" expression block ;

repeat_statement = "repeat" expression block ;

while_statement = "while" expression block ;

attempt_statement = "attempt" block "catch" IDENTIFIER block ;
```

---

## 5. Imports

```ebnf
import_statement = simple_import | selective_import ;

simple_import = "import" module_path TERMINATOR ;

selective_import = "from" module_path "import" import_list TERMINATOR ;

module_path = IDENTIFIER { "." IDENTIFIER } ;

import_list = IDENTIFIER { "," IDENTIFIER } ;
```

---

## 6. Expressions

### 6.1 Expression Hierarchy

```ebnf
expression = or_expression ;

or_expression = and_expression { "or" and_expression } ;

and_expression = equality_expression { "and" equality_expression } ;

equality_expression = comparison_expression { ( "==" | "!=" | "is" ) comparison_expression } ;

comparison_expression = range_expression { ( "<" | ">" | "<=" | ">=" ) range_expression } ;

range_expression = additive_expression [ ( ".." | "..=" ) additive_expression ] ;

additive_expression = multiplicative_expression { ( "+" | "-" ) multiplicative_expression } ;

multiplicative_expression = exponent_expression { ( "*" | "/" | "//" | "%" ) exponent_expression } ;

exponent_expression = unary_expression [ "**" exponent_expression ] ;

unary_expression = ( "-" | "not" ) unary_expression
                 | postfix_expression ;

postfix_expression = primary { postfix_operator } ;

postfix_operator = call_expression
                 | member_access
                 | index_access ;

call_expression = "(" [ argument_list ] ")" ;

argument_list = expression { "," expression } ;

member_access = "." IDENTIFIER ;

index_access = "[" expression "]" ;
```

### 6.2 Primary Expressions

```ebnf
primary = INT_LITERAL
        | FLOAT_LITERAL
        | STRING_LITERAL
        | FSTRING_LITERAL
        | "true"
        | "false"
        | "none"
        | IDENTIFIER
        | list_literal
        | map_literal
        | grouped_expression
        | ask_expression
        | new_expression
        | lambda_expression ;

list_literal = "[" [ expression { "," expression } [ "," ] ] "]" ;

map_literal = "{" [ map_entry { "," map_entry } [ "," ] ] "}" ;

map_entry = expression ":" expression ;

grouped_expression = "(" expression ")" ;

ask_expression = "ask" expression ;

new_expression = "new" IDENTIFIER "(" [ argument_list ] ")" ;

lambda_expression = "build" "(" [ parameter_list ] ")" block ;
```

---

## 7. Lexical Grammar

### 7.1 Identifiers

```ebnf
IDENTIFIER = IDENT_START { IDENT_CONTINUE } ;

IDENT_START = LETTER | "_" ;

IDENT_CONTINUE = LETTER | DIGIT | "_" ;

LETTER = "a".."z" | "A".."Z" | UNICODE_LETTER ;
```

### 7.2 Numeric Literals

```ebnf
INT_LITERAL = DECIMAL_INT | HEX_INT | BINARY_INT | OCTAL_INT ;

DECIMAL_INT = DIGIT { DIGIT | "_" } ;

HEX_INT = "0" ( "x" | "X" ) HEX_DIGIT { HEX_DIGIT | "_" } ;

BINARY_INT = "0" ( "b" | "B" ) BIN_DIGIT { BIN_DIGIT | "_" } ;

OCTAL_INT = "0" ( "o" | "O" ) OCT_DIGIT { OCT_DIGIT | "_" } ;

FLOAT_LITERAL = DIGIT { DIGIT | "_" } "." DIGIT { DIGIT | "_" } [ EXPONENT ]
              | DIGIT { DIGIT | "_" } EXPONENT ;

EXPONENT = ( "e" | "E" ) [ "+" | "-" ] DIGIT { DIGIT | "_" } ;

DIGIT = "0".."9" ;
HEX_DIGIT = "0".."9" | "a".."f" | "A".."F" ;
BIN_DIGIT = "0" | "1" ;
OCT_DIGIT = "0".."7" ;
```

### 7.3 String Literals

```ebnf
STRING_LITERAL = '"' { STRING_CHAR } '"' ;

FSTRING_LITERAL = 'f"' { FSTRING_PART } '"' ;

STRING_CHAR = ESCAPE_SEQUENCE | ANY_CHAR_EXCEPT_QUOTE_OR_BACKSLASH ;

FSTRING_PART = STRING_CHAR | INTERPOLATION ;

INTERPOLATION = "{" expression "}" ;

ESCAPE_SEQUENCE = "\\" ( "n" | "t" | "\\" | '"' | "r" | "0" | UNICODE_ESCAPE ) ;

UNICODE_ESCAPE = "u" "{" HEX_DIGIT { HEX_DIGIT } "}" ;
```

### 7.4 Comments

```ebnf
LINE_COMMENT = "//" { ANY_CHAR_EXCEPT_NEWLINE } NEWLINE ;

BLOCK_COMMENT = "/*" { BLOCK_COMMENT_BODY } "*/" ;

BLOCK_COMMENT_BODY = BLOCK_COMMENT | ANY_CHAR_EXCEPT_STAR_SLASH ;
```

### 7.5 Whitespace

```ebnf
WHITESPACE = " " | "\t" | "\r" ;

NEWLINE = "\n" | "\r\n" ;
```

---

## 8. Compatibility & Evolution Analysis

### 8.1 Compatibility Notes
- **Method declarations**: The addition of `| "fun"` to `method_declaration` ensures that existing class methods written with `fun` parse successfully. The parser marks these AST nodes with a compatibility flag to issue deprecation warnings later in the pipeline.
- **Ranges**: The exclusive `..` and inclusive `..=` range operators operate exactly as in Version 1.

### 8.2 Migration Notes
- To migrate code that uses the `fun` keyword, run the AST rewriting tool (`tech lint --fix`). This modifies the parsed grammar tokens to replace `fun` with `build` without changing structural behavior.
- Examples comparison:
  ```
  // Deprecated Syntax (parsed, emits warning)
  model Dog {
      fun bark() { say "Woof" }
  }

  // Canonical v2.0 Syntax
  model Dog {
      build bark() { say "Woof" }
  }
  ```

### 8.3 Rationale
- **Single Method production**: Accepting both `build` and `fun` inside the EBNF `method_declaration` maintains backward compatibility with Version 1 scripts without needing a separate syntax pipeline.
- **Context-Free Parsing**: Distinguishing methods from functions based solely on their nesting inside `model Name { ... }` simplifies the AST design while keeping the lexer context-free.

### 8.4 Future Roadmap
- **v2.2**: The grammar will expand to include optional type signatures:
  ```ebnf
  parameter = IDENTIFIER [ ":" type_spec ] [ "=" expression ] ;
  type_spec = IDENTIFIER { "[" type_spec "]" } ;
  ```
- **v3.0**: Native codegen optimizations will leverage explicit types defined in the updated grammar.
