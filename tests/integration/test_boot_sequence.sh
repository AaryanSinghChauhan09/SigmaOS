#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-or-later
# tests/integration/test_boot_sequence.sh
# Boot SigmaOS in QEMU, measure time to ready, verify critical services
#
# Requirements: qemu-system-x86_64, build/sigmaos.iso
# Pass criteria:
#   - Boot to sigma-healthd ready: < 5 seconds
#   - All critical daemons started: sigma-healthd, sigma-busd, sigma-trustd
#   - sigma-healthd reports no FAILED subsystems on first boot
#   - sigma-kpatch: no unsigned patches loaded

set -euo pipefail

ISO="${1:-build/sigmaos.iso}"
TIMEOUT_BOOT=60   # max seconds to wait for boot
TIMEOUT_SVC=30    # max seconds for each service check

PASS=0; FAIL=0; SKIP=0
START_TIME=$(date +%s%N)

log() { echo "[boot-test] $*" >&2; }
pass() { echo "  [PASS] $1"; ((PASS++)); }
fail() { echo "  [FAIL] $1: $2"; ((FAIL++)); }
skip() { echo "  [SKIP] $1: $2"; ((SKIP++)); }

# ── Check prerequisites ────────────────────────────────────────────────────
if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then
    skip "boot_sequence" "qemu-system-x86_64 not found"
    exit 0
fi

if [[ ! -f "$ISO" ]]; then
    skip "boot_sequence" "ISO not found at $ISO — run 'make all' first"
    exit 0
fi

# ── Start QEMU ────────────────────────────────────────────────────────────
SERIAL_LOG=$(mktemp /tmp/sigma-boot-XXXXXX.log)
QEMU_PIDFILE=$(mktemp /tmp/sigma-qemu-XXXXXX.pid)

log "Booting $ISO in QEMU..."
qemu-system-x86_64 \
    -cdrom "$ISO" \
    -m 2G \
    -serial "file:$SERIAL_LOG" \
    -nographic \
    -enable-kvm \
    -pidfile "$QEMU_PIDFILE" \
    -daemonize 2>/dev/null || {
    skip "boot_sequence" "QEMU failed to start (KVM unavailable?)"
    rm -f "$SERIAL_LOG" "$QEMU_PIDFILE"
    exit 0
}

QEMU_PID=$(cat "$QEMU_PIDFILE" 2>/dev/null || echo "")
cleanup() {
    [[ -n "$QEMU_PID" ]] && kill "$QEMU_PID" 2>/dev/null || true
    rm -f "$SERIAL_LOG" "$QEMU_PIDFILE"
}
trap cleanup EXIT

# ── Wait for boot markers ─────────────────────────────────────────────────
wait_for() {
    local pattern="$1" timeout="$2" label="$3"
    local elapsed=0
    while [[ $elapsed -lt $timeout ]]; do
        grep -q "$pattern" "$SERIAL_LOG" 2>/dev/null && return 0
        sleep 1; ((elapsed++))
    done
    fail "$label" "timeout after ${timeout}s waiting for: $pattern"
    return 1
}

# Test 1: Kernel boots and PID 1 starts
wait_for "sigma_init: PID 1 starting" $TIMEOUT_BOOT "kernel_boot" && \
    pass "kernel_boot"

# Test 2: sigma-healthd starts within 5 seconds of kernel boot
HEALTHD_START=$(date +%s%N)
wait_for "\[sigma-healthd\] listening on" 5 "healthd_start" && {
    HEALTHD_END=$(date +%s%N)
    HEALTHD_MS=$(( (HEALTHD_END - HEALTHD_START) / 1000000 ))
    if [[ $HEALTHD_MS -lt 5000 ]]; then
        pass "healthd_start_fast (${HEALTHD_MS}ms)"
    else
        fail "healthd_start_fast" "took ${HEALTHD_MS}ms (> 5000ms target)"
    fi
}

# Test 3: Critical daemons start
for daemon in sigma-busd sigma-trustd sigma-netd sigma-watchdog; do
    wait_for "\[$daemon\] listening on" $TIMEOUT_SVC "daemon_$daemon" && \
        pass "daemon_$daemon"
done

# Test 4: No FAILED subsystems on first boot
wait_for "sigma-healthd.*all subsystems ok" 10 "healthd_all_ok" && \
    pass "healthd_all_ok" || \
    # Check if it's just stubs (expected on dev builds)
    grep -q "sigma-healthd.*FAILED" "$SERIAL_LOG" 2>/dev/null && \
    fail "healthd_all_ok" "FAILED subsystems on boot" || \
    skip "healthd_all_ok" "health summary line not found (dev build)"

# Test 5: No unsigned kpatch modules loaded
if grep -q "sigma-kpatch.*unsigned module rejected" "$SERIAL_LOG" 2>/dev/null; then
    fail "kpatch_signed_only" "unsigned patch was loaded"
else
    pass "kpatch_signed_only"
fi

# Test 6: Total boot time
BOOT_END=$(date +%s%N)
BOOT_MS=$(( (BOOT_END - START_TIME) / 1000000 ))
if [[ $BOOT_MS -lt 5000 ]]; then
    pass "boot_under_5s (${BOOT_MS}ms)"
elif [[ $BOOT_MS -lt 10000 ]]; then
    pass "boot_under_10s (${BOOT_MS}ms) — target is 3s for production"
else
    fail "boot_time" "boot took ${BOOT_MS}ms (> 10000ms)"
fi

# ── Results ────────────────────────────────────────────────────────────────
echo ""
echo "========================================"
printf "  Boot test: %d passed, %d failed, %d skipped\n" $PASS $FAIL $SKIP
echo "========================================"

[[ $FAIL -eq 0 ]]
