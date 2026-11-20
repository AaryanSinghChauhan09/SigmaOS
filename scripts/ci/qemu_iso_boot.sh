#!/usr/bin/env bash
# qemu_iso_boot.sh — real boot smoke test for a SigmaOS archiso image.
#
# Extracts the kernel + initramfs from the ISO and boots them directly in
# QEMU (TCG, no KVM required) with the archiso runtime on the attached
# cdrom. Watches the serial console for proof that the live environment
# reached multi-user (login prompt / systemd target / SigmaOS banner).
#
# This catches the regression class that shipped a broken ISO: missing
# archiso hooks, missing storage modules, bad bootloader paths.
#
# Usage: qemu_iso_boot.sh <path-to.iso> [work-dir] [timeout-sec]
# Requires: qemu-system-x86_64, xorriso (or bsdtar), standard coreutils.
set -euo pipefail

ISO="${1:?usage: qemu_iso_boot.sh <iso> [workdir] [timeout]}"
WORK="${2:-$(mktemp -d /tmp/sigmaos-qemu-XXXXXX)}"
TIMEOUT="${3:-900}"
LOG="$WORK/serial.log"
KERNEL_REL="sigmaos/boot/x86_64/vmlinuz-linux"
INITRD_REL="sigmaos/boot/x86_64/initramfs-linux.img"

[[ -f "$ISO" ]] || { echo "ISO not found: $ISO" >&2; exit 1; }
mkdir -p "$WORK"
: > "$LOG"

echo "[qemu-boot] extracting kernel/initramfs from $ISO"
# Extract the two boot artifacts. Prefer xorriso (Arch/CI), then bsdtar,
# then isoinfo/7z which are commonly preinstalled.
extract_file() {
    local rel="$1" out="$2"
    if command -v xorriso >/dev/null 2>&1; then
        xorriso -osirrox on -indev "$ISO" -extract "/$rel" "$out" >/dev/null
    elif command -v bsdtar >/dev/null 2>&1; then
        bsdtar -xOf "$ISO" "/$rel" > "$out"
    elif command -v isoinfo >/dev/null 2>&1; then
        isoinfo -i "$ISO" -R -x "/$rel" > "$out" 2>/dev/null || \
        isoinfo -i "$ISO" -x "/$rel" > "$out"
    elif command -v 7z >/dev/null 2>&1; then
        7z e -so "$ISO" "$rel" > "$out"
    else
        echo "qemu-boot: no ISO extraction tool (xorriso/bsdtar/isoinfo/7z)" >&2
        return 1
    fi
}
extract_file "$KERNEL_REL" "$WORK/vmlinuz-linux"
extract_file "$INITRD_REL" "$WORK/initramfs-linux.img"
[[ -s "$WORK/vmlinuz-linux" && -s "$WORK/initramfs-linux.img" ]] || {
    echo "qemu-boot: failed to extract kernel/initramfs" >&2; exit 1; }

# console=ttyS0 puts kernel + systemd logs on the serial line so CI can
# observe boot progress; harmless on bare metal (writes are dropped).
# Use KVM when available (local dev), TCG otherwise (GitHub runners).
QEMU_ACCEL=()
if [[ -w /dev/kvm ]]; then
    QEMU_ACCEL=(-enable-kvm -cpu host)
fi
QEMU_CMD=(
    qemu-system-x86_64
    "${QEMU_ACCEL[@]}"
    -m 3072 -smp 2
    -display none -serial "file:$LOG" -monitor none
    -cdrom "$ISO"
    -kernel "$WORK/vmlinuz-linux"
    -initrd "$WORK/initramfs-linux.img"
    -append "archisobasedir=sigmaos console=ttyS0"
    -no-reboot
)

echo "[qemu-boot] starting QEMU (timeout ${TIMEOUT}s), log: $LOG"
"${QEMU_CMD[@]}" &
QPID=$!

# Markers proving the live environment reached userspace. Ordered roughly
# by boot depth; any one of them means the ISO is bootable.
MARKERS=(
    'login:'
    'Reached target'
    'Welcome to SigmaOS'
    'SigmaOS'
)

FOUND=""
ELAPSED=0
while (( ELAPSED < TIMEOUT )); do
    if ! kill -0 "$QPID" 2>/dev/null; then
        echo "[qemu-boot] QEMU exited early" >&2
        break
    fi
    for m in "${MARKERS[@]}"; do
        if grep -qF "$m" "$LOG" 2>/dev/null; then
            FOUND="$m"
            break 2
        fi
    done
    # Panic / emergency shell are definitive failures — bail early.
    if grep -qE 'Kernel panic|Failed to mount|Emergency shell' "$LOG" 2>/dev/null; then
        echo "[qemu-boot] boot failure detected in log" >&2
        break
    fi
    sleep 5
    ELAPSED=$((ELAPSED + 5))
done

kill "$QPID" 2>/dev/null || true
wait "$QPID" 2>/dev/null || true

if [[ -n "$FOUND" ]]; then
    echo "[qemu-boot] PASS — live environment reached userspace (marker: '$FOUND')"
    exit 0
fi

echo "[qemu-boot] FAIL — no boot marker within ${TIMEOUT}s" >&2
echo "--- last 60 lines of serial log ---" >&2
tail -60 "$LOG" >&2 || true
exit 1
