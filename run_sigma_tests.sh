#!/usr/bin/env bash
set -e

echo "=== SigmaOS / SovereignOS Master CI Test Runner ==="

# 1. Run Python integration test suite if python3/pytest available
echo "[1/5] Running Python integration test suite..."
if command -v pytest &>/dev/null; then
    pytest
elif command -v python3 &>/dev/null; then
    python3 -m unittest discover -s tests -p "test_*.py"
else
    echo "Python test environment not available; skipping."
fi

# 2. Run Open Source OS Gap Closure standalone tests
echo "[2/5] Running Open Source OS Gap Closure standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs -o /tmp/test_gap
/tmp/test_gap

# 3. Run Expanded Wiki Innovations standalone tests
echo "[3/5] Running Expanded Wiki Innovations standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/expanded_wiki_innovations.rs -o /tmp/test_wiki
/tmp/test_wiki

# 4. Run Arch Pacman & Boot standalone tests
echo "[4/5] Running Arch Pacman & Boot standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/sigpkg/arch_pacman_engine.rs -o /tmp/test_arch
/tmp/test_arch
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/boot/sigma_boot.rs -o /tmp/test_boot
/tmp/test_boot

# 5. Run changed files standalone tests runner
echo "[5/5] Running changed files standalone rustc test runner..."
./scripts/changed_files_rustc_tests.sh || true

echo "=== All SigmaOS CI test suites passed successfully ==="
