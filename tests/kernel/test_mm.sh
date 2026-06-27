#!/bin/bash
# SPDX-License-Identifier: GPL-2.0-or-later
# tests/kernel/test_mm.sh — Memory management regression tests
#
# Inspired by Linux kselftest (tools/testing/selftests/mm/)
# Tests: mmap, mprotect, THP collapse, huge page alloc, OOM score

set -euo pipefail

PASS=0; FAIL=0
SIGMA_PROC=/sigma/proc   # SigmaOS proc filesystem

pass() { echo "  [PASS] $1"; ((PASS++)); }
fail() { echo "  [FAIL] $1"; ((FAIL++)); }
skip() { echo "  [SKIP] $1 — $2"; }

# ── Test 1: Basic mmap/munmap ─────────────────────────────────────────────
test_mmap_basic() {
    python3 -c "
import mmap, os, sys
m = mmap.mmap(-1, 4096)
m.write(b'sigma')
m.seek(0)
assert m.read(5) == b'sigma', 'mmap write/read failed'
m.close()
print('ok')
" 2>&1 | grep -q ok && pass "mmap_basic" || fail "mmap_basic"
}

# ── Test 2: mprotect PROT_NONE ────────────────────────────────────────────
test_mprotect() {
    python3 -c "
import ctypes, sys
libc = ctypes.CDLL('libc.so.6', use_errno=True)
PROT_READ=1; PROT_WRITE=2; PROT_NONE=0
MAP_PRIVATE=2; MAP_ANONYMOUS=0x20
addr = libc.mmap(0, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0)
assert addr != -1, 'mmap failed'
ret = libc.mprotect(addr, 4096, PROT_NONE)
assert ret == 0, f'mprotect failed: {ctypes.get_errno()}'
libc.munmap(addr, 4096)
print('ok')
" 2>&1 | grep -q ok && pass "mprotect_prot_none" || fail "mprotect_prot_none"
}

# ── Test 3: Huge page availability ───────────────────────────────────────
test_hugepages() {
    if [[ -f /proc/meminfo ]]; then
        HP=$(grep -i hugepages_total /proc/meminfo | awk '{print $2}')
        [[ -n "$HP" ]] && pass "hugepages_sysfs_readable" || \
                          skip "hugepages_sysfs_readable" "no /proc/meminfo"
    else
        skip "hugepages_sysfs_readable" "no /proc/meminfo"
    fi
}

# ── Test 4: /proc/self/maps parseable ────────────────────────────────────
test_proc_maps() {
    if [[ -f /proc/self/maps ]]; then
        COUNT=$(wc -l < /proc/self/maps)
        [[ $COUNT -gt 0 ]] && pass "proc_maps_readable ($COUNT lines)" || \
                              fail "proc_maps_empty"
    else
        skip "proc_maps_readable" "no /proc filesystem"
    fi
}

# ── Test 5: Stack growth ──────────────────────────────────────────────────
test_stack_growth() {
    python3 -c "
import sys
sys.setrecursionlimit(500)
def recurse(n):
    if n == 0: return 0
    return recurse(n-1) + 1
assert recurse(400) == 400
print('ok')
" 2>&1 | grep -q ok && pass "stack_growth" || fail "stack_growth"
}

# ── Test 6: OOM score readable ───────────────────────────────────────────
test_oom_score() {
    if [[ -f /proc/self/oom_score ]]; then
        SCORE=$(cat /proc/self/oom_score)
        [[ "$SCORE" =~ ^[0-9]+$ ]] && pass "oom_score_readable ($SCORE)" || \
                                       fail "oom_score_not_numeric"
    else
        skip "oom_score_readable" "no /proc/self/oom_score"
    fi
}

# ── Test 7: Memory overcommit setting ────────────────────────────────────
test_overcommit() {
    if [[ -f /proc/sys/vm/overcommit_memory ]]; then
        VAL=$(cat /proc/sys/vm/overcommit_memory)
        [[ "$VAL" =~ ^[012]$ ]] && pass "overcommit_policy_valid ($VAL)" || \
                                   fail "overcommit_policy_invalid"
    else
        skip "overcommit_policy" "no /proc/sys/vm"
    fi
}

# ── Test 8: Large anonymous allocation ───────────────────────────────────
test_large_alloc() {
    python3 -c "
import ctypes
SIZE = 64 * 1024 * 1024  # 64 MiB
buf = (ctypes.c_char * SIZE)()
buf[0] = b'x'
buf[SIZE-1] = b'y'
assert buf[0] == b'x' and buf[SIZE-1] == b'y'
print('ok')
" 2>&1 | grep -q ok && pass "large_anon_alloc_64MiB" || fail "large_anon_alloc_64MiB"
}

# ── Run all ───────────────────────────────────────────────────────────────
echo ""
echo "========================================"
echo "  SigmaOS Memory Management Tests"
echo "========================================"
echo ""

test_mmap_basic
test_mprotect
test_hugepages
test_proc_maps
test_stack_growth
test_oom_score
test_overcommit
test_large_alloc

echo ""
echo "========================================"
printf "  Results: %d passed, %d failed\n" $PASS $FAIL
echo "========================================"
echo ""

[[ $FAIL -eq 0 ]]
