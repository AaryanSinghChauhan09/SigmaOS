#!/usr/bin/env bash
# Reassemble the split SigmaOS ISO from GitHub release parts.
#
# Usage:
#   ./scripts/reassemble-iso.sh <directory-containing-parts>
#   ./scripts/reassemble-iso.sh /path/to/sigmaos-desktop-...-x86_64.iso
#
# The ISO on GitHub is split into *.iso.NN.part files (GitHub's 2 GiB asset
# limit). This script finds them, concatenates in numeric order, and verifies
# the SHA-256 checksum if the .sha256 file is present.
set -euo pipefail

ARG="${1:?Usage: $0 <iso-basename-or-directory>}"

if [ -d "$ARG" ]; then
  DIR="$ARG"
  FIRST="$(find "$DIR" -maxdepth 1 -name '*.iso.00.part' -print -quit)"
  [ -n "$FIRST" ] || { echo "No *.iso.00.part found in $DIR"; exit 1; }
  ISO="${FIRST%.00.part}"
else
  ISO="$ARG"
  DIR="$(dirname "$ISO")"
fi

BASE="$(basename "$ISO")"
mapfile -t FILES < <(find "$DIR" -maxdepth 1 -name "${BASE}.*.part" | sort -V)
[ "${#FILES[@]}" -gt 0 ] || { echo "No part files matching ${BASE}.*.part in $DIR"; exit 1; }

echo "Reassembling $ISO from ${#FILES[@]} parts..."
cat "${FILES[@]}" > "$ISO"

SHA_FILE="$DIR/${BASE}.sha256"
if [ -f "$SHA_FILE" ]; then
  echo "Verifying SHA-256..."
  (cd "$DIR" && sha256sum -c "${BASE}.sha256") || { echo "Checksum mismatch!"; exit 1; }
else
  echo "No ${BASE}.sha256 found - skipping verification."
fi

echo "Done: $ISO"
echo "Flash: sudo dd if=$ISO of=/dev/sdX bs=4M status=progress oflag=sync"
