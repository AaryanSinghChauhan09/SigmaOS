// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Open Source OS & Arch Linux Missing Components V81 PR Gateway
// (`tests/test_open_source_os_and_arch_missing_components_v81_pr.rs`)

#[path = "../src/distro/open_source_os_missing_components_v81_pr.rs"]
mod open_source_os_missing_components_v81_pr;

#[path = "../src/distro/arch_linux_missing_components_v81_pr.rs"]
mod arch_linux_missing_components_v81_pr;

#[path = "../src/distro/open_source_os_and_arch_missing_components_v81_pr.rs"]
mod open_source_os_and_arch_missing_components_v81_pr;

use open_source_os_and_arch_missing_components_v81_pr::UniversalOpenSourceOsAndArchV81PrGatewayManager;
use open_source_os_missing_components_v81_pr::SovereignOpenSourceOsMissingComponentsV81PrSuite;
use arch_linux_missing_components_v81_pr::SovereignArchLinuxMissingComponentsV81PrSuite;

#[test]
fn test_v81_open_source_os_and_arch_missing_components_pr_suite() {
    // 1. Verify Open Source OS V81 Suite
    let os_suite = SovereignOpenSourceOsMissingComponentsV81PrSuite::new();
    let os_proposals = os_suite.execute_full_v81_pr_pipeline();
    assert_eq!(os_proposals.len(), 12);
    assert_eq!(os_suite.calculate_subsystem_interop_score(), 100);

    // 2. Verify Arch Linux V81 Suite
    let arch_suite = SovereignArchLinuxMissingComponentsV81PrSuite::new();
    let arch_prs = arch_suite.execute_full_v81_arch_pipeline();
    assert_eq!(arch_prs.len(), 12);

    // 3. Verify Master Synthesis Gateway Manager
    let manager = UniversalOpenSourceOsAndArchV81PrGatewayManager::new();
    let combined_pipeline = manager.execute_full_v81_master_pipeline();
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
