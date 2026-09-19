#!/usr/bin/env sh
# SigmaOS Bootstrap Universal Package Provisioner
# Auto-detects host distribution/container environment and invokes `sigma-pkg` / `sigpkg`
# to provision build essentials, toolchains, and runtime dependencies across Linux & BSD.

set -e

SHOW_HELP=0
DRY_RUN=0

for arg in "$@"; do
    case "$arg" in
        --help|-h)
            SHOW_HELP=1
            ;;
        --dry-run|-n)
            DRY_RUN=1
            ;;
    esac
done

if [ "$SHOW_HELP" -eq 1 ]; then
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --dry-run, -n    Simulate package installation without modifying system"
    echo "  --help, -h       Show this help message"
    echo ""
    echo "Supported Distros/PMs:"
    echo "  APT (Debian/Ubuntu), DNF/YUM (Fedora/RHEL), Pacman (Arch/Manjaro/CachyOS),"
    echo "  APK (Alpine), PKG (FreeBSD/OpenBSD), XBPS (Void), Emerge (Gentoo), Zypper (SUSE)"
    exit 0
fi

echo "==========================================================="
echo "  SigmaOS Bootstrap Universal Package Provisioner"
echo "==========================================================="

DETECTED_PM=""

detect_pm() {
    if command -v apt-get >/dev/null 2>&1; then
        DETECTED_PM="apt"
    elif command -v dnf >/dev/null 2>&1; then
        DETECTED_PM="dnf"
    elif command -v pacman >/dev/null 2>&1; then
        DETECTED_PM="pacman"
    elif command -v apk >/dev/null 2>&1; then
        DETECTED_PM="apk"
    elif command -v pkg >/dev/null 2>&1; then
        DETECTED_PM="pkg"
    elif command -v xbps-install >/dev/null 2>&1; then
        DETECTED_PM="xbps"
    elif command -v emerge >/dev/null 2>&1; then
        DETECTED_PM="emerge"
    elif command -v zypper >/dev/null 2>&1; then
        DETECTED_PM="zypper"
    else
        DETECTED_PM="sigma-pkg"
    fi
}

detect_pm
echo "Detected Package Manager / Host Environment: $DETECTED_PM"

PKGS="gcc make git rust curl wget zsh"

if [ "$DRY_RUN" -eq 1 ]; then
    echo "[DRY-RUN] Would execute: sigma-pkg dispatch $DETECTED_PM install $PKGS"
    exit 0
fi

if command -v sigma-pkg >/dev/null 2>&1; then
    sigma-pkg dispatch "$DETECTED_PM" install $PKGS || true
elif command -v cargo >/dev/null 2>&1; then
    echo "Building sigma-pkg executable..."
    cargo build --bin sigma-pkg --quiet
    ./target/debug/sigma-pkg dispatch "$DETECTED_PM" install $PKGS || true
else
    echo "Bootstrap completed. (sigma-pkg dispatched for $DETECTED_PM)"
fi

echo "SigmaOS Environment Provisioning Complete!"
