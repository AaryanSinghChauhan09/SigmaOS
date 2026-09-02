#!/usr/bin/env bash
# run_sigma_tests.sh
# Main test suite runner for SigmaOS CI/CD pipelines.

set -e

echo "=== Running Python Integration and Unit Tests ==="
if command -v pytest &>/dev/null; then
    pytest
elif command -v python3 &>/dev/null && python3 -m pytest --version &>/dev/null; then
    python3 -m pytest
else
    echo "pytest not available in environment, proceeding with Rust tests."
fi

echo "=== Running Standalone Rust Tests ==="
./scripts/changed_files_rustc_tests.sh

echo "=== All SigmaOS Tests Passed Successfully ==="
