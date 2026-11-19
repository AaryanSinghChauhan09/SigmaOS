#!/usr/bin/env bash
# qemu-test-helper.sh — Helper script for QEMU VM testing
#
# This script automates QEMU VM testing of the SigmaOS ISO.
# Following: DETERMINISTIC TESTING, TEST CODE USING CI/CD, FAST-ISOLATED-REPEATABLE
#
# Usage:
#   ./scripts/qemu-test-helper.sh [options]
#
# Options:
#   --iso PATH       Path to ISO file (default: out/sigmaos-desktop-*.iso)
#   --disk PATH      Path to disk image (default: /tmp/sigmaos_test.qcow2)
#   --memory SIZE    VM memory in MB (default: 4096)
#   --no-kvm         Disable KVM acceleration

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'
BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}[qemu-test]${RESET} $*"; }
success() { echo -e "${GREEN}[qemu-test]${RESET} $*"; }
warn()    { echo -e "${YELLOW}[qemu-test]${RESET} $*"; }
err()     { echo -e "${RED}[qemu-test]${RESET} $*" >&2; }

# Default values
ISO_PATH=""
DISK_PATH="/tmp/sigmaos_test.qcow2"
MEMORY="4096"
USE_KVM=1

# Parse arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --iso) ISO_PATH="$2"; shift 2 ;;
        --disk) DISK_PATH="$2"; shift 2 ;;
        --memory) MEMORY="$2"; shift 2 ;;
        --no-kvm) USE_KVM=0; shift ;;
        -h|--help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --iso PATH       Path to ISO file"
            echo "  --disk PATH      Path to disk image"
            echo "  --memory SIZE    VM memory in MB"
            echo "  --no-kvm         Disable KVM acceleration"
            exit 0
            ;;
        *) err "Unknown option: $1"; exit 1 ;;
    esac
done

# Auto-detect ISO if not specified
if [[ -z "$ISO_PATH" ]]; then
    info "Auto-detecting ISO file..."
    ISO_PATH=$(find "$ROOT_DIR/out" -name "sigmaos-desktop-*.iso" 2>/dev/null | head -1)
    
    if [[ -z "$ISO_PATH" ]]; then
        err "No ISO file found in $ROOT_DIR/out"
        err "Build the ISO first: sudo scripts/build-iso-helper.sh --docker"
        exit 1
    fi
fi

# Check ISO exists
if [[ ! -f "$ISO_PATH" ]]; then
    err "ISO file not found: $ISO_PATH"
    exit 1
fi

# Create disk image if it doesn't exist
if [[ ! -f "$DISK_PATH" ]]; then
    info "Creating disk image: $DISK_PATH (32GB)"
    qemu-img create -f qcow2 "$DISK_PATH" 32G
fi

# Check KVM availability
if [[ $USE_KVM -eq 1 ]]; then
    if [[ ! -e /dev/kvm ]]; then
        warn "/dev/kvm not found, disabling KVM acceleration"
        USE_KVM=0
    fi
fi

# Build QEMU command
QEMU_CMD="qemu-system-x86_64"
QEMU_CMD+=" -m $MEMORY"
QEMU_CMD+=" -drive file=$DISK_PATH,format=qcow2,if=virtio"
QEMU_CMD+=" -cdrom $ISO_PATH"
QEMU_CMD+=" -boot d"

if [[ $USE_KVM -eq 1 ]]; then
    QEMU_CMD+=" -enable-kvm"
fi

# Run QEMU
echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}${CYAN}║   SigmaOS QEMU Test Helper                 ║${RESET}"
echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════╝${RESET}"
echo ""
info "ISO: $ISO_PATH"
info "Disk: $DISK_PATH"
info "Memory: ${MEMORY}MB"
info "KVM: $([ $USE_KVM -eq 1 ] && echo "enabled" || echo "disabled")"
echo ""
info "Starting QEMU..."
info "Press Ctrl+A then H for help, Ctrl+A then X to exit"
echo ""

$QEMU_CMD
