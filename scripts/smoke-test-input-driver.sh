#!/bin/bash
# Smoke test for Sigma Input Driver prototype

set -e

echo "🛡️ Sigma Input Driver Smoke Test"
echo "====================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Function to report test result
report_test() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}✗${NC} $2"
        ((TESTS_FAILED++))
    fi
}

# Change to prototype directory
cd "$(dirname "$0")/.."
PROTOTYPE_DIR="shards/essential/input"

echo "1. Checking prototype directory..."
test -d "$PROTOTYPE_DIR" && report_test 0 "Prototype directory exists" || report_test 1 "Prototype directory missing"
echo ""

echo "2. Checking build.zig..."
test -f "$PROTOTYPE_DIR/build.zig" && report_test 0 "build.zig exists" || report_test 1 "build.zig missing"
echo ""

echo "3. Checking source files..."
test -f "$PROTOTYPE_DIR/src/lib.zig" && report_test 0 "lib.zig exists" || report_test 1 "lib.zig missing"
test -f "$PROTOTYPE_DIR/src/main.zig" && report_test 0 "main.zig exists" || report_test 1 "main.zig missing"
echo ""

echo "4. Building prototype..."
cd "$PROTOTYPE_DIR"
if zig build 2>/dev/null; then
    report_test 0 "Prototype builds successfully"
else
    report_test 1 "Prototype build failed"
    echo ""
    echo "Build output:"
    zig build
fi
echo ""

echo "5. Running unit tests..."
if zig build test 2>/dev/null; then
    report_test 0 "Unit tests pass"
else
    report_test 1 "Unit tests failed"
    echo ""
    echo "Test output:"
    zig build test
fi
echo ""

echo "6. Testing CLI device detection..."
if zig build run -- detect 2>/dev/null | grep -q "Input device detection complete"; then
    report_test 0 "CLI detect command works"
else
    report_test 1 "CLI detect command failed"
fi
echo ""

echo "7. Testing CLI list devices..."
if zig build run -- list 2>/dev/null | grep -q "Input Devices"; then
    report_test 0 "CLI list command works"
else
    report_test 1 "CLI list command failed"
fi
echo ""

# Summary
echo "====================================="
echo "Smoke Test Summary"
echo "====================================="
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Failed: $TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All smoke tests passed!${NC}"
    exit 0
else
    echo -e "${YELLOW}Some smoke tests failed.${NC}"
    exit 1
fi
