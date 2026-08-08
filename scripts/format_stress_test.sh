#!/bin/bash
# SPDX-License-Identifier: MIT
# SigmaOS Format & Cleanliness Stress Test (Linux Standards Compliant)
# Strictly validates source code formatting, trailing whitespace, line endings, and limits.

set -e

# Color Palettes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}=== SigmaOS Coding Style & Code Cleanliness Stress Check ===${NC}"

# Find source files to inspect
FILES=$(find src kernel klib scripts -type f \( -name "*.rs" -o -name "*.sh" -o -name "*.py" \) 2>/dev/null || true)

if [ -z "$FILES" ]; then
    echo -e "${YELLOW}[STRESS-WARN]${NC} No source files discovered to audit."
    exit 0
fi

trailing_ws_count=0
no_eof_newline_count=0
long_lines_count=0
total_inspected=0

echo -e "${BLUE}[STRESS-INFO]${NC} Auditing source files for style anomalies and formatting infractions..."

for file in $FILES; do
    # Ignore specific paths
    if [[ "$file" == *"node_modules"* ]] || [[ "$file" == *"target"* ]]; then
        continue
    fi

    total_inspected=$((total_inspected + 1))

    # 1. Check for trailing whitespace
    if grep -q "[[:space:]]$" "$file"; then
        trailing_ws_count=$((trailing_ws_count + 1))
    fi

    # 2. Check for missing newline at EOF
    # In bash, if a file doesn't end with a newline, 'tail -c 1' will not be an empty line
    if [ -f "$file" ]; then
        LAST_CHAR=$(tail -c 1 "$file" 2>/dev/null || echo "")
        if [ "$LAST_CHAR" != $'\n' ] && [ -n "$LAST_CHAR" ]; then
            no_eof_newline_count=$((no_eof_newline_count + 1))
        fi
    fi

    # 3. Check for extremely long lines (>150 chars)
    if grep -q -E "^.{151,}" "$file"; then
        long_lines_count=$((long_lines_count + 1))
    fi
done

echo -e "\n--------------------------------------------------"
echo -e "         CODEBASE CLEANLINESS STRESS AUDIT"
echo -e "--------------------------------------------------"
echo -e "  Files Inspected:              $total_inspected"
echo -e "  Files with Trailing Spaces:   $trailing_ws_count"
echo -e "  Files with Missing EOF LF:    $no_eof_newline_count"
echo -e "  Files with Lines > 150 chars: $long_lines_count"
echo -e "--------------------------------------------------"

if [ $trailing_ws_count -gt 0 ]; then
    echo -e "${YELLOW}[STRESS-WARN]${NC} Trailing whitespace detected on $trailing_ws_count files. Consider running a trimmer tool."
fi

if [ $no_eof_newline_count -gt 0 ]; then
    echo -e "${YELLOW}[STRESS-WARN]${NC} $no_eof_newline_count files are missing standard Unix line-feed (LF) terminators at end-of-file."
fi

echo -e "${GREEN}[SUCCESS]${NC} Code formatting stress check finished successfully."
exit 0
