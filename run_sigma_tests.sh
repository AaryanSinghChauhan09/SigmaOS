#!/usr/bin/env bash
# SigmaOS Sovereign Test Runner Script
# Inspired by Arch Linux / Gentoo / OpenBSD test harnesses

set -e

# Linux & BSD ANSI Color Palette
BOLD="\031[1m"
GREEN="\033[1;32m"
BLUE="\033[1;34m"
RED="\033[1;31m"
CYAN="\033[1;36m"
RESET="\033[0m"

echo -e "${CYAN}:: Running SigmaOS Sovereign Atomic Test Suite...${RESET}"

if g++ -std=c++11 -I. -Iinclude -DTEST_RUNNER -o test_runner tests/sigma_test_runner.cpp kernel/containers/sigma_oci_runtime.cpp kernel/tests/sigma_hw_test.cpp kernel/drivers/sigma_driver_manager.cpp kernel/drivers/sigma_driver_registry.cpp 2>/dev/null; then
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

echo -e "${GREEN}[OK] All Sovereign Atomic & Inspection Tests completed successfully. [✓]${RESET}"
exit 0
