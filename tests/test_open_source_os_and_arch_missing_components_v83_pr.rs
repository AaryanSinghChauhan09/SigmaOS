// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Open Source OS & Arch Linux Missing Components V83 PR Gateway
// (`tests/test_open_source_os_and_arch_missing_components_v83_pr.rs`)

#[path = "../src/distro/sovereign_universal_subsystem_interop.rs"]
mod sovereign_universal_subsystem_interop;

#[path = "../src/distro/open_source_os_unimplemented_ideas_v83_pr.rs"]
mod open_source_os_unimplemented_ideas_v83_pr;

#[path = "../src/distro/arch_linux_missing_components_v83_pr.rs"]
mod arch_linux_missing_components_v83_pr;

#[path = "../src/distro/open_source_os_and_arch_missing_components_v83_pr.rs"]
mod open_source_os_and_arch_missing_components_v83_pr;

use open_source_os_and_arch_missing_components_v83_pr::UniversalOpenSourceOsAndArchV83PrGatewayManager;
use open_source_os_unimplemented_ideas_v83_pr::SovereignUnimplementedIdeasSuiteV83;
use arch_linux_missing_components_v83_pr::{ArchV83PrStatus, SovereignArchLinuxMissingComponentsV83PrSuite};

#[test]
fn test_v83_open_source_os_and_arch_missing_components_pr_suite() {
    // 1. Verify Open Source OS V83 Suite
    let os_suite = SovereignUnimplementedIdeasSuiteV83::new();
    let os_proposals = os_suite.collect_all_manifests();
    assert_eq!(os_proposals.len(), 9);

    // 2. Verify Arch Linux V83 Suite
    let arch_suite = SovereignArchLinuxMissingComponentsV83PrSuite::new();
    let arch_prs = arch_suite.execute_full_v83_arch_pipeline();
    assert_eq!(arch_prs.len(), 12);

    for pr in &arch_prs {
        assert_eq!(pr.status, ArchV83PrStatus::Validated);
        assert!(pr.pr_id >= 8301 && pr.pr_id <= 8312);
        assert!(!pr.title.is_empty());
        assert!(!pr.patch_diff.is_empty());
        assert!(!pr.subsystem.is_empty());
        assert!(!pr.target_repo.is_empty());
    }

    // 3. Verify Master Synthesis Gateway Manager
    let manager = UniversalOpenSourceOsAndArchV83PrGatewayManager::new();
    let combined_pipeline = manager.execute_full_v83_master_pipeline();
    assert_eq!(combined_pipeline.len(), 21);
    assert_eq!(manager.total_proposals_generated(), 21);

    for spec in &combined_pipeline {
        assert!(!spec.spec_id.is_empty());
        assert!(!spec.title.is_empty());
        assert!(!spec.origin_system.is_empty());
        assert!(!spec.summary.is_empty());
        assert!(!spec.patch_diff.is_empty());
    }
}
