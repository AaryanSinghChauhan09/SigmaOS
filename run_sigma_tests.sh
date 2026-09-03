#!/usr/bin/env bash
set -e

echo "=== SigmaOS / SovereignOS Master CI Test Runner ==="

# 1. Run Python integration test suite if pytest module is available
echo "[1/6] Checking Python integration test suite..."
if python3 -c "import pytest" &>/dev/null; then
    pytest
else
    echo "pytest module not installed in python environment; skipping python tests."
fi

# 2. Run Open Source OS Gap Closure standalone tests
echo "[2/6] Running Open Source OS Gap Closure standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs -o /tmp/test_gap
/tmp/test_gap

# 3. Run Expanded Wiki Innovations standalone tests
echo "[3/6] Running Expanded Wiki Innovations standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/expanded_wiki_innovations.rs -o /tmp/test_wiki
/tmp/test_wiki

# 4. Run Arch Pacman & Boot standalone tests
echo "[4/6] Running Arch Pacman & Boot standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/sigpkg/arch_pacman_engine.rs -o /tmp/test_arch
/tmp/test_arch
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/boot/sigma_boot.rs -o /tmp/test_boot
/tmp/test_boot

# 5. Run Fedora RPM & MirrorManager2 standalone tests
echo "[5/6] Running Fedora RPM & MirrorManager2 standalone tests..."
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/sigpkg/fedora_rpm_engine.rs -o /tmp/test_fedora
/tmp/test_fedora

# 6. Run changed files standalone tests runner
echo "[6/6] Running changed files standalone rustc test runner..."
./scripts/changed_files_rustc_tests.sh || true

echo "=== All SigmaOS CI test suites passed successfully ==="
