#!/bin/bash
# =========================================================================
# SIGMAOS: SOVEREIGN BUILD TOOLCHAIN SETUP
# =========================================================================
# Installs necessary cross-compiler, nasm, xorriso, grub, and QEMU tools.
# Inspired by FreeBSD pkg, OpenBSD pkg_add, Arch pacman, Debian apt, Alpine apk.

set -euo pipefail

# --- Colorized Console Output ---
if [ -t 1 ] && [ "${TERM:-dumb}" != "dumb" ]; then
    BOLD_GREEN='\033[1;32m'
    BOLD_BLUE='\033[1;34m'
    BOLD_RED='\033[1;31m'
    RESET='\033[0m'
else
    BOLD_GREEN=''
    BOLD_BLUE=''
    BOLD_RED=''
    RESET=''
fi

log_info() { printf "${BOLD_BLUE}[SIGMA-SETUP]${RESET} %s\n" "$*"; }
log_success() { printf "${BOLD_GREEN}[SIGMA-SETUP SUCCESS]${RESET} %s\n" "$*"; }
log_error() { printf "${BOLD_RED}[SIGMA-SETUP ERROR]${RESET} %s\n" "$*" >&2; exit 1; }

trap 'log_error "Setup script interrupted or failed unexpectedly."' ERR INT TERM

log_info "Initializing Multi-Distro / Multi-OS Toolchain Installation..."

if [ -x "$(command -v apt-get)" ]; then
    log_info "Debian/Ubuntu Linux distribution detected."
    sudo apt-get update
    sudo apt-get install -y build-essential \
                            nasm \
                            xorriso \
                            grub-pc-bin \
                            grub-efi-amd64-bin \
                            qemu-system-x86 \
                            gcc-x86-64-linux-gnu \
                            g++-x86-64-linux-gnu
elif [ -x "$(command -v pacman)" ]; then
    log_info "Arch Linux distribution detected."
    sudo pacman -Syu --noconfirm base-devel nasm xorriso grub qemu-desktop
elif [ -x "$(command -v dnf)" ]; then
    log_info "Fedora / RHEL Linux distribution detected."
    sudo dnf install -y gcc gcc-c++ make nasm xorriso grub2-tools-extra qemu-system-x86
elif [ -x "$(command -v apk)" ]; then
    log_info "Alpine Linux distribution detected."
    sudo apk add build-base nasm xorriso grub qemu-system-x86_64
elif [ -x "$(command -v pkg)" ] && [ "$(uname -s)" = "FreeBSD" ]; then
    log_info "FreeBSD operating system detected."
    sudo pkg install -y nasm xorriso grub2-pcbsd qemu
elif [ -x "$(command -v pkg_add)" ] && [ "$(uname -s)" = "OpenBSD" ]; then
    log_info "OpenBSD operating system detected."
    sudo pkg_add nasm xorriso qemu
elif [ -x "$(command -v brew)" ]; then
    log_info "macOS (Homebrew) detected."
    brew install nasm xorriso qemu x86_64-elf-gcc x86_64-elf-binutils
else
    log_error "Unsupported OS or missing supported package manager (apt-get, pacman, dnf, apk, pkg, pkg_add, brew)."
fi

log_success "Toolchain successfully provisioned. You can now execute 'make all' or './scripts/sigma_build.sh'."
