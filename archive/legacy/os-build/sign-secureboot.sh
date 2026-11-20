#!/usr/bin/env bash
# sign-secureboot.sh — sign EFI binaries so they boot under Secure Boot.
#
# Signs every *.efi under a target directory (a mounted ESP, an extracted
# efiboot.img tree, or /boot) with sbsign using a PEM key+cert, then prints
# the certificate users must enrol (MOK / firmware db) for the signatures
# to be trusted.
#
# Usage:
#   scripts/sign-secureboot.sh [TARGET_DIR] [KEY_DIR]
#
#   TARGET_DIR  directory containing the .efi files to sign
#               (default: /boot). Mount the ESP or extract efiboot.img first.
#   KEY_DIR     directory holding db.{key,crt} (default: ./secureboot-keys,
#               auto-generated on first run — THROWAWAY test key only).
#
# NOTE: A self-generated key is for TESTING. For a release ISO, sign with
# the project's release key and distribute the .crt for one-time MOK/db
# enrolment. The live ISO is unsigned until this is run against its
# efiboot.img during the build.
set -euo pipefail

TARGET="${1:-/boot}"
KEYDIR="${2:-./secureboot-keys}"
KEY="$KEYDIR/db.key"
CRT="$KEYDIR/db.crt"

[[ -d "$TARGET" ]] || { echo "error: target dir '$TARGET' not found" >&2; exit 1; }
command -v sbsign >/dev/null || { echo "error: sbsign not installed (pacman -S sbctl)" >&2; exit 1; }

# --- keys -----------------------------------------------------------------
if [[ ! -f "$KEY" || ! -f "$CRT" ]]; then
    mkdir -p "$KEYDIR"
    echo "Generating throwaway signing key in $KEYDIR (use a release key for production)..."
    openssl req -new -x509 -newkey rsa:2048 -keyout "$KEY" -out "$CRT" \
        -days 3650 -nodes -subj "/CN=SigmaOS Secure Boot Test Key/" 2>/dev/null
    chmod 600 "$KEY"
fi

# --- sign -----------------------------------------------------------------
mapfile -d '' EFIS < <(find "$TARGET" -type f -iname '*.efi' -print0)
if [[ ${#EFIS[@]} -eq 0 ]]; then
    echo "No *.efi files under $TARGET — mount the ESP or extract efiboot.img first." >&2
    exit 1
fi

echo "Signing ${#EFIS[@]} EFI binary(ies) under $TARGET ..."
for efi in "${EFIS[@]}"; do
    sbsign --key "$KEY" --cert "$CRT" "$efi" >/dev/null 2>&1 \
        && echo "  signed: $efi" \
        || echo "  FAILED: $efi" >&2
done

# --- enrolment instructions ------------------------------------------------
cat <<EOF

Done. To trust these signatures under Secure Boot, enrol this certificate
ONCE per machine (MOK or firmware db):

  Certificate: $CRT

  sbctl path (installed system):
    sudo sbctl create-keys            # or import $CRT as a MOK
    sudo sbctl enroll-keys
    # reboot -> MOK Manager -> Enroll MOK -> Continue -> Yes

  Manual firmware path:
    Copy $CRT to removable media and use the firmware's "Enroll Key" /
    "Install Custom Certificate" option, or append it to the PK/KEK/db.

Verify a signature:  sbverify --list <file.efi>
EOF
