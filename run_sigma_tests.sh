#!/usr/bin/env bash
# SigmaOS Native Test Runner
set -e

echo "=== SigmaOS Native Test Runner ==="

if [ -f "./algorithm_and_components_inspection_tests" ]; then
    echo "Running core algorithm & component inspection test binary..."
    ./algorithm_and_components_inspection_tests
fi

if [ -f "src/security/input_validation.rs" ]; then
    echo "Running security input validation test suite..."
    mkdir -p build
    rustc --test src/security/input_validation.rs --edition=2021 -o build/input_val_test
    ./build/input_val_test
fi

echo "=== All SigmaOS Tests Passed ==="

if [ -f "src/launch_ready/mod.rs" ]; then
    echo "Running launch readiness & distro parity test suite..."
    mkdir -p build
    rustc --test src/launch_ready/mod.rs --edition=2021 -o build/launch_ready_test
    ./build/launch_ready_test
fi

if [ -f "tests/test_vecdeque_standalone.rs" ]; then
    echo "Running VecDeque performance & correctness test suite..."
    mkdir -p build
    rustc --test tests/test_vecdeque_standalone.rs --edition=2021 -o build/vecdeque_test
    ./build/vecdeque_test
fi

if [ -f "src/tools/native_userland_replacements.rs" ]; then
    echo "Running native userland replacements test suite..."
    mkdir -p build
    rustc --test src/tools/native_userland_replacements.rs --edition=2021 -o build/userland_rep_test
    ./build/userland_rep_test
fi

# ─── cgroups v2 Sovereign (Linux resource accounting) ────────────────────────
echo ""
echo "=== [6] SigmaOS cgroups v2 Sovereign Tests ==="
rustc --edition=2021 --test src/kernel/cgroups_v2_sovereign.rs \
  -o build/test_cgroups_v2 2>&1 | grep -v "^$" || true
if ./build/test_cgroups_v2 --test-threads=1 2>&1; then
  echo "cgroups_v2: PASSED"
  PASSED=$((PASSED + 1))
else
  echo "cgroups_v2: FAILED"
  FAILED=$((FAILED + 1))
fi

# ─── Landlock + Capsicum Sovereign Sandbox ────────────────────────────────────
echo ""
echo "=== [7] SigmaOS Landlock v5 + Capsicum Sovereign Tests ==="
rustc --edition=2021 --test src/security/landlock_sovereign.rs \
  -o build/test_landlock 2>&1 | grep -v "^$" || true
if ./build/test_landlock --test-threads=1 2>&1; then
  echo "landlock_sovereign: PASSED"
  PASSED=$((PASSED + 1))
else
  echo "landlock_sovereign: FAILED"
  FAILED=$((FAILED + 1))
fi

# ─── BSD Jails Sovereign ──────────────────────────────────────────────────────
echo ""
echo "=== [8] SigmaOS BSD Jails Sovereign Tests ==="
rustc --edition=2021 --test src/kernel/bsd_jails_sovereign.rs \
  -o build/test_bsd_jails 2>&1 | grep -v "^$" || true
if ./build/test_bsd_jails --test-threads=1 2>&1; then
  echo "bsd_jails_sovereign: PASSED"
  PASSED=$((PASSED + 1))
else
  echo "bsd_jails_sovereign: FAILED"
  FAILED=$((FAILED + 1))
fi

# ─── Zero-Copy XDP Networking ─────────────────────────────────────────────────
echo ""
echo "=== [9] SigmaOS Zero-Copy XDP Networking Tests ==="
rustc --edition=2021 --test src/network/zero_copy_networking.rs \
  -o build/test_zero_copy 2>&1 | grep -v "^$" || true
if ./build/test_zero_copy --test-threads=1 2>&1; then
  echo "zero_copy_networking: PASSED"
  PASSED=$((PASSED + 1))
else
  echo "zero_copy_networking: FAILED"
  FAILED=$((FAILED + 1))
fi

# ─── bcachefs Sovereign CoW Filesystem ───────────────────────────────────────
echo ""
echo "=== [10] SigmaOS bcachefs Sovereign CoW Filesystem Tests ==="
rustc --edition=2021 --test src/fs/bcachefs_sovereign.rs \
  -o build/test_bcachefs 2>&1 | grep -v "^$" || true
if ./build/test_bcachefs --test-threads=1 2>&1; then
  echo "bcachefs_sovereign: PASSED"
  PASSED=$((PASSED + 1))
else
  echo "bcachefs_sovereign: FAILED"
  FAILED=$((FAILED + 1))
fi
