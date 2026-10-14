#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Master Test Suite Runner for SigmaOS

set -e

echo "=== SigmaOS Master Verification & Test Suite ==="

if [ -f "src/kernel/missing_linux_kernel_components.rs" ]; then
    echo "Running Missing Linux Kernel Execution Components test suite..."
    mkdir -p build
    rustc --test src/kernel/missing_linux_kernel_components.rs --edition=2021 -o build/missing_kernel_test
    ./build/missing_kernel_test
fi

if [ -f "src/kernel/tier1_kernel_execution_suite.rs" ]; then
    echo "Running Tier 1 Critical Kernel Execution Suite test suite..."
    mkdir -p build
    rustc --test src/kernel/tier1_kernel_execution_suite.rs --edition=2021 -o build/tier1_kernel_test
    ./build/tier1_kernel_test
fi

if [ -f "src/kernel/tier2_usability_stack.rs" ]; then
    echo "Running Tier 2 Usability Stack test suite..."
    mkdir -p build
    rustc --test src/kernel/tier2_usability_stack.rs --edition=2021 -o build/tier2_usability_test
    ./build/tier2_usability_test
fi

if [ -f "src/drivers/usb_subsystem.rs" ]; then
    echo "Running USB Subsystem Driver Stack test suite..."
    mkdir -p build
    rustc --test src/drivers/usb_subsystem.rs --edition=2021 -o build/usb_subsystem_test
    ./build/usb_subsystem_test
fi

if [ -f "src/drivers/phase7_hardware_expansion.rs" ]; then
    echo "Running Phase 7 Hardware Driver Expansion test suite..."
    mkdir -p build
    rustc --test src/drivers/phase7_hardware_expansion.rs --edition=2021 -o build/phase7_hardware_test
    ./build/phase7_hardware_test
fi

if [ -f "src/resilience/backup.rs" ]; then
    echo "Running Advanced Backup & Linux Mint Timeshift Pro Engine test suite..."
    mkdir -p build
    rustc --test src/resilience/backup.rs --edition=2021 -o build/backup_test
    ./build/backup_test
fi

if [ -f "src/desktop/elementary_pantheon_innovations.rs" ]; then
    echo "Running Elementary OS Pantheon & Granite Ecosystem Innovations Suite test suite..."
    mkdir -p build
    rustc --test src/desktop/elementary_pantheon_innovations.rs --edition=2021 -o build/elementary_pantheon_test
    ./build/elementary_pantheon_test
fi

if [ -f "src/desktop/elementary_os_github_inspired_components.rs" ]; then
    echo "Running Elementary OS GitHub Inspired Components Suite test suite..."
    mkdir -p build
    rustc --test src/desktop/elementary_os_github_inspired_components.rs --edition=2021 -o build/elementary_github_components_test
    ./build/elementary_github_components_test
fi

if [ -f "src/desktop/granite_contractor.rs" ]; then
    echo "Running Granite Contractor & Switchboard Hub test suite..."
    mkdir -p build
    rustc --test src/desktop/granite_contractor.rs --edition=2021 -o build/granite_contractor_test
    ./build/granite_contractor_test
fi

if [ -f "src/desktop/zenith_display_bridge.rs" ]; then
    echo "Running Zenith Display Compositor Bridge test suite..."
    mkdir -p build
    rustc --test src/desktop/zenith_display_bridge.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/zenith_bridge_test
    ./build/zenith_bridge_test
fi

if [ -f "src/compatibility/mint_linux.rs" ]; then
    echo "Running Linux Mint Compatibility Layer & Icon Theme Engine test suite..."
    mkdir -p build
    rustc --test src/compatibility/mint_linux.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/mint_compatibility_test
    ./build/mint_compatibility_test
fi

if [ -f "src/userland/user_management.rs" ]; then
    echo "Running User & Group Management Subsystem test suite..."
    mkdir -p build
    rustc --test src/userland/user_management.rs --edition=2021 -o build/user_management_test
    ./build/user_management_test
fi

if [ -f "src/package/universal.rs" ]; then
    echo "Running Universal Package Manager Subsystem test suite..."
    mkdir -p build
    rustc --test src/package/universal.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/universal_package_test
    ./build/universal_package_test
fi

if [ -f "src/package/sovereign_distro_package_advancements_v6.rs" ]; then
    echo "Running Sovereign Distro Package Advancements V6 test suite..."
    mkdir -p build
    rustc --test src/package/sovereign_distro_package_advancements_v6.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/package_v6_test
    ./build/package_v6_test
fi

if [ -f "src/tools/warpinator_lan_sharing.rs" ]; then
    echo "Running Warpinator LAN Sharing Suite test suite..."
    mkdir -p build
    rustc --test src/tools/warpinator_lan_sharing.rs --edition=2021 -o build/warpinator_test
    ./build/warpinator_test
fi

if [ -f "src/bluetooth/mod.rs" ]; then
    echo "Running Bluetooth Stack Subsystem test suite..."
    mkdir -p build
    rustc --test src/bluetooth/mod.rs --edition=2021 -o build/bluetooth_test
    ./build/bluetooth_test
fi

if [ -f "src/security/address_sanitizer.rs" ]; then
    echo "Running Address Sanitizer Subsystem test suite..."
    mkdir -p build
    rustc --test src/security/address_sanitizer.rs --edition=2021 -o build/asan_test
    ./build/asan_test
fi

if [ -f "src/distro/linux_bsd_inspirations.rs" ]; then
    echo "Running Subsystem Linux/BSD Inspirations test suite..."
    mkdir -p build
    rustc --test src/distro/linux_bsd_inspirations.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/linux_bsd_inspirations_test
    ./build/linux_bsd_inspirations_test
fi

if [ -f "src/distro/improvements.rs" ]; then
    echo "Running Distro Improvements test suite..."
    mkdir -p build
    rustc --test src/distro/improvements.rs --edition=2021 -o build/distro_improvements_test
    ./build/distro_improvements_test
fi

if [ -f "src/distro/debian_gap_closure_advancements_v26.rs" ]; then
    echo "Running Debian Gap Closure Advancements V26 test suite..."
    mkdir -p build
    rustc --test src/distro/debian_gap_closure_advancements_v26.rs --edition=2021 -o build/debian_v26_test
    ./build/debian_v26_test
fi

if [ -f "src/distro/open_source_os_nextgen_ideas_pr_suite.rs" ]; then
    echo "Running Open Source OS Nextgen Ideas PR Suite test suite..."
    mkdir -p build
    rustc --test src/distro/open_source_os_nextgen_ideas_pr_suite.rs --edition=2021 -o build/open_source_os_nextgen_test
    ./build/open_source_os_nextgen_test
fi

if [ -f "src/distro/open_source_os_debian_unimplemented_ideas_pr.rs" ]; then
    echo "Running Open Source OS Debian Unimplemented Ideas PR Suite test suite..."
    mkdir -p build
    rustc --test src/distro/open_source_os_debian_unimplemented_ideas_pr.rs --edition=2021 -o build/open_source_os_debian_test
    ./build/open_source_os_debian_test
fi

if [ -f "src/distro/open_source_os_unimplemented_ideas_pr.rs" ]; then
    echo "Running Open Source OS Unimplemented Ideas PR Engine test suite..."
    mkdir -p build
    rustc --test src/distro/open_source_os_unimplemented_ideas_pr.rs --edition=2021 -o build/open_source_os_engine_test
    ./build/open_source_os_engine_test
fi

if [ -f "src/distro/open_source_os_unimplemented_ideas_pr_gateway.rs" ]; then
    echo "Running Open Source OS Unimplemented Ideas PR Gateway test suite..."
    mkdir -p build
    rustc --test src/distro/open_source_os_unimplemented_ideas_pr_gateway.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/open_source_os_gateway_test
    ./build/open_source_os_gateway_test
fi

if [ -f "src/distro/open_source_os_unimplemented_ideas_pr_suite.rs" ]; then
    echo "Running Open Source OS Unimplemented Ideas PR Suite test suite..."
    mkdir -p build
    rustc --test src/distro/open_source_os_unimplemented_ideas_pr_suite.rs --edition=2021 -o build/test_os_ideas_suite
    ./build/test_os_ideas_suite
fi

if [ -f "src/distro/arch_linux_github_repo_components_pr_engine.rs" ]; then
    echo "Running Arch Linux GitHub Repo Components PR Engine test suite..."
    mkdir -p build
    rustc --test src/distro/arch_linux_github_repo_components_pr_engine.rs --edition=2021 -o build/test_arch_github_repo_components
    ./build/test_arch_github_repo_components
fi

if [ -f "src/distro/sovereign_open_source_os_fedora_github_pr_suite.rs" ]; then
    echo "Running Sovereign Open Source OS Fedora GitHub PR Suite test suite..."
    mkdir -p build
    rustc --test src/distro/sovereign_open_source_os_fedora_github_pr_suite.rs --edition=2021 -o build/test_fedora_github_pr_suite
    ./build/test_fedora_github_pr_suite
fi

if [ -f "src/distro/debian_github_repo_inspirations_pr.rs" ]; then
    echo "Running Debian GitHub Repo Inspirations PR Suite test suite..."
    mkdir -p build
    rustc --test src/distro/debian_github_repo_inspirations_pr.rs --edition=2021 -o build/test_debian_github_pr
    ./build/test_debian_github_pr
fi

if [ -f "tests/test_md_wiki_ideas_verification.rs" ]; then
    echo "Running .MD Files & Wiki Ideas Master Verification test suite..."
    mkdir -p build
    rustc --test tests/test_md_wiki_ideas_verification.rs --edition=2021 -o build/test_md_wiki
    ./build/test_md_wiki
fi

if [ -f "installer/src/recovery/mod.rs" ]; then
    echo "Running Installer Recovery Subsystem test suite..."
    mkdir -p build
    rustc --test installer/src/recovery/mod.rs --edition=2021 -o build/test_installer_recovery
    ./build/test_installer_recovery
fi

echo "=== All Active Master Test Suites Executed Successfully ==="
