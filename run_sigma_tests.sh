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

if [ -f "src/security/kernel_hardening.rs" ]; then
    echo "Running kernel hardening and protection rings test suite..."
    mkdir -p build
    rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/security/kernel_hardening.rs -o build/kernel_hardening_test
    ./build/kernel_hardening_test
fi

if [ -f "src/init/systemd_init.rs" ]; then
    echo "Running systemd init & service manager test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/init/systemd_init.rs -o build/systemd_test
    ./build/systemd_test
fi

if [ -f "src/memory/huge_pages.rs" ]; then
    echo "Running huge pages & transparent huge pages (THP) test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/memory/huge_pages.rs -o build/huge_pages_test
    ./build/huge_pages_test
fi

if [ -f "src/memory/kswapd.rs" ]; then
    echo "Running kswapd LRU page reclaim & ZRAM test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/memory/kswapd.rs -o build/kswapd_test
    ./build/kswapd_test
fi

if [ -f "src/filesystem/btrfs.rs" ]; then
    echo "Running Btrfs metadata and fail-closed backend test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/filesystem/btrfs.rs -o build/btrfs_test
    ./build/btrfs_test
fi

if [ -f "src/desktop/shortcuts.rs" ]; then
    echo "Running keyboard shortcut discovery and matching test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/desktop/shortcuts.rs -o build/shortcuts_test
    ./build/shortcuts_test
fi

if [ -f "src/desktop/mint_update_manager.rs" ]; then
    echo "Running Mint-inspired update policy and fail-closed backend test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/desktop/mint_update_manager.rs -o build/mint_update_manager_test
    ./build/mint_update_manager_test
fi

if [ -f "src/desktop/onboarding_wizard.rs" ]; then
    echo "Running first-run onboarding policy and unavailable-action test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/desktop/onboarding_wizard.rs -o build/onboarding_wizard_test
    ./build/onboarding_wizard_test
fi

if [ -f "src/desktop/mint_backup_tool.rs" ]; then
    echo "Running Mint-inspired backup manifest and fail-closed archive test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/desktop/mint_backup_tool.rs -o build/mint_backup_tool_test
    ./build/mint_backup_tool_test
fi

if [ -f "src/distro/linux_bsd_inspirations.rs" ]; then
    echo "Running Linux & BSD distro inspirations & subsystem bridge test suite..."
    mkdir -p build
    rustc --test src/distro/linux_bsd_inspirations.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/distro_inspirations_test
    ./build/distro_inspirations_test
fi

if [ -f "src/distro/sovereign_netbsd_parity_engine.rs" ]; then
    echo "Running Sovereign NetBSD Parity Engine test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_netbsd_parity_engine.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_sovereign_netbsd_parity
    ./build/test_sovereign_netbsd_parity
fi

if [ -f "src/distro/debian_parity.rs" ]; then
    echo "Running Debian Parity Engine test suite..."
    mkdir -p build
    rustc --test src/distro/debian_parity.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_debian_parity
    ./build/test_debian_parity
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

if [ -f "src/distro/sovereign_thousands_distro_components.rs" ]; then
    echo "Running Sovereign Thousands Distro Components test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_thousands_distro_components.rs --edition=2021 -o build/test_thousands
    ./build/test_thousands
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

if [ -f "src/distro/sovereign_linux_bsd_distro_next_gen_innovations.rs" ]; then
    echo "Running Next-Gen Sovereign Linux & BSD Distro Innovations test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_linux_bsd_distro_next_gen_innovations.rs --edition=2021 -o build/test_next_gen_distro
    ./build/test_next_gen_distro
fi

if [ -f "src/network/approximation_proxy_firewall.rs" ]; then
    echo "Running Sovereign Approximation Proxy Firewall test suite..."
    mkdir -p build
    rustc --test src/network/approximation_proxy_firewall.rs --edition=2021 -o build/approximation_proxy_firewall_test
    ./build/approximation_proxy_firewall_test
fi

if [ -f "src/network/nftables.rs" ]; then
    echo "Running nftables & iptables packet filtering test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/network/nftables.rs -o build/nftables_test
    ./build/nftables_test
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

if [ -f "src/automation/macro.rs" ]; then
    echo "Running Macro recorder and playback test suite..."
    mkdir -p build
    rustc --test --edition=2021 src/automation/macro.rs -o build/macro_test
    ./build/macro_test
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

if [ -f "src/access/mod.rs" ]; then
    echo "Running Sovereign Access Subsystem test suite..."
    mkdir -p build
    rustc --test src/access/mod.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/access_test
    ./build/access_test
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

if [ -f "src/distro/arch_linux_parity_pr_suite.rs" ]; then
    echo "Running Arch Linux Parity PR Suite test suite..."
    mkdir -p build
    rustc --test src/distro/arch_linux_parity_pr_suite.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_arch_pr_suite
    ./build/test_arch_pr_suite
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

if [ -f "src/package/sovereign_package_apc_engine.rs" ]; then
    echo "Running Sovereign Package Async Procedure Call (APC) test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_package_apc_engine.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_apc_test
    ./build/sovereign_apc_test
fi

if [ -f "src/package/sovereign_package_access_engine.rs" ]; then
    echo "Running Sovereign Package Access & Security Token test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_package_access_engine.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_access_test
    ./build/sovereign_access_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v5.rs" ]; then
    echo "Running Sovereign Distro Package Advancements Suite V5 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v5.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v5_test
    ./build/sovereign_advancements_v5_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v6.rs" ]; then
    echo "Running Sovereign Distro Package Advancements Suite V6 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v6.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v6_test
    ./build/sovereign_advancements_v6_test
fi
if [ -f "src/package/sovereign_distro_package_advancements_v7.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V7 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v7.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v7_test
    ./build/sovereign_advancements_v7_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v12.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V12 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v12.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v12_test
    ./build/sovereign_advancements_v12_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v16.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V16 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v16.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v16_test
    ./build/sovereign_advancements_v16_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v18.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V18 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v18.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v18_test
    ./build/sovereign_advancements_v18_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v20.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V20 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v20.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v20_test
    ./build/sovereign_advancements_v20_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v22.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V22 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v22.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v22_test
    ./build/sovereign_advancements_v22_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v25.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V25 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v25.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v25_test
    ./build/sovereign_advancements_v25_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v26.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V26 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v26.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v26_test
    ./build/sovereign_advancements_v26_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v27.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V27 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v27.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v27_test
    ./build/sovereign_advancements_v27_test
fi

if [ -f "src/package/sovereign_universal_pm_pr_bridge.rs" ]; then
    echo "Running Sovereign Universal PM PR Bridge Engine test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_universal_pm_pr_bridge.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_universal_pm_pr_bridge_test
    ./build/sovereign_universal_pm_pr_bridge_test
fi

if [ -f "src/package/sovereign_pr_package_gateway.rs" ]; then
    echo "Running Sovereign PR Package Gateway Engine test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_pr_package_gateway.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_pr_package_gateway_test
    ./build/sovereign_pr_package_gateway_test
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

if [ -f "src/package/sovereign_distro_package_advancements_v4.rs" ]; then
    echo "Running Sovereign Universal Package Advancements Suite V4 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v4.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v4_test
    ./build/sovereign_advancements_v4_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v5.rs" ]; then
    echo "Running Sovereign Distro Package Advancements Suite V5 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v5.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/sovereign_advancements_v5_test
    ./build/sovereign_advancements_v5_test
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
    rustc --test src/open_source_os_gap_closure.rs --edition=2021 --cfg 'feature="gap_closure_test"' -o build/test_open_source_gap_closure
    ./build/test_open_source_gap_closure
fi

if [ -f "src/open_source_obsoletion.rs" ]; then
    echo "Running Open Source Obsoletion test suite..."
    mkdir -p build
    rustc src/open_source_obsoletion.rs --crate-type=lib --test --edition=2021 --cfg 'feature="obsoletion_test"' -o build/test_open_source_obsoletion
    ./build/test_open_source_obsoletion
fi

if [ -f "src/boot/grub_engine.rs" ]; then
    echo "Running GRUB2 & BSD Loader engine test suite..."
    mkdir -p build
    rustc --test src/boot/grub_engine.rs --edition=2021 -o build/test_grub_engine
    ./build/test_grub_engine
fi

if [ -f "src/distro/sovereign_linux_bsd_ecosystem_pinnacle_suite.rs" ]; then
    echo "Running Sovereign Linux & BSD Ecosystem Pinnacle Suite test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_linux_bsd_ecosystem_pinnacle_suite.rs --edition=2021 -o build/test_ecosystem_pinnacle
    ./build/test_ecosystem_pinnacle
fi

if [ -f "src/distro/sovereign_linux_bsd_pinnacle_innovations_v14.rs" ]; then
    echo "Running Sovereign Linux & BSD Pinnacle Innovations Suite V14 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_linux_bsd_pinnacle_innovations_v14.rs --edition=2021 -o build/test_pinnacle_v14
    ./build/test_pinnacle_v14
fi

if [ -f "src/distro/sovereign_linux_bsd_media_wiki_unimplemented_ideas_engine.rs" ]; then
    echo "Running Sovereign Linux & BSD Media/Wiki Unimplemented Ideas Engine test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_linux_bsd_media_wiki_unimplemented_ideas_engine.rs --edition=2021 -o build/test_media_wiki_engine
    ./build/test_media_wiki_engine
fi

if [ -f "src/compatibility/omarchy_supreme_engine.rs" ]; then
    echo "Running Sovereign Omarchy Supreme Engine test suite..."
    mkdir -p build
    rustc --test src/compatibility/omarchy_supreme_engine.rs --edition=2021 -o build/test_omarchy
    ./build/test_omarchy
fi

if [ -f "src/compatibility/linuxmint_xapp_supreme_engine.rs" ]; then
    echo "Running Sovereign LinuxMint XApp Supreme Engine test suite..."
    mkdir -p build
    rustc --test src/compatibility/linuxmint_xapp_supreme_engine.rs --edition=2021 -o build/test_linuxmint
    ./build/test_linuxmint
fi

if [ -f "src/theming/omarchy_theme_suite.rs" ]; then
    echo "Running Sovereign Omarchy Theme Suite test suite..."
    mkdir -p build
    rustc --test src/theming/omarchy_theme_suite.rs --edition=2021 -o build/test_theming
    ./build/test_theming
fi

if [ -f "src/compatibility/sovereign_apex_mint_omarchy_supremacy.rs" ]; then
    echo "Running Sovereign Apex Mint & Omarchy Supremacy test suite..."
    mkdir -p build
    rustc --test src/compatibility/sovereign_apex_mint_omarchy_supremacy.rs --edition=2021 -o build/test_apex
    ./build/test_apex
fi

if [ -f "src/media/sovereign_hypnotix_stream_engine.rs" ]; then
    echo "Running Sovereign Hypnotix Stream Engine test suite..."
    mkdir -p build
    rustc --test src/media/sovereign_hypnotix_stream_engine.rs --edition=2021 -o build/test_hypnotix
    ./build/test_hypnotix
fi

if [ -f "src/desktop/sovereign_bulky_batch_renamer.rs" ]; then
    echo "Running Sovereign Bulky Batch Renamer test suite..."
    mkdir -p build
    rustc --test src/desktop/sovereign_bulky_batch_renamer.rs --edition=2021 -o build/test_bulky
    ./build/test_bulky
fi

if [ -f "src/desktop/omarchy_disktree_inspector.rs" ]; then
    echo "Running Sovereign DiskTree Inspector test suite..."
    mkdir -p build
    rustc --test src/desktop/omarchy_disktree_inspector.rs --edition=2021 -o build/test_disktree
    ./build/test_disktree
fi

if [ -f "src/desktop/omarchy_omakase.rs" ]; then
    echo "Running Sovereign Omarchy Omakase Developer Suite test suite..."
    mkdir -p build
    rustc --test src/desktop/omarchy_omakase.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_omarchy_omakase
    ./build/test_omarchy_omakase
fi

if [ -f "src/desktop/omarchy_chord_rebind_engine.rs" ]; then
    echo "Running Sovereign Chord Rebind Engine test suite..."
    mkdir -p build
    rustc --test src/desktop/omarchy_chord_rebind_engine.rs --edition=2021 -o build/test_chord
    ./build/test_chord
fi

if [ -f "src/ai/omarchy_multi_agent_provider.rs" ]; then
    echo "Running Sovereign Multi-Agent Provider test suite..."
    mkdir -p build
    rustc --test src/ai/omarchy_multi_agent_provider.rs --edition=2021 -o build/test_multi_agent
    ./build/test_multi_agent
fi

if [ -f "src/media/sovereign_document_reader.rs" ]; then
    echo "Running Sovereign Document Reader test suite..."
    mkdir -p build
    rustc --test src/media/sovereign_document_reader.rs --edition=2021 -o build/test_doc_reader
    ./build/test_doc_reader
fi

if [ -f "src/desktop/sovereign_xed_code_editor.rs" ]; then
    echo "Running Sovereign Xed Code Editor test suite..."
    mkdir -p build
    rustc --test src/desktop/sovereign_xed_code_editor.rs --edition=2021 -o build/test_xed
    ./build/test_xed
fi

if [ -f "src/desktop/omarchy_autosave_capture_engine.rs" ]; then
    echo "Running Sovereign Autosave Capture Engine test suite..."
    mkdir -p build
    rustc --test src/desktop/omarchy_autosave_capture_engine.rs --edition=2021 -o build/test_capture
    ./build/test_capture
fi

if [ -f "src/system/omarchy_atreyu_system_plugin.rs" ]; then
    echo "Running Sovereign Atreyu System Plugin test suite..."
    mkdir -p build
    rustc --test src/system/omarchy_atreyu_system_plugin.rs --edition=2021 -o build/test_atreyu
    ./build/test_atreyu
fi

if [ -f "src/desktop/omarchy_browser_theme_sync.rs" ]; then
    echo "Running Sovereign Browser Theme Sync test suite..."
    mkdir -p build
    rustc --test src/desktop/omarchy_browser_theme_sync.rs --edition=2021 -o build/test_browser_sync
    ./build/test_browser_sync
fi

if [ -f "src/kernel/xdp_engine_sovereign.rs" ]; then
    echo "Running XDP packet parsing and filter tests..."
    mkdir -p build
    rustc --test --edition=2021 src/kernel/xdp_engine_sovereign.rs -o build/xdp_engine_test
    ./build/xdp_engine_test
fi

if command -v node >/dev/null 2>&1; then
    echo "Running desktop and installation preview UI tests..."
    node tests/test_command_palette.js
    node tests/test_installer_preview.js
else
    echo "Skipping desktop UI tests (Node.js is not installed)."
fi

if [ -f "src/distro/sovereign_linux_bsd_ecosystem_advancements_v22.rs" ]; then
    echo "Running Sovereign Linux & BSD Ecosystem Advancements V22 & PR Gateway test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_linux_bsd_ecosystem_advancements_v22.rs --edition=2021 -o build/test_advancements_v22
    ./build/test_advancements_v22
fi

if [ -f "src/distro/sovereign_architecture_development_decision_plan.rs" ]; then
    echo "Running Sovereign Architecture Development Decision Plan test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_architecture_development_decision_plan.rs --edition=2021 -o build/test_arch_decision_plan
    ./build/test_arch_decision_plan
fi

if [ -f "src/kernel/sovereign_clean_code_and_os_principles_engine.rs" ]; then
    echo "Running Sovereign Clean Code & OS Principles test suite..."
    mkdir -p build
    rustc --test src/kernel/sovereign_clean_code_and_os_principles_engine.rs --edition=2021 -o build/test_clean_code_os_principles
    ./build/test_clean_code_os_principles
fi

if [ -f "src/distro/omarchy_linux_gap_closure_pr_suite.rs" ]; then
    echo "Running Omarchy Linux Gap Closure & PR Gateway test suite..."
    mkdir -p build
    rustc --test src/distro/omarchy_linux_gap_closure_pr_suite.rs --edition=2021 -o build/test_omarchy_gap_closure
    ./build/test_omarchy_gap_closure
fi

if [ -f "src/distro/sovereign_open_source_os_gap_closure_v27.rs" ]; then
    echo "Running Open Source OS Gap Closure V27 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_open_source_os_gap_closure_v27.rs --edition=2021 -o build/test_open_source_v27
    ./build/test_open_source_v27
fi

if [ -f "src/distro/sovereign_open_source_os_gap_closure_v31_pr.rs" ]; then
    echo "Running Open Source OS Gap Closure V31 PR test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_open_source_os_gap_closure_v31_pr.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_open_source_v31_pr
    ./build/test_open_source_v31_pr
fi

if [ -f "src/distro/sovereign_open_source_os_pinnacle_pr_v35.rs" ]; then
    echo "Running Open Source OS Pinnacle PR V35 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_open_source_os_pinnacle_pr_v35.rs --edition=2021 -o build/test_open_source_v35
    ./build/test_open_source_v35
fi

if [ -f "src/distro/sovereign_linux_bsd_ecosystem_advancements_v28.rs" ]; then
    echo "Running Sovereign Linux & BSD Ecosystem Advancements V28 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_linux_bsd_ecosystem_advancements_v28.rs --edition=2021 -o build/test_advancements_v28
    ./build/test_advancements_v28
fi

if [ -f "src/distro/sovereign_universal_subsystem_interop.rs" ]; then
    echo "Running Sovereign Universal Subsystem Interoperability test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_universal_subsystem_interop.rs --edition=2021 -o build/universal_subsystem_interop_test
    ./build/universal_subsystem_interop_test
fi

if [ -f "src/distro/sovereign_linux_bsd_ecosystem_advancements_v29.rs" ]; then
    echo "Running Sovereign Linux & BSD Ecosystem Advancements V29 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_linux_bsd_ecosystem_advancements_v29.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_advancements_v29
    ./build/test_advancements_v29
fi

if [ -f "src/distro/sovereign_mint_omarchy_apex_dominance_v30.rs" ]; then
    echo "Running Sovereign Linux Mint & Omarchy Apex Dominance Suite V30 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_mint_omarchy_apex_dominance_v30.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_apex_dominance_v30
    ./build/test_apex_dominance_v30
fi

if [ -f "src/distro/sovereign_mint_omarchy_innovations_v31.rs" ]; then
    echo "Running Sovereign Linux Mint & Omarchy Innovations Suite V31 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_mint_omarchy_innovations_v31.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_innovations_v31
    ./build/test_innovations_v31
fi

if [ -f "src/distro/sovereign_mint_omarchy_v32_apex_arsenal.rs" ]; then
    echo "Running Sovereign Linux Mint & Omarchy Apex Arsenal Suite V32 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_mint_omarchy_v32_apex_arsenal.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_apex_arsenal_v32
    ./build/test_apex_arsenal_v32
fi

if [ -f "src/distro/sovereign_mint_omarchy_v33_apex_vanguard.rs" ]; then
    echo "Running Sovereign Linux Mint & Omarchy Apex Vanguard Suite V33 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_mint_omarchy_v33_apex_vanguard.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_apex_vanguard_v33
    ./build/test_apex_vanguard_v33
fi

if [ -f "src/distro/sovereign_mint_omarchy_v34_apex_pantheon.rs" ]; then
    echo "Running Sovereign Linux Mint & Omarchy Apex Pantheon Suite V34 test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_mint_omarchy_v34_apex_pantheon.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_apex_pantheon_v34
    ./build/test_apex_pantheon_v34
fi

if [ -f "scripts/sovereign_mint_omarchy_supremacy_test.sh" ]; then
    echo "Running Sovereign Mint & Omarchy Supremacy Benchmark Suite..."
    ./scripts/sovereign_mint_omarchy_supremacy_test.sh
fi

if [ -f "scripts/sovereign_mint_omarchy_v31_innovations_test.sh" ]; then
    echo "Running Sovereign Mint & Omarchy V31 Innovations Benchmark Suite..."
    ./scripts/sovereign_mint_omarchy_v31_innovations_test.sh
fi

if [ -f "scripts/sovereign_mint_omarchy_v32_arsenal_test.sh" ]; then
    echo "Running Sovereign Mint & Omarchy V32 Arsenal Benchmark Suite..."
    ./scripts/sovereign_mint_omarchy_v32_arsenal_test.sh
fi

if [ -f "scripts/sovereign_mint_omarchy_v33_vanguard_test.sh" ]; then
    echo "Running Sovereign Mint & Omarchy V33 Vanguard Benchmark Suite..."
    ./scripts/sovereign_mint_omarchy_v33_vanguard_test.sh
fi

if [ -f "src/onboarding/first_run_migration_wizard.rs" ]; then
    echo "Running First-Run Migration Wizard test suite..."
    mkdir -p build
    rustc --test src/onboarding/first_run_migration_wizard.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_migration_wizard
    ./build/test_migration_wizard
fi

if [ -f "scripts/sovereign_mint_omarchy_v34_pantheon_test.sh" ]; then
    echo "Running Sovereign Mint & Omarchy V34 Pantheon Benchmark Suite..."
    ./scripts/sovereign_mint_omarchy_v34_pantheon_test.sh
fi

if [ -f "src/installer/migration_installer_pipeline.rs" ]; then
    echo "Running Migration Installer Pipeline test suite..."
    mkdir -p build
    rustc --test src/installer/migration_installer_pipeline.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_installer_pipeline
    ./build/test_installer_pipeline
fi

if [ -f "src/compatibility/mint_omarchy_migration_bridge.rs" ]; then
    echo "Running Mint & Omarchy Migration Bridge test suite..."
    mkdir -p build
    rustc --test src/compatibility/mint_omarchy_migration_bridge.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_migration_bridge
    ./build/test_migration_bridge
fi

if [ -f "scripts/sovereign_migration_first_benchmarks.sh" ]; then
    echo "Running Sovereign Migration-First Desktop Benchmark Suite..."
    ./scripts/sovereign_migration_first_benchmarks.sh
fi

if [ -f "scripts/release_gate_mint_omarchy_migration.sh" ]; then
    echo "Running Automated Release Validation Gate (Mint & Omarchy)..."
    ./scripts/release_gate_mint_omarchy_migration.sh
fi

echo "All SigmaOS test suites completed."
