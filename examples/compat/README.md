# TechScript v1.0.8 Compatibility Test Suite

This directory contains `.txs` scripts that verify backward-compatibility
between TechScript **v1.0.8** and **2.0**.

## Files

| File | What it tests |
|------|---------------|
| `01_keywords.txs` | Alias keywords: `keep`, `give`, `stop`, `skip`, `each`, `when`, `attempt`, `be`, `equals`, `typeof`, `then`/`end` |
| `02_range.txs` | `..` is inclusive on both ends (`1..5` → `[1,2,3,4,5]`) |
| `03_error_handling.txs` | `err` in `catch` blocks is a Map with `.message` and `.kind` |
| `04_model_init.txs` | Model `init` body is called when the model is constructed |
| `05_modules.txs` | `math.*`, `random.*` etc. accessible without a prior `import` |
| `06_repeat.txs` | `repeat condition` is a while-loop; `repeat N` is a count shorthand |
| `07_mixed_dialect.txs` | v1.0.8 and v2.0 syntax mixed freely in one file |

## Running

```bash
# Run all compat examples
for f in examples/compat/*.txs; do
  echo "=== $f ==="
  tech run "$f"
done
```

## Expected Behavior per v1.0.8 Spec

- `1..5` yields `[1, 2, 3, 4, 5]` (inclusive)
- `typeof "x"` → `"str"`, `typeof {}` → `"dict"`
- `err.message` in catch blocks holds the error text
- `math.sqrt`, `random.randint` etc. work without import
- `then...end` and `{...}` blocks are interchangeable
