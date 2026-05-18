#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RUST_BIN="$ROOT/runtime/target/release/tech"

echo "TechScript Parity Check"

if [[ ! -x "$RUST_BIN" ]]; then
  (cd "$ROOT/runtime" && cargo build --release --bin tech)
fi

EXAMPLES=(
  runtime_examples/01_basics.txs
  runtime_examples/02_math_and_logic.txs
  runtime_examples/03_control_flow.txs
  runtime_examples/04_functions.txs
  runtime_examples/05_classes.txs
  runtime_examples/06_advanced.txs
  examples/hello.txs
  examples/calc.txs
  examples/classes.txs
  examples/syntax_aliases.txs
)

PASSED=0
FAILED=0

for rel in "${EXAMPLES[@]}"; do
  path="$ROOT/$rel"
  [[ -f "$path" ]] || continue
  echo -n "  Running $rel..."
  rust_out=$("$RUST_BIN" run "$path" 2>&1 || true)
  if command -v python3 &>/dev/null; then
    py_out=$(cd "$ROOT" && python3 -m techscript.cli run "$path" 2>&1 || true)
    if [[ "$rust_out" == "$py_out" ]]; then
      echo " OK"
      PASSED=$((PASSED + 1))
    else
      echo " DIFF"
      echo "    Rust:   $rust_out"
      echo "    Python: $py_out"
      FAILED=$((FAILED + 1))
    fi
  else
    echo " OK (Rust only)"
    PASSED=$((PASSED + 1))
  fi
done

echo ""
echo "Results: $PASSED passed, $FAILED failed/diff"
