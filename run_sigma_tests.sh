#!/usr/bin/env bash
set -e
echo "Building and running Sovereign Atomic Tests..."
g++ -std=c++11 -I. -I./include -I./klib/include -o test_runner tests/sigma_test_runner.cpp
./test_runner
echo "[✓] All tests passed successfully."
exit 0
