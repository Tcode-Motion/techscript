#!/usr/bin/env bash
# TechScript Skill Validation Helper (POSIX)
set -e

TEMP_FILE=$(mktemp /tmp/verify_techscript_skill.XXXXXX.txs)
cat << 'EOF' > "$TEMP_FILE"
constant message is "TechScript AI Skill Verified!"
say "{message}"
EOF

TSC_BIN="./target/release/tsc"
if [ ! -f "$TSC_BIN" ]; then
    TSC_BIN="./target/release/tsc.exe"
fi
if [ ! -f "$TSC_BIN" ]; then
    TSC_BIN="tsc"
fi

echo "Running verification program with 'tsc run'..."
"$TSC_BIN" run "$TEMP_FILE"

rm -f "$TEMP_FILE"
echo "✓ TechScript compiler verified and operational."
