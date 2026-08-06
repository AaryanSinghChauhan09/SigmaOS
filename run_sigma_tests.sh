#!/usr/bin/env bash
set -e
echo "Building and running Sovereign Atomic Tests..."
g++ -std=c++11 -I. -Iinclude -DTEST_RUNNER -o test_runner tests/sigma_test_runner.cpp kernel/containers/sigma_oci_runtime.cpp kernel/tests/sigma_hw_test.cpp
./test_runner
set -e
echo "Building and running Sovereign Atomic Tests..."
g++ -std=c++11 -I. -o test_runner tests/sigma_test_runner.cpp
./test_runner
echo "[✓] All tests passed successfully."
exit 0
