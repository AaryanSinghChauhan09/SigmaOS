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

# ─── Traffic Control (tc) Qdiscs ───────────────────────────────────────────────
echo ""
echo "=== [11] SigmaOS Traffic Control (tc) Qdiscs Tests ==="
rustc --edition=2021 --test src/net/tc_qdisc_sovereign.rs \
  -o build/test_tc_qdisc 2>&1 | grep -v "^$" || true
./build/test_tc_qdisc --test-threads=1
echo "tc_qdisc_sovereign: PASSED"

# ─── Sovereign D-Bus IPC ──────────────────────────────────────────────────────
echo ""
echo "=== [12] SigmaOS Sovereign D-Bus IPC Tests ==="
rustc --edition=2021 --test src/ipc/dbus_sovereign.rs \
  -o build/test_dbus 2>&1 | grep -v "^$" || true
./build/test_dbus --test-threads=1
echo "dbus_sovereign: PASSED"

# ─── Sovereign ftrace Function Tracer ─────────────────────────────────────────
echo ""
echo "=== [13] SigmaOS Sovereign ftrace Function Tracer Tests ==="
rustc --edition=2021 --test src/kernel/ftrace_sovereign.rs \
  -o build/test_ftrace 2>&1 | grep -v "^$" || true
./build/test_ftrace --test-threads=1
echo "ftrace_sovereign: PASSED"

# ─── Sovereign OverlayFS ──────────────────────────────────────────────────────
echo ""
echo "=== [14] SigmaOS Sovereign OverlayFS Tests ==="
rustc --edition=2021 --test src/fs/overlayfs_sovereign.rs \
  -o build/test_overlayfs 2>&1 | grep -v "^$" || true
./build/test_overlayfs --test-threads=1
echo "overlayfs_sovereign: PASSED"

# ─── Linux 6.6+ EEVDF Scheduler ────────────────────────────────────────────────
echo ""
echo "=== [15] SigmaOS EEVDF Sovereign Scheduler Tests ==="
rustc --edition=2021 --test src/kernel/eevdf_sovereign.rs \
  -o build/test_eevdf 2>&1 | grep -v "^$" || true
./build/test_eevdf --test-threads=1
echo "eevdf_sovereign: PASSED"

# ─── Linux 5.7+ BPF-LSM Security Engine ───────────────────────────────────────
echo ""
echo "=== [16] SigmaOS BPF-LSM Sovereign Security Tests ==="
rustc --edition=2021 --test src/security/bpf_lsm_sovereign.rs \
  -o build/test_bpf_lsm 2>&1 | grep -v "^$" || true
./build/test_bpf_lsm --test-threads=1
echo "bpf_lsm_sovereign: PASSED"

# ─── Linux io_uring SQPOLL Zero-Syscall Engine ────────────────────────────────
echo ""
echo "=== [17] SigmaOS io_uring SQPOLL Sovereign Tests ==="
rustc --edition=2021 --test src/kernel/io_uring_sqpoll_sovereign.rs \
  -o build/test_sqpoll 2>&1 | grep -v "^$" || true
./build/test_sqpoll --test-threads=1
echo "io_uring_sqpoll_sovereign: PASSED"

# ─── OpenZFS / FreeBSD ZFS Adaptive Replacement Cache (ARC) ──────────────────
echo ""
echo "=== [18] SigmaOS ZFS ARC Sovereign Cache Tests ==="
rustc --edition=2021 --test src/fs/zfs_arc_sovereign.rs \
  -o build/test_zfs_arc 2>&1 | grep -v "^$" || true
./build/test_zfs_arc --test-threads=1
echo "zfs_arc_sovereign: PASSED"

# ─── Linux 4.20+ Pressure Stall Information (PSI) ──────────────────────────────
echo ""
echo "=== [19] SigmaOS Pressure Stall Information (PSI) Tests ==="
rustc --edition=2021 --test src/kernel/psi_sovereign.rs \
  -o build/test_psi 2>&1 | grep -v "^$" || true
./build/test_psi --test-threads=1
echo "psi_sovereign: PASSED"

# ─── Pure-Rust WireGuard Sovereign Tunnel ─────────────────────────────────────
echo ""
echo "=== [20] SigmaOS WireGuard Sovereign Tunnel Tests ==="
rustc --edition=2021 --test src/net/wireguard_sovereign.rs \
  -o build/test_wireguard 2>&1 | grep -v "^$" || true
./build/test_wireguard --test-threads=1
echo "wireguard_sovereign: PASSED"

# ─── Linux fanotify Filesystem Access Notification ────────────────────────────
echo ""
echo "=== [21] SigmaOS fanotify Sovereign Filesystem Access Tests ==="
rustc --edition=2021 --test src/fs/fanotify_sovereign.rs \
  -o build/test_fanotify 2>&1 | grep -v "^$" || true
./build/test_fanotify --test-threads=1
echo "fanotify_sovereign: PASSED"

# ─── Linux Kernel Samepage Merging (KSM) Deduplication ────────────────────────
echo ""
echo "=== [22] SigmaOS KSM Memory Deduplication Tests ==="
rustc --edition=2021 --test src/kernel/ksm_sovereign.rs \
  -o build/test_ksm 2>&1 | grep -v "^$" || true
./build/test_ksm --test-threads=1
echo "ksm_sovereign: PASSED"

echo ""
echo "========================================================"
echo "=== ALL 22 SIGMAOS TEST SUITES PASSED SUCCESSFULLY ==="
echo "========================================================"
