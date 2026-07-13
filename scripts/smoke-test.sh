#!/bin/bash
# SigmaOS Smoke Test Script
# This script performs basic smoke tests to verify the repository is in a working state

set -e

echo "🛡️ SigmaOS Smoke Test"
echo "====================="
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

# Test 1: Check if critical files exist
echo "1. Checking critical files..."
test -f README.md && report_test 0 "README.md exists" || report_test 1 "README.md missing"
test -f Cargo.toml && report_test 0 "Cargo.toml exists" || report_test 1 "Cargo.toml missing"
test -f Makefile && report_test 0 "Makefile exists" || report_test 1 "Makefile missing"
test -f docs/doc_audit_backlog.md && report_test 0 "Documentation audit exists" || report_test 1 "Documentation audit missing"
echo ""

# Test 2: Check if Rust toolchain is available
echo "2. Checking Rust toolchain..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    report_test 0 "Rust toolchain available: $RUST_VERSION"
else
    report_test 1 "Rust toolchain not found"
fi
echo ""

# Test 3: Check if Cargo can validate the project
echo "3. Validating Cargo project..."
if command -v cargo &> /dev/null; then
    if cargo check --quiet 2>/dev/null; then
        report_test 0 "Cargo project validates successfully"
    else
        report_test 1 "Cargo project validation failed (may be expected for kernel project)"
    fi
else
    report_test 1 "Cargo not found"
fi
echo ""

# Test 4: Check documentation links
echo "4. Checking documentation links..."
if grep -q "docs/doc_audit_backlog.md" README.md; then
    report_test 0 "README.md links to documentation audit"
else
    report_test 1 "README.md missing documentation audit link"
fi
echo ""

# Test 5: Check if scripts directory exists
echo "5. Checking scripts directory..."
if [ -d "scripts" ]; then
    report_test 0 "scripts directory exists"
else
    report_test 1 "scripts directory missing"
fi
echo ""

# Test 6: Check if .github/workflows exists
echo "6. Checking CI configuration..."
if [ -d ".github/workflows" ]; then
    report_test 0 ".github/workflows directory exists"
else
    report_test 1 ".github/workflows directory missing"
fi
echo ""

# Test 7: Check markdown linting configuration
echo "7. Checking markdown linting..."
if [ -f ".markdownlint.json" ]; then
    report_test 0 ".markdownlint.json exists"
else
    report_test 1 ".markdownlint.json missing"
fi
echo ""

# Summary
echo "====================="
echo "Smoke Test Summary"
echo "====================="
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Failed: $TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All smoke tests passed!${NC}"
    exit 0
else
    echo -e "${YELLOW}Some smoke tests failed. This may be expected during development.${NC}"
    exit 1
fi
