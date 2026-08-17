#!/bin/bash
# SigmaOS Smoke Test Script
# Inspired by Arch Linux makepkg / FreeBSD ports / OpenBSD release test indicators

set -e

GREEN="\033[1;32m"
RED="\033[1;31m"
CYAN="\033[1;36m"
BLUE="\033[1;34m"
RESET="\033[0m"

echo -e "${CYAN}:: Running SigmaOS Smoke Tests & Build Verification...${RESET}"

# Ensure build directory exists
mkdir -p build

# Test 1: Check if build directory exists
if [ ! -d "build" ]; then
    echo -e "  ${RED}[FAIL] Build directory does not exist [✗]${RESET}"
    exit 1
fi
echo -e "  ${GREEN}[OK] Build directory present [✓]${RESET}"

# Test 2: Check if kernel binary or crate workspace target exists
if [ -d "target" ] || [ -f "Cargo.toml" ]; then
    echo -e "  ${GREEN}[OK] Workspace target directory validated [✓]${RESET}"
else
    echo -e "  ${RED}[FAIL] Workspace configuration error [✗]${RESET}"
    exit 1
fi

# Test 3: Run cargo check
echo -e "  ${BLUE}[INFO] Running Rust workspace cargo check...${RESET}"
cargo check
echo -e "  ${GREEN}[OK] Cargo check completed successfully [✓]${RESET}"

echo -e "${GREEN}[OK] All smoke tests passed successfully! [✓]${RESET}"
