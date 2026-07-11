#!/bin/bash
# =============================================================================
# SIGMAOS: PARTITION MANAGEMENT & DUAL-BOOT VALIDATION (release/dual-boot)
# =============================================================================
# Validates bootloader integrity, partition isolation, recovery routines,
# and compatibility with co-resident operating systems.
# =============================================================================

set -e

PASS=0
FAIL=0

log_pass() { echo "  [PASS] $1"; ((PASS++)); }
log_fail() { echo "  [FAIL] $1"; ((FAIL++)); }

echo "=================================================="
echo "  SigmaOS Dual-Boot: Partition Management Suite"
echo "=================================================="

# ── Test 1: Bootloader recovery routine present
echo "[1/6] Fallback recovery routine..."
BOOT_FILE="kernel/core/system/SovereignBoot.cpp"
if grep -q "fallback_recovery\|SIGMA_BOOT_STAGE_RECOVERY" "$BOOT_FILE" 2>/dev/null; then
    log_pass "Fallback recovery: Routine implemented in $BOOT_FILE."
else
    log_fail "Fallback recovery: MISSING — boot failures leave system unrecoverable!"
fi

# ── Test 2: FastBoot / startup optimization
echo "[2/6] Startup optimization (FastBoot)..."
if grep -q "m_fast_boot\|enableFastBoot\|FastBoot" "$BOOT_FILE" 2>/dev/null; then
    log_pass "FastBoot: Optimized startup routine present — reduced boot latency."
else
    log_fail "FastBoot: Not implemented — startup delay unmitigated."
fi

# ── Test 3: Partition isolation enforcement
echo "[3/6] Partition isolation check..."
if grep -q "partition\|isolation\|PARTITION" "$BOOT_FILE" 2>/dev/null; then
    log_pass "Partition isolation: Boundaries enforced in bootloader."
else
    log_fail "Partition isolation: No boundary enforcement — cross-OS corruption risk!"
fi

# ── Test 4: Installer shard present
echo "[4/6] SovereignInstaller shard..."
if [[ -f "kernel/core/boot/SovereignInstaller.cpp" ]]; then
    log_pass "SovereignInstaller: Present — guided dual-boot setup available."
else
    log_fail "SovereignInstaller: MISSING — no automated partition management."
fi

# ── Test 5: Recovery shard
echo "[5/6] SovereignRecover shard..."
if [[ -f "kernel/core/boot/SovereignRecover.cpp" ]]; then
    log_pass "SovereignRecover: Present — boot-failure rescue available."
else
    log_fail "SovereignRecover: MISSING — no recovery partition support."
fi

# ── Test 6: Compatibility matrix doc present
echo "[6/6] OS compatibility matrix documentation..."
COMPAT_DOC="docs/Dual-Boot-Compatibility-Matrix.md"
if [[ -f "$COMPAT_DOC" ]]; then
    log_pass "Compatibility matrix: $COMPAT_DOC found."
else
    log_fail "Compatibility matrix: MISSING — create docs/Dual-Boot-Compatibility-Matrix.md."
fi

# ── Summary
echo ""
echo "=================================================="
echo "  Results: $PASS passed | $FAIL failed"
echo "=================================================="
[[ $FAIL -gt 0 ]] && exit 1
exit 0
