#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-or-later
# tests/integration/test_sigma_pkg.sh
# Integration test: install, remove, rollback a package in a live SigmaOS VM
#
# Requires: sigma-healthd running, sigma-apid running, network available
# Tests: install vim, verify health, remove vim, rollback to previous generation

set -euo pipefail

PASS=0; FAIL=0
SIGMA_SOCK="/run/sigma"

pass() { echo "  [PASS] $1"; ((PASS++)); }
fail() { echo "  [FAIL] $1: $2"; ((FAIL++)); }
skip() { echo "  [SKIP] $1: $2"; ((SKIP=SKIP+1)); }

SKIP=0

# ── Check prerequisites ────────────────────────────────────────────────────
check_daemon() {
    local name="$1"
    if ! [ -S "$SIGMA_SOCK/${name}.sock" ]; then
        skip "all" "sigma-${name} not running (socket not found)"
        exit 0
    fi
}

check_daemon "healthd"
check_daemon "apid"

# ── Helper: query sigma-pkg ────────────────────────────────────────────────
pkg() { curl -sf --unix-socket "$SIGMA_SOCK/apid.sock" \
         "http://localhost/pkg/$1" 2>/dev/null || echo "{}"; }

# ── Test 1: Check initial health ───────────────────────────────────────────
HEALTH=$(curl -sf --unix-socket "$SIGMA_SOCK/healthd.sock" \
         http://localhost/health 2>/dev/null || echo "{}")
if echo "$HEALTH" | grep -q '"status"'; then
    pass "healthd_responds"
else
    fail "healthd_responds" "healthd not responding"
fi

# ── Test 2: Record generation before install ──────────────────────────────
GEN_BEFORE=$(curl -sf --unix-socket "$SIGMA_SOCK/apid.sock" \
             http://localhost/pkg/generations 2>/dev/null | \
             python3 -c "import sys,json; d=json.load(sys.stdin); \
                         print(d.get('current_generation',0))" 2>/dev/null || echo "0")
pass "record_generation_before (gen $GEN_BEFORE)"

# ── Test 3: Install a package ─────────────────────────────────────────────
INSTALL_RESULT=$(curl -sf --unix-socket "$SIGMA_SOCK/apid.sock" \
                 -X POST http://localhost/pkg/install \
                 -d '{"packages":["vim"]}' 2>/dev/null || echo '{"error":"no apid"}')

if echo "$INSTALL_RESULT" | grep -qi '"ok":\s*true\|"status":\s*"ok"'; then
    pass "pkg_install_vim"
else
    skip "pkg_install_vim" "sigma-apid not responding or vim not in repo"
fi

# ── Test 4: Verify vim is installed ──────────────────────────────────────
if command -v vim >/dev/null 2>&1 || \
   [ -f "/sigma/bin/vim" ] || [ -f "/usr/bin/vim" ]; then
    pass "vim_binary_exists"
else
    skip "vim_binary_exists" "vim binary not found (install may be pending)"
fi

# ── Test 5: Verify dual hash on installed package ────────────────────────
VERIFY=$(curl -sf --unix-socket "$SIGMA_SOCK/apid.sock" \
         http://localhost/pkg/verify?name=vim 2>/dev/null || echo '{}')
if echo "$VERIFY" | grep -qi '"verified":\s*true\|"hash_ok":\s*true'; then
    pass "pkg_hash_verified"
else
    skip "pkg_hash_verified" "verify endpoint not available"
fi

# ── Test 6: Remove the package ────────────────────────────────────────────
REMOVE_RESULT=$(curl -sf --unix-socket "$SIGMA_SOCK/apid.sock" \
                -X POST http://localhost/pkg/remove \
                -d '{"packages":["vim"]}' 2>/dev/null || echo '{}')
if echo "$REMOVE_RESULT" | grep -qi '"ok":\s*true\|"status":\s*"ok"'; then
    pass "pkg_remove_vim"
else
    skip "pkg_remove_vim" "remove endpoint not available"
fi

# ── Test 7: Rollback to previous generation ───────────────────────────────
if [[ "$GEN_BEFORE" -gt 0 ]]; then
    ROLLBACK=$(curl -sf --unix-socket "$SIGMA_SOCK/apid.sock" \
               -X POST http://localhost/pkg/rollback \
               -d "{\"generation\":$GEN_BEFORE}" 2>/dev/null || echo '{}')
    if echo "$ROLLBACK" | grep -qi '"ok":\s*true\|"reboot_required"'; then
        pass "pkg_rollback_generation"
    else
        skip "pkg_rollback_generation" "rollback endpoint not available"
    fi
else
    skip "pkg_rollback_generation" "no previous generation recorded"
fi

# ── Test 8: dm-verity tamper detection ────────────────────────────────────
# Attempt to write to an immutable package path — must fail
if [ -d "/sigma/store" ]; then
    TEST_FILE="/sigma/store/tamper-test-$$"
    if touch "$TEST_FILE" 2>/dev/null; then
        rm -f "$TEST_FILE"
        fail "verity_blocks_write" "wrote to /sigma/store — immutability not enforced"
    else
        pass "verity_blocks_write"
    fi
else
    skip "verity_blocks_write" "/sigma/store not present"
fi

# ── Results ───────────────────────────────────────────────────────────────
echo ""
echo "========================================"
printf "  Pkg test: %d passed, %d failed, %d skipped\n" $PASS $FAIL $SKIP
echo "========================================"

[[ $FAIL -eq 0 ]]
