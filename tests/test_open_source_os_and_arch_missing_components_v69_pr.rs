// SPDX-License-Identifier: MIT
// Comprehensive Integration Test Suite for Open Source OS Unimplemented Ideas & Arch Linux Missing Components V69 PR Engines

#[path = "../src/distro/open_source_os_unimplemented_ideas_v69_pr.rs"]
mod open_source_os_unimplemented_ideas_v69_pr;

#[path = "../src/distro/arch_linux_missing_components_v69_pr.rs"]
mod arch_linux_missing_components_v69_pr;

use arch_linux_missing_components_v69_pr::*;
use open_source_os_unimplemented_ideas_v69_pr::*;

#[test]
fn test_open_source_os_unimplemented_ideas_v69_pr_suite_full_cycle() {
    let mut suite = OpenSourceOsUnimplementedIdeasV69PrSuite::new();
    assert_eq!(suite.execute_pr_pipeline(), 9);
    assert!(suite.verify_all_pr_engines());

    let proposals = suite.collect_all_proposals();
    assert_eq!(proposals.len(), 9);

    for prop in proposals {
        assert!(prop.pr_id >= 6901 && prop.pr_id <= 6909);
        assert_eq!(prop.status, OsPrStatusV69::Validated);
        assert!(prop.parity_score >= 95);
        assert!(!prop.title.is_empty());
        assert!(!prop.subsystem.is_empty());
        assert!(!prop.upstream_target.is_empty());
        assert!(!prop.diff_patch.is_empty());
    }
}

#[test]
fn test_arch_linux_v69_pr_suite_full_cycle() {
    let suite = ArchLinuxMissingComponentsV69PrSuite::new();
    let prs = suite.run_all_prs();

    assert_eq!(
        prs.len(),
        10,
        "Expected 10 Arch Linux V69 PR specifications"
    );
    assert_eq!(
        suite.compute_gap_closure_score(),
        100,
        "Gap closure score must be 100"
    );

    let expected_target_repos = [
        "archlinux/pacman",
        "Jguer/yay",
        "archlinux/mkinitcpio",
        "CachyOS/linux-cachyos",
        "archlinux/archinstall",
        "archlinux/pacman",
        "archlinux/archiso",
        "archlinux/pacman-contrib",
        "archlinux/archnews",
        "archlinux/archlinux-keyring",
    ];

    for (idx, pr) in prs.iter().enumerate() {
        assert_eq!(pr.status, ArchV69PrStatus::Merged);
        assert_eq!(pr.target_repo, expected_target_repos[idx]);
        assert_eq!(pr.pr_id, (6901 + idx) as u32);
        assert!(!pr.summary.is_empty());
        assert!(!pr.patch_diff.is_empty());
    }

    // Sub-engine verification tests
    let parallel_download = ArchPacmanParallelDownloadConnectionPoolPrEngine::new();
    assert!(parallel_download.verify());
    let pr1 = parallel_download.generate_pr();
    assert_eq!(pr1.pr_id, 6901);
    assert!(pr1.title.contains("Parallel Download"));

    let yay_resolver = ArchAurHelperYayDependencyResolverPrEngine::new();
    assert!(yay_resolver.verify());
    let pr2 = yay_resolver.generate_pr();
    assert_eq!(pr2.pr_id, 6902);

    let uki_stub = ArchMkinitcpioSystemdUkiStubEnginePrEngine::new();
    assert!(uki_stub.verify());
    let pr3 = uki_stub.generate_pr();
    assert_eq!(pr3.pr_id, 6903);

    let cachyos = ArchCachyOsEevdfSchedulerBoreAutoTunerPrEngine::new();
    assert!(cachyos.verify());

    let btrfs = ArchArchinstallAutomatedBtrfsSubvolLayoutPrEngine::new();
    assert!(btrfs.verify());

    let repo_custom = ArchPacmanConfigRepositoryCustomDbPrEngine::new();
    assert!(repo_custom.verify());

    let archiso = ArchArchisoSquashfsZstdBootstrapPrEngine::new();
    assert!(archiso.verify());

    let paccache = ArchPacmanCachePaccacheRetentionPrEngine::new();
    assert!(paccache.verify());

    let archnews = ArchArchNewsSystemNotificationPrEngine::new();
    assert!(archnews.verify());

    let wkd_keyring = ArchKeyringWebOfTrustAutoRefreshPrEngine::new();
    assert!(wkd_keyring.verify());
}
