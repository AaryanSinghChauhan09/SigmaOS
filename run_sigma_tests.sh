#!/usr/bin/env bash
set -e

echo "=== Running SigmaOS Test Suite ==="

mkdir -p build

echo "[1/4] Testing Package Caching Engine..."
rustc --test --edition 2021 src/package/cache.rs -o build/test_cache
./build/test_cache

echo "[2/4] Testing Universal Package Adapter Engine..."
rustc --test --edition 2021 tests/test_universal_adapter.rs -o build/test_universal_adapter
./build/test_universal_adapter

echo "[3/4] Testing Unimplemented Features Suite..."
rustc --test --edition 2021 src/unimplemented_features.rs -o build/test_unimplemented_features
./build/test_unimplemented_features

echo "[4/4] Testing Unimplemented Tools Suite..."
rustc --test --edition 2021 src/unimplemented_tools.rs -o build/test_unimplemented_tools
./build/test_unimplemented_tools

echo "=== All SigmaOS Tests Passed Successfully ==="
