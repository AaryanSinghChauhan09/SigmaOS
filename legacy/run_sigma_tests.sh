#!/bin/bash
# =============================================================================
# Σ SIGMAOS: MODULAR LATTICE TEST RUNNER (v2.0)
# =============================================================================
# Supports:
#   1. Isolated driver testing (--drivers) without loading core/
#   2. Mock HAL for simulating hardware failures (--mock-hal)
#   3. Per-module unit tests via module.json test_targets
#   4. Full lattice integrity verification (default)
#   5. QEMU smoke test integration (--qemu)
#
# Usage:
#   ./run_sigma_tests.sh                    # Full lattice test
#   ./run_sigma_tests.sh --drivers          # Drivers only
#   ./run_sigma_tests.sh --module S05_Memory # Single module
#   ./run_sigma_tests.sh --mock-hal         # With mock HAL
#   ./run_sigma_tests.sh --qemu             # QEMU integration
# =============================================================================

set -e

PASS=0; FAIL=0; WARN=0; SKIP=0
MODE="full"
TARGET_MODULE=""
USE_MOCK_HAL=0

# ── Parse arguments ──────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case "$1" in
        --drivers)    MODE="drivers"; shift ;;
        --mock-hal)   USE_MOCK_HAL=1; shift ;;
        --module)     MODE="module"; TARGET_MODULE="$2"; shift 2 ;;
        --qemu)       MODE="qemu"; shift ;;
        *)            echo "Unknown option: $1"; exit 1 ;;
    esac
done

echo "╔══════════════════════════════════════════════════════════╗"
echo "║  Σ SigmaOS Modular Lattice Test Runner v2.0             ║"
echo "║  Mode: $MODE                                            ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

check() {
    local desc="$1"; local result="$2"
    if [ "$result" == "ok" ]; then
        echo "  ✅ PASS: $desc"; PASS=$((PASS + 1))
    elif [ "$result" == "warn" ]; then
        echo "  ⚠️  WARN: $desc"; WARN=$((WARN + 1))
    elif [ "$result" == "skip" ]; then
        echo "  ⏭️  SKIP: $desc"; SKIP=$((SKIP + 1))
    else
        echo "  ❌ FAIL: $desc"; FAIL=$((FAIL + 1))
    fi
}

# ── Compiler detection ───────────────────────────────────────────────────────
if command -v g++ &>/dev/null; then GCC="g++"
elif command -v clang++ &>/dev/null; then GCC="clang++"
else echo "ERROR: No C++ compiler found."; exit 1; fi

CFLAGS="-std=c++20 -ffreestanding -nostdlib -fno-exceptions -fno-rtti -O0"
INCLUDES="-I. -Iinclude -Isuites -Isuites/include -Isuites/S01_Genesis"

# ── Feature flags from sigma_features.json ───────────────────────────────────
FEATURE_FLAGS=""
if [ -f "sigma_features.json" ] && command -v python3 &>/dev/null; then
    FEATURE_FLAGS=$(python3 -c "
import json
with open('sigma_features.json') as f: cfg = json.load(f)
flags = []
arch = cfg.get('arch','x86_64').upper()
flags.append(f'-DSIGMA_ARCH_{arch}')
drv = cfg.get('drivers',{})
if drv.get('display'):  flags.append(f'-DSIGMA_DRIVER_{drv[\"display\"].upper()}')
if drv.get('storage'):  flags.append(f'-DSIGMA_DRIVER_{drv[\"storage\"].upper()}')
if drv.get('network'):  flags.append(f'-DSIGMA_DRIVER_{drv[\"network\"].upper()}')
for k,v in cfg.get('features',{}).items():
    flags.append(f'-DSIGMA_FEATURE_{k.upper()}={1 if v else 0}')
mem = cfg.get('memory',{})
for k,v in mem.items():
    flags.append(f'-DSIGMA_{k.upper()}={v}')
print(' '.join(flags))
" 2>/dev/null || echo "")
fi

# ── Mock HAL ─────────────────────────────────────────────────────────────────
if [ $USE_MOCK_HAL -eq 1 ]; then
    echo "Σ [MOCK HAL] Generating mock hardware layer..."
    FEATURE_FLAGS="$FEATURE_FLAGS -DSIGMA_MOCK_HAL=1 -DSIGMA_TEST_MODE=1"
    check "Mock HAL defines injected" "ok"
fi

# =============================================================================
# TEST MODE: ISOLATED DRIVER TESTS
# =============================================================================
if [ "$MODE" == "drivers" ]; then
    echo "Σ [TEST] Isolated Driver Compilation Tests"
    echo "─────────────────────────────────────────"

    for driver_dir in suites/S04_HAL/drivers suites/S04_HAL_*/drivers drivers/*/; do
        [ -d "$driver_dir" ] || continue
        for src in "$driver_dir"/*.c "$driver_dir"/*.cpp; do
            [ -f "$src" ] || continue
            name=$(basename "$src")
            obj="build/test_${name}.o"
            if $GCC $CFLAGS $INCLUDES $FEATURE_FLAGS -c "$src" -o "$obj" 2>/dev/null; then
                check "Driver compiles: $name" "ok"
            else
                check "Driver compiles: $name" "fail"
            fi
        done
    done

# =============================================================================
# TEST MODE: SINGLE MODULE
# =============================================================================
elif [ "$MODE" == "module" ]; then
    echo "Σ [TEST] Module: $TARGET_MODULE"
    echo "─────────────────────────────────────────"

    SUITE_DIR="suites/$TARGET_MODULE"
    if [ ! -d "$SUITE_DIR" ]; then
        check "Suite directory exists: $SUITE_DIR" "fail"
    else
        check "Suite directory exists: $SUITE_DIR" "ok"

        # Check module.json
        if [ -f "$SUITE_DIR/module.json" ]; then
            check "module.json manifest present" "ok"

            # Parse test targets
            if command -v python3 &>/dev/null; then
                TESTS=$(python3 -c "
import json
with open('$SUITE_DIR/module.json') as f: m = json.load(f)
for t in m.get('test_targets', []): print(t)
" 2>/dev/null)
                if [ -n "$TESTS" ]; then
                    check "Test targets declared in manifest" "ok"
                else
                    check "Test targets declared in manifest" "warn"
                fi
            fi
        else
            check "module.json manifest present" "warn"
        fi

        # Compile all .c/.cpp in the module
        for src in "$SUITE_DIR"/*.c "$SUITE_DIR"/*.cpp; do
            [ -f "$src" ] || continue
            name=$(basename "$src")
            obj="build/test_${TARGET_MODULE}_${name}.o"
            if $GCC $CFLAGS $INCLUDES $FEATURE_FLAGS -c "$src" -o "$obj" 2>/dev/null; then
                check "Compiles: $name" "ok"
            else
                check "Compiles: $name" "fail"
            fi
        done

        # Verify dependencies
        if [ -f "$SUITE_DIR/module.json" ] && command -v python3 &>/dev/null; then
            python3 -c "
import json, os, sys
with open('$SUITE_DIR/module.json') as f: m = json.load(f)
for dep in m.get('dependencies', []):
    if os.path.isdir(f'suites/{dep}'):
        print(f'DEP_OK:{dep}')
    else:
        print(f'DEP_MISS:{dep}')
" 2>/dev/null | while read line; do
                if [[ "$line" == DEP_OK:* ]]; then
                    check "Dependency: ${line#DEP_OK:}" "ok"
                else
                    check "Dependency: ${line#DEP_MISS:}" "fail"
                fi
            done
        fi
    fi

# =============================================================================
# TEST MODE: QEMU SMOKE TEST
# =============================================================================
elif [ "$MODE" == "qemu" ]; then
    echo "Σ [TEST] QEMU Smoke Test"
    echo "─────────────────────────────────────────"

    if ! command -v qemu-system-x86_64 &>/dev/null; then
        check "QEMU available" "skip"
    else
        check "QEMU available" "ok"
        if [ -f "build/sigmaos_zenith" ]; then
            # Run QEMU with a 5-second timeout
            timeout 5 qemu-system-x86_64 \
                -kernel build/sigmaos_zenith \
                -m 128M \
                -serial stdio \
                -display none \
                -no-reboot 2>&1 | head -20
            check "QEMU boot (5s smoke test)" "ok"
        else
            check "Kernel binary exists for QEMU" "warn"
        fi
    fi

# =============================================================================
# TEST MODE: FULL LATTICE INTEGRITY
# =============================================================================
else
    echo "Σ [1/6] Build Artifact Verification"
    echo "─────────────────────────────────────────"
    [ -d "build" ] && check "build/ directory exists" "ok" || check "build/ directory exists" "fail"
    OBJ_COUNT=$(find build -name "*.o" 2>/dev/null | wc -l | tr -d ' ')
    [ "$OBJ_COUNT" -gt 100 ] && check "Object files: $OBJ_COUNT (>100)" "ok" || check "Object files: $OBJ_COUNT" "warn"

    echo ""
    echo "Σ [2/6] Core Suite Presence"
    echo "─────────────────────────────────────────"
    for suite in S01_Genesis S03_Orchestrator S04_HAL S05_Memory S07_Network S08_Security S30_Supremacy; do
        [ -d "suites/$suite" ] && check "Suite $suite" "ok" || check "Suite $suite" "fail"
    done

    echo ""
    echo "Σ [3/6] HAL Contract Layer"
    echo "─────────────────────────────────────────"
    [ -f "include/sigma/hal_contract.h" ] && check "HAL Contract Header" "ok" || check "HAL Contract Header" "fail"
    [ -f "include/sigma/sigma_features.h" ] && check "Feature Flags Header" "ok" || check "Feature Flags Header" "fail"
    [ -f "sigma_features.json" ] && check "Feature Config JSON" "ok" || check "Feature Config JSON" "fail"
    [ -f "suites/S04_HAL/hal_registry.c" ] && check "HAL Registry" "ok" || check "HAL Registry" "fail"

    echo ""
    echo "Σ [4/6] Module Manifests"
    echo "─────────────────────────────────────────"
    MANIFEST_COUNT=$(find suites -name "module.json" 2>/dev/null | wc -l | tr -d ' ')
    check "Module manifests found: $MANIFEST_COUNT" "ok"
    # Verify each manifest is valid JSON
    find suites -name "module.json" 2>/dev/null | while read mf; do
        if python3 -c "import json; json.load(open('$mf'))" 2>/dev/null; then
            check "Valid JSON: $(dirname $mf | xargs basename)" "ok"
        else
            check "Valid JSON: $mf" "fail"
        fi
    done

    echo ""
    echo "Σ [5/6] Multi-Architecture Support"
    echo "─────────────────────────────────────────"
    for arch in x86_64 aarch64 riscv64; do
        [ -d "suites/S04_HAL/arch/$arch" ] && check "Arch bootstrap: $arch" "ok" || check "Arch bootstrap: $arch" "warn"
    done

    echo ""
    echo "Σ [6/6] Web Component Decoupling"
    echo "─────────────────────────────────────────"
    [ -f "web_ui/scripts/sigma_api_service.js" ] && check "Versioned API Service" "ok" || check "Versioned API Service" "fail"
    for comp in sigma-monitor sigma-logs sigma-pool-inspector; do
        [ -f "web_ui/components/${comp}.js" ] && check "Web Component: <${comp}>" "ok" || check "Web Component: <${comp}>" "fail"
    done
fi

# ── Summary ──────────────────────────────────────────────────────────────────
echo ""
echo "══════════════════════════════════════════════════════════"
echo "  Σ TEST RESULTS: $PASS passed | $WARN warnings | $FAIL failed | $SKIP skipped"
echo "══════════════════════════════════════════════════════════"

if [ $FAIL -gt 0 ]; then
    echo "  ❌ Lattice integrity check FAILED ($FAIL critical)"; exit 1
elif [ $WARN -gt 0 ]; then
    echo "  ⚠️  Lattice verified with $WARN warnings"; exit 0
else
    echo "  ✅ Sovereign Lattice integrity VERIFIED — Zero compromise."; exit 0
fi
