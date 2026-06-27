#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-or-later
# sigma_repro_build.sh — Reproducible, hermetic SigmaOS build script
#
# Two builds of the same source on different machines produce IDENTICAL binaries.
# This allows anyone to verify they are running exactly what is in the repository.
#
# Verification approach:
#   1. Build on machine A: sha256sum sigmaos.iso > sigmaos.iso.sha256
#   2. Build on machine B: sha256sum sigmaos.iso > sigmaos.iso.sha256
#   3. diff the two .sha256 files — must be empty (identical digests)
#
# Techniques used:
#   SOURCE_DATE_EPOCH=0      — no embedded timestamps in object files
#   KBUILD_BUILD_USER=sigma  — no username in kernel version string
#   KBUILD_BUILD_HOST=sigma  — no hostname in kernel version string
#   -fdebug-prefix-map       — strip absolute build paths from DWARF
#   -fmacro-prefix-map       — strip __FILE__ absolute paths
#   -ffile-prefix-map        — unified modern replacement (GCC 8+)
#   objcopy --enable-deterministic-archives — strip ar timestamps
#   strip --deterministic    — strip timestamps from ELF symbol tables
#   grub-mkrescue in deterministic mode
#
# Inspired by:
#   • NixOS reproducible builds (https://reproducible.nixos.org/)
#   • Debian reproducible builds (https://reproducible-builds.org/)
#   • Tor Browser deterministic builds
#   • Linux kernel reproducible builds (scripts/setlocalversion)

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${REPO_ROOT}/build"
ISO_OUT="${BUILD_DIR}/sigmaos.iso"

# ── Reproducibility environment ───────────────────────────────────────────────

export SOURCE_DATE_EPOCH=0
export KBUILD_BUILD_USER="sigma"
export KBUILD_BUILD_HOST="sigma-build"
export TZ="UTC"
export LC_ALL="C"
export LANG="C"

# Compiler flags for reproducibility
REPRO_CFLAGS=(
    "-ffile-prefix-map=${REPO_ROOT}=."
    "-fmacro-prefix-map=${REPO_ROOT}=."
    "-Wno-builtin-macro-redefined"
    "-D__DATE__=\"1970-01-01\""
    "-D__TIME__=\"00:00:00\""
)

# ── Toolchain selection ───────────────────────────────────────────────────────

CC="${CC:-x86_64-linux-gnu-gcc}"
CXX="${CXX:-x86_64-linux-gnu-g++}"
LD="${LD:-x86_64-linux-gnu-ld}"
AR="${AR:-x86_64-linux-gnu-ar}"
OBJCOPY="${OBJCOPY:-x86_64-linux-gnu-objcopy}"
STRIP="${STRIP:-x86_64-linux-gnu-strip}"
NASM="${NASM:-nasm}"

echo "[sigma-repro] Toolchain:"
echo "  CC      = $CC ($(${CC} --version | head -1))"
echo "  CXX     = $CXX"
echo "  LD      = $LD"
echo "  SOURCE_DATE_EPOCH = $SOURCE_DATE_EPOCH"

# ── Build ─────────────────────────────────────────────────────────────────────

mkdir -p "${BUILD_DIR}"

echo "[sigma-repro] Configuring build..."
cmake \
    -B "${BUILD_DIR}/cmake" \
    -S "${REPO_ROOT}" \
    -DCMAKE_C_COMPILER="${CC}" \
    -DCMAKE_CXX_COMPILER="${CXX}" \
    -DCMAKE_BUILD_TYPE=Release \
    -DSOURCE_DATE_EPOCH=0 \
    -DSIGMA_REPRO_BUILD=1 \
    -DCMAKE_C_FLAGS="${REPRO_CFLAGS[*]}" \
    -DCMAKE_CXX_FLAGS="${REPRO_CFLAGS[*]}" \
    -G Ninja

echo "[sigma-repro] Building kernel..."
ninja -C "${BUILD_DIR}/cmake" sigmaos.bin

# ── Deterministic archive step ────────────────────────────────────────────────

# Strip all timestamps from .a archives
find "${BUILD_DIR}" -name "*.a" | while read -r lib; do
    "${OBJCOPY}" --enable-deterministic-archives "${lib}" "${lib}.det"
    mv "${lib}.det" "${lib}"
done

# Strip symbol table timestamps
find "${BUILD_DIR}" -name "*.o" -o -name "*.ko" | while read -r obj; do
    "${STRIP}" --strip-debug --deterministic "${obj}" 2>/dev/null || true
done

# ── Build ISO in deterministic mode ──────────────────────────────────────────

echo "[sigma-repro] Building ISO..."
mkdir -p "${BUILD_DIR}/iso/boot/grub"
cp "${BUILD_DIR}/cmake/sigmaos.bin" "${BUILD_DIR}/iso/boot/"

cat > "${BUILD_DIR}/iso/boot/grub/grub.cfg" << 'GRUBEOF'
set default=0
set timeout=3
menuentry 'SigmaOS Zenith' {
    multiboot /boot/sigmaos.bin
    boot
}
GRUBEOF

# grub-mkrescue in deterministic mode (SOURCE_DATE_EPOCH=0 already set)
grub-mkrescue \
    --modules="multiboot iso9660 biosdisk" \
    -o "${ISO_OUT}" \
    "${BUILD_DIR}/iso" \
    -- \
    -volid "SIGMAOS" \
    -volume_date all_file_dates 0

# ── Digest ────────────────────────────────────────────────────────────────────

SHA256=$(sha256sum "${ISO_OUT}" | awk '{print $1}')
echo "[sigma-repro] Build complete: ${ISO_OUT}"
echo "[sigma-repro] SHA-256: ${SHA256}"

# Write digest file
echo "${SHA256}  sigmaos.iso" > "${ISO_OUT}.sha256"
echo "[sigma-repro] Digest written to: ${ISO_OUT}.sha256"

cat << EOF

╔═══════════════════════════════════════════════════════╗
║  SigmaOS Reproducible Build Complete                  ║
╠═══════════════════════════════════════════════════════╣
║  ISO:    ${ISO_OUT}
║  SHA256: ${SHA256}
╠═══════════════════════════════════════════════════════╣
║  To verify: rebuild on another machine and compare    ║
║             sha256sum files.  They must be identical. ║
╚═══════════════════════════════════════════════════════╝

EOF
