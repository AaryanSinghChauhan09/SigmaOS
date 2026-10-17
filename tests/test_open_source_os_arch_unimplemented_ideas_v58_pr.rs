// SPDX-License-Identifier: MIT
// Comprehensive Integration Test Suite for Open Source OS Unimplemented Ideas
// & Arch Linux GitHub Repo Missing Components V58 PR Suite

#[path = "../src/distro/open_source_os_arch_unimplemented_ideas_pr_v58.rs"]
mod open_source_os_arch_unimplemented_ideas_pr_v58;

use open_source_os_arch_unimplemented_ideas_pr_v58::*;

#[test]
fn test_v58_os_and_arch_prs_full_cycle() {
    let mut suite = OpenSourceOsAndArchUnimplementedIdeasV58PrSuite::new();
    assert_eq!(suite.get_merged_pr_count(), 14);
    assert!(suite.execute_and_verify_all_os_prs());

    let arch_prs = suite.execute_all_arch_prs();
    assert_eq!(arch_prs.len(), 3, "Expected 3 Arch Linux GitHub repo PR proposals");

    let expected_repos = [
        "archlinux/archinstall & archlinux/archiso",
        "archlinux/reflector & powerpill",
        "archlinux/pacman & archlinux/arch-testing",
    ];

    for (idx, pr) in arch_prs.iter().enumerate() {
        assert_eq!(pr.pr_id, (2501 + idx) as u32);
        assert!(pr.is_merged);
        assert_eq!(pr.target_repo, expected_repos[idx]);
        assert!(!pr.diagnostic_info.is_empty());
        assert!(pr.branch_name.contains("feat/pr-"));
    }
}
