#!/usr/bin/env bash
# SigmaOS Sovereign Lattice Build Script
# Inspired by Arch Linux / Gentoo / FreeBSD build output indicators

set -e

GREEN="\033[1;32m"
CYAN="\033[1;36m"
BLUE="\033[1;34m"
RESET="\033[0m"

echo -e "${CYAN}:: Building Sovereign Lattice Subsystem...${RESET}"
mkdir -p build/

echo -e "  ${BLUE}[INFO]${RESET} Compiling Zenith orchestrator target..."
g++ -std=c++20 orchestrator/main.cpp -o build/sigmaos_zenith

echo -e "${GREEN}[OK] Sovereign Lattice built successfully. [✓]${RESET}"
exit 0
