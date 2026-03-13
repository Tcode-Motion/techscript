"""TechScript token types and keyword definitions.

Every token produced by the lexer is one of the ``TokenType`` variants
listed here. The ``KEYWORDS`` set contains all reserved words that the
parser treats specially (as opposed to user identifiers).
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum, auto


# ---------------------------------------------------------------------------
# Token type enumeration
# ---------------------------------------------------------------------------

class TokenType(Enum):
    """All token categories recognised by the TechScript lexer."""

    # -- Literals --
    NUMBER_INT = auto()
    NUMBER_FLOAT = auto()
    STRING = auto()
    FSTRING = auto()
    BOOL_TRUE = auto()
    BOOL_FALSE = auto()
    NONE = auto()

    # -- Identifiers & keywords --
    IDENTIFIER = auto()
    KEYWORD = auto()

    # -- Arithmetic --
    PLUS = auto()           # +
    MINUS = auto()          # -
    STAR = auto()           # *
    SLASH = auto()          # /
    DOUBLE_SLASH = auto()   # //
    PERCENT = auto()        # %
    POWER = auto()          # **

    # -- Assignment --
    ASSIGN = auto()         # =
    PLUS_ASSIGN = auto()    # +=
    MINUS_ASSIGN = auto()   # -=
    STAR_ASSIGN = auto()    # *=
    SLASH_ASSIGN = auto()   # /=

    # -- Comparison --
    EQUAL = auto()          # ==
    NOT_EQUAL = auto()      # !=
    LESS = auto()           # <
    GREATER = auto()        # >
    LESS_EQUAL = auto()     # <=
    GREATER_EQUAL = auto()  # >=

    # -- Special operators --
    ARROW = auto()          # =>
    PIPE = auto()           # |>
    QUESTION = auto()       # ?
    DOT = auto()            # .
    DOTDOT = auto()         # ..
    DOTDOT_EQUAL = auto()   # ..=
    SPREAD = auto()         # ...
    AT = auto()             # @
    HASH = auto()           # #
    NULLISH = auto()        # ??
    OPTIONAL_CHAIN = auto() # ?.

    # -- Delimiters --
    LPAREN = auto()         # (
    RPAREN = auto()         # )
    LBRACKET = auto()       # [
    RBRACKET = auto()       # ]
    LBRACE = auto()         # {
    RBRACE = auto()         # }
    COMMA = auto()          # ,
    COLON = auto()          # :

    # -- Structural --
    NEWLINE = auto()
    EOF = auto()


# ---------------------------------------------------------------------------
# Reserved keywords – 52 words
# ---------------------------------------------------------------------------

KEYWORDS: frozenset[str] = frozenset({
    # I/O
    "say", "ask",
    # Variables
    "make", "keep", "mut", "drop", "global",
    # Functions & classes
    "build", "send", "model", "self", "base", "new",
    # Control flow
    "when", "alt", "else", "each", "repeat", "in",
    "unless", "until", "match", "case",
    "stop", "skip", "pass",
    # Error handling
    "attempt", "rescue", "fail", "always",
    # Modules
    "use", "take", "share", "as",
    # Scope / misc
    "do", "end", "with", "defer", "guard",
    # Literals
    "true", "false", "none",
    # Logical
    "and", "or", "not",
    # Type / identity
    "is", "has", "typeof",
    # Async
    "async", "await", "yield",
})


# ---------------------------------------------------------------------------
# Token data class
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class Token:
    """A single token produced by the lexer."""

    type: TokenType
    value: str
    line: int       # 1-indexed
    column: int     # 1-indexed

    def __repr__(self) -> str:
        return f"Token({self.type.name}, {self.value!r}, L{self.line}:{self.column})"
