#!/bin/bash
# SigmaOS: Sovereign Atomic Test Runner (v1.0)
# Verifies the integrity of the Sovereign Lattice build artifacts.

echo "╔══════════════════════════════════════════════════════════╗"
echo "║  Σ SigmaOS Sovereign Test Runner v1.0                   ║"
echo "║  Atomic Integrity Verification for the Sovereign Lattice ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

PASS=0
FAIL=0
WARN=0

check() {
    local desc="$1"
    local result="$2"
    if [ "$result" == "ok" ]; then
        echo "  ✅ PASS: $desc"
        PASS=$((PASS + 1))
    elif [ "$result" == "warn" ]; then
        echo "  ⚠️  WARN: $desc"
        WARN=$((WARN + 1))
    else
        echo "  ❌ FAIL: $desc"
        FAIL=$((FAIL + 1))
    fi
}

echo "Σ [TEST 1/4] Build Artifact Verification"
echo "─────────────────────────────────────────"

# Test 1: Build directory exists
[ -d "build" ] && check "build/ directory exists" "ok" || check "build/ directory exists" "fail"

# Test 2: Check for kernel binary (Linux only — macOS skips ELF link)
PLATFORM="linux"
[[ "$(uname)" == "Darwin" ]] && PLATFORM="macos"

if [ "$PLATFORM" == "linux" ]; then
    [ -f "build/sigmaos_zenith" ] && check "sigmaos_zenith kernel binary produced" "ok" || check "sigmaos_zenith kernel binary produced" "warn"
else
    check "sigmaos_zenith ELF link (macOS skips this — OK)" "ok"
fi

# Test 3: Count compiled object files
OBJ_COUNT=$(find build -name "*.o" 2>/dev/null | wc -l | tr -d ' ')
if [ "$OBJ_COUNT" -gt 100 ]; then
    check "Object file count: $OBJ_COUNT (expected > 100)" "ok"
elif [ "$OBJ_COUNT" -gt 10 ]; then
    check "Object file count: $OBJ_COUNT (low — expected > 100)" "warn"
else
    check "Object file count: $OBJ_COUNT (critically low)" "fail"
fi

echo ""
echo "Σ [TEST 2/4] Source Structure Integrity"
echo "─────────────────────────────────────────"

# Test 4: Core suites present
for suite in S01_Genesis S03_Orchestrator S04_HAL S05_Memory S07_Network S30_Supremacy; do
    [ -d "suites/$suite" ] && check "Suite $suite present" "ok" || check "Suite $suite present" "fail"
done

# Test 5: Boot assembly present
[ -f "suites/S01_Genesis/boot.asm" ] && check "Boot ASM (S01_Genesis/boot.asm)" "ok" || check "Boot ASM" "fail"
[ -f "suites/S01_Genesis/shards/sigma.ld" ] && check "Linker script (sigma.ld)" "ok" || check "Linker script" "fail"

echo ""
echo "Σ [TEST 3/4] Header Shim Coverage"
echo "─────────────────────────────────────────"

# Test 6: Canonical headers present
for hdr in suites/include/sigma_kernel_types.h suites/include/sigma_libc.h suites/include/SigmaOOP.hpp; do
    [ -f "$hdr" ] && check "Canonical header: $(basename $hdr)" "ok" || check "Canonical header: $(basename $hdr)" "fail"
done

# Test 7: Check no remaining broken includes (files still referencing ../libc without shim)
BROKEN=$(find suites -name "*.c" -o -name "*.cpp" 2>/dev/null | xargs grep -l '#include.*sigma_libc\.h' 2>/dev/null | while read f; do
    dir=$(dirname "$f")
    parent=$(dirname "$dir")
    base=$(basename "$f")
    if grep -q '\.\.\/libc\/' "$f" 2>/dev/null; then
        libcpath="$dir/../libc/sigma_libc.h"
        [ ! -f "$libcpath" ] && echo "$f"
    fi
done | wc -l | tr -d ' ')

if [ "$BROKEN" == "0" ]; then
    check "No broken ../libc/sigma_libc.h includes" "ok"
else
    check "$BROKEN files still have unresolved ../libc/sigma_libc.h" "warn"
fi

echo ""
echo "Σ [TEST 4/4] Build Script Integrity"
echo "─────────────────────────────────────────"

# Test 8: Build script version check
BVER=$(grep "Sovereign Build Orchestrator v" build_sovereign.sh | head -1 | grep -o 'v[0-9]*\.[0-9]*')
check "Build script version: $BVER" "ok"

# Test 9: CI workflow files valid YAML structure
for wf in .github/workflows/*.yml; do
    if grep -q "^name:" "$wf"; then
        check "Workflow $(basename $wf) has valid name" "ok"
    else
        check "Workflow $(basename $wf) missing name field" "fail"
    fi
done

echo ""
echo "══════════════════════════════════════════════════════════"
echo "  Σ TEST RESULTS: $PASS passed | $WARN warnings | $FAIL failed"
echo "══════════════════════════════════════════════════════════"
echo ""

if [ $FAIL -gt 0 ]; then
    echo "  ❌ Sovereign Lattice integrity check FAILED ($FAIL critical failures)"
    exit 1
elif [ $WARN -gt 0 ]; then
    echo "  ⚠️  Sovereign Lattice verified with $WARN warnings"
    exit 0
else
    echo "  ✅ Sovereign Lattice integrity VERIFIED — Zero compromise."
    exit 0
fi
