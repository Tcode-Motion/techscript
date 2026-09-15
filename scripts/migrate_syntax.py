#!/usr/bin/env python3
"""
TechScript 2.0 — Final Syntax Freeze Migration Script
Migrates all .txs files (except examples/compat/) to canonical 2.0 syntax.
Also adds LEGACY COMPAT TEST headers to examples/compat/ files.

Frozen decisions:
  - do / send / when / loop / repeat / for / try / catch / class / use / end
  - null (canonical), none (silent alias)
  - $"..." interpolation (canonical), f"..." deprecated
  - math.abs() qualified calls; say/ask/env/file implicit builtins
  - No semicolons, no braces, no make/build/return/attempt/give
"""

import re
import os
import sys
from pathlib import Path

ROOT = Path(__file__).parent.parent  # project root (one level up from scripts/)

COMPAT_HEADER = """\
# LEGACY COMPATIBILITY TEST
# This file intentionally uses deprecated TechScript syntax.
# It verifies backward compatibility.
# Expected: Successful compilation + TSW100x warnings.
"""

TRANSFORMS = [
    # ── Remove semicolons at end of lines ──────────────────────────────────
    (r';(\s*)$', r'\1', re.MULTILINE),

    # ── std.io.println / std.io.print → say ────────────────────────────────
    (r'std\.io\.println\((.+?)\)', r'say \1', 0),
    (r'std\.io\.print\((.+?)\)', r'say \1', 0),

    # ── std.xxx.yyy() calls → module.yyy() ─────────────────────────────────
    (r'std\.math\.', r'math.', 0),
    (r'std\.strings\.', r'string.', 0),
    (r'std\.fs\.', r'file.', 0),
    (r'std\.path\.', r'path.', 0),
    (r'std\.env\.', r'env.', 0),
    (r'std\.process\.', r'process.', 0),
    (r'std\.os\.', r'os.', 0),
    (r'std\.time\.', r'time.', 0),
    (r'std\.net\.', r'net.', 0),
    (r'std\.json\.', r'json.', 0),
    (r'std\.csv\.', r'csv.', 0),
    (r'std\.xml\.', r'xml.', 0),
    (r'std\.yaml\.', r'yaml.', 0),
    (r'std\.toml\.', r'toml.', 0),
    (r'std\.regex\.', r'regex.', 0),
    (r'std\.hash\.', r'hash.', 0),
    (r'std\.crypto\.', r'crypto.', 0),
    (r'std\.uuid\.', r'uuid.', 0),
    (r'std\.http\.', r'http.', 0),
    (r'std\.db\.', r'database.', 0),
    (r'std\.sqlite\.', r'sqlite.', 0),

    # ── f"..." → $"..." ────────────────────────────────────────────────────
    (r'\bf"', r'$"', 0),

    # ── import → use ───────────────────────────────────────────────────────
    (r'^import\s+(\S+)', r'use \1', re.MULTILINE),
    (r'^from\s+(\S+)\s+import\s+(.+)', r'use \1', re.MULTILINE),

    # ── make/let/var x = → x = ─────────────────────────────────────────────
    (r'^\s*make\s+(\w+)\s*=', lambda m: m.group(0).replace('make ', ''), re.MULTILINE),
    (r'^\s*let\s+(\w+)\s*=', lambda m: m.group(0).replace('let ', ''), re.MULTILINE),
    (r'^\s*var\s+(\w+)\s*=', lambda m: m.group(0).replace('var ', ''), re.MULTILINE),
]

# These need careful line-by-line handling
BLOCK_TRANSFORMS = [
    # build fn(...) { → do fn(...)
    (re.compile(r'^(\s*)build\s+(\w+)\s*\(([^)]*)\)\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}do {m.group(2)}({m.group(3)})"),
    # build fn(...) with params then → do fn(...)
    (re.compile(r'^(\s*)build\s+(\w+)\s+with\s+([^{then]+?)(?:\s+then)?\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}do {m.group(2)}({m.group(3).strip()})"),
    # return x → send x
    (re.compile(r'^(\s*)return\s*(.*?)\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}send {m.group(2)}" if m.group(2) else f"{m.group(1)}send"),
    # give x → send x
    (re.compile(r'^(\s*)give\s+(.*?)\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}send {m.group(2)}"),
    # attempt { → try
    (re.compile(r'^(\s*)attempt\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}try"),
    # } catch e { or } catch e → catch e
    (re.compile(r'^(\s*)\}\s*catch\s+(\w+)\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}catch {m.group(2)}"),
    # catch e { (standalone) → catch e
    (re.compile(r'^(\s*)catch\s+(\w+)\s*\{\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}catch {m.group(2)}"),
    # if cond { → when cond
    (re.compile(r'^(\s*)if\s+(.+?)\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}when {m.group(2)}"),
    # } else if / } else { → else when / else
    (re.compile(r'^(\s*)\}\s*else\s+if\s+(.+?)\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}else when {m.group(2)}"),
    (re.compile(r'^(\s*)\}\s*else\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}else"),
    # else { standalone → else
    (re.compile(r'^(\s*)else\s*\{\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}else"),
    # while cond { → repeat cond
    (re.compile(r'^(\s*)while\s+(.+?)\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}repeat {m.group(2)}"),
    # for/each x in y { → for x in y
    (re.compile(r'^(\s*)(?:for|each)\s+(\w+)\s+in\s+(.+?)\s*(?:then)?\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}for {m.group(2)} in {m.group(3)}"),
    # model Name { → class Name
    (re.compile(r'^(\s*)model\s+(\w+)\s*\{?\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}class {m.group(2)}"),
    # spawn_async(fn) → async ... end (best-effort note)
    # Standalone closing } → end
    (re.compile(r'^(\s*)\}\s*$', re.MULTILINE),
     lambda m: f"{m.group(1)}end"),
]


def migrate_content(content: str) -> str:
    """Apply all migration transforms to file content."""
    # Apply regex transforms
    for pattern, replacement, flags in TRANSFORMS:
        if callable(replacement):
            content = re.sub(pattern, replacement, content, flags=flags)
        else:
            content = re.sub(pattern, replacement, content, flags=flags)

    # Apply block transforms (order matters)
    for pattern, replacement in BLOCK_TRANSFORMS:
        content = pattern.sub(replacement, content)

    # Clean up: collapse multiple blank lines into max 2
    content = re.sub(r'\n{3,}', '\n\n', content)

    # Ensure file ends with single newline
    content = content.rstrip('\n') + '\n'

    return content


def should_skip(path: Path) -> bool:
    """Skip compat examples — keep them in legacy syntax."""
    return 'compat' in path.parts


def add_compat_header(content: str) -> str:
    """Add legacy compat header if not already present."""
    if 'LEGACY COMPATIBILITY TEST' in content:
        return content
    return COMPAT_HEADER + '\n' + content


def process_file(path: Path, dry_run: bool = False) -> tuple[bool, str]:
    """Process a single .txs file. Returns (changed, reason)."""
    original = path.read_text(encoding='utf-8', errors='replace')

    if should_skip(path):
        # Add compat header only
        new_content = add_compat_header(original)
        changed = new_content != original
        if changed and not dry_run:
            path.write_text(new_content, encoding='utf-8')
        return changed, 'compat header added'

    new_content = migrate_content(original)
    changed = new_content != original
    if changed and not dry_run:
        path.write_text(new_content, encoding='utf-8')
    return changed, 'migrated' if changed else 'already canonical'


def migrate_directory(base: Path, dry_run: bool = False):
    files = sorted(base.rglob('*.txs'))
    print(f"\nFound {len(files)} .txs files under {base.relative_to(ROOT)}")
    changed_count = 0
    for f in files:
        rel = f.relative_to(ROOT)
        changed, reason = process_file(f, dry_run)
        status = 'CHANGED' if changed else 'ok'
        print(f"  [{status}] {rel} ({reason})")
        if changed:
            changed_count += 1
    print(f"  => {changed_count}/{len(files)} files updated\n")
    return changed_count


def migrate_templates(dry_run: bool = False):
    """Rewrite templates with canonical starter content."""
    templates = {
        ROOT / 'templates/console/src/main.txs': '# Console app starter\nsay "Hello, TechScript!"\n',
        ROOT / 'templates/empty/src/main.txs':   '# Empty starter\nsay "Hello"\n',
        ROOT / 'templates/cli/src/main.txs': (
            '# CLI app starter\nuse env\n\nargs = env "ARGS"\nsay args\n'
        ),
        ROOT / 'templates/web/src/main.txs': (
            '# Web app starter\nuse web\n\npage "/"\n\n    title "My App"\n\n    hero\n\n        heading "TechScript"\n\n        subtitle "Simple as English"\n\n        button "Get Started"\n\n    end\n\nend\n\nstart\n'
        ),
        ROOT / 'templates/gui/src/main.txs': (
            '# GUI app starter\nuse gui\n\nwindow\n\n    title "My App"\n\n    size 800 600\n\n    button "OK"\n\nend\n\nshow\n'
        ),
        ROOT / 'templates/library/src/lib.txs': (
            '# Library starter\n\ndo greet(name)\n\n    send "Hello " + name\n\nend\n'
        ),
        ROOT / 'templates/package/src/main.txs': (
            '# Package starter\nsay "TechScript Package"\n'
        ),
        ROOT / 'templates/workspace/packages/core/src/lib.txs': (
            '# Workspace core library\n\ndo version()\n\n    send "2.0.0"\n\nend\n'
        ),
    }

    print(f"\nMigrating {len(templates)} templates...")
    for path, content in templates.items():
        old = path.read_text(encoding='utf-8') if path.exists() else ''
        changed = old != content
        if changed and not dry_run:
            path.write_text(content, encoding='utf-8')
        status = 'CHANGED' if changed else 'ok'
        print(f"  [{status}] {path.relative_to(ROOT)}")


if __name__ == '__main__':
    dry_run = '--dry-run' in sys.argv
    if dry_run:
        print("DRY RUN — no files will be written\n")

    print("=" * 60)
    print("TechScript 2.0 Final Syntax Freeze — Migration Script")
    print("=" * 60)

    total = 0
    total += migrate_directory(ROOT / 'examples', dry_run=dry_run)
    migrate_templates(dry_run=dry_run)

    print("=" * 60)
    print(f"Migration complete. {total} example files updated.")
    print("Next: run `cargo build --release` and `python verify_all_examples.py`")
