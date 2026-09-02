#!/usr/bin/env bash
set -e

echo "Running SigmaOS Sovereign Parity & Component Inspection Tests..."
if [ -f "./algorithm_and_components_inspection_tests" ]; then
    ./algorithm_and_components_inspection_tests
else
    rustc --edition 2021 --test tests/linux_bsd_inspection_tests.rs -o build/linux_bsd_tests && ./build/linux_bsd_tests
fi
