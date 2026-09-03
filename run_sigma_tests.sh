#!/usr/bin/env bash
set -e

echo "=== Running SigmaOS Native C++ Test Runner ==="
mkdir -p build
(cd tests && make && ./sigma_test_runner)

echo "=== Running Python Test Suite ==="
python3 -m pytest tests/

echo "=== Quality Check Gate ==="
./scripts/sigma_quality_check.sh
