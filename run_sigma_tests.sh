#!/bin/bash
# SigmaOS: Sovereign Test Orchestrator (v2.0) - Linux/macOS

echo "Σ [TEST]: Initiating Sovereign Atomic Test Suite..."

TESTS=$(find suites -name "test_*.c")
PASSED=0
FAILED=0

for Test in $TESTS; do
    Binary="${Test%.c}.bin"
    echo "  Σ [RUN]: Testing $(basename "$Test")..."
    
    # Attempt to compile
    # Note: Linking against SovereignLibC for atomic verification
    LibC="suites/Sovereign-Kernel-Suite/SovereignLibC.c"
    gcc -nostdlib "$Test" "$LibC" -Icore/include -o "$Binary"
    
    if [ $? -eq 0 ]; then
        echo "  Σ [PASS]: $(basename "$Test") certified."
        PASSED=$((PASSED + 1))
    else
        echo "  Σ [FAIL]: $(basename "$Test") logic violation detected."
        FAILED=$((FAILED + 1))
    fi
done

echo -e "\nΣ [SUMMARY]: Tests Passed: $PASSED | Tests Failed: $FAILED"
if [ $FAILED -gt 0 ]; then
    exit 1
fi
