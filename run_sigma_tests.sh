#!/usr/bin/env bash
# SigmaOS Native Test Runner
set -e

echo "=== SigmaOS Native Test Runner ==="

if [ -f "./algorithm_and_components_inspection_tests" ]; then
    echo "Running core algorithm & component inspection test binary..."
    ./algorithm_and_components_inspection_tests
fi

if [ -f "src/security/input_validation.rs" ]; then
    echo "Running security input validation test suite..."
    mkdir -p build
    rustc --test src/security/input_validation.rs --edition=2021 -o build/input_val_test
    ./build/input_val_test
fi

if [ -f "src/distro/linux_bsd_inspirations.rs" ]; then
    echo "Running Linux & BSD distro inspirations & subsystem bridge test suite..."
    mkdir -p build
    rustc --test src/distro/linux_bsd_inspirations.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/distro_inspirations_test
    ./build/distro_inspirations_test
fi

echo "=== All SigmaOS Tests Passed ==="

if [ -f "src/launch_ready/mod.rs" ]; then
    echo "Running launch readiness & distro parity test suite..."
    mkdir -p build
    rustc --test src/launch_ready/mod.rs --edition=2021 -o build/launch_ready_test
    ./build/launch_ready_test
fi

if [ -f "tests/test_vecdeque_standalone.rs" ]; then
    echo "Running VecDeque performance & correctness test suite..."
    mkdir -p build
    rustc --test tests/test_vecdeque_standalone.rs --edition=2021 -o build/vecdeque_test
    ./build/vecdeque_test
fi

if [ -f "tests/test_hashmap_standalone.rs" ]; then
    echo "Running HashMap performance & correctness test suite..."
    mkdir -p build
    rustc --test tests/test_hashmap_standalone.rs --edition=2021 -o build/hashmap_test
    ./build/hashmap_test
fi

if [ -f "src/distro/arch.rs" ]; then
    echo "Running Arch Linux parity & tooling test suite..."
    mkdir -p build
    rustc --test src/distro/arch.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/arch_test
    ./build/arch_test
fi

if [ -f "src/distro/arch_missing_components.rs" ]; then
    echo "Running Arch Linux missing components suite..."
    mkdir -p build
    rustc --test src/distro/arch_missing_components.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/arch_missing_test
    ./build/arch_missing_test
fi

if [ -f "src/package/universal.rs" ]; then
    echo "Running Universal Package Manager multi-distro test suite..."
    mkdir -p build
    rustc --test src/package/universal.rs --edition=2021 -o build/universal_pkg_test
    ./build/universal_pkg_test
fi

if [ -f "src/sigpkg/arch_pacman_engine.rs" ]; then
    echo "Running Arch Pacman Engine & AUR compilation test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/sigpkg/arch_pacman_engine.rs -o build/test_arch_engine
    ./build/test_arch_engine
fi
