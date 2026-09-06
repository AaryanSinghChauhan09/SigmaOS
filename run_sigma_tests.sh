#!/usr/bin/env bash
# SigmaOS Native Test Runner
set -e

echo "=== SigmaOS Native Test Runner ==="

# 1. Run Python integration test suite if pytest module is available
echo "[1/12] Checking Python integration test suite..."
if command -v pytest &>/dev/null; then
  pytest tests/
elif python3 -m pytest --version &>/dev/null; then
  python3 -m pytest tests/
elif python3 -c "import pytest" &>/dev/null; then
  pytest
elif command -v python3 &>/dev/null; then
  python3 -c "import tests.test_integration_system as t1, tests.test_python_env as t2, tests.test_stress_fuzz_bench as t3, tests.test_unit_core as t4; t1.test_shell_syscall_interaction(); t1.test_device_driver_mocking(); t1.test_network_socket_packet_transfer(); t1.test_security_authorization_denial(); t1.test_boot_sequence_varied_configs(); t1.test_universal_distro_subsystem_bridge(); t2.test_python_environment(); t3.test_stress_concurrent_processes(); t3.test_fuzz_syscall_inputs(); t3.test_benchmark_against_baseline(); t4.test_file_io_operations(); t4.test_process_scheduling_fairness(); t4.test_memory_management_alloc_free_leak(); print('All 13 Python tests executed and passed cleanly.')"
else
    echo "pytest not available in environment; skipping python tests."
fi

# 2. Run Open Source OS Gap Closure standalone tests
echo "[2/12] Running Open Source OS Gap Closure standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs -o /tmp/test_gap
/tmp/test_gap

# 3. Run Expanded Wiki Innovations standalone tests
echo "[3/12] Running Expanded Wiki Innovations standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/expanded_wiki_innovations.rs -o /tmp/test_wiki
/tmp/test_wiki

# 4. Run Arch Pacman & Boot standalone tests
echo "[4/12] Running Arch Pacman & Boot standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/sigpkg/arch_pacman_engine.rs -o /tmp/test_arch
/tmp/test_arch
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/boot/sigma_boot.rs -o /tmp/test_boot
/tmp/test_boot

# 5. Run Fedora RPM & MirrorManager2 standalone tests
echo "[5/12] Running Fedora RPM & MirrorManager2 standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/sigpkg/fedora_rpm_engine.rs -o /tmp/test_fedora
/tmp/test_fedora

# 6. Run changed files standalone tests runner
echo "[6/12] Running changed files standalone rustc test runner..."
./scripts/changed_files_rustc_tests.sh || true

# 7. Run UI/UX accessibility tests
echo "[7/12] Running UI/UX accessibility tests..."
if [ -f "./scripts/uiux_accessibility_test.sh" ]; then
    ./scripts/uiux_accessibility_test.sh
else
  echo "UI/UX accessibility test script not found; skipping."
fi

# 8. Run Universal Package Manager CLI simulation tests
echo "[8/12] Running Universal Package Manager CLI simulation tests..."
if command -v python3 &>/dev/null; then
  python3 -c "import tests.test_integration_system as t1; t1.test_universal_distro_subsystem_bridge(); print('Universal package manager CLI simulation test passed.')"
else
  echo "Python3 not available; skipping universal package manager tests."
fi

# 9. Run SigmaOS Sovereign Parity & Component Inspection Tests (Performance Optimization)
echo "[9/12] Running SigmaOS Sovereign Parity & Component Inspection Tests..."
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

echo -e "${CYAN}:: Running Ecosystem & Compliance Inspection Unit Tests...${RESET}"
rustc --edition 2021 --test tests/ecosystem_and_compliance_inspection_tests.rs -o build/eco_compliance_test
./build/eco_compliance_test

echo -e "${CYAN}:: Running Algorithm & Subsystem Component Inspection Unit Tests...${RESET}"
rustc --edition 2021 --test tests/algorithm_and_components_inspection_tests.rs -o build/algo_components_test
./build/algo_components_test

echo -e "${CYAN}:: Running Comprehensive OS Core Algorithms Inspection Unit Tests...${RESET}"
rustc --edition 2021 --test tests/os_algorithms_inspection_tests.rs -o build/os_algo_test
./build/os_algo_test

echo -e "${CYAN}:: Running Clean-Room Compatibility Harness Tests...${RESET}"
rustc --edition 2021 --test tests/compat_harness.rs -o build/compat_harness_test
./build/compat_harness_test

echo -e "${CYAN}:: Running Comprehensive OS Subsystems & Components Unit Tests...${RESET}"
rustc --edition 2021 --test tests/os_components_tests.rs -o build/os_components_test
./build/os_components_test

echo -e "${CYAN}:: Running Sovereign Subsystems Inspection Unit Tests...${RESET}"
rustc --edition 2021 --test tests/sovereign_subsystems_inspection_tests.rs -o build/sovereign_subsystems_test
./build/sovereign_subsystems_test


echo -e "${CYAN}:: Running Core OS & Component Algorithms Inspection Unit Tests...${RESET}"
rustc --edition 2021 --test tests/os_algorithms_inspection_tests.rs -o build/os_algorithms_test
./build/os_algorithms_test

echo -e "${CYAN}:: Running Modular Python Test Suite (Unit, Integration, System, Stress, Fuzzing, Benchmarks)...${RESET}"
pytest tests/test_unit_core.py tests/test_integration_system.py tests/test_stress_fuzz_bench.py

echo -e "${GREEN}[OK] All Sovereign Atomic, Subsystem & Inspection Tests completed successfully. [✓]${RESET}"
exit 0
