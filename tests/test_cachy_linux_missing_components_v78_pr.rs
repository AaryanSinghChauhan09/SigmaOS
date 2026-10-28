// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! Integration tests for CachyOS Missing Components Parity PR Gateway V78
//! (`tests/test_cachy_linux_missing_components_v78_pr.rs`)

#[path = "../src/distro/cachy_linux_missing_components_v78_pr.rs"]
mod cachy_linux_missing_components_v78_pr;

use cachy_linux_missing_components_v78_pr::*;

#[test]
fn test_cachy_v78_master_suite() {
    let suite = CachyLinuxMissingComponentsV78PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);

    for manifest in &manifests {
        assert_eq!(manifest.status, CachyOsPrStatusV78::Proposed);
        assert!(manifest.pr_number >= 7801 && manifest.pr_number <= 7810);
    }

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("suite").map(|s| s.as_str()), Some("CachyLinuxMissingComponentsV78PrSuite"));
    assert_eq!(summary.get("release_parity").map(|s| s.as_str()), Some("v78.0"));
}

#[test]
fn test_cachy_v78_individual_sub_engines() {
    let mut scx = CachyOsScxCentralLavdV8SchedulerPrEngine::new();
    let scx_out = scx.schedule_gaming_thread(1234, 4);
    assert!(scx_out.contains("Thread 1234"));

    let mut bolt = CachyOsKernelAutoBoltPgoV8PipelinePrEngine::new();
    assert!(bolt.optimize_kernel_binary());

    let mut scanout = CachyOsChwdVulkanWaylandDirectScanoutV8Engine::new();
    let scanout_out = scanout.bypass_compositor(42);
    assert!(scanout_out.contains("surface 42"));

    let verifier = CachyOsPacmanV8PqcDilithiumVerifierEngine::new();
    assert!(verifier.verify_package_signature("test.pkg.tar.zst"));
    assert!(!verifier.verify_package_signature("test.txt"));

    let mut fsync = CachyOsProtonWineNtsyncFsync8GovernorEngine::new();
    let fsync_out = fsync.synchronize_win32_handles(10);
    assert!(fsync_out.contains("10 Win32 primitive handles"));

    let mut ananicy = CachyOsAnanicyRealtimeEbpfV8GovernorEngine::new();
    let ananicy_out = ananicy.classify_process("cyberpunk", 9999);
    assert!(ananicy_out.contains("cyberpunk"));

    let mut btrfs = CachyOsBtrfsZstdEbpfCompressorV8TunerEngine::new();
    assert_eq!(btrfs.tune_compression_ratio(10000), 1);
    assert_eq!(btrfs.tune_compression_ratio(1000), 3);

    let pacer = CachyOsGamescopeHdr10VrrPassthroughPacerV8Engine::new();
    let pacer_out = pacer.pace_frame(100, 144);
    assert!(pacer_out.contains("Frame 100"));

    let mut zram = CachyOsZramLz4ZstdDynamicPoolV8Engine::new();
    assert!(zram.allocate_compressed_page(2048));
    assert!(!zram.allocate_compressed_page(8192));

    let mut handheld = CachyOsHandheldPowerCapThermalV8GovernorEngine::new();
    let handheld_out = handheld.set_tdp_cap(28);
    assert!(handheld_out.contains("28 Watts"));
}
