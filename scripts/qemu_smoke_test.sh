#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# SigmaOS Automated Verification & QEMU Smoke Test Runner
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$REPO_ROOT"

echo "=== [1/2] Verifying Defensive Installer Dry-Run Safety ==="
bash "$REPO_ROOT/iso_root/installer/install.sh" --auto --dry-run /dev/vda
echo "[✓] Installer dry-run safety verification passed."

echo ""
echo "=== [2/2] Checking QEMU Boot Smoke Test Capabilities ==="
QEMU_BIN="$(command -v qemu-system-x86_64 || true)"
ISO_PATH=""
for cand in "out/sigmaos-desktop.iso" "build/sigmaos-desktop-preview.iso" "build/sigmaos.iso"; do
    if [[ -f "$cand" && -s "$cand" ]]; then
        ISO_PATH="$cand"
        break
    fi
done

if [[ -z "$QEMU_BIN" ]]; then
    echo "[SKIP] qemu-system-x86_64 not found in PATH."
    echo "       To enable full VM booting: sudo apt install -y qemu-system-x86"
    exit 0
fi

if [[ -z "$ISO_PATH" ]]; then
    echo "[SKIP] No compiled bootable ISO found (searched out/ and build/)."
    echo "       To build the live ISO profile: bash scripts/build_archiso.sh"
    exit 0
fi

echo "Found ISO: $ISO_PATH"
python3 "$SCRIPT_DIR/qemu_smoke_test.py" --iso "$ISO_PATH" "$@"
