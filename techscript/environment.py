"""TechScript Environment — variable scope chain with constant support."""

from __future__ import annotations

from techscript.errors import NameErr, RuntimeErr


class Environment:
    """A lexical scope that chains to an optional parent scope.

    Variables sit in ``self.vars``; constants are tracked in ``self._consts``
    so that reassignment can be rejected at runtime.
    """

    def __init__(self, parent: Environment | None = None) -> None:
        self.vars: dict[str, object] = {}
        self._consts: set[str] = set()
        self.parent = parent

    # -- read --

    def get(self, name: str) -> object:
        """Look up *name* walking the scope chain upward."""
        if name in self.vars:
            return self.vars[name]
        if self.parent is not None:
            return self.parent.get(name)
        raise NameErr(f"Undefined variable: '{name}'")

    def has(self, name: str) -> bool:
        if name in self.vars:
            return True
        if self.parent is not None:
            return self.parent.has(name)
        return False

    # -- write --

    def set(self, name: str, value: object) -> None:
        """Bind *name* in **this** scope (create or overwrite)."""
        if name in self._consts:
            raise RuntimeErr(f"Cannot reassign constant '{name}'")
        self.vars[name] = value

    def set_const(self, name: str, value: object) -> None:
        self.vars[name] = value
        self._consts.add(name)

    def update(self, name: str, value: object) -> None:
        """Update an **existing** variable, walking up if needed."""
        if name in self.vars:
            if name in self._consts:
                raise RuntimeErr(f"Cannot reassign constant '{name}'")
            self.vars[name] = value
        elif self.parent is not None:
            self.parent.update(name, value)
        else:
            # Implicit creation in current scope
            self.vars[name] = value

    def delete(self, name: str) -> None:
        if name in self.vars:
            del self.vars[name]
            self._consts.discard(name)
        elif self.parent is not None:
            self.parent.delete(name)
        else:
            raise NameErr(f"Cannot delete undefined variable: '{name}'")

    def all_names(self) -> list[str]:
        """Return all reachable variable names (for suggestions)."""
        names = list(self.vars.keys())
        if self.parent:
            names.extend(self.parent.all_names())
        return names
