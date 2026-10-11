#!/usr/bin/env bash
set -euo pipefail

echo "========================================="
echo "SigmaOS QEMU Boot-to-Shell Smoke Test"
echo "========================================="

BUILD_DIR="build"
LOG_FILE="${BUILD_DIR}/qemu_boot_smoke.log"
mkdir -p "${BUILD_DIR}"

echo "[INFO] Running boot-to-shell validation harness..."
python3 tests/boot/boot_to_shell_test.py 2>&1 | tee "${LOG_FILE}"

echo "[INFO] Boot smoke test finished successfully."
