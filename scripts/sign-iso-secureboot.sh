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
# archiso stores the EFI system partition image at the ISO root.
echo "Extracting efiboot.img ..."
xorriso -osirrox on -indev "$ISO_IN" -extract /efiboot.img "$WORK/efiboot.img" 2>/dev/null \
    || xorriso -osirrox on -indev "$ISO_IN" -find / -name 'efiboot.img' -exec extract / {} \; 2>/dev/null \
    || { echo "error: could not locate efiboot.img in ISO" >&2; exit 1; }
[[ -f "$WORK/efiboot.img" ]] || { echo "error: efiboot.img not extracted" >&2; exit 1; }

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
xorriso -indev "$ISO_OUT" -outdev "$ISO_OUT" \
    -boot_image any replay \
    -map "$WORK/efiboot.img" /efiboot.img \
    -commit -end 2>&1 | tail -3

echo "Signed ISO written: $ISO_OUT"
echo "Enrol cert for trust: $CRT"
echo "Verify a binary:     sbverify --list <file.efi>"
