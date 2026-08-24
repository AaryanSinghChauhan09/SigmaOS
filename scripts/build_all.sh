#!/bin/bash
# =============================================================================
# SIGMAOS: UNIFIED BUILD ORCHESTRATOR
# Inspired by OpenBSD build.sh and FreeBSD release/build script paradigms.
# =============================================================================

set -euo pipefail

# --- Defaults & Config ---
ARCHS=("x86_64" "aarch64" "riscv64")
DRY_RUN=0
VERBOSE=0

# --- Console Color Support ---
if [ -t 1 ] && [ "${TERM:-dumb}" != "dumb" ]; then
    COLOR_CYAN='\033[1;36m'
    COLOR_GREEN='\033[1;32m'
    COLOR_RED='\033[1;31m'
    COLOR_YELLOW='\033[1;33m'
    COLOR_RESET='\033[0m'
else
    COLOR_CYAN=''
    COLOR_GREEN=''
    COLOR_RED=''
    COLOR_YELLOW=''
    COLOR_RESET=''
fi

log_step() { printf "${COLOR_CYAN}[BUILD]${COLOR_RESET} %s\n" "$*"; }
log_ok()   { printf "${COLOR_GREEN}[SUCCESS]${COLOR_RESET} %s\n" "$*"; }
log_warn() { printf "${COLOR_YELLOW}[WARN]${COLOR_RESET} %s\n" "$*"; }
log_fail() { printf "${COLOR_RED}[FAILURE]${COLOR_RESET} %s\n" "$*" >&2; }

trap 'log_fail "Build orchestrator terminated prematurely due to an unexpected error."' ERR INT TERM

usage() {
    cat <<EOF
SigmaOS Unified Multi-Architecture Build Orchestrator

Usage:
  $(basename "$0") [options]

Options:
  -d          Dry-run mode (print make commands without building)
  -v          Enable verbose tracing
  -h          Show this help dialog
EOF
}

while getopts "dvh" opt; do
    case "$opt" in
        d) DRY_RUN=1 ;;
        v) VERBOSE=1; set -x ;;
        h) usage; exit 0 ;;
        *) usage; exit 1 ;;
    esac
done

log_step "Starting Unified SigmaOS Zenith Multi-Architecture Build Orchestration..."

for ARCH in "${ARCHS[@]}"; do
    log_step "Orchestrating $ARCH Shard Lattice..."
    if [ "$DRY_RUN" -eq 1 ]; then
        log_ok "[DRY-RUN] make singularity ARCH=$ARCH"
    else
        if make singularity ARCH="$ARCH"; then
            log_ok "$ARCH Shard Verified."
        else
            log_fail "$ARCH Shard Build Failed."
            exit 1
        fi
    fi
done

log_step "Generating Unified Manifest & Listing Images..."
if [ "$DRY_RUN" -eq 1 ]; then
    log_ok "[DRY-RUN] ls -lh sigmaos-*.bin"
else
    ls -lh sigmaos-*.bin 2>/dev/null || log_warn "No matching sigmaos-*.bin images generated yet."
fi

log_step "Orchestrating Special Industrial Formats (x86_64)..."
if [ "$DRY_RUN" -eq 1 ]; then
    log_ok "[DRY-RUN] make build-embedded ARCH=x86_64"
    log_ok "[DRY-RUN] make build-rtos ARCH=x86_64"
    log_ok "[DRY-RUN] make build-cloud ARCH=x86_64"
else
    make build-embedded ARCH=x86_64 || log_warn "Embedded build skipped/failed."
    make build-rtos ARCH=x86_64 || log_warn "RTOS build skipped/failed."
    make build-cloud ARCH=x86_64 || log_warn "Cloud build skipped/failed."
fi

log_ok "All Industrial Shards and Formats Processed Successfully."
