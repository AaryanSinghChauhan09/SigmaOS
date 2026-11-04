// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! Integration Test Suite for V84 Open Source OS Unimplemented Ideas & Cachy Linux Missing Components PR Engine
//! (`tests/test_open_source_os_and_cachy_linux_v84_pr.rs`)

#[path = "../src/distro/cachy_linux_missing_components_v84_pr.rs"]
mod cachy_linux_missing_components_v84_pr;
#[path = "../src/distro/open_source_os_and_cachy_linux_v84_pr.rs"]
mod open_source_os_and_cachy_linux_v84_pr;
#[path = "../src/distro/open_source_os_unimplemented_ideas_v84_pr.rs"]
mod open_source_os_unimplemented_ideas_v84_pr;

use cachy_linux_missing_components_v84_pr::*;
use open_source_os_and_cachy_linux_v84_pr::*;
use open_source_os_unimplemented_ideas_v84_pr::*;

#[test]
fn test_open_source_os_v84_engines_and_manifests() {
    let suite = SovereignUnimplementedIdeasSuiteV84::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 9);

    assert_eq!(manifests[0].pr_id, "PR-SIGMA-V84-01-LINUX-638");
    assert!(manifests[0].title.contains("Linux 6.38+"));

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_proposals").unwrap(), "9");
    assert_eq!(summary.get("suite_version").unwrap(), "v84.0");
}

#[test]
fn test_cachy_linux_v84_engines_and_manifests() {
    let suite = CachyLinuxMissingComponentsV84PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);

    assert_eq!(manifests[0].pr_number, 8401);
    assert_eq!(
        manifests[0].component_id,
        "cachyos-scx-central-lavd-v10-scheduler"
    );

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_components").unwrap(), "10");
    assert_eq!(summary.get("release_parity").unwrap(), "v84.0");

    let mut scx = CachyOsScxCentralLavdV10SchedulerPrEngine::new();
    let scx_msg = scx.schedule_gaming_thread(1024, 16);
    assert!(scx_msg.contains("scx_central_lavd_v10"));

    let mut bolt = CachyOsKernelAutoBoltPgoV10PipelinePrEngine::new();
    assert!(bolt.optimize_kernel_binary());

    let mut chwd = CachyOsChwdVulkanWaylandDirectScanoutV10Engine::new();
    let chwd_msg = chwd.bypass_compositor(101);
    assert!(chwd_msg.contains("surface 101"));

    let pacman = CachyOsPacmanV10PqcDilithiumVerifierEngine::new();
    assert!(pacman.verify_package_signature("test.pkg.tar.zst"));

    let mut fsync = CachyOsProtonWineNtsyncFsync10GovernorEngine::new();
    let fsync_msg = fsync.synchronize_win32_handles(12);
    assert!(fsync_msg.contains("12 Win32 handles"));

    let mut ananicy = CachyOsAnanicyRealtimeEbpfV10GovernorEngine::new();
    let ananicy_msg = ananicy.classify_process("game.exe", 7777);
    assert!(ananicy_msg.contains("PID 7777"));

    let mut btrfs = CachyOsBtrfsZstdEbpfCompressorV10TunerEngine::new();
    assert_eq!(btrfs.tune_compression_ratio(8000), 1);
    assert_eq!(btrfs.tune_compression_ratio(1000), 3);

    let gamescope = CachyOsGamescopeHdr10VrrPassthroughPacerV10Engine::new();
    let frame_msg = gamescope.pace_frame(1200, 144);
    assert!(frame_msg.contains("Frame 1200"));
    assert!(frame_msg.contains("144 Hz"));

    let mut zram = CachyOsZramLz4ZstdDynamicPoolV10Engine::new();
    assert!(zram.allocate_compressed_page(2048));
    assert!(!zram.allocate_compressed_page(8192));

    let mut handheld = CachyOsHandheldPowerCapThermalV10GovernorEngine::new();
    let tdp_msg = handheld.set_tdp_cap(30);
    assert!(tdp_msg.contains("30 Watts"));
    assert_eq!(handheld.tdp_watts, 30);
}

#[test]
fn test_v84_gateway_execution() {
    let gateway = SovereignOpenSourceOsAndCachyLinuxV84Gateway::new();
    assert!(gateway.execute_all_v84_validations());
}
