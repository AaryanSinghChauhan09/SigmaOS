// SPDX-License-Identifier: MIT
// Comprehensive Integration Test Suite for CachyOS Missing Components PR Engine (V71)

#[path = "../src/distro/cachy_linux_missing_components_v71_pr.rs"]
mod cachy_linux_missing_components_v71_pr;

use cachy_linux_missing_components_v71_pr::*;

#[test]
fn test_cachy_linux_v71_pr_suite_full_cycle() {
    let suite = CachyLinuxMissingComponentsV71PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);

    for (idx, manifest) in manifests.iter().enumerate() {
        assert_eq!(manifest.pr_number, (7101 + idx) as u32);
        assert_eq!(manifest.status, CachyOsPrStatusV71::Proposed);
        assert!(!manifest.title.is_empty());
        assert!(!manifest.component_id.is_empty());
        assert!(!manifest.patch_description.is_empty());
        assert!(!manifest.diff_summary.is_empty());
    }

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_components").unwrap(), "10");
}

#[test]
fn test_individual_cachyos_v71_engines() {
    let mut scx = CachyOsScxCentralLavdV5SchedulerPrEngine::new();
    let msg1 = scx.schedule_gaming_thread(8192, 4);
    assert!(msg1.contains("Thread 8192"));

    let mut bolt = CachyOsKernelAutoboltPgoV5PipelinePrEngine::new();
    let msg2 = bolt.execute_tri_stage_optimization();
    assert!(msg2.contains("BOLT v5"));

    let mut scanout = CachyOsChwdVulkanWaylandDirectScanoutV5PrEngine::new();
    let msg3 = scanout.configure_direct_scanout("vk_game_surface");
    assert!(msg3.contains("vk_game_surface"));

    let mut pqc = CachyOsPacmanV4PqcDilithiumVerifierV71PrEngine::new();
    let msg4 = pqc.verify_pqc_package("linux-cachyos-v71");
    assert!(msg4.contains("linux-cachyos-v71"));

    let mut ntsync = CachyOsProtonWineNtsyncFsync5GovernorPrEngine::new();
    let msg5 = ntsync.dispatch_event_array(64);
    assert!(msg5.contains("64"));

    let mut ananicy = CachyOsAnanicyRealtimeEbpfV5GovernorPrEngine::new();
    let msg6 = ananicy.prioritize_game_process("cyberpunk2077.exe");
    assert!(msg6.contains("cyberpunk2077.exe"));

    let mut btrfs = CachyOsBtrfsZstdEbpfCompressorV5TunerPrEngine::new();
    let msg7 = btrfs.optimize_file_compression(true);
    assert!(msg7.contains("level 1"));

    let mut gamescope = CachyOsGamescopeHdr10VrrPassthroughPacerV5PrEngine::new();
    let msg8 = gamescope.pace_vulkan_presentation(2000);
    assert!(msg8.contains("2000"));

    let mut zram = CachyOsZramLz4ZstdDynamicPoolV5PrEngine::new();
    let msg9 = zram.migrate_cold_swap_pages();
    assert!(msg9.contains("ZRAM Hybrid Pool v5"));

    let mut power = CachyOsHandheldPowerCapThermalV5GovernorPrEngine::new();
    let msg10 = power.apply_handheld_settings("Overclock Extreme v5", 35);
    assert!(msg10.contains("Overclock Extreme v5"));
}
