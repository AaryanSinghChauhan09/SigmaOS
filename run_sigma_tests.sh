#!/usr/bin/env bash
# run_sigma_tests.sh
# Main test suite runner for SigmaOS CI/CD pipelines.

set -e

echo "=== Running Python Integration and Unit Tests ==="
pytest

echo "=== Running Standalone Rust Tests ==="
./scripts/changed_files_rustc_tests.sh

echo "=== All SigmaOS Tests Passed Successfully ==="
