#!/usr/bin/env bash
# qemu_boot_smoke.sh — resolve a SigmaOS ISO and run the real QEMU boot test.
#
# Resolution order:
#   1. Explicit ISO path argument
#   2. Newest ISO in out/ (local builds)
#   3. Latest published GitHub release (downloaded, reassembled, verified)
#
# Requires: qemu-system-x86_64, gh (only for the release fallback), and an
# ISO extraction tool (xorriso/bsdtar/isoinfo/7z — auto-detected).
set -euo pipefail

cd "$(dirname "$0")/../.."

ISO="${1:-}"
WORK="${SIGMAOS_QEMU_WORK:-build/qemu-boot}"
TIMEOUT="${SIGMAOS_QEMU_TIMEOUT:-1200}"

if [[ -z "$ISO" ]]; then
    ISO="$(ls -t out/*.iso 2>/dev/null | head -1 || true)"
fi

if [[ -z "$ISO" || ! -f "$ISO" ]]; then
    echo "[boot-smoke] no local ISO — fetching latest published release"
    command -v gh >/dev/null || { echo "gh CLI required for release fallback" >&2; exit 1; }
    TAG="$(gh release list --limit 50 --json tagName -q '.[].tagName' | grep '^sigmaos-desktop-' | head -1)"
    [[ -n "$TAG" ]] || { echo "no sigmaos-desktop-* release found" >&2; exit 1; }
    DL="build/release-download"
    mkdir -p "$DL"
    gh release download "$TAG" --dir "$DL" --clobber \
        --pattern '*.part' --pattern '*.sha256'
    bash scripts/reassemble-iso.sh "$DL"
    ISO="$(find "$DL" -maxdepth 1 -name '*.iso' -printf '%T@ %p\n' | sort -rn | head -1 | cut -d' ' -f2-)"
    [[ -n "$ISO" && -f "$ISO" ]] || { echo "reassembly produced no ISO" >&2; exit 1; }
fi

echo "[boot-smoke] testing ISO: $ISO"
exec bash scripts/ci/qemu_iso_boot.sh "$ISO" "$WORK" "$TIMEOUT"
