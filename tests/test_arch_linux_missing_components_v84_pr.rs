// SPDX-License-Identifier: MIT
// Integration Test for Arch Linux Missing Components V84 PR Suite

#[path = "../src/distro/arch_linux_missing_components_v84_pr.rs"]
mod arch_linux_missing_components_v84_pr;

use arch_linux_missing_components_v84_pr::{
    ArchV84PrStatus, SovereignArchLinuxMissingComponentsV84PrSuite,
};

#[test]
fn test_arch_linux_missing_components_v84() {
    let suite = SovereignArchLinuxMissingComponentsV84PrSuite::new();
    let prs = suite.execute_full_v84_arch_pipeline();
    assert_eq!(prs.len(), 12);

    for pr in &prs {
        assert_eq!(pr.status, ArchV84PrStatus::Validated);
        assert!(pr.pr_id >= 8401 && pr.pr_id <= 8412);
        assert!(!pr.title.is_empty());
        assert!(!pr.patch_diff.is_empty());
        assert!(!pr.subsystem.is_empty());
        assert!(!pr.target_repo.is_empty());
        assert!(!pr.summary.is_empty());
    }
}
