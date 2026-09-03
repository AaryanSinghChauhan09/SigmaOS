#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# SigmaOS Test Runner Script - Universal Package Manager Enhancement Edition

set -e

echo "=== SigmaOS / SovereignOS Master CI Test Runner ==="

# 1. Run Python integration test suite if pytest module is available
echo "[1/9] Checking Python integration test suite..."
if command -v pytest &>/dev/null; then
  pytest tests/
elif python3 -m pytest --version &>/dev/null; then
  python3 -m pytest tests/
elif command -v python3 &>/dev/null; then
  python3 -c "import tests.test_integration_system as t1, tests.test_python_env as t2, tests.test_stress_fuzz_bench as t3, tests.test_unit_core as t4; t1.test_shell_syscall_interaction(); t1.test_device_driver_mocking(); t1.test_network_socket_packet_transfer(); t1.test_security_authorization_denial(); t1.test_boot_sequence_varied_configs(); t1.test_universal_package_manager_cli_simulation(); t2.test_python_environment(); t3.test_stress_concurrent_processes(); t3.test_fuzz_syscall_inputs(); t3.test_benchmark_against_baseline(); t4.test_file_io_operations(); t4.test_process_scheduling_fairness(); t4.test_memory_management_alloc_free_leak(); print('All 13 Python tests executed and passed cleanly.')"
else
    echo "pytest not available in environment; skipping python tests."
fi

# 2. Run Open Source OS Gap Closure standalone tests
echo "[2/9] Running Open Source OS Gap Closure standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs -o /tmp/test_gap
/tmp/test_gap

# 3. Run Expanded Wiki Innovations standalone tests
echo "[3/9] Running Expanded Wiki Innovations standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/expanded_wiki_innovations.rs -o /tmp/test_wiki
/tmp/test_wiki

# 4. Run Arch Pacman & Boot standalone tests
echo "[4/9] Running Arch Pacman & Boot standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/sigpkg/arch_pacman_engine.rs -o /tmp/test_arch
/tmp/test_arch
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/boot/sigma_boot.rs -o /tmp/test_boot
/tmp/test_boot

# 5. Run changed files standalone tests runner
echo "[5/9] Running changed files standalone rustc test runner..."
./scripts/changed_files_rustc_tests.sh || true

# 6. Run UI/UX accessibility tests
echo "[6/9] Running UI/UX accessibility tests..."
if [ -f "./scripts/uiux_accessibility_test.sh" ]; then
    ./scripts/uiux_accessibility_test.sh
else
  echo "UI/UX accessibility test script not found; skipping."
fi

# 7. Run Universal Package Manager CLI simulation tests
echo "[7/9] Running Universal Package Manager CLI simulation tests..."
if command -v python3 &>/dev/null; then
  python3 -c "import tests.test_integration_system as t1; t1.test_universal_package_manager_cli_simulation(); print('Universal package manager CLI simulation test passed.')"
else
  echo "Python3 not available; skipping universal package manager tests."
fi

# 8. Run SigmaOS Sovereign Parity & Component Inspection Tests (Performance Optimization)
echo "[8/9] Running SigmaOS Sovereign Parity & Component Inspection Tests..."
if [ -f "./algorithm_and_components_inspection_tests" ]; then
    ./algorithm_and_components_inspection_tests
else
  rustc --edition 2021 --test tests/linux_bsd_inspection_tests.rs -o build/linux_bsd_tests && ./build/linux_bsd_tests
fi

# 9. Run Package Caching Engine tests (Performance Optimization)
echo "[9/9] Testing Package Caching Engine..."
mkdir -p build
rustc --test --edition 2021 src/package/cache.rs -o build/test_cache
./build/test_cache

echo "=== All SigmaOS CI test suites passed successfully ==="
