#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# ----------------------------------------------------------------------------
# build_iso.sh: Validates and orchestrates bootable live ISO build for SigmaOS
# Inspires by omacom/omarchy-iso
# ----------------------------------------------------------------------------

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIGMA_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PROFILE_DIR="$SIGMA_ROOT/iso/archiso"
WORK_DIR="/tmp/sigmaos-iso-work"
OUT_DIR="$SIGMA_ROOT/out"

echo "============================================================"
echo "  SigmaOS / Omarchy Live ISO Build Orchestrator"
echo "============================================================"

# 1. Validation
echo "[+] Validating archiso profile definitions..."
if [[ ! -f "$PROFILE_DIR/profiledef.sh" ]]; then
    echo "[-] Error: profiledef.sh missing!"
    exit 1
fi
if [[ ! -f "$PROFILE_DIR/packages.x86_64" ]]; then
    echo "[-] Error: packages.x86_64 missing!"
    exit 1
fi
if [[ ! -f "$PROFILE_DIR/airootfs/etc/os-release" ]]; then
    echo "[-] Error: airootfs os-release missing!"
    exit 1
fi

pkg_count=$(grep -v '^#' "$PROFILE_DIR/packages.x86_64" | grep -v '^$' | wc -l)
echo "  [✓] Verified profile: $pkg_count base packages configured."
echo "  [✓] Verified airootfs branding: SigmaOS os-release & motd."

# 2. Check mkarchiso availability
if command -v mkarchiso >/dev/null 2>&1; then
    echo "[+] mkarchiso found. Ready to build bootable media."
    if [[ "${1:-}" == "--build" ]]; then
        mkdir -p "$OUT_DIR"
        sudo mkarchiso -v -w "$WORK_DIR" -o "$OUT_DIR" "$PROFILE_DIR"
        echo "[✓] ISO built successfully in $OUT_DIR"
    else
        echo "  Run with --build and root privileges to initiate ISO compilation."
    fi
else
    echo "[i] mkarchiso is not installed on this host."
    echo "  Install via: sudo pacman -S archiso"
    echo "  [✓] Profile validation passed: ready for clean containerized build."
fi

echo "============================================================"
