// SPDX-License-Identifier: MIT
// Integration Test Suite for Arch Linux Missing Components V75 PR Engines

#[path = "../src/distro/arch_linux_missing_components_v75_pr.rs"]
mod arch_linux_missing_components_v75_pr;

use arch_linux_missing_components_v75_pr::*;

#[test]
fn test_arch_linux_missing_components_v75_suite() {
    let suite = ArchLinuxMissingComponentsV75PrSuite::new();
    let prs = suite.run_all_prs();

    assert_eq!(prs.len(), 10, "Expected 10 Arch Linux V75 PR specs");
    assert_eq!(
        suite.compute_gap_closure_score(),
        100,
        "Gap closure score must be 100"
    );

    let expected_target_repos = [
        "archlinux/pacman",
        "archlinux/aurweb",
        "archlinux/mkinitcpio",
        "CachyOS/sched-ext",
        "archlinux/archinstall",
        "archlinux/reflector",
        "archlinux/pacman",
        "archlinux/archiso",
        "archlinux/arch-security-tracker",
        "archlinux/archlinux-keyring",
    ];

    for (idx, pr) in prs.iter().enumerate() {
        assert_eq!(pr.status, ArchV75PrStatus::Merged);
        assert_eq!(pr.target_repo, expected_target_repos[idx]);
        assert_eq!(pr.pr_id, (7501 + idx) as u32);
        assert!(!pr.summary.is_empty());
        assert!(!pr.patch_diff.is_empty());
    }

    // Individual sub-engine verification tests
    let io_uring_http3 = ArchPacman75IoUringParallelHttp3PoolPrEngine::new();
    assert!(io_uring_http3.verify());
    let pr1 = io_uring_http3.generate_pr();
    assert_eq!(pr1.pr_id, 7501);
    assert!(pr1.title.contains("io_uring Async & HTTP/3 QUIC"));

    let aurweb_auditor = ArchAurwebPqcDilithium5PkgbuildAuditorPrEngine::new();
    assert!(aurweb_auditor.verify());
    let pr2 = aurweb_auditor.generate_pr();
    assert_eq!(pr2.pr_id, 7502);

    let sbat_sealer = ArchMkinitcpioUkiSbatTpm2PolicySealerPrEngine::new();
    assert!(sbat_sealer.verify());

    let cachyos_tuner = ArchCachyOsScxLavenderEevdfSchedulerTunerPrEngine::new();
    assert!(cachyos_tuner.verify());

    let archinstall_layout = ArchArchinstallAutomatedBtrfsZsysSnapperLayoutPrEngine::new();
    assert!(archinstall_layout.verify());

    let reflector_ranker = ArchReflectorPqcSignedGeoIpMirrorRankerPrEngine::new();
    assert!(reflector_ranker.verify());

    let pacman_dedup = ArchPacmanCacheZstdDictDeduplicatorPrEngine::new();
    assert!(pacman_dedup.verify());

    let archiso_netboot = ArchisoPxeHttp3NetbootMemfsOverlayLiveEnginePr::new();
    assert!(archiso_netboot.verify());

    let security_tracker = ArchSecurityTrackerCveAutoHotfixEnginePr::new();
    assert!(security_tracker.verify());

    let keyring_rotation = ArchKeyringWkdPqcDilithium5AutoRotationPrEngine::new();
    assert!(keyring_rotation.verify());
}

#[test]
fn test_markdown_proposal_v75_output() {
    let suite = ArchLinuxMissingComponentsV75PrSuite::new();
    let proposal = suite.generate_markdown_pr_proposal();
    assert!(proposal.contains("# PR Proposal: Sovereign Arch Linux Gap Closure V75"));
    assert!(proposal.contains("Gap Closure Score: 100%"));
}
