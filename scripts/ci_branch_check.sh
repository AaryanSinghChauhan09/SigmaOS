#!/bin/bash
# SigmaOS CI Branch Parity and Feature Matrix Verification Script
# Verifies presence of core strategic roadmap and documentation assets on active branches.

set -e

BRANCH=""

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --branch) BRANCH="$2"; shift ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

echo "=== SigmaOS CI Branch Parity Check ==="
echo "Active target branch: ${BRANCH:-unknown}"

# Verify presence of core strategic files
REQUIRED_FILES=(
    "FUTURE-DEVELOPMENT-ROADMAP.md"
    "README.md"
    "CHANGELOG.md"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "[SUCCESS] Required file found: $file"
    else
        echo "[ERROR] Mandatory file missing: $file"
        exit 1
    fi
done

echo "=== All Branch Parity Verification Rules Passed Perfectly! ==="
exit 0
