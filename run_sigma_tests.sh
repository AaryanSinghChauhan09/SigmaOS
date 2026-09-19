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

if [ -f "src/distro/distro_inspiration_synthesis.rs" ]; then
    echo "Running Linux & BSD distro inspiration synthesis test suite..."
    mkdir -p build
    rustc --test src/distro/distro_inspiration_synthesis.rs --edition=2021 -o build/distro_synthesis_test
    ./build/distro_synthesis_test
fi

if [ -f "src/distro/linux_bsd_ultimate_synthesis.rs" ]; then
    echo "Running Linux & BSD ultimate synthesis test suite..."
    mkdir -p build
    rustc --test src/distro/linux_bsd_ultimate_synthesis.rs --edition=2021 -o build/linux_bsd_ultimate_synthesis_test
    ./build/linux_bsd_ultimate_synthesis_test
fi

if [ -f "src/distro/sovereign_2026_distro_leap_engine.rs" ]; then
    echo "Running 2026 Distro Leap test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_2026_distro_leap_engine.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_2026_leap
    ./build/test_2026_leap
fi

if [ -f "src/distro/linux_bsd_distro_breakthroughs.rs" ]; then
    echo "Running Linux & BSD Distro Breakthroughs test suite..."
    mkdir -p build
    rustc --test src/distro/linux_bsd_distro_breakthroughs.rs --edition=2021 -o build/distro_breakthroughs_test
    ./build/distro_breakthroughs_test
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

if [ -f "tests/test_string_parser_standalone.rs" ]; then
    echo "Running String, Config & TOML parser performance & correctness test suite..."
    mkdir -p build
    rustc --test tests/test_string_parser_standalone.rs --edition=2021 -o build/string_parser_test
    ./build/string_parser_test
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

if [ -f "src/package/universal_package_innovations_suite.rs" ]; then
    echo "Running Universal Package Innovations Suite test suite..."
    mkdir -p build
    rustc --test src/package/universal_package_innovations_suite.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/universal_innovations_test
    ./build/universal_innovations_test
fi

if [ -f "src/package/sovereign_distro_package_innovations.rs" ]; then
    echo "Running Sovereign Distro Package Innovations test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_innovations.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_innovations_test
    ./build/sovereign_innovations_test
fi

if [ -f "src/sigpkg/arch_pacman_engine.rs" ]; then
    echo "Running Arch Pacman Engine & AUR compilation test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/sigpkg/arch_pacman_engine.rs -o build/test_arch_engine
    ./build/test_arch_engine
fi

if [ -f "src/unimplemented_features.rs" ]; then
    echo "Running Unimplemented Features & Distro Parity test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/unimplemented_features.rs -o build/test_unimplemented_features
    ./build/test_unimplemented_features
fi

if [ -f "src/unimplemented_tools.rs" ]; then
    echo "Running Unimplemented Tools & 100-Ideas test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/unimplemented_tools.rs -o build/test_unimplemented_tools
    ./build/test_unimplemented_tools
fi

if [ -f "src/wiki_unimplemented_ideas.rs" ]; then
    echo "Running Wiki Unimplemented Ideas test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/wiki_unimplemented_ideas.rs -o build/test_wiki_unimplemented_ideas
    ./build/test_wiki_unimplemented_ideas
fi

if [ -f "src/sovereign_wiki_master_engine.rs" ]; then
    echo "Running Sovereign Wiki Master Engine test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/sovereign_wiki_master_engine.rs -o build/test_sovereign_wiki_master_engine
    ./build/test_sovereign_wiki_master_engine
fi

if [ -f "src/expanded_wiki_innovations.rs" ]; then
    echo "Running Expanded Wiki Innovations test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/expanded_wiki_innovations.rs -o build/test_expanded_wiki_innovations
    ./build/test_expanded_wiki_innovations
fi

if [ -f "tests/test_md_wiki_ideas_verification.rs" ]; then
    echo "Running .MD Files & Wiki Ideas Master Verification test suite..."
    mkdir -p build
    rustc --test --edition=2021 tests/test_md_wiki_ideas_verification.rs -o build/test_md_wiki_ideas
    ./build/test_md_wiki_ideas
fi

if [ -f "src/wiki_unimplemented_ideas.rs" ]; then
    echo "Running GitHub Wiki Unimplemented Ideas Parity test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/wiki_unimplemented_ideas.rs -o build/test_wiki_unimplemented
    ./build/test_wiki_unimplemented
fi

if [ -f "src/drivers/sovereign_distro_driver_suite.rs" ]; then
    echo "Running Sovereign Universal Distro Driver Suite test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/drivers/sovereign_distro_driver_suite.rs -o build/test_sovereign_distro_driver_suite
    ./build/test_sovereign_distro_driver_suite
fi

if [ -f "src/process/checkpoint.rs" ]; then
    echo "Running Process CRIU Checkpoint/Restore test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/process/checkpoint.rs -o build/test_checkpoint
    ./build/test_checkpoint
fi

if [ -f "src/process/uprobes.rs" ]; then
    echo "Running Process Uprobes Instrumentation test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/process/uprobes.rs -o build/test_uprobes
    ./build/test_uprobes
fi

if [ -f "src/process/ipc_ring.rs" ]; then
    echo "Running Process Shared Memory Ring IPC test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/process/ipc_ring.rs -o build/test_ipc_ring
    ./build/test_ipc_ring
fi

if [ -f "src/kernel/bsd_capsicum_enhanced.rs" ]; then
    echo "Running Enhanced FreeBSD Capsicum Capability test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/kernel/bsd_capsicum_enhanced.rs -o build/test_capsicum_enhanced
    ./build/test_capsicum_enhanced
fi

if [ -f "src/system/structure_synthesis.rs" ]; then
    echo "Running System Structure Synthesis test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/system/structure_synthesis.rs -o build/test_struct_synthesis
    ./build/test_struct_synthesis
fi

if [ -f "src/tools/strace_cli.rs" ]; then
    echo "Running Strace Syscall Inspector test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/tools/strace_cli.rs -o build/test_strace_cli
    ./build/test_strace_cli
fi

if [ -f "src/tools/crypto_verifier.rs" ]; then
    echo "Running Signify PQC Verifier test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/tools/crypto_verifier.rs -o build/test_crypto_verifier
    ./build/test_crypto_verifier
fi

if [ -f "src/tools/fsck_repair.rs" ]; then
    echo "Running Fsck Integrity Repair test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/tools/fsck_repair.rs -o build/test_fsck_repair
    ./build/test_fsck_repair
fi

if [ -f "src/tools/gpu_monitor.rs" ]; then
    echo "Running GPU Process Telemetry test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/tools/gpu_monitor.rs -o build/test_gpu_monitor
    ./build/test_gpu_monitor
fi

if [ -f "src/tools/nix_graph.rs" ]; then
    echo "Running Nix Store Graph Visualizer test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/tools/nix_graph.rs -o build/test_nix_graph
    ./build/test_nix_graph
fi
