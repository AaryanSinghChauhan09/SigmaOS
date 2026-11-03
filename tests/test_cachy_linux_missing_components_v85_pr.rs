// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! Integration Test Suite for V85 Cachy Linux Missing Components PR Engine
//! (`tests/test_cachy_linux_missing_components_v85_pr.rs`)

#[path = "../src/distro/cachy_linux_missing_components_v85_pr.rs"]
mod cachy_linux_missing_components_v85_pr;

use cachy_linux_missing_components_v85_pr::*;

#[test]
fn test_cachy_linux_v85_suite_manifests_and_execution() {
    let suite = CachyLinuxMissingComponentsV85PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);

    assert_eq!(manifests[0].pr_number, 8501);
    assert_eq!(
        manifests[0].component_id,
        "cachyos-scx-central-lavd-v11-scheduler"
    );
    assert_eq!(manifests[0].status, CachyOsPrStatusV85::Active);

    assert_eq!(manifests[1].pr_number, 8502);
    assert_eq!(
        manifests[1].component_id,
        "cachyos-kernel-autobolt-pgo-v11-pipeline"
    );

    assert_eq!(manifests[2].pr_number, 8503);
    assert_eq!(
        manifests[2].component_id,
        "cachyos-chwd-vulkan-wayland-direct-scanout-v11"
    );

    assert_eq!(manifests[3].pr_number, 8504);
    assert_eq!(
        manifests[3].component_id,
        "cachyos-pacman-v11-pqc-dilithium-verifier"
    );

    assert_eq!(manifests[4].pr_number, 8505);
    assert_eq!(
        manifests[4].component_id,
        "cachyos-proton-wine-ntsync-fsync11-governor"
    );

    assert_eq!(manifests[5].pr_number, 8506);
    assert_eq!(
        manifests[5].component_id,
        "cachyos-ananicy-realtime-ebpf-v11-governor"
    );

    assert_eq!(manifests[6].pr_number, 8507);
    assert_eq!(
        manifests[6].component_id,
        "cachyos-btrfs-zstd-ebpf-compressor-v11-tuner"
    );

    assert_eq!(manifests[7].pr_number, 8508);
    assert_eq!(
        manifests[7].component_id,
        "cachyos-gamescope-hdr10-vrr-passthrough-pacer-v11"
    );

    assert_eq!(manifests[8].pr_number, 8509);
    assert_eq!(
        manifests[8].component_id,
        "cachyos-zram-lz4-zstd-dynamic-pool-v11"
    );

    assert_eq!(manifests[9].pr_number, 8510);
    assert_eq!(
        manifests[9].component_id,
        "cachyos-handheld-power-cap-thermal-v11-governor"
    );

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_components").unwrap(), "10");
    assert_eq!(summary.get("release_parity").unwrap(), "v85.0");
    assert_eq!(summary.get("gaming_optimizations").unwrap(), "Active");
}

#[test]
fn test_cachy_linux_individual_v85_engine_functionality() {
    let mut scx = CachyOsScxCentralLavdV11SchedulerPrEngine::new();
    let scx_msg = scx.schedule_gaming_thread(2048, 8);
    assert!(scx_msg.contains("scx_central_lavd_v11"));
    assert!(scx_msg.contains("2048"));

    let mut bolt = CachyOsKernelAutoBoltPgoV11PipelinePrEngine::new();
    assert!(bolt.optimize_kernel_binary());
    assert_eq!(bolt.pgo_profiles_processed, 2049);

    let mut chwd = CachyOsChwdVulkanWaylandDirectScanoutV11Engine::new();
    let chwd_msg = chwd.bypass_compositor(77);
    assert!(chwd_msg.contains("surface 77"));

    let pacman = CachyOsPacmanV11PqcDilithiumVerifierEngine::new();
    assert!(pacman.verify_package_signature("linux-cachyos-6.39.pkg.tar.zst"));
    assert!(!pacman.verify_package_signature("invalid_package.deb"));

    let mut fsync = CachyOsProtonWineNtsyncFsync11GovernorEngine::new();
    let fsync_msg = fsync.synchronize_win32_handles(16);
    assert!(fsync_msg.contains("16 Win32 handles"));

    let mut ananicy = CachyOsAnanicyRealtimeEbpfV11GovernorEngine::new();
    let ananicy_msg = ananicy.classify_process("cyberpunk2077.exe", 8888);
    assert!(ananicy_msg.contains("PID 8888"));

    let mut btrfs = CachyOsBtrfsZstdEbpfCompressorV11TunerEngine::new();
    assert_eq!(btrfs.tune_compression_ratio(7500), 1);
    assert_eq!(btrfs.tune_compression_ratio(1200), 3);

    let gamescope = CachyOsGamescopeHdr10VrrPassthroughPacerV11Engine::new();
    let frame_msg = gamescope.pace_frame(3600, 240);
    assert!(frame_msg.contains("Frame 3600"));
    assert!(frame_msg.contains("240 Hz"));

    let mut zram = CachyOsZramLz4ZstdDynamicPoolV11Engine::new();
    assert!(zram.allocate_compressed_page(2048));
    assert!(!zram.allocate_compressed_page(8192));

    let mut handheld = CachyOsHandheldPowerCapThermalV11GovernorEngine::new();
    let tdp_msg = handheld.set_tdp_cap(45);
    assert!(tdp_msg.contains("45 Watts"));
    assert_eq!(handheld.tdp_watts, 45);
}
