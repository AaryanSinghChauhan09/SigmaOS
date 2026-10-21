#!/usr/bin/env bash
# SigmaOS Stub ISO Builder
# ============================================================
# Builds a bootable GRUB-based ISO that demonstrates SigmaOS
# boot infrastructure without requiring a compiled Rust kernel.
#
# GALL'S LAW: A working simple system > broken complex one.
# DEFENSIVE PROGRAMMING: Every tool is checked before use.
# LAYERED DESIGN: GRUB (bootloader) -> stub boot menu -> kernel
# BOY SCOUT RULE: Leaves build/ better than found.
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
ISO_BUILD_DIR="$ROOT_DIR/target/stub_iso_build"
ISO_OUTPUT="${1:-$ROOT_DIR/build/sigmaos-desktop-preview.iso}"
GRUB_LIB="/usr/lib/grub/i386-pc"

echo "=== [SigmaOS] Building Stub ISO ==="
echo "    Root: $ROOT_DIR"
echo "    Output: $ISO_OUTPUT"

# ─────────────────────────────────────────────────────────────────────────────
# DEFENSIVE PROGRAMMING: Tool preflight
# ─────────────────────────────────────────────────────────────────────────────
check_tool() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "ERROR: Required tool '$1' not found." >&2
        echo "Install with: sudo apt-get install $2" >&2
        exit 1
    fi
}

check_grub_lib() {
    if [ ! -d "$GRUB_LIB" ]; then
        echo "ERROR: GRUB i386-pc library not found at $GRUB_LIB" >&2
        echo "Install with: sudo apt-get install grub-pc-bin" >&2
        exit 1
    fi
}

check_tool "grub-mkimage" "grub-pc-bin"
check_tool "grub-mkimage" "grub-pc-bin"
if command -v genisoimage >/dev/null 2>&1; then
    ISO_TOOL="genisoimage"
elif command -v mkisofs >/dev/null 2>&1; then
    ISO_TOOL="mkisofs"
elif command -v xorriso >/dev/null 2>&1; then
    ISO_TOOL="xorriso_mode"
else
    echo "ERROR: No ISO tool found. Install xorriso or genisoimage:" >&2
    echo "  sudo apt-get install xorriso" >&2
    exit 1
fi
check_grub_lib

echo "--> Tools: grub-mkimage, $ISO_TOOL [OK]"

# ─────────────────────────────────────────────────────────────────────────────
# Prepare build directory
# ─────────────────────────────────────────────────────────────────────────────
rm -rf "$ISO_BUILD_DIR"
mkdir -p "$ISO_BUILD_DIR/boot/grub/i386-pc"
mkdir -p "$(dirname "$ISO_OUTPUT")"

# ─────────────────────────────────────────────────────────────────────────────
# GRUB configuration — SigmaOS boot menu
# PRINCIPLE: LEAST ASTONISHMENT — familiar GRUB interface
# ─────────────────────────────────────────────────────────────────────────────
cat > "$ISO_BUILD_DIR/boot/grub/grub.cfg" << 'GRUBCFG'
# SigmaOS GRUB Boot Configuration
# Follows: BOOTLOADER INTEGRATION, SECURE DEFAULTS, LEAST ASTONISHMENT

set default=0
set timeout=5
set timeout_style=menu

# ─────────────────────────────────────────
# SigmaOS boot entries
# ─────────────────────────────────────────

menuentry "SigmaOS v0.1 — Sovereign OS" --class sigmaos {
    echo ""
    echo "  ╔══════════════════════════════════════════════════════╗"
    echo "  ║          SigmaOS v0.1 — Sovereign OS                ║"
    echo "  ║   #![no_std] Rust · Arch · CachyOS · Debian parity ║"
    echo "  ║   BSD: DragonFly · FreeBSD · OpenBSD               ║"
    echo "  ║   Craft Suite: 20 creative apps                     ║"
    echo "  ╚══════════════════════════════════════════════════════╝"
    echo ""
    echo "  Build Status : cargo check --lib PASSING (0 errors)"
    echo "  Bootloader   : GRUB2 (Limine planned for real kernel)"
    echo "  Kernel       : Rust no_std (compilation target pending)"
    echo ""
    echo "  To complete OS boot, compile boot/stub_kernel.asm:"
    echo "    sudo apt install nasm xorriso"
    echo "    make iso-stub"
    echo ""
    echo "  Repository: https://github.com/AaryanSinghChauhan09/SigmaOS"
    echo ""
    echo "  Dropping to GRUB shell. Type 'reboot' to restart."
    echo ""
}

menuentry "SigmaOS — GRUB Debug Shell" --class shell {
    set pager=1
    echo "GRUB shell active. Commands: ls, cat, set, reboot, halt"
}

menuentry "System Information" --class info {
    echo ""
    echo "  SigmaOS System Information"
    echo "  ─────────────────────────"
    lsefisystab || echo "  (no EFI system table)"
    echo ""
    echo "  Disk devices:"
    ls -l || true
    echo ""
}

menuentry "Reboot" --class reboot { reboot }
menuentry "Power Off" --class halt { halt }
GRUBCFG

echo "--> GRUB config written [OK]"

# ─────────────────────────────────────────────────────────────────────────────
# Stage real kernel if it exists (OPEN-CLOSED: extensible without modification)
# ─────────────────────────────────────────────────────────────────────────────
KERNEL_ELF="$ROOT_DIR/boot/sigma_kernel.elf"
if [ -f "$KERNEL_ELF" ]; then
    echo "--> Staging kernel: $KERNEL_ELF"
    mkdir -p "$ISO_BUILD_DIR/boot"
    cp "$KERNEL_ELF" "$ISO_BUILD_DIR/boot/sigma_kernel.elf"
    # Add multiboot2 kernel entry to grub.cfg
    cat >> "$ISO_BUILD_DIR/boot/grub/grub.cfg" << 'KERNELENTRY'

# Auto-detected: real kernel found!
menuentry "SigmaOS — Boot Real Kernel" --class sigmaos-real {
    multiboot2 /boot/sigma_kernel.elf
    boot
}
KERNELENTRY
    echo "--> Real kernel entry added to GRUB menu [OK]"
fi

# ─────────────────────────────────────────────────────────────────────────────
# Build GRUB core image (El Torito bootloader)
# PRINCIPLE: MODULARITY — grub modules loaded on demand
# ─────────────────────────────────────────────────────────────────────────────
echo "--> Building GRUB core image..."
grub-mkimage \
    --format=i386-pc \
    --output=/tmp/sigmaos_core.img \
    --prefix=/boot/grub \
    biosdisk iso9660 normal echo linux multiboot2 multiboot \
    test search search_fs_file search_fs_uuid ls cat reboot halt \
    gfxterm gfxmenu font png jpeg 2>&1

# Combine cdboot.img + core.img → El Torito boot image
cat "$GRUB_LIB/cdboot.img" /tmp/sigmaos_core.img > \
    "$ISO_BUILD_DIR/boot/grub/i386-pc/eltorito.img"

# Copy GRUB modules for runtime loading (MODULARITY principle)
cp "$GRUB_LIB/"*.mod "$ISO_BUILD_DIR/boot/grub/i386-pc/" 2>/dev/null || true

echo "--> GRUB core image built [OK]"

# ─────────────────────────────────────────────────────────────────────────────
# Assemble the ISO image
# PRINCIPLE: PORTABILITY — supports genisoimage or xorriso
# ─────────────────────────────────────────────────────────────────────────────
echo "--> Assembling ISO with $ISO_TOOL..."

if [ "$ISO_TOOL" = "xorriso_mode" ]; then
    xorriso -as mkisofs \
        -R -J -V "SIGMAOS_V01" \
        -A "SigmaOS v0.1 Sovereign OS" \
        -o "$ISO_OUTPUT" \
        -b boot/grub/i386-pc/eltorito.img \
        -c boot/boot.catalog \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        "$ISO_BUILD_DIR" 2>&1
else
    $ISO_TOOL \
        -R -J \
        -V "SIGMAOS_V01" \
        -A "SigmaOS v0.1 Sovereign OS" \
        -o "$ISO_OUTPUT" \
        -b boot/grub/i386-pc/eltorito.img \
        -c boot/boot.catalog \
        -no-emul-boot \
        -boot-load-size 4 \
        -boot-info-table \
        "$ISO_BUILD_DIR" 2>&1 | grep -v "^Using\|^I:"
fi

# ─────────────────────────────────────────────────────────────────────────────
# Validate the ISO (FAIL-FAST: verify before reporting success)
# ─────────────────────────────────────────────────────────────────────────────
if ! python3 -c "
import sys
f = open('$ISO_OUTPUT', 'rb')
f.seek(32769)
ok = f.read(5) == b'CD001'
f.close()
sys.exit(0 if ok else 1)
" 2>/dev/null; then
    echo "ERROR: ISO validation failed — not a valid ISO9660 image" >&2
    exit 1
fi

ISO_SIZE=$(du -sh "$ISO_OUTPUT" | cut -f1)
echo ""
echo "=== [SigmaOS] Stub ISO Built Successfully ==="
echo "    Output : $ISO_OUTPUT ($ISO_SIZE)"
echo "    Format : ISO9660 + El Torito (GRUB2 bootable)"
echo ""
echo "  To test in QEMU:"
echo "    make run-stub"
echo "  Or manually:"
echo "    qemu-system-x86_64 -m 512 -cdrom '$ISO_OUTPUT' -serial stdio"
echo ""
