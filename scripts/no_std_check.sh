#!/usr/bin/env bash
# scripts/no_std_check.sh
# Scans kernel, drivers, and core subsystems for illegal 'use std::' statements outside tests.

set -e

echo "Running no_std enforcement audit across core modules..."

SEARCH_DIRS=("kernel" "drivers" "klib")
VIOLATIONS=0

for dir in "${SEARCH_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        # Search for 'use std::' ignoring test blocks or test configs where possible
        FOUND=$(grep -rn "use std::" "$dir" | grep -v "#\[cfg(test)\]" | grep -v "//" || true)
        if [ -n "$FOUND" ]; then
            echo "WARNING: Potential std:: usage detected in $dir:"
            echo "$FOUND"
            # Filter out known test files if any
            UNALLOWED=$(echo "$FOUND" | grep -v "test" || true)
            if [ -n "$UNALLOWED" ]; then
                VIOLATIONS=$((VIOLATIONS + 1))
            fi
        fi
    fi
done

if [ "$VIOLATIONS" -gt 0 ]; then
    echo "FAILED: Found $VIOLATIONS directory/directories with unallowed std:: imports in no_std modules."
    exit 1
fi

echo "SUCCESS: no_std enforcement check passed. No unallowed std:: imports found."
exit 0
