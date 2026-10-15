// SPDX-License-Identifier: MIT
// Comprehensive Integration Test Suite for Cachy Linux Missing Components PR Gateway
// (`tests/test_cachy_linux_missing_components_pr.rs`)

#[path = "../src/distro/cachy_linux_missing_components_pr_gateway.rs"]
mod cachy_linux_missing_components_pr_gateway;

use cachy_linux_missing_components_pr_gateway::*;

#[test]
fn test_cachy_linux_missing_components_pr_gateway_full_lifecycle() {
    let mut gateway = CachyLinuxMissingComponentsPrGateway::new();

    // Verify all 13 staged PR proposals
    assert_eq!(
        gateway.get_merged_pr_count(),
        13,
        "Expected 13 merged Cachy Linux PR proposals"
    );

    // Verify patch diff generation for all PR proposals (IDs 2001..=2013)
    for pr_id in 2001..=2013 {
        let diff_opt = gateway.generate_pr_diff_patch(pr_id);
        assert!(
            diff_opt.is_some(),
            "Diff patch for Cachy PR #{} must exist",
            pr_id
        );
        let diff_str = diff_opt.unwrap();
        assert!(
            diff_str.contains(&format!("PR #{}", pr_id)),
            "Diff patch for PR #{} must contain PR ID header",
            pr_id
        );
        assert!(
            diff_str.contains("Status: Merged"),
            "Diff patch for PR #{} must indicate Merged status",
            pr_id
        );
    }

    // Verify execution and runtime verification of all PR proposals
    let success = gateway.execute_and_verify_all_prs();
    assert!(
        success,
        "Execution and runtime verification of all Cachy Linux PR proposals failed"
    );
}

#[test]
fn test_cachy_linux_pr_categories() {
    let categories = [
        (CachyPrCategory::SchedExtScheduler, "cachyos-kernel & scx sched_ext BPF schedulers"),
        (CachyPrCategory::HardwareDetectionChwd, "cachyos-chwd hardware detection & x86-64-v3/v4 ISA"),
        (CachyPrCategory::PerformanceSettingsTuner, "cachyos-settings sysctl & gaming tuner"),
        (CachyPrCategory::RateMirrorsRanker, "cachyos-rate-mirrors latency ranker"),
        (CachyPrCategory::HelloGamingMeta, "cachyos-hello & gaming-meta runtime environment"),
        (CachyPrCategory::CalamaresInstaller, "cachyos-calamares-settings & Btrfs subvolumes"),
        (CachyPrCategory::UksmDeduplicator, "cachyos-ksm UKSM background RAM deduplicator"),
        (CachyPrCategory::HandheldGaming, "cachyos-handheld Steam Deck & ROG Ally TDP control"),
        (CachyPrCategory::SnapperBtrfsBackup, "cachyos-snapper-support Btrfs snapshots & Limine sync"),
        (CachyPrCategory::InteractiveKernelBuilder, "cachyos-kernel-manager interactive kernel compiler"),
        (CachyPrCategory::NetworkTuning, "cachyos-network-tuner TCP BBRv3 & DoH server"),
        (CachyPrCategory::ProtonWineRuntime, "cachyos-gaming-meta-v2 Proton/Wine eSync/fSync/NTSync"),
        (CachyPrCategory::ChwdGuiInstaller, "cachyos-chwd-gui hardware detection & driver installer"),
    ];

    for (cat, expected_str) in categories {
        assert_eq!(cat.as_str(), expected_str);
    }
}
