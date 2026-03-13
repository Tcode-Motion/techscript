"""TechScript Lexer — converts raw source text into a stream of tokens.

Handles:
* number literals (int, float, hex, binary, octal, scientific, underscore sep)
* string literals (single/double/triple-quoted, escape sequences)
* f-strings with ``{expr}`` interpolation markers
* identifiers and reserved keywords
* all operators and delimiters defined in ``tokens.py``
* Python-style INDENT / DEDENT tracking
* single-line ``#`` and block ``#[ … ]#`` comments
* tab rejection with a friendly error
"""

from __future__ import annotations

from techscript.tokens import Token, TokenType, KEYWORDS
from techscript.errors import LexerError


class Lexer:
    """Tokenise a TechScript source string."""

    def __init__(self, source: str, filename: str = "<stdin>") -> None:
        self.source = source
        self.filename = filename
        self.pos = 0
        self.line = 1
        self.column = 1
        self.tokens: list[Token] = []

    # ------------------------------------------------------------------
    # Helpers
    # ------------------------------------------------------------------

    def _peek(self, offset: int = 0) -> str | None:
        idx = self.pos + offset
        return self.source[idx] if idx < len(self.source) else None

    def _advance(self) -> str:
        ch = self.source[self.pos]
        self.pos += 1
        if ch == "\n":
            self.line += 1
            self.column = 1
        else:
            self.column += 1
        return ch

    def _match(self, expected: str) -> bool:
        if self.pos < len(self.source) and self.source[self.pos] == expected:
            self._advance()
            return True
        return False

    def _add(self, tt: TokenType, value: str, *, line: int | None = None, col: int | None = None) -> None:
        self.tokens.append(Token(tt, value, line or self.line, col or self.column))

    def _error(self, msg: str) -> LexerError:
        return LexerError(msg, line=self.line, column=self.column)

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def tokenize(self) -> list[Token]:
        """Run the tokeniser and return the full token list."""
        while self.pos < len(self.source):
            ch = self._peek()

            # Skip whitespace
            if ch in (" ", "\r", "\t"):
                self._advance()
                continue

            # Newline
            if ch == "\n":
                self._add(TokenType.NEWLINE, "\\n")
                self._advance()
                continue

            # Comments
            if ch == "#":
                self._skip_comment()
                continue

            # Numbers
            if ch is not None and ch.isdigit():
                self._read_number()
                continue

            # Strings
            if ch in ('"', "'"):
                self._read_string(ch)
                continue

            # f-strings
            if ch == "f" and self._peek(1) in ('"', "'"):
                self._advance()  # skip 'f'
                self._read_fstring(self._peek())  # type: ignore[arg-type]
                continue

            # r-strings (raw)
            if ch == "r" and self._peek(1) in ('"', "'"):
                self._advance()
                self._read_string(self._peek(), raw=True)  # type: ignore[arg-type]
                continue

            # Identifiers / keywords
            if ch is not None and (ch.isalpha() or ch == "_"):
                self._read_identifier()
                continue

            # Operators & delimiters
            self._read_symbol()

        self._add(TokenType.EOF, "")
        return self.tokens

    # ------------------------------------------------------------------
    # Comments
    # ------------------------------------------------------------------

    def _skip_comment(self) -> None:
        self._advance()  # skip '#'
        # Block comment  #[ … ]#
        if self._peek() == "[":
            self._advance()  # skip '['
            while self.pos < len(self.source):
                if self.source[self.pos] == "]" and self._peek(1) == "#":
                    self._advance()  # ]
                    self._advance()  # #
                    return
                self._advance()
            raise self._error("Unterminated block comment (missing ]#)")
        # Single-line comment
        while self.pos < len(self.source) and self.source[self.pos] != "\n":
            self._advance()

    # ------------------------------------------------------------------
    # Numbers
    # ------------------------------------------------------------------

    def _read_number(self) -> None:
        start_col = self.column
        num = ""

        # Hex / binary / octal prefixes
        if self._peek() == "0" and self._peek(1) in ("x", "X", "b", "B", "o", "O"):
            num += self._advance()  # '0'
            num += self._advance()  # prefix letter
            while self.pos < len(self.source) and (self.source[self.pos].isalnum() or self.source[self.pos] == "_"):
                ch = self._advance()
                if ch != "_":
                    num += ch
            self._add(TokenType.NUMBER_INT, num, col=start_col)
            return

        is_float = False
        while self.pos < len(self.source) and (self.source[self.pos].isdigit() or self.source[self.pos] == "_"):
            ch = self._advance()
            if ch != "_":
                num += ch

        # Decimal point (not range ..)
        if (
            self.pos < len(self.source)
            and self.source[self.pos] == "."
            and self._peek(1) is not None
            and self._peek(1) not in (".",)  # avoid eating ..
            and (self._peek(1).isdigit() if self._peek(1) else False)
        ):
            is_float = True
            num += self._advance()  # '.'
            while self.pos < len(self.source) and (self.source[self.pos].isdigit() or self.source[self.pos] == "_"):
                ch = self._advance()
                if ch != "_":
                    num += ch

        # Scientific notation
        if self.pos < len(self.source) and self.source[self.pos] in ("e", "E"):
            is_float = True
            num += self._advance()
            if self.pos < len(self.source) and self.source[self.pos] in ("+", "-"):
                num += self._advance()
            while self.pos < len(self.source) and self.source[self.pos].isdigit():
                num += self._advance()

        tt = TokenType.NUMBER_FLOAT if is_float else TokenType.NUMBER_INT
        self._add(tt, num, col=start_col)

    # ------------------------------------------------------------------
    # Strings
    # ------------------------------------------------------------------

    def _read_string(self, quote: str, *, raw: bool = False) -> None:
        start_col = self.column
        self._advance()  # opening quote
        result = ""

        # Triple-quoted
        if self._peek() == quote and self._peek(1) == quote:
            self._advance()
            self._advance()
            while self.pos < len(self.source):
                if self.source[self.pos] == quote and self._peek(1) == quote and self._peek(2) == quote:
                    self._advance(); self._advance(); self._advance()
                    self._add(TokenType.STRING, result, col=start_col)
                    return
                if not raw and self.source[self.pos] == "\\":
                    result += self._read_escape()
                else:
                    result += self._advance()
            raise self._error("Unterminated triple-quoted string")

        # Single-line string
        while self.pos < len(self.source) and self.source[self.pos] != quote:
            if self.source[self.pos] == "\n":
                raise self._error("Unterminated string (use triple quotes for multi-line)")
            if not raw and self.source[self.pos] == "\\":
                result += self._read_escape()
            else:
                result += self._advance()

        if self.pos >= len(self.source):
            raise self._error("Unterminated string")

        self._advance()  # closing quote
        self._add(TokenType.STRING, result, col=start_col)

    def _read_fstring(self, quote: str) -> None:
        start_col = self.column
        self._advance()  # opening quote
        result = ""
        while self.pos < len(self.source) and self.source[self.pos] != quote:
            if self.source[self.pos] == "\n":
                raise self._error("Unterminated f-string")
            result += self._advance()
        if self.pos >= len(self.source):
            raise self._error("Unterminated f-string")
        self._advance()  # closing quote
        self._add(TokenType.FSTRING, result, col=start_col)

    def _read_escape(self) -> str:
        self._advance()  # backslash
        if self.pos >= len(self.source):
            return "\\"
        ch = self._advance()
        return {"n": "\n", "t": "\t", "r": "\r", "\\": "\\", "'": "'", '"': '"', "0": "\0"}.get(ch, "\\" + ch)

    # ------------------------------------------------------------------
    # Identifiers / keywords
    # ------------------------------------------------------------------

    def _read_identifier(self) -> None:
        start_col = self.column
        name = ""
        while self.pos < len(self.source) and (self.source[self.pos].isalnum() or self.source[self.pos] == "_"):
            name += self._advance()

        if name == "true":
            self._add(TokenType.BOOL_TRUE, name, col=start_col)
        elif name == "false":
            self._add(TokenType.BOOL_FALSE, name, col=start_col)
        elif name == "none":
            self._add(TokenType.NONE, name, col=start_col)
        elif name in KEYWORDS:
            self._add(TokenType.KEYWORD, name, col=start_col)
        else:
            self._add(TokenType.IDENTIFIER, name, col=start_col)

    # ------------------------------------------------------------------
    # Operators / delimiters
    # ------------------------------------------------------------------

    def _read_symbol(self) -> None:
        start_col = self.column
        ch = self._advance()

        # --- multi-char operators ---

        # ** and *=
        if ch == "*":
            if self._match("*"):
                self._add(TokenType.POWER, "**", col=start_col); return
            if self._match("="):
                self._add(TokenType.STAR_ASSIGN, "*=", col=start_col); return
            self._add(TokenType.STAR, "*", col=start_col); return

        # .. ..= ...
        if ch == ".":
            if self._peek() == ".":
                self._advance()
                if self._match("."):
                    self._add(TokenType.SPREAD, "...", col=start_col); return
                if self._match("="):
                    self._add(TokenType.DOTDOT_EQUAL, "..=", col=start_col); return
                self._add(TokenType.DOTDOT, "..", col=start_col); return
            self._add(TokenType.DOT, ".", col=start_col); return

        # Two-char lookup tables
        _two = {
            "=": {"=": TokenType.EQUAL, ">": TokenType.ARROW},
            "!": {"=": TokenType.NOT_EQUAL},
            "<": {"=": TokenType.LESS_EQUAL},
            ">": {"=": TokenType.GREATER_EQUAL},
            "+": {"=": TokenType.PLUS_ASSIGN},
            "-": {"=": TokenType.MINUS_ASSIGN},
            "/": {"/": TokenType.DOUBLE_SLASH, "=": TokenType.SLASH_ASSIGN},
            "|": {">": TokenType.PIPE},
            "?": {".": TokenType.OPTIONAL_CHAIN, "?": TokenType.NULLISH},
        }

        if ch in _two and self.pos < len(self.source):
            nxt = self.source[self.pos]
            if nxt in _two[ch]:
                self._advance()
                self._add(_two[ch][nxt], ch + nxt, col=start_col)
                return

        _single = {
            "+": TokenType.PLUS, "-": TokenType.MINUS,
            "/": TokenType.SLASH, "%": TokenType.PERCENT,
            "(": TokenType.LPAREN, ")": TokenType.RPAREN,
            "[": TokenType.LBRACKET, "]": TokenType.RBRACKET,
            "{": TokenType.LBRACE, "}": TokenType.RBRACE,
            ",": TokenType.COMMA, ":": TokenType.COLON,
            "@": TokenType.AT, "?": TokenType.QUESTION,
            "=": TokenType.ASSIGN, "<": TokenType.LESS,
            ">": TokenType.GREATER,
        }

        if ch in _single:
            self._add(_single[ch], ch, col=start_col)
            return

        raise self._error(f"Unexpected character: '{ch}'")
