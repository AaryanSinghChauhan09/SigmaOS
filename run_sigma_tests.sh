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

if [ -f "src/distro/arch_boxes.rs" ]; then
    echo "Running Arch Linux Boxes & cloud-init test suite..."
    mkdir -p build
    rustc --test src/distro/arch_boxes.rs --edition=2021 -o build/arch_boxes_test
    ./build/arch_boxes_test
fi

if [ -f "src/distro/arch_gap_closure.rs" ]; then
    echo "Running Arch Linux gap closure test suite..."
    mkdir -p build
    rustc --test src/distro/arch_gap_closure.rs --edition=2021 -o build/arch_gap_closure_test
    ./build/arch_gap_closure_test
fi

if [ -f "src/distro/arch_ultimate_gap_closure.rs" ]; then
    echo "Running Arch Linux ultimate gap closure test suite..."
    mkdir -p build
    rustc --test src/distro/arch_ultimate_gap_closure.rs --edition=2021 -o build/arch_ultimate_gap_closure_test
    ./build/arch_ultimate_gap_closure_test
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

if [ -f "src/driver/driver_test_framework.rs" ]; then
    echo "Running Essential Hardware Drivers test suite..."
    mkdir -p build
    rustc --test src/driver/driver_test_framework.rs --edition=2021 -o build/driver_test_framework
    ./build/driver_test_framework
fi

if [ -f "src/distro/omarchy.rs" ]; then
    echo "Running Omarchy Linux desktop & theme test suite..."
    mkdir -p build
    rustc --test src/distro/omarchy.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/omarchy_test
    ./build/omarchy_test
fi

if [ -f "src/distro/omarchy_ultimate_gap_closure.rs" ]; then
    echo "Running Omarchy Linux ultimate gap closure test suite..."
    mkdir -p build
    rustc --test src/distro/omarchy_ultimate_gap_closure.rs --edition=2021 -o build/omarchy_ultimate_test
    ./build/omarchy_ultimate_test
fi

if [ -f "src/tools/mint_driver_manager.rs" ]; then
    echo "Running Mint-inspired Driver Manager test suite..."
    mkdir -p build
    rustc --test src/tools/mint_driver_manager.rs --edition=2021 -o build/mint_driver_test
    ./build/mint_driver_test
fi

if [ -f "src/drivers/sovereign_driver_lifecycle.rs" ]; then
    echo "Running Sovereign Universal Driver Lifecycle test suite..."
    mkdir -p build
    rustc --test src/drivers/sovereign_driver_lifecycle.rs --edition=2021 -o build/driver_lifecycle_test
    ./build/driver_lifecycle_test
fi
