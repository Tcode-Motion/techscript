#!/usr/bin/env bash
# TechScript Official Examples Verification Script (Bash)
set -e

echo "==================================================="
echo "   TechScript Official Examples Verification (Bash)"
echo "==================================================="
echo ""

TSC_BIN="./target/release/tsc"
if [ ! -f "$TSC_BIN" ]; then
    TSC_BIN="./target/release/tsc.exe"
fi
if [ ! -f "$TSC_BIN" ]; then
    TSC_BIN="./target/debug/tsc"
fi
if [ ! -f "$TSC_BIN" ]; then
    TSC_BIN="./target/debug/tsc.exe"
fi

if [ ! -f "$TSC_BIN" ]; then
    echo "[ERROR] Compiled tsc binary not found! Run 'cargo build --release' first."
    exit 1
fi

echo "Using binary: $TSC_BIN"
echo ""

PASSED=0
FAILED=0
TOTAL=0

# Iterate over all .txs files in examples
while IFS= read -r txs_file; do
    TOTAL=$((TOTAL + 1))
    dir_name=$(dirname "$txs_file")
    expected_file="$dir_name/expected.txt"
    
    printf "Verifying %s ... " "$txs_file"
    
    if output=$("$TSC_BIN" run "$txs_file" 2>&1); then
        if [ -f "$expected_file" ]; then
            expected_content=$(tr -d '\r' < "$expected_file" | sed -e 's/[[:space:]]*$//')
            actual_content=$(echo "$output" | tr -d '\r' | sed -e 's/[[:space:]]*$//')
            if [ "$expected_content" = "$actual_content" ]; then
                echo "OK"
                PASSED=$((PASSED + 1))
            else
                echo "FAILED (Output mismatch)"
                FAILED=$((FAILED + 1))
            fi
        else
            echo "OK"
            PASSED=$((PASSED + 1))
        fi
    else
        echo "FAILED (Execution error)"
        FAILED=$((FAILED + 1))
    fi
done < <(find examples -type f -name "*.txs" | sort)

echo ""
echo "==================================================="
echo "Summary: $PASSED / $TOTAL passed, $FAILED failed."
echo "==================================================="

if [ $FAILED -ne 0 ]; then
    exit 1
fi
exit 0
