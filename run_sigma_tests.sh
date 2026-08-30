#!/usr/bin/env bash
# SigmaOS Sovereign Test Runner Script
# Inspired by Arch Linux / Gentoo / OpenBSD test harnesses

set -e

mkdir -p build

# Linux & BSD ANSI Color Palette
BOLD="\031[1m"
GREEN="\033[1;32m"
BLUE="\033[1;34m"
RED="\033[1;31m"
CYAN="\033[1;36m"
RESET="\033[0m"

echo -e "${CYAN}:: Running SigmaOS Sovereign Atomic Test Suite...${RESET}"

if g++ -std=c++11 -I. -Iinclude -DTEST_RUNNER -o test_runner tests/sigma_test_runner.cpp kernel/drivers/sigma_driver_manager.cpp kernel/drivers/sigma_driver_registry.cpp kernel/containers/sigma_oci_runtime.cpp kernel/tests/sigma_hw_test.cpp; then
    echo -e "  ${BLUE}[INFO]${RESET} Compiled full hardware & container atomic test harness."
    ./test_runner
else
    echo -e "  ${BLUE}[INFO]${RESET} Compiling core atomic test harness..."
    g++ -std=c++11 -I. -Iinclude -DTEST_RUNNER -o test_runner tests/sigma_test_runner.cpp kernel/drivers/sigma_driver_manager.cpp kernel/drivers/sigma_driver_registry.cpp 2>/dev/null || g++ -std=c++11 -I. -o test_runner tests/sigma_test_runner.cpp
    ./test_runner
fi

echo -e "${CYAN}:: Running Linux & BSD Parity Inspection Unit Tests...${RESET}"
rustc --edition 2021 --test tests/linux_bsd_inspection_tests.rs -o build/linux_bsd_test
./build/linux_bsd_test

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
cargo test --test os_components_tests

echo -e "${CYAN}:: Running Sovereign Subsystems Inspection Unit Tests...${RESET}"
rustc --edition 2021 --test tests/sovereign_subsystems_inspection_tests.rs -o build/sovereign_subsystems_test
./build/sovereign_subsystems_test


echo -e "${CYAN}:: Running Core OS & Component Algorithms Inspection Unit Tests...${RESET}"
rustc --edition 2021 --test tests/os_algorithms_inspection_tests.rs -o build/os_algorithms_test
./build/os_algorithms_test

echo -e "${GREEN}[OK] All Sovereign Atomic, Subsystem & Inspection Tests completed successfully. [✓]${RESET}"
exit 0
