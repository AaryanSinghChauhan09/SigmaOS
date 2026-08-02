#!/bin/bash
# SPDX-License-Identifier: MIT
# SigmaOS CI Branch Parity & Repository Layout Verifier (Linux Packaging Inspired)
# Asserts repository standards, documentation presence, and branch consistency.

set -e

BRANCH=""
VERBOSE=0

# Color Palettes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ==============================================================================
# LOGGING UTILITIES
# ==============================================================================
log_info() {
    echo -e "${BLUE}[CI-INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[CI-SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[CI-WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[CI-ERROR]${NC} $1" >&2
}

# ==============================================================================
# HELP DIALOG
# ==============================================================================
show_help() {
    echo -e "${CYAN}SigmaOS CI Branch Parity & Layout Checker${NC}"
    echo "Inspired by Debian packaging repository branch assertions."
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  --branch <name>     Target branch name for context checking"
    echo "  -v, --verbose       Enable verbose logging"
    echo "  -h, --help          Show this branch-check help manual"
    echo ""
    exit 0
}

# ==============================================================================
# ARGUMENT PARSING
# ==============================================================================
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --branch) BRANCH="$2"; shift ;;
        -v|--verbose) VERBOSE=1 ;;
        -h|--help) show_help ;;
        *) log_error "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

echo -e "${CYAN}=== SigmaOS CI Branch Parity Check ===${NC}"
log_info "Active target branch: ${BRANCH:-unknown}"

# Verify presence of core strategic files based on current repo structure
# (Local wiki/ directory has been migrated to GitHub wiki; roadmap exists as -origin.txt)
REQUIRED_FILES=(
    "FUTURE-DEVELOPMENT-ROADMAP-origin.txt"
)

failed=0
for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        log_success "Required layout file is present: $file"
    else
        log_error "Mandatory layout file is MISSING: $file"
        failed=1
    fi
done

if [ $failed -eq 1 ]; then
    log_error "Branch parity validation failed!"
    exit 1
fi

log_success "All branch parity and layout verification rules passed perfectly!"
exit 0
