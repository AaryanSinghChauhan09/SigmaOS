#!/bin/bash
# =========================================================================
# SIGMAOS: SOVEREIGN BUILD TOOLCHAIN SETUP
# =========================================================================
# Installs necessary cross-compiler, nasm, xorriso, grub, and QEMU tools.
# Inspired by FreeBSD pkg, OpenBSD pkg_add, Arch pacman, Debian apt, Alpine apk.
#
# Usage:
#   bash scripts/setup.sh          # Install all tools
#   bash scripts/setup.sh --check  # Check what's missing without installing
# =========================================================================

set -uo pipefail

# --- Colorized Console Output ---
if [ -t 1 ] && [ "${TERM:-dumb}" != "dumb" ]; then
    BOLD_GREEN='\033[1;32m'
    BOLD_BLUE='\033[1;34m'
    BOLD_RED='\033[1;31m'
    BOLD_YELLOW='\033[1;33m'
    RESET='\033[0m'
else
    BOLD_GREEN=''
    BOLD_BLUE=''
    BOLD_RED=''
    BOLD_YELLOW=''
    RESET=''
fi

log_info()    { printf "${BOLD_BLUE}[SIGMA-SETUP]${RESET} %s\n" "$*"; }
log_success() { printf "${BOLD_GREEN}[SIGMA-SETUP SUCCESS]${RESET} %s\n" "$*"; }
log_warn()    { printf "${BOLD_YELLOW}[SIGMA-SETUP WARN]${RESET} %s\n" "$*"; }
log_error()   { printf "${BOLD_RED}[SIGMA-SETUP ERROR]${RESET} %s\n" "$*" >&2; }

CHECK_ONLY=0
if [ "${1:-}" = "--check" ]; then
    CHECK_ONLY=1
fi

# ─────────────────────────────────────────────────────────────────────────────
# 1. Check what's already installed
# ─────────────────────────────────────────────────────────────────────────────
log_info "Checking installed tools..."

MISSING=()
for tool in nasm xorriso grub-mkimage grub-mkrescue genisoimage qemu-system-x86_64; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        MISSING+=("$tool")
        log_warn "  missing: $tool"
    else
        log_info "  found:   $tool ($(command -v "$tool"))"
    fi
done

# Check Rust
if command -v cargo >/dev/null 2>&1; then
    log_info "  found:   cargo ($(rustc --version 2>/dev/null || echo unknown))"
else
    MISSING+=("rust")
    log_warn "  missing: cargo/rust"
fi

if [ "${#MISSING[@]}" -eq 0 ]; then
    log_success "All tools already installed — nothing to do."
    exit 0
fi

if [ "$CHECK_ONLY" -eq 1 ]; then
    log_warn "Missing tools: ${MISSING[*]}"
    log_info "Run: bash scripts/setup.sh   (to install)"
    exit 1
fi

log_info "Installing missing tools: ${MISSING[*]}"

# ─────────────────────────────────────────────────────────────────────────────
# 2. Rust (no sudo needed)
# ─────────────────────────────────────────────────────────────────────────────
if ! command -v cargo >/dev/null 2>&1; then
    log_info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env" 2>/dev/null || true
fi

# ─────────────────────────────────────────────────────────────────────────────
# 3. System packages (distro-specific)
# ─────────────────────────────────────────────────────────────────────────────
if [ -x "$(command -v apt-get)" ]; then
    log_info "Debian/Ubuntu/Mint detected — using apt-get."
    sudo apt-get update -qq
    sudo apt-get install -y \
        build-essential \
        nasm \
        xorriso \
        grub-pc-bin \
        grub-efi-amd64-bin \
        genisoimage \
        qemu-system-x86 \
        gcc-multilib \
        g++-multilib
elif [ -x "$(command -v pacman)" ]; then
    log_info "Arch/Omarchy/CachyOS detected — using pacman."
    sudo pacman -Syu --noconfirm base-devel nasm xorriso grub qemu-desktop cdrtools
elif [ -x "$(command -v dnf)" ]; then
    log_info "Fedora/RHEL detected — using dnf."
    sudo dnf install -y gcc gcc-c++ make nasm xorriso grub2-tools-extra qemu-system-x86 genisoimage
elif [ -x "$(command -v apk)" ]; then
    log_info "Alpine Linux detected — using apk."
    sudo apk add build-base nasm xorriso grub qemu-system-x86_64 cdrkit
elif [ -x "$(command -v pkg)" ] && [ "$(uname -s)" = "FreeBSD" ]; then
    log_info "FreeBSD detected — using pkg."
    sudo pkg install -y nasm xorriso grub2-pcbsd qemu cdrtools
elif [ -x "$(command -v pkg_add)" ] && [ "$(uname -s)" = "OpenBSD" ]; then
    log_info "OpenBSD detected — using pkg_add."
    sudo pkg_add nasm xorriso qemu cdrtools
elif [ -x "$(command -v brew)" ]; then
    log_info "macOS/Homebrew detected."
    brew install nasm xorriso qemu x86_64-elf-gcc x86_64-elf-binutils cdrtools
else
    log_error "Unsupported OS or missing package manager."
    log_error "Please manually install: ${MISSING[*]}"
    log_error "Then re-run: bash scripts/setup.sh --check"
    exit 1
fi

# ─────────────────────────────────────────────────────────────────────────────
# 4. Verify
# ─────────────────────────────────────────────────────────────────────────────
log_info "Verifying installation..."
STILL_MISSING=()
for tool in nasm grub-mkimage genisoimage; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        STILL_MISSING+=("$tool")
    fi
done

if [ "${#STILL_MISSING[@]}" -gt 0 ]; then
    log_warn "Some tools could not be installed: ${STILL_MISSING[*]}"
    log_warn "SigmaOS will still work — ISO builds use genisoimage (found: $(command -v genisoimage 2>/dev/null || echo no))."
else
    log_success "All tools installed successfully."
fi

log_success "Setup complete. Run: make launch"

