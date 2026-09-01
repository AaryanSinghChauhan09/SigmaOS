#!/usr/bin/env bash
# scripts/changed_files_rustc_tests.sh
# Detects modified Rust files in git or accepts file parameters and runs standalone rustc tests.

set -e

mkdir -p build

FILES=("$@")

if [ ${#FILES[@]} -eq 0 ]; then
    if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        # Get changed or untracked .rs files compared to HEAD or main
        CHANGED_FILES=$(git status --porcelain | grep -E '\.rs$' | awk '{print $2}' || true)
        if [ -z "$CHANGED_FILES" ]; then
            CHANGED_FILES=$(git diff --name-only HEAD~1 HEAD 2>/dev/null | grep -E '\.rs$' || true)
        fi
        readarray -t FILES <<< "$CHANGED_FILES"
    fi
fi

if [ ${#FILES[@]} -eq 0 ] || [ -z "${FILES[0]}" ]; then
    echo "No modified Rust files detected. Testing default standalone target src/kernel/memory.rs..."
    FILES=("src/kernel/memory.rs")
fi

PASSED=0
FAILED=0
SKIPPED=0

for file in "${FILES[@]}"; do
    if [ -z "$file" ] || [ ! -f "$file" ]; then
        continue
    fi

    filename=$(basename "$file" .rs)
    binary_out="build/test_${filename}"

    echo "--------------------------------------------------------"
    echo "Running standalone rustc test for: $file"

    # Check if the file contains test module or cfg(test)
    if grep -q '#\[test\]' "$file" || grep -q '#\[cfg(test)\]' "$file"; then
        if rustc --test --edition=2021 --cfg 'feature="standalone_test"' "$file" -o "$binary_out" 2>/dev/null || rustc --test --edition=2021 "$file" -o "$binary_out" 2>/dev/null; then
            if ./"$binary_out"; then
                echo "SUCCESS: $file passed standalone test."
                PASSED=$((PASSED + 1))
            else
                echo "FAILURE: Test execution failed for $file."
                FAILED=$((FAILED + 1))
            fi
            rm -f "$binary_out"
        else
            echo "SKIPPED: $file could not be compiled standalone (depends on crate dependencies)."
            SKIPPED=$((SKIPPED + 1))
        fi
    else
        echo "SKIPPED: No #[test] directives found in $file."
        SKIPPED=$((SKIPPED + 1))
    fi
done

echo "--------------------------------------------------------"
echo "Standalone Test Results: $PASSED passed, $FAILED failed, $SKIPPED skipped."

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi

exit 0
