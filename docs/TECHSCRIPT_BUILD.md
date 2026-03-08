# TechScript — Build Guide Part 1: Lexer & Parser

> **How to Build the TechScript Interpreter from Scratch (in Python)**
> Companion to [TECHSCRIPT_SPEC.md](./TECHSCRIPT_SPEC.md)

---

## Overview & Roadmap

Building a programming language involves these stages:

```
Phase 1: Lexer (tokenizer)        ← THIS FILE
Phase 2: Parser (grammar → AST)   ← THIS FILE
Phase 3: AST Nodes                ← Part 2
Phase 4: Interpreter (evaluator)  ← Part 2
Phase 5: Built-ins & Stdlib       ← Part 2
Phase 6: CLI Tool & REPL          ← Part 2
Phase 7: Error System             ← Part 2
Phase 8: Packaging & Distribution ← Part 2
```

### Project Structure

```
techscript/
├── pyproject.toml
├── setup.py
├── src/
│   └── techscript/
│       ├── __init__.py
│       ├── __main__.py        # CLI entry point
│       ├── lexer.py           # Tokenizer
│       ├── tokens.py          # Token types
│       ├── parser.py          # Parser → AST
│       ├── ast_nodes.py       # AST node definitions
│       ├── interpreter.py     # AST walker / evaluator
│       ├── environment.py     # Variable scope management
│       ├── builtins.py        # Built-in functions
│       ├── errors.py          # Error types & formatting
│       ├── repl.py            # Interactive REPL
│       ├── cli.py             # Command-line interface
│       └── stdlib/
│           ├── math_mod.py
│           ├── io_mod.py
│           ├── text_mod.py
│           └── ...
├── tests/
│   ├── test_lexer.py
│   ├── test_parser.py
│   └── test_interpreter.py
└── examples/
    ├── hello.txs
    └── calculator.txs
```

---

## Phase 1: Token Definitions (`tokens.py`)

```python
"""Token types for TechScript."""
from enum import Enum, auto
from dataclasses import dataclass


class TokenType(Enum):
    # Literals
    NUMBER_INT = auto()
    NUMBER_FLOAT = auto()
    STRING = auto()
    FSTRING = auto()
    BOOL_TRUE = auto()
    BOOL_FALSE = auto()
    NONE = auto()

    # Identifiers & Keywords
    IDENTIFIER = auto()
    KEYWORD = auto()

    # Arithmetic
    PLUS = auto()          # +
    MINUS = auto()         # -
    STAR = auto()          # *
    SLASH = auto()         # /
    DOUBLE_SLASH = auto()  # //
    PERCENT = auto()       # %
    POWER = auto()         # **

    # Assignment
    ASSIGN = auto()        # =
    PLUS_ASSIGN = auto()   # +=
    MINUS_ASSIGN = auto()  # -=
    STAR_ASSIGN = auto()   # *=
    SLASH_ASSIGN = auto()  # /=

    # Comparison
    EQUAL = auto()         # ==
    NOT_EQUAL = auto()     # !=
    LESS = auto()          # <
    GREATER = auto()       # >
    LESS_EQUAL = auto()    # <=
    GREATER_EQUAL = auto() # >=

    # Logical (handled as keywords: and, or, not)

    # Special
    ARROW = auto()         # =>
    PIPE = auto()          # |>
    QUESTION = auto()      # ?
    DOT = auto()           # .
    DOTDOT = auto()        # ..
    DOTDOT_EQUAL = auto()  # ..=
    SPREAD = auto()        # ...
    AT = auto()            # @
    HASH = auto()          # #
    NULLISH = auto()       # ??
    OPTIONAL = auto()      # ?.

    # Delimiters
    LPAREN = auto()        # (
    RPAREN = auto()        # )
    LBRACKET = auto()      # [
    RBRACKET = auto()      # ]
    LBRACE = auto()        # {
    RBRACE = auto()        # }
    COMMA = auto()         # ,
    COLON = auto()         # :

    # Structure
    NEWLINE = auto()
    INDENT = auto()
    DEDENT = auto()
    EOF = auto()


# Reserved keywords
KEYWORDS = {
    "say", "ask", "set", "fn", "return",
    "if", "elif", "else", "for", "while",
    "in", "do", "end", "break", "skip",
    "match", "case", "try", "catch", "throw",
    "class", "self", "new", "import", "from",
    "export", "as", "with", "defer", "guard",
    "true", "false", "none", "and", "or",
    "not", "is", "has", "typeof", "await",
    "async", "yield", "unless", "until", "each",
    "del", "mut", "const", "global", "pass",
    "finally", "super",
}


@dataclass
class Token:
    """A single token produced by the lexer."""
    type: TokenType
    value: str             # Raw text of the token
    line: int              # 1-indexed line number
    column: int            # 1-indexed column number

    def __repr__(self):
        return f"Token({self.type.name}, {self.value!r}, L{self.line}:{self.column})"
```

---

## Phase 2: Lexer (`lexer.py`)

The lexer reads raw source code and produces a stream of `Token` objects.

```python
"""TechScript Lexer — Converts source code to tokens."""
from .tokens import Token, TokenType, KEYWORDS


class LexerError(Exception):
    def __init__(self, message, line, column):
        self.message = message
        self.line = line
        self.column = column
        super().__init__(f"LexerError at L{line}:{column}: {message}")


class Lexer:
    def __init__(self, source: str):
        self.source = source
        self.pos = 0
        self.line = 1
        self.column = 1
        self.tokens: list[Token] = []
        self.indent_stack = [0]  # Stack of indentation levels

    def peek(self) -> str | None:
        if self.pos < len(self.source):
            return self.source[self.pos]
        return None

    def advance(self) -> str:
        ch = self.source[self.pos]
        self.pos += 1
        if ch == '\n':
            self.line += 1
            self.column = 1
        else:
            self.column += 1
        return ch

    def match(self, expected: str) -> bool:
        if self.pos < len(self.source) and self.source[self.pos] == expected:
            self.advance()
            return True
        return False

    def add_token(self, type: TokenType, value: str, line=None, col=None):
        self.tokens.append(Token(
            type=type,
            value=value,
            line=line or self.line,
            column=col or self.column
        ))

    def tokenize(self) -> list[Token]:
        """Main tokenization loop."""
        while self.pos < len(self.source):
            self._skip_comments()
            if self.pos >= len(self.source):
                break

            ch = self.peek()

            # Newline — handle indentation on next line
            if ch == '\n':
                self.add_token(TokenType.NEWLINE, '\\n')
                self.advance()
                self._handle_indentation()
                continue

            # Skip spaces (not at line start — those are indentation)
            if ch in (' ', '\r'):
                self.advance()
                continue

            # Tab check
            if ch == '\t':
                raise LexerError(
                    "Tab character detected. TechScript uses spaces for indentation.",
                    self.line, self.column
                )

            # Numbers
            if ch.isdigit():
                self._read_number()
                continue

            # Strings
            if ch in ('"', "'"):
                self._read_string()
                continue

            # F-strings
            if ch == 'f' and self.pos + 1 < len(self.source) and self.source[self.pos + 1] in ('"', "'"):
                self.advance()  # skip 'f'
                self._read_fstring()
                continue

            # Identifiers and keywords
            if ch.isalpha() or ch == '_':
                self._read_identifier()
                continue

            # Operators and delimiters
            self._read_symbol()

        # Emit remaining DEDENTs
        while len(self.indent_stack) > 1:
            self.indent_stack.pop()
            self.add_token(TokenType.DEDENT, '<dedent>')

        self.add_token(TokenType.EOF, '')
        return self.tokens

    def _skip_comments(self):
        """Skip single-line and block comments."""
        if self.pos >= len(self.source):
            return
        if self.source[self.pos] == '#':
            # Block comment: #[ ... ]#
            if self.pos + 1 < len(self.source) and self.source[self.pos + 1] == '[':
                self.advance()  # #
                self.advance()  # [
                while self.pos < len(self.source):
                    if self.source[self.pos] == ']' and self.pos + 1 < len(self.source) and self.source[self.pos + 1] == '#':
                        self.advance()  # ]
                        self.advance()  # #
                        return
                    self.advance()
                raise LexerError("Unterminated block comment", self.line, self.column)
            else:
                # Single-line comment
                while self.pos < len(self.source) and self.source[self.pos] != '\n':
                    self.advance()

    def _handle_indentation(self):
        """Process indentation at the start of a new line."""
        spaces = 0
        while self.pos < len(self.source) and self.source[self.pos] == ' ':
            spaces += 1
            self.advance()

        # Skip blank lines
        if self.pos < len(self.source) and self.source[self.pos] == '\n':
            return
        # Skip comment-only lines
        if self.pos < len(self.source) and self.source[self.pos] == '#':
            return

        current_indent = self.indent_stack[-1]
        if spaces > current_indent:
            self.indent_stack.append(spaces)
            self.add_token(TokenType.INDENT, '<indent>')
        elif spaces < current_indent:
            while len(self.indent_stack) > 1 and self.indent_stack[-1] > spaces:
                self.indent_stack.pop()
                self.add_token(TokenType.DEDENT, '<dedent>')
            if self.indent_stack[-1] != spaces:
                raise LexerError(
                    f"Inconsistent indentation: expected {self.indent_stack[-1]} spaces, got {spaces}",
                    self.line, self.column
                )

    def _read_number(self):
        """Read integer or float literal."""
        start_col = self.column
        num_str = ''

        # Hex, binary, octal
        if self.peek() == '0' and self.pos + 1 < len(self.source):
            next_ch = self.source[self.pos + 1]
            if next_ch in ('x', 'X', 'b', 'B', 'o', 'O'):
                num_str += self.advance()  # '0'
                num_str += self.advance()  # prefix
                while self.pos < len(self.source) and (self.source[self.pos].isalnum() or self.source[self.pos] == '_'):
                    if self.source[self.pos] != '_':
                        num_str += self.advance()
                    else:
                        self.advance()
                self.add_token(TokenType.NUMBER_INT, num_str, self.line, start_col)
                return

        is_float = False
        while self.pos < len(self.source) and (self.source[self.pos].isdigit() or self.source[self.pos] == '_'):
            if self.source[self.pos] != '_':
                num_str += self.advance()
            else:
                self.advance()

        if self.pos < len(self.source) and self.source[self.pos] == '.' and \
           self.pos + 1 < len(self.source) and self.source[self.pos + 1].isdigit():
            is_float = True
            num_str += self.advance()  # '.'
            while self.pos < len(self.source) and (self.source[self.pos].isdigit() or self.source[self.pos] == '_'):
                if self.source[self.pos] != '_':
                    num_str += self.advance()
                else:
                    self.advance()

        # Scientific notation
        if self.pos < len(self.source) and self.source[self.pos] in ('e', 'E'):
            is_float = True
            num_str += self.advance()
            if self.pos < len(self.source) and self.source[self.pos] in ('+', '-'):
                num_str += self.advance()
            while self.pos < len(self.source) and self.source[self.pos].isdigit():
                num_str += self.advance()

        token_type = TokenType.NUMBER_FLOAT if is_float else TokenType.NUMBER_INT
        self.add_token(token_type, num_str, self.line, start_col)

    def _read_string(self):
        """Read a string literal (single or double quotes, including triple-quoted)."""
        start_col = self.column
        quote = self.advance()
        result = ''

        # Triple-quoted string
        if self.pos + 1 < len(self.source) and self.source[self.pos] == quote and self.source[self.pos + 1] == quote:
            self.advance()  # second quote
            self.advance()  # third quote
            while self.pos < len(self.source):
                if self.source[self.pos] == quote and \
                   self.pos + 2 < len(self.source) and \
                   self.source[self.pos + 1] == quote and \
                   self.source[self.pos + 2] == quote:
                    self.advance(); self.advance(); self.advance()
                    self.add_token(TokenType.STRING, result, self.line, start_col)
                    return
                result += self.advance()
            raise LexerError("Unterminated triple-quoted string", self.line, self.column)

        # Single-line string
        while self.pos < len(self.source) and self.source[self.pos] != quote:
            if self.source[self.pos] == '\n':
                raise LexerError("Unterminated string (use triple quotes for multi-line)", self.line, self.column)
            if self.source[self.pos] == '\\':
                self.advance()
                if self.pos < len(self.source):
                    escape_ch = self.advance()
                    escape_map = {'n': '\n', 't': '\t', '\\': '\\', '"': '"', "'": "'"}
                    result += escape_map.get(escape_ch, '\\' + escape_ch)
            else:
                result += self.advance()

        if self.pos >= len(self.source):
            raise LexerError("Unterminated string", self.line, self.column)
        self.advance()  # closing quote
        self.add_token(TokenType.STRING, result, self.line, start_col)

    def _read_fstring(self):
        """Read an f-string with interpolation."""
        # Simplified: store as FSTRING token, parser handles {} parts
        start_col = self.column
        quote = self.advance()
        result = ''
        while self.pos < len(self.source) and self.source[self.pos] != quote:
            if self.source[self.pos] == '\n':
                raise LexerError("Unterminated f-string", self.line, self.column)
            result += self.advance()
        if self.pos >= len(self.source):
            raise LexerError("Unterminated f-string", self.line, self.column)
        self.advance()  # closing quote
        self.add_token(TokenType.FSTRING, result, self.line, start_col)

    def _read_identifier(self):
        """Read an identifier or keyword."""
        start_col = self.column
        name = ''
        while self.pos < len(self.source) and (self.source[self.pos].isalnum() or self.source[self.pos] == '_'):
            name += self.advance()

        if name == 'true':
            self.add_token(TokenType.BOOL_TRUE, name, self.line, start_col)
        elif name == 'false':
            self.add_token(TokenType.BOOL_FALSE, name, self.line, start_col)
        elif name == 'none':
            self.add_token(TokenType.NONE, name, self.line, start_col)
        elif name in KEYWORDS:
            self.add_token(TokenType.KEYWORD, name, self.line, start_col)
        else:
            self.add_token(TokenType.IDENTIFIER, name, self.line, start_col)

    def _read_symbol(self):
        """Read operators and delimiter symbols."""
        start_col = self.column
        ch = self.advance()

        TWO_CHAR = {
            '=': {'=': TokenType.EQUAL, '>': TokenType.ARROW},
            '!': {'=': TokenType.NOT_EQUAL},
            '<': {'=': TokenType.LESS_EQUAL},
            '>': {'=': TokenType.GREATER_EQUAL},
            '+': {'=': TokenType.PLUS_ASSIGN},
            '-': {'=': TokenType.MINUS_ASSIGN},
            '/': {'/': TokenType.DOUBLE_SLASH, '=': TokenType.SLASH_ASSIGN},
            '|': {'>': TokenType.PIPE},
            '?': {'.': TokenType.OPTIONAL, '?': TokenType.NULLISH},
        }

        SINGLE_CHAR = {
            '+': TokenType.PLUS, '-': TokenType.MINUS,
            '/': TokenType.SLASH, '%': TokenType.PERCENT,
            '(': TokenType.LPAREN, ')': TokenType.RPAREN,
            '[': TokenType.LBRACKET, ']': TokenType.RBRACKET,
            '{': TokenType.LBRACE, '}': TokenType.RBRACE,
            ',': TokenType.COMMA, ':': TokenType.COLON,
            '@': TokenType.AT, '?': TokenType.QUESTION,
            '=': TokenType.ASSIGN, '<': TokenType.LESS,
            '>': TokenType.GREATER,
        }

        # ** (power) and *= special cases
        if ch == '*':
            if self.match('*'):
                self.add_token(TokenType.POWER, '**', self.line, start_col)
            elif self.match('='):
                self.add_token(TokenType.STAR_ASSIGN, '*=', self.line, start_col)
            else:
                self.add_token(TokenType.STAR, '*', self.line, start_col)
            return

        # .. and ..= and ...
        if ch == '.':
            if self.match('.'):
                if self.match('.'):
                    self.add_token(TokenType.SPREAD, '...', self.line, start_col)
                elif self.match('='):
                    self.add_token(TokenType.DOTDOT_EQUAL, '..=', self.line, start_col)
                else:
                    self.add_token(TokenType.DOTDOT, '..', self.line, start_col)
            else:
                self.add_token(TokenType.DOT, '.', self.line, start_col)
            return

        # Two-character operators
        if ch in TWO_CHAR and self.pos < len(self.source):
            next_ch = self.source[self.pos]
            if next_ch in TWO_CHAR[ch]:
                self.advance()
                self.add_token(TWO_CHAR[ch][next_ch], ch + next_ch, self.line, start_col)
                return

        # Single-character operators
        if ch in SINGLE_CHAR:
            self.add_token(SINGLE_CHAR[ch], ch, self.line, start_col)
            return

        raise LexerError(f"Unexpected character: '{ch}'", self.line, start_col)
```

---

## Phase 3: Parser (`parser.py`)

The parser consumes tokens and builds an Abstract Syntax Tree (AST).

```python
"""TechScript Parser — Converts token stream to AST."""
from .tokens import Token, TokenType
from .ast_nodes import *


class ParseError(Exception):
    def __init__(self, message, token):
        self.token = token
        super().__init__(f"ParseError at L{token.line}:{token.column}: {message}")


class Parser:
    def __init__(self, tokens: list[Token]):
        self.tokens = tokens
        self.pos = 0

    def peek(self) -> Token:
        return self.tokens[self.pos]

    def advance(self) -> Token:
        token = self.tokens[self.pos]
        self.pos += 1
        return token

    def expect(self, type: TokenType, value: str = None) -> Token:
        token = self.peek()
        if token.type != type:
            raise ParseError(f"Expected {type.name}, got {token.type.name} ('{token.value}')", token)
        if value and token.value != value:
            raise ParseError(f"Expected '{value}', got '{token.value}'", token)
        return self.advance()

    def match_token(self, type: TokenType, value: str = None) -> Token | None:
        token = self.peek()
        if token.type == type and (value is None or token.value == value):
            return self.advance()
        return None

    def skip_newlines(self):
        while self.peek().type == TokenType.NEWLINE:
            self.advance()

    # === Entry Point ===

    def parse(self) -> Program:
        """Parse entire program."""
        self.skip_newlines()
        statements = []
        while self.peek().type != TokenType.EOF:
            stmt = self.parse_statement()
            if stmt:
                statements.append(stmt)
            self.skip_newlines()
        return Program(body=statements)

    # === Statement Parsing ===

    def parse_statement(self):
        token = self.peek()

        if token.type == TokenType.KEYWORD:
            match token.value:
                case "say":     return self.parse_say()
                case "set":     return self.parse_set()
                case "if":      return self.parse_if()
                case "unless":  return self.parse_unless()
                case "for":     return self.parse_for()
                case "while":   return self.parse_while()
                case "until":   return self.parse_until()
                case "fn":      return self.parse_fn()
                case "class":   return self.parse_class()
                case "return":  return self.parse_return()
                case "break":   self.advance(); return BreakStmt()
                case "skip":    self.advance(); return SkipStmt()
                case "pass":    self.advance(); return PassStmt()
                case "try":     return self.parse_try()
                case "throw":   return self.parse_throw()
                case "match":   return self.parse_match()
                case "import":  return self.parse_import()
                case "from":    return self.parse_from_import()
                case "del":     return self.parse_del()
                case "defer":   return self.parse_defer()
                case "guard":   return self.parse_guard()
                case "with":    return self.parse_with()
                case "const":   return self.parse_const()
                case "export":  return self.parse_export()

        # Expression statement (could be assignment or function call)
        return self.parse_expression_statement()

    def parse_say(self):
        self.advance()  # 'say'
        values = [self.parse_expression()]
        while self.match_token(TokenType.COMMA):
            values.append(self.parse_expression())
        return SayStmt(values=values)

    def parse_set(self):
        self.advance()  # 'set'
        name = self.expect(TokenType.IDENTIFIER).value
        self.expect(TokenType.ASSIGN)
        value = self.parse_expression()
        return SetStmt(name=name, value=value)

    def parse_if(self):
        self.advance()  # 'if'
        condition = self.parse_expression()
        self.expect(TokenType.COLON)
        body = self.parse_block()

        elif_clauses = []
        while self.match_token(TokenType.KEYWORD, "elif"):
            elif_cond = self.parse_expression()
            self.expect(TokenType.COLON)
            elif_body = self.parse_block()
            elif_clauses.append((elif_cond, elif_body))

        else_body = None
        if self.match_token(TokenType.KEYWORD, "else"):
            self.expect(TokenType.COLON)
            else_body = self.parse_block()

        return IfStmt(condition, body, elif_clauses, else_body)

    def parse_for(self):
        self.advance()  # 'for'
        var_name = self.expect(TokenType.IDENTIFIER).value
        self.expect(TokenType.KEYWORD, "in")
        iterable = self.parse_expression()
        self.expect(TokenType.COLON)
        body = self.parse_block()
        return ForStmt(var_name=var_name, iterable=iterable, body=body)

    def parse_while(self):
        self.advance()  # 'while'
        condition = self.parse_expression()
        self.expect(TokenType.COLON)
        body = self.parse_block()
        return WhileStmt(condition=condition, body=body)

    def parse_fn(self):
        self.advance()  # 'fn'
        name = self.expect(TokenType.IDENTIFIER).value
        self.expect(TokenType.LPAREN)
        params = self.parse_param_list()
        self.expect(TokenType.RPAREN)
        self.expect(TokenType.COLON)
        body = self.parse_block()
        return FnStmt(name=name, params=params, body=body)

    def parse_return(self):
        self.advance()  # 'return'
        value = None
        if self.peek().type not in (TokenType.NEWLINE, TokenType.EOF, TokenType.DEDENT):
            value = self.parse_expression()
        return ReturnStmt(value=value)

    def parse_block(self) -> list:
        """Parse an indented block of statements."""
        self.skip_newlines()
        self.expect(TokenType.INDENT)
        statements = []
        while self.peek().type != TokenType.DEDENT and self.peek().type != TokenType.EOF:
            self.skip_newlines()
            if self.peek().type == TokenType.DEDENT:
                break
            stmt = self.parse_statement()
            if stmt:
                statements.append(stmt)
            self.skip_newlines()
        if self.peek().type == TokenType.DEDENT:
            self.advance()
        return statements

    def parse_param_list(self) -> list:
        params = []
        if self.peek().type == TokenType.RPAREN:
            return params
        params.append(self.parse_param())
        while self.match_token(TokenType.COMMA):
            params.append(self.parse_param())
        return params

    def parse_param(self):
        name = self.expect(TokenType.IDENTIFIER).value
        default = None
        if self.match_token(TokenType.ASSIGN):
            default = self.parse_expression()
        return Param(name=name, default=default)

    # === Expression Parsing (Pratt Parser / Recursive Descent) ===

    def parse_expression(self):
        return self.parse_ternary()

    def parse_ternary(self):
        expr = self.parse_or()
        if self.match_token(TokenType.KEYWORD, "if"):
            condition = self.parse_or()
            self.expect(TokenType.KEYWORD, "else")
            false_val = self.parse_ternary()
            return TernaryExpr(true_val=expr, condition=condition, false_val=false_val)
        return expr

    def parse_or(self):
        left = self.parse_and()
        while self.match_token(TokenType.KEYWORD, "or"):
            right = self.parse_and()
            left = BinaryOp(left=left, op="or", right=right)
        return left

    def parse_and(self):
        left = self.parse_not()
        while self.match_token(TokenType.KEYWORD, "and"):
            right = self.parse_not()
            left = BinaryOp(left=left, op="and", right=right)
        return left

    def parse_not(self):
        if self.match_token(TokenType.KEYWORD, "not"):
            operand = self.parse_not()
            return UnaryOp(op="not", operand=operand)
        return self.parse_comparison()

    def parse_comparison(self):
        left = self.parse_addition()
        comp_ops = {TokenType.EQUAL, TokenType.NOT_EQUAL, TokenType.LESS,
                    TokenType.GREATER, TokenType.LESS_EQUAL, TokenType.GREATER_EQUAL}
        while self.peek().type in comp_ops or \
              (self.peek().type == TokenType.KEYWORD and self.peek().value in ("is", "in")):
            op = self.advance()
            right = self.parse_addition()
            left = BinaryOp(left=left, op=op.value, right=right)
        return left

    def parse_addition(self):
        left = self.parse_multiplication()
        while self.peek().type in (TokenType.PLUS, TokenType.MINUS):
            op = self.advance()
            right = self.parse_multiplication()
            left = BinaryOp(left=left, op=op.value, right=right)
        return left

    def parse_multiplication(self):
        left = self.parse_unary()
        while self.peek().type in (TokenType.STAR, TokenType.SLASH, TokenType.DOUBLE_SLASH, TokenType.PERCENT):
            op = self.advance()
            right = self.parse_unary()
            left = BinaryOp(left=left, op=op.value, right=right)
        return left

    def parse_unary(self):
        if self.peek().type in (TokenType.MINUS, TokenType.PLUS):
            op = self.advance()
            operand = self.parse_unary()
            return UnaryOp(op=op.value, operand=operand)
        return self.parse_power()

    def parse_power(self):
        base = self.parse_call()
        if self.match_token(TokenType.POWER):
            exp = self.parse_unary()
            return BinaryOp(left=base, op="**", right=exp)
        return base

    def parse_call(self):
        expr = self.parse_primary()
        while True:
            if self.match_token(TokenType.LPAREN):
                args = []
                if self.peek().type != TokenType.RPAREN:
                    args.append(self.parse_expression())
                    while self.match_token(TokenType.COMMA):
                        args.append(self.parse_expression())
                self.expect(TokenType.RPAREN)
                expr = CallExpr(callee=expr, args=args)
            elif self.match_token(TokenType.LBRACKET):
                index = self.parse_expression()
                self.expect(TokenType.RBRACKET)
                expr = IndexExpr(obj=expr, index=index)
            elif self.match_token(TokenType.DOT):
                member = self.expect(TokenType.IDENTIFIER).value
                expr = MemberExpr(obj=expr, member=member)
            elif self.match_token(TokenType.PIPE):
                func = self.parse_primary()
                expr = CallExpr(callee=func, args=[expr])
            else:
                break
        return expr

    def parse_primary(self):
        token = self.peek()

        if token.type == TokenType.NUMBER_INT:
            self.advance()
            return NumberLit(value=int(token.value, 0))
        if token.type == TokenType.NUMBER_FLOAT:
            self.advance()
            return NumberLit(value=float(token.value))
        if token.type == TokenType.STRING:
            self.advance()
            return StringLit(value=token.value)
        if token.type == TokenType.FSTRING:
            self.advance()
            return FStringLit(raw=token.value)
        if token.type == TokenType.BOOL_TRUE:
            self.advance()
            return BoolLit(value=True)
        if token.type == TokenType.BOOL_FALSE:
            self.advance()
            return BoolLit(value=False)
        if token.type == TokenType.NONE:
            self.advance()
            return NoneLit()
        if token.type == TokenType.IDENTIFIER:
            self.advance()
            return Identifier(name=token.value)
        if token.type == TokenType.KEYWORD and token.value == "ask":
            self.advance()
            prompt = self.parse_expression()
            return AskExpr(prompt=prompt)
        if token.type == TokenType.QUESTION:
            self.advance()
            prompt = self.parse_expression()
            return AskExpr(prompt=prompt)
        if token.type == TokenType.LBRACKET:
            return self.parse_list_literal()
        if token.type == TokenType.LBRACE:
            return self.parse_map_literal()
        if token.type == TokenType.LPAREN:
            return self.parse_grouped_or_lambda()

        raise ParseError(f"Unexpected token: '{token.value}'", token)

    def parse_list_literal(self):
        self.advance()  # [
        elements = []
        if self.peek().type != TokenType.RBRACKET:
            elements.append(self.parse_expression())
            while self.match_token(TokenType.COMMA):
                if self.peek().type == TokenType.RBRACKET:
                    break
                elements.append(self.parse_expression())
        self.expect(TokenType.RBRACKET)
        return ListLit(elements=elements)

    def parse_map_literal(self):
        self.advance()  # {
        entries = []
        self.skip_newlines()
        if self.peek().type != TokenType.RBRACE:
            entries.append(self.parse_map_entry())
            while self.match_token(TokenType.COMMA):
                self.skip_newlines()
                if self.peek().type == TokenType.RBRACE:
                    break
                entries.append(self.parse_map_entry())
        self.skip_newlines()
        self.expect(TokenType.RBRACE)
        return MapLit(entries=entries)

    def parse_map_entry(self):
        self.skip_newlines()
        key = self.parse_expression()
        self.expect(TokenType.COLON)
        value = self.parse_expression()
        return (key, value)

    def parse_grouped_or_lambda(self):
        # Could be (expr) or (params) => expr
        self.advance()  # (
        if self.peek().type == TokenType.RPAREN:
            self.advance()
            self.expect(TokenType.ARROW)
            body = self.parse_expression()
            return LambdaExpr(params=[], body=body)

        # Try to determine if this is a lambda or grouping
        expr = self.parse_expression()
        if self.match_token(TokenType.RPAREN):
            if self.match_token(TokenType.ARROW):
                # Lambda with single param
                if isinstance(expr, Identifier):
                    body = self.parse_expression()
                    return LambdaExpr(params=[Param(name=expr.name)], body=body)
            return expr  # Grouped expression

        # Multiple params in lambda
        if self.peek().type == TokenType.COMMA and isinstance(expr, Identifier):
            params = [Param(name=expr.name)]
            while self.match_token(TokenType.COMMA):
                name = self.expect(TokenType.IDENTIFIER).value
                params.append(Param(name=name))
            self.expect(TokenType.RPAREN)
            self.expect(TokenType.ARROW)
            body = self.parse_expression()
            return LambdaExpr(params=params, body=body)

        self.expect(TokenType.RPAREN)
        return expr

    def parse_expression_statement(self):
        expr = self.parse_expression()
        # Check for assignment
        if isinstance(expr, Identifier) and self.peek().type in (
            TokenType.ASSIGN, TokenType.PLUS_ASSIGN,
            TokenType.MINUS_ASSIGN, TokenType.STAR_ASSIGN,
            TokenType.SLASH_ASSIGN
        ):
            op = self.advance()
            value = self.parse_expression()
            return AssignStmt(target=expr, op=op.value, value=value)
        return ExpressionStmt(expression=expr)
```

---

*Continued in [TECHSCRIPT_BUILD_2.md](./TECHSCRIPT_BUILD_2.md) — Interpreter, Built-ins, CLI, REPL, Error System, and Packaging.*
