// SPDX-License-Identifier: MIT
// Integration Test Suite for CachyOS Missing Components V73 PR Engine

#[path = "../src/distro/cachy_linux_missing_components_v73_pr.rs"]
mod cachy_linux_missing_components_v73_pr;

use cachy_linux_missing_components_v73_pr::*;

#[test]
fn test_cachy_v73_pr_suite_manifests_and_execution() {
    let suite = CachyLinuxMissingComponentsV73PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);

    for (idx, manifest) in manifests.iter().enumerate() {
        assert_eq!(manifest.status, CachyOsPrStatusV73::Proposed);
        assert_eq!(manifest.pr_number, 7301 + idx as u32);
        assert!(!manifest.title.is_empty());
        assert!(!manifest.component_id.is_empty());
    }

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_components").map(|s| s.as_str()), Some("10"));
    assert_eq!(summary.get("release_parity").map(|s| s.as_str()), Some("v73.0"));
}

#[test]
fn test_cachy_v73_sub_engines_behavior() {
    let mut suite = CachyLinuxMissingComponentsV73PrSuite::new();

    let sched_msg = suite.scx_scheduler.schedule_gaming_thread(1234, 4);
    assert!(sched_msg.contains("Thread 1234"));

    assert!(suite.kernel_pipeline.optimize_kernel_binary());

    let scanout_msg = suite.chwd_scanout.bypass_compositor(88);
    assert!(scanout_msg.contains("surface 88"));

    assert!(suite.pacman_verifier.verify_package_signature("kernel.pkg.tar.zst"));

    let fsync_msg = suite.proton_fsync.synchronize_win32_handles(16);
    assert!(fsync_msg.contains("16 Win32"));

    let ananicy_msg = suite.ananicy_governor.classify_process("cyberpunk.exe", 9999);
    assert!(ananicy_msg.contains("cyberpunk.exe"));

    let comp_level = suite.btrfs_tuner.tune_compression_ratio(10000);
    assert_eq!(comp_level, 1);

    let pace_msg = suite.gamescope_pacer.pace_frame(500, 144);
    assert!(pace_msg.contains("144 Hz"));

    assert!(suite.zram_pool.allocate_compressed_page(2048));

    let handheld_msg = suite.handheld_governor.set_tdp_cap(25);
    assert!(handheld_msg.contains("25 Watts"));
}
