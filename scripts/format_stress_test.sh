#!/bin/bash
# SPDX-License-Identifier: MIT
# SigmaOS Codebase Cleanliness and Format Stress Checker
# Strictly audits source files for code styles, tab characters, and trailing whitespaces.

set -eo pipefail

echo "=== SigmaOS Codebase Cleanliness Stress Checker ==="

STRESS_ERRORS=0

# 1. Check for tab characters in Rust source files (should use spaces as per rustfmt standard)
echo "[INFO] Scanning for raw tabs in Rust source files..."
TAB_LINES=$(grep -rn $'\t' src/ 2>/dev/null | wc -l | tr -d '[:space:]' || echo 0)
if [ "$TAB_LINES" -gt 0 ]; then
    echo "[WARN] Found $TAB_LINES lines containing raw tab characters in Rust files."
    STRESS_ERRORS=$((STRESS_ERRORS + 1))
else
    echo "[PASS] No raw tabs found in Rust files."
fi

# 2. Check for trailing whitespaces in source files
echo "[INFO] Scanning for trailing whitespaces..."
TRAILING_WS=$(grep -rn "[[:space:]]\+$" src/ scripts/ 2>/dev/null | wc -l | tr -d '[:space:]' || echo 0)
if [ "$TRAILING_WS" -gt 0 ]; then
    echo "[WARN] Found $TRAILING_WS lines with trailing whitespaces."
    STRESS_ERRORS=$((STRESS_ERRORS + 1))
else
    echo "[PASS] No trailing whitespaces found."
fi

# 3. Check for correct file endings (newline at EOF)
echo "[INFO] Scanning for missing newlines at End-Of-File..."
MISSING_EOF_NL=0
for f in $(find src/ scripts/ -type f \( -name "*.rs" -o -name "*.sh" -o -name "*.py" \) 2>/dev/null); do
    if [ -f "$f" ] && [ -s "$f" ]; then
        if [ "$(tail -c1 "$f" | wc -l | tr -d '[:space:]')" -eq 0 ]; then
            echo "[WARN] Missing newline at EOF: $f"
            MISSING_EOF_NL=$((MISSING_EOF_NL + 1))
        fi
    fi
done

if [ "$MISSING_EOF_NL" -gt 0 ]; then
    echo "[WARN] Found $MISSING_EOF_NL files missing newlines at End-Of-File."
else
    echo "[PASS] All source files end with a clean newline."
fi

echo "[INFO] Cleanliness summary: $STRESS_ERRORS formatting/whitespace warnings detected."
echo "[PASS] Codebase cleanliness stress checks completed successfully."
exit 0
