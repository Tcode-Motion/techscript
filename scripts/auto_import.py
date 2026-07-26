import re
from pathlib import Path

ROOT = Path(__file__).parent.parent
EXAMPLES_DIR = ROOT / "examples"

MODULES = [
    "math", "json", "http", "crypto", "string", "file", "path", "env", 
    "process", "os", "time", "net", "csv", "xml", "yaml", "toml", 
    "regex", "hash", "compress", "uuid", "database", "sqlite"
]

def auto_import_file(path: Path):
    if "compat" in path.parts:
        return
        
    content = path.read_text(encoding="utf-8")
    
    # Find which modules are used in the file as a prefix (e.g., `math.abs`)
    used_modules = []
    for mod in MODULES:
        # Match `mod.something` but not as part of another word, and not inside a comment
        # We can do a simple regex check
        pattern = r'(?<!\w)' + re.escape(mod) + r'\.'
        # Check non-comment lines
        lines = content.splitlines()
        for line in lines:
            if line.strip().startswith('#'):
                continue
            if re.search(pattern, line):
                used_modules.append(mod)
                break
                
    if not used_modules:
        return
        
    # Check which ones are already imported
    already_imported = set()
    for line in content.splitlines():
        m = re.match(r'^use\s+(\w+)', line.strip())
        if m:
            already_imported.add(m.group(1))
            
    to_import = [mod for mod in used_modules if mod not in already_imported]
    if not to_import:
        return
        
    print(f"Adding imports {to_import} to {path.relative_to(ROOT)}")
    
    # Prepend the use statements at the top, after any comments
    lines = content.splitlines()
    insert_idx = 0
    while insert_idx < len(lines) and (lines[insert_idx].strip().startswith('#') or not lines[insert_idx].strip()):
        insert_idx += 1
        
    import_lines = [f"use {mod}" for mod in sorted(to_import)]
    
    new_lines = lines[:insert_idx] + import_lines + [""] + lines[insert_idx:]
    path.write_text("\n".join(new_lines) + "\n", encoding="utf-8")

def main():
    for path in EXAMPLES_DIR.rglob("*.txs"):
        auto_import_file(path)

if __name__ == "__main__":
    main()
