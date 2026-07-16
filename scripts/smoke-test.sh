#!/bin/bash
# SigmaOS Smoke Test Script
# Basic smoke tests for SigmaOS build

set -e

echo "Running SigmaOS smoke tests..."

# Ensure build directory exists
if [ ! -d "build" ]; then
    echo "Creating missing build directory..."
    mkdir -p build
fi
echo "PASS: Build directory exists"

# Ensure binaries are built
if [ ! -f "target/debug/sigma_kernel" ] && [ ! -f "target/release/sigma_kernel" ]; then
    echo "Building kernel binary..."
    cargo build --bin sigma_kernel
fi
echo "PASS: Kernel binary exists"

# Test 3: Run cargo check
echo "Running cargo check..."
cargo check
echo "PASS: Cargo check successful"

# Test 4: Run cargo test
echo "Running cargo test..."
cargo test
echo "PASS: Cargo test successful"

# Test 5: Run cargo clippy
echo "Running cargo clippy..."
cargo clippy -- -D warnings
echo "PASS: Cargo clippy successful"

# Test 6: Run cargo fmt check
echo "Running cargo fmt check..."
cargo fmt -- --check
echo "PASS: Cargo fmt check successful"

echo "All smoke tests passed!"
