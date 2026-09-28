#!/usr/bin/env bash
# SigmaOS Automated Reorganization & Migration Script
# Restructures SigmaOS codebase into a Linux-inspired 3-tier architecture.
set -euo pipefail

echo "=========================================================="
echo "  SigmaOS Automated Reorganization & Migration Script     "
echo "=========================================================="

# 1. Create 3-Tier src/ Directory Structure
echo "[1/4] Creating 3-Tier src/ directory structure..."
mkdir -p src/{boot,arch/x86_64,kernel,klib,drivers,fs,net,ipc,security,userland,desktop,ai,virtualization,compat,distro,tools,testing,planning,reference}
mkdir -p src/userland/{libc,coreutils,shell,init,tools}
mkdir -p src/drivers/{console,timer,acpi,pci,block,dev}

# 2. Create Documentation Hierarchy
echo "[2/4] Creating centralized docs/ hierarchy..."
mkdir -p docs/{architecture,kernel,fs,net,security,userland,roadmap,api,contributing,faq}

# 3. Create Compatibility Re-exports for Duplicate Subsystems
echo "[3/4] Generating compatibility re-export stubs..."
mkdir -p src/mm src/network src/driver src/dev
echo "pub use crate::memory::*;" > src/mm/mod.rs
echo "pub use crate::net::*;" > src/network/mod.rs
echo "pub use crate::drivers::*;" > src/driver/mod.rs
echo "pub use crate::drivers::dev::*;" > src/dev/mod.rs

echo "[4/4] Migration script execution complete!"
