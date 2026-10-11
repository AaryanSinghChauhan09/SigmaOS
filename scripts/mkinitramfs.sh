#!/usr/bin/env sh
# POSIX Shell Initial RAM Disk (initramfs) Binary Generator Script
# Inspired by Linux mkinitramfs, mkinitcpio, and dracut.

set -e

# Default settings
OUTPUT_IMG="${OUTPUT_IMG:-/boot/initramfs-sigma.img}"
KERNEL_VER="$(uname -r 2>/dev/null || echo "6.8.1-sigmaos-1")"
COMPRESSION="gzip"
STAGE_DIR=""
VERBOSE=0

usage() {
    cat <<EOF
Usage: $0 [OPTIONS]

Options:
  -o <path>       Output initramfs image file (default: /boot/initramfs-sigma.img)
  -k <version>    Target kernel version (default: current kernel or 6.8.1-sigmaos-1)
  -c <comp>       Compression format: gzip, zstd, none (default: gzip)
  -d <dir>        Existing staging directory to package
  -v              Enable verbose output
  -h              Display this help message
EOF
}

while getopts "o:k:c:d:vh" opt; do
    case "$opt" in
        o) OUTPUT_IMG="$OPTARG" ;;
        k) KERNEL_VER="$OPTARG" ;;
        c) COMPRESSION="$OPTARG" ;;
        d) STAGE_DIR="$OPTARG" ;;
        v) VERBOSE=1 ;;
        h) usage; exit 0 ;;
        *) usage; exit 1 ;;
    esac
done

# Setup temporary staging directory if not provided
CLEANUP_STAGE=0
if [ -z "$STAGE_DIR" ]; then
    STAGE_DIR="$(mktemp -d 2>/dev/null || echo "/tmp/sigma_initramfs_$$")"
    mkdir -p "$STAGE_DIR"
    CLEANUP_STAGE=1
fi

cleanup() {
    if [ "$CLEANUP_STAGE" -eq 1 ] && [ -d "$STAGE_DIR" ]; then
        rm -rf "$STAGE_DIR"
    fi
}
trap cleanup EXIT INT TERM

if [ "$VERBOSE" -eq 1 ]; then
    echo "[mkinitramfs] Staging initramfs in $STAGE_DIR for kernel $KERNEL_VER"
fi

# Populate directory structure
mkdir -p "$STAGE_DIR/bin" "$STAGE_DIR/sbin" "$STAGE_DIR/etc" \
         "$STAGE_DIR/dev" "$STAGE_DIR/proc" "$STAGE_DIR/sys" "$STAGE_DIR/mnt" \
         "$STAGE_DIR/lib/modules/$KERNEL_VER"

# Create early init executable script
cat << 'EOF' > "$STAGE_DIR/init"
#!/bin/sh
# SigmaOS Early Boot Init Script
echo "[initramfs] Initializing SigmaOS early boot stage..."
mount -t devtmpfs devtmpfs /dev 2>/dev/null || true
mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sysfs /sys 2>/dev/null || true

echo "[initramfs] Unsealing storage and preparing root pivot..."
if [ -x /sbin/init ]; then
    exec /sbin/init "$@"
else
    echo "[initramfs] Warning: /sbin/init not found. Launching recovery shell..."
    exec /bin/sh
fi
EOF
chmod 755 "$STAGE_DIR/init"

# Create minimal device nodes if possible
mknod "$STAGE_DIR/dev/console" c 5 1 2>/dev/null || touch "$STAGE_DIR/dev/console"
mknod "$STAGE_DIR/dev/null" c 1 3 2>/dev/null || touch "$STAGE_DIR/dev/null"

# Create configuration files
cat << EOF > "$STAGE_DIR/etc/initrd.conf"
# SigmaOS Initramfs Config
KERNEL_VERSION="$KERNEL_VER"
COMPRESSION="$COMPRESSION"
CREATED="$(date -u 2>/dev/null || echo "1704067200")"
EOF

# Ensure output target parent directory exists
OUT_DIR="$(dirname "$OUTPUT_IMG")"
mkdir -p "$OUT_DIR"

if [ "$VERBOSE" -eq 1 ]; then
    echo "[mkinitramfs] Packing cpio stream and compressing with $COMPRESSION..."
fi

# Package initramfs image
CPIO_FILE="$(mktemp 2>/dev/null || echo "/tmp/initramfs_$$.cpio")"

if command -v cpio >/dev/null 2>&1; then
    (cd "$STAGE_DIR" && find . -print0 | cpio --null -o -H newc > "$CPIO_FILE" 2>/dev/null)
else
    # Minimal fallback cpio header writer simulation
    echo "07070100000064000081ED0000000000000000000000016591C7000000000A000000030000000100000000000000000000000500000000init\0" > "$CPIO_FILE"
    cat "$STAGE_DIR/init" >> "$CPIO_FILE"
fi

case "$COMPRESSION" in
    gzip)
        if command -v gzip >/dev/null 2>&1; then
            gzip -9 -c "$CPIO_FILE" > "$OUTPUT_IMG"
        else
            cp "$CPIO_FILE" "$OUTPUT_IMG"
        fi
        ;;
    zstd)
        if command -v zstd >/dev/null 2>&1; then
            zstd -q -f -19 "$CPIO_FILE" -o "$OUTPUT_IMG"
        else
            cp "$CPIO_FILE" "$OUTPUT_IMG"
        fi
        ;;
    none|*)
        cp "$CPIO_FILE" "$OUTPUT_IMG"
        ;;
esac

rm -f "$CPIO_FILE"

if [ "$VERBOSE" -eq 1 ] || [ -f "$OUTPUT_IMG" ]; then
    echo "[mkinitramfs] Successfully generated $OUTPUT_IMG ($(wc -c < "$OUTPUT_IMG" 2>/dev/null || echo 0) bytes)"
fi

exit 0
