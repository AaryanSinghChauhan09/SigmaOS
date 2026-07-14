#!/bin/bash
# Smoke test for SovereignADRTracker prototype

set -e

echo "🛡️ SovereignADRTracker Smoke Test"
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
PROTOTYPE_DIR="sovereign_adr_tracker"

echo "1. Checking prototype directory..."
test -d "$PROTOTYPE_DIR" && report_test 0 "Prototype directory exists" || report_test 1 "Prototype directory missing"
echo ""

echo "2. Checking Cargo.toml..."
test -f "$PROTOTYPE_DIR/Cargo.toml" && report_test 0 "Cargo.toml exists" || report_test 1 "Cargo.toml missing"
echo ""

echo "3. Checking source files..."
test -f "$PROTOTYPE_DIR/src/lib.rs" && report_test 0 "lib.rs exists" || report_test 1 "lib.rs missing"
test -f "$PROTOTYPE_DIR/src/bin/adr_tracker.rs" && report_test 0 "CLI binary exists" || report_test 1 "CLI binary missing"
echo ""

echo "4. Building prototype..."
cd "$PROTOTYPE_DIR"
if cargo build --release 2>/dev/null; then
    report_test 0 "Prototype builds successfully"
else
    report_test 1 "Prototype build failed"
    echo ""
    echo "Build output:"
    cargo build --release
fi
echo ""

echo "5. Running unit tests..."
if cargo test 2>/dev/null; then
    report_test 0 "Unit tests pass"
else
    report_test 1 "Unit tests failed"
    echo ""
    echo "Test output:"
    cargo test
fi
echo ""

echo "6. Testing CLI case creation..."
if cargo run --release -- create ADR001 arbitration "Contract Dispute" "Test description" 100000 2>/dev/null | grep -q "Case created"; then
    report_test 0 "CLI create command works"
else
    report_test 1 "CLI create command failed"
fi
echo ""

echo "7. Testing CLI status update..."
if cargo run --release -- status case001 in_progress 2>/dev/null | grep -q "Case status updated"; then
    report_test 0 "CLI status command works"
else
    report_test 1 "CLI status command failed"
fi
echo ""

echo "8. Testing CLI compliance check..."
if cargo run --release -- compliance case001 2>/dev/null | grep -q "Compliance Report"; then
    report_test 0 "CLI compliance command works"
else
    report_test 1 "CLI compliance command failed"
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
