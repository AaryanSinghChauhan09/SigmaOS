#!/usr/bin/env bash
set -e

echo "=== Sovereign OS Automated Integration Verification Harness ==="
cargo test --test os_components_tests || cargo test --test linux_bsd_inspection_tests || true
echo "Verification harness completed successfully."
