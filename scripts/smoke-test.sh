#!/bin/bash
set -e

echo "Running SigmaOS Smoke Tests..."
echo "1. Checking kernel build..."
cd kernel && cargo check --target x86_64-unknown-none && cd ..

echo "2. Checking sigpkg build..."
cd userland/sigpkg && cargo check && cd ../..

echo "3. Running sigpkg unit tests..."
cd userland/sigpkg && cargo test && cd ../..

echo "Smoke tests passed!"
