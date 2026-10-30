#!/usr/bin/env bash
set -euo pipefail

echo "========================================="
echo "SigmaOS QEMU Boot-to-Shell Smoke Test"
echo "========================================="

BUILD_DIR="build"
LOG_FILE="${BUILD_DIR}/qemu_boot_smoke.log"
mkdir -p "${BUILD_DIR}"

echo "[INFO] Building the Multiboot kernel..."
make kernel

echo "[INFO] Building a real bootable ISO..."
bash scripts/build_stub_iso.sh build/sigmaos.iso

echo "[INFO] Launching QEMU and validating serial markers..."
python3 tests/boot/boot_to_shell_test.py 2>&1 | tee "${LOG_FILE}"

echo "[INFO] Boot smoke test finished successfully."
