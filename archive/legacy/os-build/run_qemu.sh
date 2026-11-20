#!/usr/bin/env bash
# SigmaOS QEMU Runner
# ============================================================
# Boots SigmaOS (stub or real) in QEMU for smoke testing.
# DEFENSIVE PROGRAMMING: Every precondition is checked.
# LEAST PRIVILEGE: Runs QEMU as user, no root needed.
# DISCOVERABILITY: Clear error messages guide the developer.
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
ISO="${1:-$ROOT_DIR/build/sigmaos-desktop-preview.iso}"

# ─────────────────────────────────────────────────────────────────────────────
# Preflight checks (FAIL-FAST)
# ─────────────────────────────────────────────────────────────────────────────
if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then
    echo ""
    echo "ERROR: qemu-system-x86_64 not found." >&2
    echo ""
    echo "Install QEMU:"
    echo "  sudo apt-get install qemu-system-x86   # Ubuntu/Debian"
    echo "  sudo pacman -S qemu-full               # Arch/CachyOS"
    echo "  sudo dnf install qemu-system-x86       # Fedora"
    echo ""
    exit 1
fi

if [ ! -f "$ISO" ]; then
    echo ""
    echo "ERROR: ISO not found at: $ISO" >&2
    echo ""
    echo "Build the stub ISO first:"
    echo "  make iso-stub"
    echo "  # or directly:"
    echo "  bash scripts/build_stub_iso.sh"
    echo ""
    exit 1
fi

# Validate ISO9660 signature (FAIL-FAST before wasting QEMU startup time)
if ! python3 -c "
import sys
f = open('$ISO', 'rb')
f.seek(32769)
ok = f.read(5) == b'CD001'
f.close()
sys.exit(0 if ok else 1)
" 2>/dev/null; then
    echo "ERROR: '$ISO' is not a valid ISO9660 image." >&2
    echo "Rebuild with: make iso-stub" >&2
    exit 1
fi

ISO_SIZE=$(du -sh "$ISO" | cut -f1)

echo ""
echo "=== [SigmaOS] QEMU Boot Test ==="
echo "    ISO    : $ISO ($ISO_SIZE)"
echo "    Memory : 512M"
echo "    Serial : stdout (for stub kernel output)"
echo ""
echo "  QEMU keybindings:"
echo "    Ctrl+Alt+G : Release mouse grab"
echo "    Ctrl+Alt+2 : QEMU monitor"
echo "    Ctrl+C     : Exit QEMU (in -nographic mode)"
echo ""

# ─────────────────────────────────────────────────────────────────────────────
# Boot in QEMU
# PRINCIPLE: PORTABILITY — detect KVM acceleration if available
# ─────────────────────────────────────────────────────────────────────────────
KVM_FLAG=""
if [ -e /dev/kvm ] && [ -r /dev/kvm ]; then
    KVM_FLAG="-enable-kvm -cpu host"
    echo "  KVM acceleration: ENABLED"
else
    echo "  KVM acceleration: disabled (running in TCG mode)"
fi

echo ""
exec qemu-system-x86_64 \
    -m 512 \
    -cdrom "$ISO" \
    -serial stdio \
    -no-reboot \
    -vga std \
    $KVM_FLAG \
    -name "SigmaOS v0.1 Stub" \
    "$@"
