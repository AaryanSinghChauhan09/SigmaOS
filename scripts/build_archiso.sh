#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# build_archiso.sh: Builds the reproducible SigmaOS Arch Linux ISO
set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
REPO_ROOT=$(cd "${SCRIPT_DIR}/.." && pwd)
PROFILE_DIR="${REPO_ROOT}/iso/archiso"
WORK_DIR="/tmp/sigmaos-archiso-work"
OUT_DIR="${REPO_ROOT}/out"

echo "=== Building SigmaOS Arch Linux ISO Image ==="
echo "Profile: ${PROFILE_DIR}"
echo "Output:  ${OUT_DIR}"

if ! command -v mkarchiso &>/dev/null; then
    echo "[!] mkarchiso not found in PATH; running dry-run profile validation..."
    test -f "${PROFILE_DIR}/profiledef.sh"
    test -f "${PROFILE_DIR}/packages.x86_64"
    test -f "${PROFILE_DIR}/pacman.conf"
    test -d "${PROFILE_DIR}/grub"
    test -d "${PROFILE_DIR}/syslinux"
    test -d "${PROFILE_DIR}/efiboot"
    echo "[OK] Profile validation successful."
    exit 0
fi

mkdir -p "${OUT_DIR}"
mkarchiso -v -w "${WORK_DIR}" -o "${OUT_DIR}" "${PROFILE_DIR}"
echo "=== ISO built successfully in ${OUT_DIR} ==="
