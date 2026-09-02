#!/usr/bin/env bash
set -e

echo "=== Running SigmaOS Test Suite ==="

mkdir -p build

echo "[1/3] Testing Package Caching Engine..."
rustc --test --edition 2021 src/package/cache.rs -o build/test_cache
./build/test_cache

echo "[2/3] Testing Unimplemented Features Suite..."
rustc --test --edition 2021 src/unimplemented_features.rs -o build/test_unimplemented_features
./build/test_unimplemented_features

echo "[3/3] Testing Unimplemented Tools Suite..."
rustc --test --edition 2021 src/unimplemented_tools.rs -o build/test_unimplemented_tools
./build/test_unimplemented_tools

echo "=== All SigmaOS Tests Passed Successfully ==="
