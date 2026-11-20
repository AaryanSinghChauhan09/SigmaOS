#!/usr/bin/env bash
# sign-iso-secureboot.sh — produce a Secure-Boot-bootable variant of a
# built SigmaOS ISO by signing the EFI binaries inside its efiboot.img.
#
# Pipeline: extract efiboot.img -> sign every *.efi (systemd-boot loader,
# kernel EFI stub, any shipped .efi) with sbsign -> rebuild -> re-insert
# into the ISO with xorriso, preserving the El Torito EFI boot image.
#
# Usage:
#   scripts/sign-iso-secureboot.sh <input.iso> [output.iso] [KEY_DIR]
#
#   KEY_DIR  dir with db.{key,crt}; auto-generated (throwaway) if absent.
#
# REQUIREMENTS: xorriso, mtools (mcopy/mdir), sbsign, openssl.
# Run on Arch or in the build container. The result still needs the
# signing cert enrolled once per machine (MOK/db) — see sign-secureboot.sh.
#
# NOTE: This signs the loader + kernel EFI stub. Full Secure Boot also
# needs the kernel's EFI stub signature to be honoured by the boot chain;
# verify on real hardware/VM with SB enabled before publishing.
set -euo pipefail

ISO_IN="${1:?usage: sign-iso-secureboot.sh <input.iso> [output.iso] [KEY_DIR]}"
ISO_OUT="${2:-${ISO_IN%.iso}-secureboot.iso}"
KEYDIR="${3:-./secureboot-keys}"
WORK="$(mktemp -d)"; trap 'rm -rf "$WORK"' EXIT

for t in xorriso mcopy mdir sbsign openssl; do
    command -v "$t" >/dev/null || { echo "error: need $t" >&2; exit 1; }
done
[[ -f "$ISO_IN" ]] || { echo "error: $ISO_IN not found" >&2; exit 1; }

KEY="$KEYDIR/db.key"; CRT="$KEYDIR/db.crt"
if [[ ! -f "$KEY" || ! -f "$CRT" ]]; then
    mkdir -p "$KEYDIR"
    echo "Generating throwaway signing key in $KEYDIR ..."
    openssl req -new -x509 -newkey rsa:2048 -keyout "$KEY" -out "$CRT" \
        -days 3650 -nodes -subj "/CN=SigmaOS Secure Boot Key/" 2>/dev/null
    chmod 600 "$KEY"
fi

# --- extract efiboot.img ---------------------------------------------------
# archiso does NOT store efiboot.img as a regular ISO9660 file — it is the
# El Torito EFI boot image and/or an appended GPT partition (isohybrid).
# Try, in order: (a) a real file in the ISO, (b) an appended FAT partition
# carved out with dd, (c) fail loudly.
echo "Extracting efiboot.img ..."
ESP_OFFSET=""; ESP_SECTORS=""

is_fat_esp() { # $1 = candidate image; true if it looks like the ESP
    [[ -f "$1" ]] || return 1
    # mdir lists the root; the ESP always has an EFI/ directory.
    mdir -i "$1" ::/EFI >/dev/null 2>&1
}

xorriso -osirrox on -indev "$ISO_IN" -extract /efiboot.img "$WORK/efiboot.img" 2>/dev/null || true
if ! is_fat_esp "$WORK/efiboot.img"; then
    rm -f "$WORK/efiboot.img"
    # Fallback: scan appended partitions for a FAT ESP and carve it out.
    echo "efiboot.img not a plain file — scanning appended partitions ..."
    local_start=0
    # fdisk -l reports appended partitions with start sector + size.
    while read -r _ part_start part_sectors _; do
        [[ "$part_start" =~ ^[0-9]+$ && "$part_sectors" =~ ^[0-9]+$ ]] || continue
        [[ "$part_start" -gt 0 ]] || continue
        dd if="$ISO_IN" of="$WORK/esp_candidate.img" bs=512 \
            skip="$part_start" count="$part_sectors" status=none 2>/dev/null || continue
        if is_fat_esp "$WORK/esp_candidate.img"; then
            mv "$WORK/esp_candidate.img" "$WORK/efiboot.img"
            ESP_OFFSET="$part_start"; ESP_SECTORS="$part_sectors"
            echo "  carved ESP from appended partition @sector $part_start"
            break
        fi
    done < <(fdisk -l "$ISO_IN" 2>/dev/null | awk '/\.img|^\/|^Disk/{next} $2 ~ /^[0-9]+$/ && $4 ~ /^[0-9]+$/ {print $2, $4}')
fi
[[ -f "$WORK/efiboot.img" ]] && is_fat_esp "$WORK/efiboot.img" \
    || { echo "error: could not locate a valid EFI system partition in ISO" >&2; exit 1; }
echo "efiboot.img ready ($(du -h "$WORK/efiboot.img" | cut -f1))."

# --- extract .efi files via mtools (no root/mount needed) -------------------
mkdir -p "$WORK/efi" "$WORK/efisrc"
echo "Extracting .efi payloads ..."
# Recursively copy all files out of the FAT image.
( cd "$WORK/efisrc" && mcopy -s -i "$WORK/efiboot.img" :: . ) 2>/dev/null || true
# Sign .efi bootloaders AND the kernel EFI stub (vmlinuz-*) — Secure Boot
# requires every EFI application in the chain to carry a trusted signature.
mapfile -d '' EFIS < <(find "$WORK/efisrc" -type f \( -iname '*.efi' -o -iname 'vmlinuz*' \) -print0)
if [[ ${#EFIS[@]} -eq 0 ]]; then
    echo "error: no signable EFI binaries found inside efiboot.img" >&2; exit 1
fi

# --- sign ------------------------------------------------------------------
echo "Signing ${#EFIS[@]} EFI binaries ..."
for efi in "${EFIS[@]}"; do
    rel="${efi#$WORK/efisrc/}"
    sbsign --key "$KEY" --cert "$CRT" "$efi" >/dev/null 2>&1 \
        && echo "  signed: $rel" || echo "  FAILED: $rel" >&2
done

# --- copy signed files back into efiboot.img --------------------------------
echo "Writing signatures back into efiboot.img ..."
( cd "$WORK/efisrc" && mcopy -s -o -i "$WORK/efiboot.img" . :: ) 2>/dev/null || true

# --- rebuild ISO with the signed efiboot.img --------------------------------
echo "Rebuilding ISO -> $ISO_OUT ..."
cp "$ISO_IN" "$ISO_OUT"
if [[ -n "$ESP_OFFSET" ]]; then
    # ESP was an appended partition: write the signed image back to the same
    # sector offset. The El Torito/partition table is untouched, so the boot
    # chain still points at the (now signed) ESP.
    dd if="$WORK/efiboot.img" of="$ISO_OUT" bs=512 \
        seek="$ESP_OFFSET" conv=notrunc status=none
    echo "  wrote signed ESP back @sector $ESP_OFFSET"
else
    # efiboot.img was a regular ISO file: replace it in-place.
    xorriso -indev "$ISO_OUT" -outdev "$ISO_OUT" \
        -boot_image any replay \
        -map "$WORK/efiboot.img" /efiboot.img \
        -commit -end 2>&1 | tail -3
fi

echo "Signed ISO written: $ISO_OUT"
echo "Enrol cert for trust: $CRT"
echo "Verify a binary:     sbverify --list <file.efi>"
