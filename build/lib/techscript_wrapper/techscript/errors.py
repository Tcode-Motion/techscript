"""TechScript error hierarchy and pretty error formatting.

Provides typed error classes and a ``format_error`` helper that produces
coloured, user-friendly messages including *"Did you mean …?"* suggestions
powered by ``difflib.get_close_matches``.
"""

from __future__ import annotations

from difflib import get_close_matches
from typing import Sequence


# ---------------------------------------------------------------------------
# Error hierarchy
# ---------------------------------------------------------------------------

class TechScriptError(Exception):
    """Base error for all TechScript runtime/compile errors."""

    def __init__(self, message: str, line: int | None = None, column: int | None = None):
        self.message = message
        self.line = line
        self.column = column
        super().__init__(message)


class LexerError(TechScriptError):
    """Raised by the lexer on invalid input."""


class ParseError(TechScriptError):
    """Raised by the parser on unexpected tokens."""


class NameErr(TechScriptError):
    """Undefined or misspelled name."""


class TypeErr(TechScriptError):
    """Type mismatch."""


class ValueErr(TechScriptError):
    """Invalid value."""


class IndexErr(TechScriptError):
    """Index out of bounds."""


class KeyErr(TechScriptError):
    """Key not found in map."""


class FileErr(TechScriptError):
    """File system error."""


class ImportErr(TechScriptError):
    """Module import error."""


class RuntimeErr(TechScriptError):
    """General runtime error (stack overflow, division by zero, …)."""


# ---------------------------------------------------------------------------
# "Did you mean …?" suggestion engine
# ---------------------------------------------------------------------------

def suggest_correction(
    unknown: str,
    known_words: Sequence[str],
    max_results: int = 3,
    cutoff: float = 0.6,
) -> list[str]:
    """Return a list of close matches for *unknown* from *known_words*."""
    return get_close_matches(unknown, known_words, n=max_results, cutoff=cutoff)


# ---------------------------------------------------------------------------
# Pretty error formatter
# ---------------------------------------------------------------------------

def format_error(
    error: TechScriptError,
    source_lines: list[str] | None = None,
    known_names: Sequence[str] | None = None,
) -> str:
    """Return a nicely formatted, multi-line error string.

    Parameters
    ----------
    error:
        The TechScript error to format.
    source_lines:
        The source code split into lines (for contextual display).
    known_names:
        Optional vocabulary to use for "Did you mean …?" hints.
    """
    err_type = type(error).__name__
    # Make class names friendlier
    friendly = {
        "NameErr": "NameError",
        "TypeErr": "TypeError",
        "ValueErr": "ValueError",
        "IndexErr": "IndexError",
        "KeyErr": "KeyError",
        "FileErr": "FileError",
        "ImportErr": "ImportError",
        "RuntimeErr": "RuntimeError",
        "LexerError": "SyntaxError",
        "ParseError": "SyntaxError",
    }
    display_type = friendly.get(err_type, err_type)

    parts: list[str] = [
        "",
        "╭─ TechScript Error ─────────────────────────────────",
        "│",
        f"│  {display_type}: {error.message}",
        "│",
    ]

    # Show source context
    if error.line and source_lines:
        ln = error.line
        if 0 < ln <= len(source_lines):
            code = source_lines[ln - 1]
            parts.append(f"│    {ln} │  {code}")
            if error.column and error.column > 0:
                pointer = " " * (error.column - 1) + "^^^"
                parts.append(f"│      │  {pointer}")
            parts.append("│")

    # "Did you mean …?" suggestions
    if known_names and isinstance(error, (NameErr, ParseError)):
        # Try to extract the misspelled word from the message
        word = ""
        if "'" in error.message:
            try:
                word = error.message.split("'")[1]
            except IndexError:
                pass
        if word:
            suggestions = suggest_correction(word, list(known_names))
            if suggestions:
                parts.append(f"│  Did you mean: {suggestions[0]}?")
                parts.append("│")

    parts.append("╰─────────────────────────────────────────────────────")
    return "\n".join(parts)
