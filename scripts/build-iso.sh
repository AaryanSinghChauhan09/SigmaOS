#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-or-later
# =============================================================================
# SigmaOS ISO builder — mkinitcpio-style initramfs + grub-mkrescue
# (Arch Linux initramfs hook system inspired)
#
# Usage: ./scripts/build-iso.sh [BUILDDIR] [PROFILE]
#   BUILDDIR  default: build
#   PROFILE   default: standalone  (standalone | iot-arm64 | cloud-x86 | embedded-riscv)
# =============================================================================
set -euo pipefail

BUILDDIR="${1:-build}"
PROFILE="${2:-standalone}"
HOOKS_DIR="initramfs/hooks"
INITRAMFS_ROOT="$BUILDDIR/initramfs_root"
ISO_DIR="$BUILDDIR/iso"
KERNEL_BIN="$BUILDDIR/sigmaos.bin"
ISO_NAME="sigmaos-$PROFILE-$(date +%Y%m%d).iso"

# Reproducible builds: honour SOURCE_DATE_EPOCH if set
if [[ -n "${SOURCE_DATE_EPOCH:-}" ]]; then
    echo "[build-iso] Reproducible build: SOURCE_DATE_EPOCH=$SOURCE_DATE_EPOCH"
    TOUCH_DATE=$(date -d "@$SOURCE_DATE_EPOCH" +"%Y%m%d%H%M.%S" 2>/dev/null || \
                 date -r "$SOURCE_DATE_EPOCH"  +"%Y%m%d%H%M.%S")
fi

echo "[build-iso] Profile: $PROFILE"
echo "[build-iso] Build dir: $BUILDDIR"

# ---- 1. Verify kernel binary exists ----------------------------------------
if [[ ! -f "$KERNEL_BIN" ]]; then
    echo "[build-iso] ERROR: $KERNEL_BIN not found. Run 'make all' first." >&2
    exit 1
fi

# ---- 2. Build initramfs via hooks ------------------------------------------
echo "[build-iso] Building initramfs..."
rm -rf "$INITRAMFS_ROOT"
mkdir -p "$INITRAMFS_ROOT"/{bin,dev,etc/sigma,proc,sys,tmp,run,sigma}

# Copy essential binaries
cp "$BUILDDIR/bin/sigma_init"          "$INITRAMFS_ROOT/bin/"     2>/dev/null || true
cp "$BUILDDIR/bin/sigma_cryptfs_unlock" "$INITRAMFS_ROOT/bin/"    2>/dev/null || true
cp "$BUILDDIR/bin/busybox"              "$INITRAMFS_ROOT/bin/"     2>/dev/null || true

# Run initramfs hooks (Arch mkinitcpio-inspired)
if [[ -d "$HOOKS_DIR" ]]; then
    for hook in "$HOOKS_DIR"/sigma-*; do
        [[ -x "$hook" ]] || continue
        echo "[build-iso] Running hook: $(basename "$hook")"
        # Each hook exports build() which populates $INITRAMFS_ROOT
        INITRAMFS_ROOT="$INITRAMFS_ROOT" bash -c "source $hook; build"
    done
fi

# Copy crypttab if present (sigma-crypt hook)
[[ -f etc/sigma/crypttab ]] && cp etc/sigma/crypttab "$INITRAMFS_ROOT/etc/sigma/"

# Pack initramfs as newc cpio, gzip-compressed
echo "[build-iso] Packing initramfs..."
mkdir -p "$ISO_DIR/boot"
(cd "$INITRAMFS_ROOT" && find . | sort | cpio -o -H newc 2>/dev/null) \
    | gzip -9 > "$ISO_DIR/boot/initrd.img"

# ---- 3. Copy kernel ---------------------------------------------------------
cp "$KERNEL_BIN" "$ISO_DIR/boot/sigmaos.bin"

# ---- 4. Write GRUB config --------------------------------------------------
mkdir -p "$ISO_DIR/boot/grub"
cat > "$ISO_DIR/boot/grub/grub.cfg" <<'EOF'
set timeout=3
set default=0

menuentry "SigmaOS Zenith" {
    multiboot2 /boot/sigmaos.bin
    module2    /boot/initrd.img
    boot
}

menuentry "SigmaOS Zenith (recovery)" {
    multiboot2 /boot/sigmaos.bin sigma.recovery=1
    module2    /boot/initrd.img
    boot
}
EOF

# ---- 5. Assemble ISO with grub-mkrescue ------------------------------------
echo "[build-iso] Assembling ISO..."
grub-mkrescue -o "$BUILDDIR/$ISO_NAME" "$ISO_DIR" \
    --modules="normal iso9660 multiboot2 gzio" \
    2>&1 | grep -v "^$" || true

echo "[build-iso] ✓ ISO ready: $BUILDDIR/$ISO_NAME"
echo "[build-iso] Size: $(du -h "$BUILDDIR/$ISO_NAME" | cut -f1)"
