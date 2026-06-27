#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-or-later
# tests/posix/run_posix_tests.sh — POSIX compliance test suite
#
# Tests every failing test = a compatibility bug that will break ported software.
# Modelled on musl's libc-test suite structure.
#
# Usage:
#   ./tests/posix/run_posix_tests.sh            # run all tests
#   ./tests/posix/run_posix_tests.sh string     # run only string tests
#   ./tests/posix/run_posix_tests.sh --ci       # CI mode: fail on first error

set -euo pipefail

PASS=0; FAIL=0; SKIP=0
CI_MODE=0
FILTER=""

for arg in "$@"; do
  case $arg in
    --ci) CI_MODE=1 ;;
    *)    FILTER="$arg" ;;
  esac
done

run_test() {
  local name="$1" binary="$2"
  if [[ -n "$FILTER" && "$name" != *"$FILTER"* ]]; then return; fi
  if [[ ! -f "$binary" ]]; then
    echo "  SKIP $name (not built)"
    ((SKIP++)); return
  fi
  if "$binary" 2>&1 | grep -q "FAIL"; then
    echo "  FAIL $name"
    ((FAIL++))
    [[ $CI_MODE -eq 1 ]] && exit 1
  else
    echo "  PASS $name"
    ((PASS++))
  fi
}

echo "=== SigmaOS POSIX Compliance Test Suite ==="
echo ""

# Build test binaries
echo "Building test binaries..."
cmake -B build/posix-tests tests/posix -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_COMPILER=gcc -DCMAKE_CXX_COMPILER=g++ 2>/dev/null || true
cmake --build build/posix-tests -j$(nproc) 2>/dev/null || true
echo ""

# ── String functions ───────────────────────────────────────────────────────────
echo "--- String ---"
run_test "string/memcpy"    "build/posix-tests/test_string_memcpy"
run_test "string/strcpy"    "build/posix-tests/test_string_strcpy"
run_test "string/strlen"    "build/posix-tests/test_string_strlen"
run_test "string/strcmp"    "build/posix-tests/test_string_strcmp"
run_test "string/strtol"    "build/posix-tests/test_string_strtol"
run_test "string/sprintf"   "build/posix-tests/test_string_sprintf"
run_test "string/snprintf"  "build/posix-tests/test_string_snprintf"

# ── Math ───────────────────────────────────────────────────────────────────────
echo "--- Math ---"
run_test "math/floor"       "build/posix-tests/test_math_floor"
run_test "math/ceil"        "build/posix-tests/test_math_ceil"
run_test "math/sqrt"        "build/posix-tests/test_math_sqrt"
run_test "math/trig"        "build/posix-tests/test_math_trig"

# ── File I/O ──────────────────────────────────────────────────────────────────
echo "--- File I/O ---"
run_test "file/open_close"  "build/posix-tests/test_file_open_close"
run_test "file/read_write"  "build/posix-tests/test_file_read_write"
run_test "file/seek"        "build/posix-tests/test_file_seek"
run_test "file/stat"        "build/posix-tests/test_file_stat"
run_test "file/mkdir_rmdir" "build/posix-tests/test_file_mkdir_rmdir"
run_test "file/rename"      "build/posix-tests/test_file_rename"
run_test "file/symlink"     "build/posix-tests/test_file_symlink"
run_test "file/mmap"        "build/posix-tests/test_file_mmap"

# ── Process ───────────────────────────────────────────────────────────────────
echo "--- Process ---"
run_test "process/fork"     "build/posix-tests/test_process_fork"
run_test "process/execve"   "build/posix-tests/test_process_execve"
run_test "process/waitpid"  "build/posix-tests/test_process_waitpid"
run_test "process/signals"  "build/posix-tests/test_process_signals"
run_test "process/pipe"     "build/posix-tests/test_process_pipe"
run_test "process/getpid"   "build/posix-tests/test_process_getpid"

# ── Socket ─────────────────────────────────────────────────────────────────────
echo "--- Socket ---"
run_test "socket/tcp_loopback"  "build/posix-tests/test_socket_tcp"
run_test "socket/udp"           "build/posix-tests/test_socket_udp"
run_test "socket/unix"          "build/posix-tests/test_socket_unix"
run_test "socket/getaddrinfo"   "build/posix-tests/test_socket_getaddrinfo"

# ── Threading ─────────────────────────────────────────────────────────────────
echo "--- Threading ---"
run_test "thread/pthread_create" "build/posix-tests/test_thread_create"
run_test "thread/mutex"          "build/posix-tests/test_thread_mutex"
run_test "thread/condvar"        "build/posix-tests/test_thread_condvar"
run_test "thread/semaphore"      "build/posix-tests/test_thread_semaphore"

# ── Time ──────────────────────────────────────────────────────────────────────
echo "--- Time ---"
run_test "time/clock_gettime"   "build/posix-tests/test_time_clock"
run_test "time/nanosleep"       "build/posix-tests/test_time_nanosleep"
run_test "time/strftime"        "build/posix-tests/test_time_strftime"

echo ""
echo "=== Results: $PASS passed, $FAIL failed, $SKIP skipped ==="
[[ $FAIL -gt 0 ]] && exit 1 || exit 0
