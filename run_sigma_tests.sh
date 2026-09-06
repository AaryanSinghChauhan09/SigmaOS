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
