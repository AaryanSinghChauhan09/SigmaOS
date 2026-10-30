// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Open Source OS & Arch Linux Missing Components V82 PR Gateway
// (`tests/test_open_source_os_and_arch_missing_components_v82_pr.rs`)

#[path = "../src/distro/sovereign_universal_subsystem_interop.rs"]
mod sovereign_universal_subsystem_interop;

#[path = "../src/distro/open_source_os_missing_components_v82_pr.rs"]
mod open_source_os_missing_components_v82_pr;

#[path = "../src/distro/arch_linux_missing_components_v82_pr.rs"]
mod arch_linux_missing_components_v82_pr;

#[path = "../src/distro/open_source_os_and_arch_missing_components_v82_pr.rs"]
mod open_source_os_and_arch_missing_components_v82_pr;

use open_source_os_and_arch_missing_components_v82_pr::UniversalOpenSourceOsAndArchV82PrGatewayManager;
use open_source_os_missing_components_v82_pr::SovereignOpenSourceOsMissingComponentsV82PrSuite;
use arch_linux_missing_components_v82_pr::SovereignArchLinuxMissingComponentsV82PrSuite;

#[test]
fn test_v82_open_source_os_and_arch_missing_components_pr_suite() {
    // 1. Verify Open Source OS V82 Suite
    let os_suite = SovereignOpenSourceOsMissingComponentsV82PrSuite::new();
    let os_proposals = os_suite.execute_full_v82_pr_pipeline();
    assert_eq!(os_proposals.len(), 12);
    assert_eq!(os_suite.calculate_subsystem_interop_score(), 100);

    // 2. Verify Arch Linux V82 Suite
    let arch_suite = SovereignArchLinuxMissingComponentsV82PrSuite::new();
    let arch_prs = arch_suite.execute_full_v82_arch_pipeline();
    assert_eq!(arch_prs.len(), 12);

    // 3. Verify Master Synthesis Gateway Manager
    let manager = UniversalOpenSourceOsAndArchV82PrGatewayManager::new();
    let combined_pipeline = manager.execute_full_v82_master_pipeline();
    assert_eq!(combined_pipeline.len(), 24);
    assert_eq!(manager.total_proposals_generated(), 24);

    for spec in &combined_pipeline {
        assert!(!spec.spec_id.is_empty());
        assert!(!spec.title.is_empty());
        assert!(!spec.origin_system.is_empty());
        assert!(!spec.summary.is_empty());
        assert!(!spec.patch_diff.is_empty());
    }
}
