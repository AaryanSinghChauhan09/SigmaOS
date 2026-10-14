#!/usr/bin/env bash
set -euo pipefail

echo "==> Preparing SigmaOS Release Candidate Build..."

echo "[1/4] Running standard test runner..."
./run_sigma_tests.sh

echo "[2/4] Executing QEMU boot smoke test..."
./scripts/ci/qemu_boot_smoke.sh

echo "[3/4] Running installer dry-run verification..."
rustc --test installer/tests/dry_run.rs --edition=2021 -o build/test_installer_dry_run
./build/test_installer_dry_run

echo "[4/4] Release preparation complete. Verify checklist in docs/release/RELEASE_CHECKLIST.md."
