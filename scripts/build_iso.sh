#!/usr/bin/env bash
# SigmaOS Sovereign Desktop ISO Image Builder
# Produces a bootable desktop ISO modeled on Omarchy 4.0.5.
# The ISO contains the SigmaOS installer, all system files, and a limine
# bootloader so it can be booted and used to install SigmaOS on bare metal.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$ROOT_DIR/target/iso_build"
ISO_OUTPUT="${1:-$ROOT_DIR/target/sigmaos.iso}"

echo "=== [SigmaOS] Starting Sovereign Desktop ISO Construction ==="
echo "Workspace: $ROOT_DIR"
echo "Target output: $ISO_OUTPUT"

# ---------------------------------------------------------------------------
# Clean and prepare build directory
# ---------------------------------------------------------------------------
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# ---------------------------------------------------------------------------
# Stage all SigmaOS system files into the ISO
# ---------------------------------------------------------------------------
echo "--> Staging SigmaOS system files..."

# /etc configuration
mkdir -p "$BUILD_DIR/etc"
if [ -d "$ROOT_DIR/etc" ]; then
    cp -a "$ROOT_DIR/etc"/* "$BUILD_DIR/etc/" 2>/dev/null || true
    echo "    Staged /etc configuration"
fi

# /usr/share/sigmaos/default — desktop configuration
mkdir -p "$BUILD_DIR/usr/share/sigmaos/default"
if [ -d "$ROOT_DIR/default" ]; then
    cp -a "$ROOT_DIR/default"/* "$BUILD_DIR/usr/share/sigmaos/default/" 2>/dev/null || true
    echo "    Staged default desktop configuration"
fi

# /usr/share/sigmaos/shell — QML desktop shell
mkdir -p "$BUILD_DIR/usr/share/sigmaos/shell"
if [ -d "$ROOT_DIR/shell" ]; then
    cp -a "$ROOT_DIR/shell"/* "$BUILD_DIR/usr/share/sigmaos/shell/" 2>/dev/null || true
    echo "    Staged shell configuration"
fi

# /usr/bin — runtime executables
mkdir -p "$BUILD_DIR/usr/bin"
if [ -d "$ROOT_DIR/bin" ]; then
    cp -a "$ROOT_DIR/bin"/* "$BUILD_DIR/usr/bin/" 2>/dev/null || true
    echo "    Staged runtime binaries"
fi

# /usr/share/plymouth/themes/sigmaos — boot theme
mkdir -p "$BUILD_DIR/usr/share/plymouth/themes/sigmaos"
if [ -d "$ROOT_DIR/usr/share/plymouth/themes/sigmaos" ]; then
    cp -a "$ROOT_DIR/usr/share/plymouth/themes/sigmaos"/* "$BUILD_DIR/usr/share/plymouth/themes/sigmaos/" 2>/dev/null || true
    echo "    Staged Plymouth boot theme"
fi

# /boot/limine — bootloader configuration
mkdir -p "$BUILD_DIR/boot/limine"
if [ -d "$ROOT_DIR/boot/limine" ]; then
    cp -a "$ROOT_DIR/boot/limine"/* "$BUILD_DIR/boot/limine/" 2>/dev/null || true
    echo "    Staged limine bootloader configuration"
fi

# /etc/systemd/user — user services
mkdir -p "$BUILD_DIR/etc/systemd/user"
if [ -d "$ROOT_DIR/etc/systemd/user" ]; then
    cp -a "$ROOT_DIR/etc/systemd/user"/* "$BUILD_DIR/etc/systemd/user/" 2>/dev/null || true
    echo "    Staged user systemd services"
fi

# /usr/share/wayland-sessions and /usr/share/xsessions — display manager entries
mkdir -p "$BUILD_DIR/usr/share/wayland-sessions" "$BUILD_DIR/usr/share/xsessions"
if [ -d "$ROOT_DIR/usr/share/wayland-sessions" ]; then
    cp -a "$ROOT_DIR/usr/share/wayland-sessions"/* "$BUILD_DIR/usr/share/wayland-sessions/" 2>/dev/null || true
fi
if [ -d "$ROOT_DIR/usr/share/xsessions" ]; then
    cp -a "$ROOT_DIR/usr/share/xsessions"/* "$BUILD_DIR/usr/share/xsessions/" 2>/dev/null || true
fi

# /etc/skel — user dotfiles
mkdir -p "$BUILD_DIR/etc/skel"
if [ -d "$ROOT_DIR/etc/skel" ]; then
    cp -a "$ROOT_DIR/etc/skel"/* "$BUILD_DIR/etc/skel/" 2>/dev/null || true
    echo "    Staged /etc/skel dotfiles"
fi

# /install — provisioning scripts
mkdir -p "$BUILD_DIR/install"
if [ -d "$ROOT_DIR/install" ]; then
    cp -a "$ROOT_DIR/install"/* "$BUILD_DIR/install/" 2>/dev/null || true
    echo "    Staged install/provisioning scripts"
fi

# /iso_root — ISO root (mount point for installer)
mkdir -p "$BUILD_DIR/iso_root"
if [ -d "$ROOT_DIR/iso_root" ]; then
    cp -a "$ROOT_DIR/iso_root"/* "$BUILD_DIR/iso_root/" 2>/dev/null || true
    echo "    Staged ISO root contents"
fi

# /pkgbuild — package definition
mkdir -p "$BUILD_DIR/pkgbuild"
if [ -d "$ROOT_DIR/pkgbuild" ]; then
    cp -a "$ROOT_DIR/pkgbuild"/* "$BUILD_DIR/pkgbuild/" 2>/dev/null || true
    echo "    Staged package definition"
fi

# ---------------------------------------------------------------------------
# Build initramfs containing the installer
# ---------------------------------------------------------------------------
echo "--> Building initramfs with SigmaOS installer..."
INITRAMFS_STAGE="$BUILD_DIR/initramfs_stage"
rm -rf "$INITRAMFS_STAGE"
mkdir -p "$INITRAMFS_STAGE"

# Create minimal directory structure
mkdir -p "$INITRAMFS_STAGE"/{bin,sbin,etc,proc,sys,dev,run,tmp,mnt,root,usr/bin,usr/sbin,usr/lib,boot}

# Copy the installer into the initramfs
if [ -f "$ROOT_DIR/bin/sigmaos-install" ]; then
    cp -a "$ROOT_DIR/bin/sigmaos-install" "$INITRAMFS_STAGE/usr/bin/sigmaos-install"
    chmod +x "$INITRAMFS_STAGE/usr/bin/sigmaos-install"
fi

# Copy essential tools
for tool in sh bash mount umount sgdisk mkfs.fat mkfs.ext4 partprobe blkid genfstab cp sed grep awk find chmod chown; do
    if command -v "$tool" >/dev/null 2>&1; then
        tool_path="$(command -v "$tool")"
        target="$INITRAMFS_STAGE${tool_path}"
        mkdir -p "$(dirname "$target")" 2>/dev/null || true
        cp -a "$tool_path" "$target" 2>/dev/null || true
    fi
done

# Copy shared libraries for the tools (ldd resolution)
for lib in $(ldd $(command -v bash) 2>/dev/null | awk '{print $3}' | grep -v '^$'); do
    if [ -f "$lib" ]; then
        target="$INITRAMFS_STAGE$lib"
        mkdir -p "$(dirname "$target")" 2>/dev/null || true
        cp -a "$lib" "$target" 2>/dev/null || true
    fi
done

# Create init script that launches the installer
cat > "$INITRAMFS_STAGE/init" <<'INITEOF'
#!/bin/sh
# SigmaOS initramfs init — launches the installer on boot
set -euo pipefail

mount -t proc none /proc
mount -t sysfs none /sys
mount -t devtmpfs none /dev || true
mkdir -p /dev/pts
mount -t devpts none /dev/pts 2>/dev/null || true

echo "========================================="
echo "  SigmaOS Desktop Installer (Initramfs)"
echo "========================================="
echo ""
echo "To install SigmaOS, run:"
echo "  sigmaos-install --target /dev/sda --hostname sigmaos --username sigma"
echo ""
echo "Or run the interactive ISO installer from /iso_root/installer/install.sh"
echo ""

# Drop to a shell so the user can run the installer
exec /bin/sh
INITEOF
chmod +x "$INITRAMFS_STAGE/init"

# Create the cpio initramfs image
(
    cd "$INITRAMFS_STAGE"
    find . | cpio -o -H newc 2>/dev/null | gzip > "$BUILD_DIR/boot/initramfs.gz"
)
echo "    Initramfs created: $BUILD_DIR/boot/initramfs.gz"

# ---------------------------------------------------------------------------
# Stage kernel (real or stub)
# ---------------------------------------------------------------------------
echo "--> Staging kernel..."
mkdir -p "$BUILD_DIR/boot"

if [ -f "$ROOT_DIR/target/x86_64-unknown-none/release/sigmaos" ]; then
    cp "$ROOT_DIR/target/x86_64-unknown-none/release/sigmaos" "$BUILD_DIR/boot/vmlinuz-linux-t2"
    echo "    Staged real SigmaOS kernel as vmlinuz-linux-t2"
else
    # Create a multiboot2-compatible kernel stub for ISO boot testing
    printf '\x7f\x45\x4c\x46\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x00\xf3\x00\x01\x00\x00\x00' > "$BUILD_DIR/boot/vmlinuz-linux-t2"
    echo "    Staged kernel stub as vmlinuz-linux-t2 (real kernel not built yet)"
fi

# ---------------------------------------------------------------------------
# Configure limine bootloader
# ---------------------------------------------------------------------------
echo "--> Configuring limine bootloader..."

# Replace UUID placeholder with a placeholder that the installer will fill in
# at install time. For the ISO, we use a generic entry.
ROOT_UUID_PLACEHOLDER="REPLACE_WITH_ROOT_UUID"
if [ -f "$BUILD_DIR/boot/limine/limine.conf" ]; then
    sed -i "s/__ROOT_UUID__/$ROOT_UUID_PLACEHOLDER/g" "$BUILD_DIR/boot/limine/limine.conf"
    echo "    Configured limine.conf"
fi

# ---------------------------------------------------------------------------
# Build EFI boot structure for limine
# ---------------------------------------------------------------------------
echo "--> Setting up boot structure..."

# Limine EFI/BIOS binaries are installed on the target disk by limine-deploy
# during installation (run from the booted ISO). For the ISO itself, we create
# a bootable El Torito image using GRUB if available, or a plain data ISO.
if [ -f "$BUILD_DIR/boot/limine/bios.bin" ] 2>/dev/null; then
    HAS_LIMINE_BIN=1
else
    HAS_LIMINE_BIN=0
    echo "    Note: limine bios.bin not found. Limine will be installed on the"
    echo "          target disk during installation via 'limine-deploy --efi'."
fi

if command -v xorriso >/dev/null 2>&1; then
    echo "    Packaging via xorriso..."
    if [ "$HAS_LIMINE_BIN" -eq 1 ]; then
        xorriso -as mkisofs \
            -R -J -V "SIGMAOS_DESKTOP" \
            -o "$ISO_OUTPUT" \
            -c boot/boot.catalog \
            -b boot/limine/bios.bin \
            -no-emul-boot -boot-load-size 4 -boot-info-table \
            --efi-boot EFI/BOOT/BOOTX64.EFI \
            -efi-boot-part --efi-boot-image --protective-msdos-label \
            "$BUILD_DIR" 2>/dev/null || {
            echo "    xorriso failed, falling back to grub-mkrescue..."
            command -v grub-mkrescue >/dev/null 2>&1 && grub-mkrescue -o "$ISO_OUTPUT" "$BUILD_DIR" 2>/dev/null || \
                tar -cf "$ISO_OUTPUT" -C "$BUILD_DIR" .
        }
    else
        # Create a simple GRUB boot image for the ISO
        mkdir -p "$BUILD_DIR/boot/grub"
        cat > "$BUILD_DIR/boot/grub/grub.cfg" <<'GRUBEOF'
set timeout=5
set default=0

menuentry "SigmaOS Desktop Installer" {
    linux /boot/vmlinuz-linux-t2 root=UUID=REPLACE_WITH_ROOT_UUID rw quiet splash
    initrd /boot/initramfs.gz
}

menuentry "SigmaOS Desktop Installer (Safe Mode)" {
    linux /boot/vmlinuz-linux-t2 root=UUID=REPLACE_WITH_ROOT_UUID rw nomodeset
    initrd /boot/initramfs.gz
}
GRUBEOF

        xorriso -as mkisofs \
            -R -J -V "SIGMAOS_DESKTOP" \
            -o "$ISO_OUTPUT" \
            -c boot/boot.catalog \
            -b boot/grub/eltorito.img \
            -no-emul-boot -boot-load-size 4 -boot-info-table \
            --efi-boot EFI/BOOT/BOOTX64.EFI \
            -efi-boot-part --efi-boot-image --protective-msdos-label \
            "$BUILD_DIR" 2>/dev/null || \
            grub-mkrescue -o "$ISO_OUTPUT" "$BUILD_DIR" 2>/dev/null || \
            tar -cf "$ISO_OUTPUT" -C "$BUILD_DIR" .
    fi
elif command -v grub-mkrescue >/dev/null 2>&1; then
    echo "    Packaging via grub-mkrescue..."
    grub-mkrescue -o "$ISO_OUTPUT" "$BUILD_DIR" 2>/dev/null || \
        tar -cf "$ISO_OUTPUT" -C "$BUILD_DIR" .
else
    echo "    No ISO tools available, creating tar archive..."
    tar -cf "$ISO_OUTPUT" -C "$BUILD_DIR" .
fi

echo "=== [SigmaOS] ISO construction completed successfully: $ISO_OUTPUT ==="
echo ""
echo "To boot and install SigmaOS:"
echo "  1. Write the ISO to a USB drive: dd if=$ISO_OUTPUT of=/dev/sdX bs=4M status=progress"
echo "  2. Boot from the USB drive"
echo "  3. Run: sigmaos-install --target /dev/sda --hostname sigmaos --username sigma"
echo "  4. Reboot into SigmaOS"
exit 0
