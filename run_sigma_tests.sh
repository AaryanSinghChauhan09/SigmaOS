#!/usr/bin/env bash
set -e

# SigmaOS Test Harness Runner
echo "Running SigmaOS test harness..."

if [ -f ./algorithm_and_components_inspection_tests ]; then
    ./algorithm_and_components_inspection_tests
else
    echo "Running cargo test..."
    cargo test --lib
fi

echo "All tests passed successfully."
