// SPDX-License-Identifier: MIT
// Comprehensive Test Suite for Open Source OS Unimplemented Ideas PR Gateway
// and Arch Linux GitHub Repo Missing Components PR Engine

#[path = "../src/distro/open_source_os_unimplemented_ideas_pr_gateway.rs"]
mod open_source_os_unimplemented_ideas_pr_gateway;

#[path = "../src/distro/arch_linux_github_repo_components_pr_engine.rs"]
mod arch_linux_github_repo_components_pr_engine;

use open_source_os_unimplemented_ideas_pr_gateway::*;
use arch_linux_github_repo_components_pr_engine::*;

#[test]
fn test_open_source_os_unimplemented_ideas_pr_gateway_full_cycle() {
    let mut gateway = OpenSourceOsUnimplementedIdeasPrGateway::new();
    assert_eq!(gateway.get_merged_pr_count(), 14);

    for pr_id in 3001..=3014 {
        let diff = gateway.generate_pr_diff_patch(pr_id);
        assert!(diff.is_some(), "Diff patch for PR #{} must exist", pr_id);
        let diff_text = diff.unwrap();
        assert!(diff_text.contains(&format!("PR #{}", pr_id)));
    }

    let success = gateway.execute_and_verify_all_prs();
    assert!(success, "Execution and runtime verification of all 14 Open Source OS PR proposals failed");
}

#[test]
fn test_arch_linux_github_repo_components_pr_engine_full_cycle() {
    let master = SovereignArchLinuxGitHubRepoComponentsPrEngine::new();
    let prs = master.execute_all_arch_prs();
    assert_eq!(prs.len(), 7, "Expected 7 Arch Linux GitHub repo PR proposals");

    let expected_repos = [
        "archlinux/archinstall & archlinux/archiso",
        "archlinux/reflector & powerpill",
        "archlinux/pacman & archlinux/arch-testing",
        "archlinux/devtools & archlinux/dbscripts",
        "archlinux/arch-security-tracker & modprobed-db",
        "archlinux/aurutils & downgrade",
        "archlinux/arch-boxes & sysctl.d",
    ];

    for (idx, pr) in prs.iter().enumerate() {
        assert_eq!(pr.pr_id, (2001 + idx) as u32);
        assert!(pr.is_merged);
        assert_eq!(pr.target_repo, expected_repos[idx]);
        assert!(!pr.diagnostic_info.is_empty());
        assert!(pr.branch_name.contains("feat/pr-"));
    }
}
