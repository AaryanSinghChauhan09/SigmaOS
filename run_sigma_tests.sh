#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# SigmaOS Test Runner Script
set -e

echo "=== Running SigmaOS Test Suite ==="
if command -v pytest &>/dev/null; then
  pytest tests/
elif command -v python3 &>/dev/null; then
  python3 -m pytest tests/
fi

echo "=== All SigmaOS Tests Completed Successfully ==="
