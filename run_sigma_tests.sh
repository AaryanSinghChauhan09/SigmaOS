#!/usr/bin/env bash
set -eo pipefail

echo "=== Executing SigmaOS System & Regression Test Suite ==="

if [ -f "scripts/app_regression_test.sh" ]; then
    bash scripts/app_regression_test.sh
fi

if [ -f "scripts/run_static_analysis.sh" ]; then
    bash scripts/run_static_analysis.sh
fi

echo "=== SigmaOS System & Regression Test Suite Passed ==="
