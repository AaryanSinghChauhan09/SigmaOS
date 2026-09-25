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

if [ -f "src/security/pledge.rs" ]; then
    echo "Running security pledge and unveil test suite..."
    mkdir -p build
    rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/security/pledge.rs -o build/pledge_test
    ./build/pledge_test
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

if [ -f "src/distro/missing_linux_bsd_components.rs" ]; then
    echo "Running Missing Linux & BSD Components test suite..."
    mkdir -p build
    rustc --test src/distro/missing_linux_bsd_components.rs --edition=2021 -o build/missing_components_test
    ./build/missing_components_test
fi

if [ -f "src/distro/sovereign_linux_bsd_distro_master_suite.rs" ]; then
    echo "Running Sovereign Linux & BSD Distro Innovations Master Suite test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_linux_bsd_distro_master_suite.rs --edition=2021 -o build/distro_master_suite_test
    ./build/distro_master_suite_test
fi

if [ -f "src/distro/sovereign_linux_bsd_master_synthesis.rs" ]; then
    echo "Running Sovereign Linux & BSD Master Synthesis test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_linux_bsd_master_synthesis.rs --edition=2021 -o build/linux_bsd_master_synthesis_test
    ./build/linux_bsd_master_synthesis_test
fi

if [ -f "src/distro/sovereign_open_source_distro_synthesis.rs" ]; then
    echo "Running Sovereign Open-Source Distro Synthesis test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_open_source_distro_synthesis.rs --edition=2021 -o build/open_source_distro_synthesis_test
    ./build/open_source_distro_synthesis_test
fi

if [ -f "src/distro/sovereign_media_and_distro_unimplemented_innovations.rs" ]; then
    echo "Running Sovereign Media & Distro Unimplemented Innovations test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_media_and_distro_unimplemented_innovations.rs --edition=2021 -o build/media_distro_unimplemented_test
    ./build/media_distro_unimplemented_test
fi

if [ -f "src/network/approximation_proxy_firewall.rs" ]; then
    echo "Running Sovereign Approximation Proxy Firewall test suite..."
    mkdir -p build
    rustc --test src/network/approximation_proxy_firewall.rs --edition=2021 -o build/approximation_proxy_firewall_test
    ./build/approximation_proxy_firewall_test
fi

if [ -f "src/distro/linux_bsd_ecosystem_synthesis.rs" ]; then
    echo "Running Extended Linux & BSD Distro Ecosystem Synthesis test suite..."
    mkdir -p build
    rustc --test src/distro/linux_bsd_ecosystem_synthesis.rs --edition=2021 -o build/ecosystem_synthesis_test
    ./build/ecosystem_synthesis_test
fi

if [ -f "src/toolchain/distro_compiler_innovations.rs" ]; then
    echo "Running Linux & BSD Distro Compiler Innovations test suite..."
    mkdir -p build
    rustc --test src/toolchain/distro_compiler_innovations.rs --edition=2021 -o build/distro_compiler_test
    ./build/distro_compiler_test
fi

if [ -f "src/automation/sovereign_hotkeys_synthesis.rs" ]; then
    echo "Running Sovereign Global Hotkeys Synthesis test suite..."
    mkdir -p build
    rustc --test src/automation/sovereign_hotkeys_synthesis.rs --edition=2021 -o build/sovereign_hotkeys_test
    ./build/sovereign_hotkeys_test
fi

if [ -f "src/drivers/sovereign_trackpad_synthesis.rs" ]; then
    echo "Running Sovereign Trackpad Subsystem Synthesis test suite..."
    mkdir -p build
    rustc --test src/drivers/sovereign_trackpad_synthesis.rs --edition=2021 -o build/sovereign_trackpad_test
    ./build/sovereign_trackpad_test
fi

if [ -f "src/drivers/sovereign_sound_hda_synthesis.rs" ]; then
    echo "Running Sovereign Sound HDA Subsystem Synthesis test suite..."
    mkdir -p build
    rustc --test src/drivers/sovereign_sound_hda_synthesis.rs --edition=2021 -o build/sovereign_sound_hda_test
    ./build/sovereign_sound_hda_test
fi

if [ -f "src/drivers/sovereign_distro_driver_suite.rs" ]; then
    echo "Running Sovereign Universal Distro Driver Suite test suite..."
    mkdir -p build
    rustc --test src/drivers/sovereign_distro_driver_suite.rs --edition=2021 -o build/sovereign_distro_driver_test
    ./build/sovereign_distro_driver_test
fi

if [ -f "src/access/sovereign_access_operations_suite.rs" ]; then
    echo "Running Sovereign Access Operations Suite test suite..."
    mkdir -p build
    rustc --test src/access/sovereign_access_operations_suite.rs --edition=2021 -o build/sovereign_access_test
    ./build/sovereign_access_test
fi

if [ -f "src/access/sovereign_access_matrix_expansion.rs" ]; then
    echo "Running Sovereign Access Matrix Expansion test suite..."
    mkdir -p build
    rustc --test src/access/sovereign_access_matrix_expansion.rs --edition=2021 -o build/sovereign_access_matrix_test
    ./build/sovereign_access_matrix_test
fi

if [ -f "src/governance/sovereign_task_guidelines_wiki_sync_engine.rs" ]; then
    echo "Running Sovereign Task Guidelines & Wiki Sync Engine test suite..."
    mkdir -p build
    rustc --test src/governance/sovereign_task_guidelines_wiki_sync_engine.rs --edition=2021 -o build/sovereign_governance_wiki_test
    ./build/sovereign_governance_wiki_test
fi

if [ -f "src/kernel/panic_handler.rs" ]; then
    echo "Running Sovereign Kernel Panic & Crash Dump Subsystem test suite..."
    mkdir -p build
    rustc --test src/kernel/panic_handler.rs --edition=2021 -o build/sovereign_kernel_panic_test
    ./build/sovereign_kernel_panic_test
fi

if [ -f "src/kernel/sovereign_smp_xhci_apc_synthesis.rs" ]; then
    echo "Running Sovereign SMP Multi-Core, xHCI & APC Subsystem test suite..."
    mkdir -p build
    rustc --test src/kernel/sovereign_smp_xhci_apc_synthesis.rs --edition=2021 -o build/sovereign_smp_xhci_apc_test
    ./build/sovereign_smp_xhci_apc_test
fi

if [ -f "src/iso/sovereign_rufus_installer_synthesis.rs" ]; then
    echo "Running Sovereign Rufus & ISOHybrid Installer Synthesis test suite..."
    mkdir -p build
    rustc --test src/iso/sovereign_rufus_installer_synthesis.rs --edition=2021 -o build/sovereign_rufus_test
    ./build/sovereign_rufus_test
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
    rustc --test tests/test_string_parser_standalone.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/string_parser_test
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
    rustc --test src/package/universal.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/universal_pkg_test
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

if [ -f "src/package/sovereign_distro_package_master_suite.rs" ]; then
    echo "Running Sovereign Distro Package Master Suite test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_master_suite.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_master_suite_test
    ./build/sovereign_master_suite_test
fi

if [ -f "src/package/sovereign_distro_package_matrix_expansion.rs" ]; then
    echo "Running Sovereign Distro Package Matrix Expansion test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_matrix_expansion.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_matrix_expansion_test
    ./build/sovereign_matrix_expansion_test
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

if [ -f "src/tools/tech_media_innovations.rs" ]; then
    echo "Running Sovereign Tech Media Innovations test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/tools/tech_media_innovations.rs -o build/tech_media_test
    ./build/tech_media_test
fi

if [ -f "src/distro/tech_media_distro_innovations.rs" ]; then
    echo "Running Sovereign Tech Media Distro Innovations test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/distro/tech_media_distro_innovations.rs -o build/tech_media_distro_test
    ./build/tech_media_distro_test
fi

if [ -f "src/open_source_os_gap_closure.rs" ]; then
    echo "Running Open Source OS Gap Closure test suite..."
    mkdir -p build
    rustc --test src/open_source_os_gap_closure.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_open_source_gap_closure
    ./build/test_open_source_gap_closure
fi

if [ -f "src/open_source_obsoletion.rs" ]; then
    echo "Running Open Source Obsoletion test suite..."
    mkdir -p build
    rustc src/open_source_obsoletion.rs --crate-type=lib --test --edition=2021 -o build/test_open_source_obsoletion
    ./build/test_open_source_obsoletion
fi
