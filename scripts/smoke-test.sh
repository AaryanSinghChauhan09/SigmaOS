#!/bin/bash
# SPDX-License-Identifier: MIT
# SigmaOS Smoke Test Script (Enhanced QA Sweep)
# Performs rigorous quality audits, formatting validation, license checks, and technical debt scans.

set -eo pipefail

# ANSI Color Codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info()  { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_phase() { echo -e "\n${BLUE}=== Phase: $1 ===${NC}"; }

log_info "Running SigmaOS Distro-Inspired Smoke Tests..."

# Phase 1: Directory Setup
log_phase "Environment Check"
mkdir -p build
if [ ! -d "build" ]; then
    log_error "FAIL: Build directory does not exist or could not be created."
    exit 1
fi
log_info "PASS: Build directory exists"

# Phase 2: Check for Kernel Binaries
log_phase "Kernel Binary Presence check"
KERNEL_FOUND=0
if [ -f "target/release/sigma_kernel" ] || [ -f "target/debug/sigma_kernel" ] || [ -f "iso_root/boot/sigmaos.bin" ]; then
    KERNEL_FOUND=1
fi

if [ "$KERNEL_FOUND" -eq 1 ]; then
    log_info "PASS: At least one compiled kernel binary or staged boot bin exists"
else
    log_warn "Notice: No pre-compiled kernel binary found. Continuing tests but skipping boot image audits."
fi

# Phase 3: Shell Script Syntax Audit
log_phase "Shell Script Format & Syntax Check"
BAD_SCRIPTS=0
for script in scripts/*.sh; do
    if [ -f "$script" ]; then
        if bash -n "$script" 2>/dev/null; then
            log_info "PASS: Syntax OK for $script"
        else
            log_error "FAIL: Syntax errors found in $script"
            BAD_SCRIPTS=$((BAD_SCRIPTS + 1))
        fi
    fi
done

if [ "$BAD_SCRIPTS" -gt 0 ]; then
    log_error "Shell script syntax audit failed."
    exit 1
fi

# Phase 4: SPDX License Compliance Verification
log_phase "SPDX License Audit"
MISSING_LICENSES=0
for file in scripts/*.sh src/kernel/ipc.rs src/runtime/io/file.rs; do
    if [ -f "$file" ]; then
        if grep -q "SPDX-License-Identifier" "$file"; then
            log_info "PASS: License found in $file"
        else
            log_warn "Missing SPDX license tag in $file"
            MISSING_LICENSES=$((MISSING_LICENSES + 1))
        fi
    fi
done
log_info "License Audit completed with $MISSING_LICENSES warnings."

# Phase 5: Technical Debt & TODO Scans
log_phase "Technical Debt Scan"
log_info "Scanning for common technical debt patterns (TODO/FIXME/HACK/XXX)..."
DEBT_COUNT=0
# Search files for TODO markers (excluding target/ and node_modules/)
if command -v grep >/dev/null 2>&1; then
    DEBT_COUNT=$(grep -rnE "(TODO|FIXME|HACK|XXX)" src/ scripts/ 2>/dev/null | wc -l || echo 0)
    log_info "Total identified technical debt markers: $DEBT_COUNT"
else
    log_warn "grep command not available, skipping technical debt scan."
fi

# Phase 6: Subsystem Validation Runner
log_phase "Executing Dynamic Subsystem Validation Suites"

# Run shell builtins verification
if [ -f "scripts/sigma_builtins_test.sh" ]; then
    log_info "Executing POSIX / Linux Builtins verification suite..."
    ./scripts/sigma_builtins_test.sh
fi

# Run accelerator diagnostics
if [ -f "scripts/accelerators_diagnostics.sh" ]; then
    log_info "Executing coprocessors/accelerators hardware diagnostics..."
    ./scripts/accelerators_diagnostics.sh
fi

# Run sovereign sectors diagnostics
if [ -f "scripts/sovereign_sectors_diagnostics.sh" ]; then
    log_info "Executing sovereign sector workspace diagnostics..."
    ./scripts/sovereign_sectors_diagnostics.sh
fi

# Run package translation check
if [ -f "scripts/app_regression_test.sh" ]; then
    log_info "Executing package format translation testing..."
    ./scripts/app_regression_test.sh
fi

# Run static regression check
if [ -f "scripts/regression_check.sh" ]; then
    log_info "Executing static memory usage and code limit analysis..."
    ./scripts/regression_check.sh
fi

# Run formatting stress check
if [ -f "scripts/format_stress_test.sh" ]; then
    log_info "Executing codebase cleanliness stress test..."
    ./scripts/format_stress_test.sh
fi

log_info "All QA smoke tests completed successfully!"
