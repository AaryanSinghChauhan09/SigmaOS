// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! Integration Test Suite for V86 Cachy Linux Missing Components PR Engine
//! (`tests/test_cachy_linux_missing_components_v86_pr.rs`)

#[path = "../src/distro/cachy_linux_missing_components_v86_pr.rs"]
mod cachy_linux_missing_components_v86_pr;

use cachy_linux_missing_components_v86_pr::*;

#[test]
fn test_cachy_linux_v86_suite_manifests_and_execution() {
    let suite = CachyLinuxMissingComponentsV86PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);

    assert_eq!(manifests[0].pr_number, 8601);
    assert_eq!(
        manifests[0].component_id,
        "cachyos-scx-central-lavd-v12-scheduler"
    );
    assert_eq!(manifests[0].status, CachyOsPrStatusV86::Active);

    assert_eq!(manifests[1].pr_number, 8602);
    assert_eq!(
        manifests[1].component_id,
        "cachyos-kernel-autobolt-pgo-v12-pipeline"
    );

    assert_eq!(manifests[2].pr_number, 8603);
    assert_eq!(
        manifests[2].component_id,
        "cachyos-chwd-vulkan-wayland-direct-scanout-v12"
    );

    assert_eq!(manifests[3].pr_number, 8604);
    assert_eq!(
        manifests[3].component_id,
        "cachyos-pacman-v12-pqc-dilithium-verifier"
    );

    assert_eq!(manifests[4].pr_number, 8605);
    assert_eq!(
        manifests[4].component_id,
        "cachyos-proton-wine-ntsync-fsync12-governor"
    );

    assert_eq!(manifests[5].pr_number, 8606);
    assert_eq!(
        manifests[5].component_id,
        "cachyos-ananicy-realtime-ebpf-v12-governor"
    );

    assert_eq!(manifests[6].pr_number, 8607);
    assert_eq!(
        manifests[6].component_id,
        "cachyos-btrfs-zstd-ebpf-compressor-v12-tuner"
    );

    assert_eq!(manifests[7].pr_number, 8608);
    assert_eq!(
        manifests[7].component_id,
        "cachyos-gamescope-hdr10-vrr-passthrough-pacer-v12"
    );

    assert_eq!(manifests[8].pr_number, 8609);
    assert_eq!(
        manifests[8].component_id,
        "cachyos-zram-lz4-zstd-dynamic-pool-v12"
    );

    assert_eq!(manifests[9].pr_number, 8610);
    assert_eq!(
        manifests[9].component_id,
        "cachyos-handheld-power-cap-thermal-v12-governor"
    );

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_components").unwrap(), "10");
    assert_eq!(summary.get("release_parity").unwrap(), "v86.0");
    assert_eq!(summary.get("gaming_optimizations").unwrap(), "Active");
}

#[test]
fn test_cachy_linux_individual_v86_engine_functionality() {
    let mut scx = CachyOsScxCentralLavdV12SchedulerPrEngine::new();
    let scx_msg = scx.schedule_gaming_thread(2048, 8);
    assert!(scx_msg.contains("scx_central_lavd_v12"));
    assert!(scx_msg.contains("2048"));

    let mut bolt = CachyOsKernelAutoBoltPgoV12PipelinePrEngine::new();
    assert!(bolt.optimize_kernel_binary());
    assert_eq!(bolt.pgo_profiles_processed, 2049);

    let mut chwd = CachyOsChwdVulkanWaylandDirectScanoutV12Engine::new();
    let chwd_msg = chwd.bypass_compositor(77);
    assert!(chwd_msg.contains("surface 77"));

    let pacman = CachyOsPacmanV12PqcDilithiumVerifierEngine::new();
    assert!(pacman.verify_package_signature("linux-cachyos-6.40.pkg.tar.zst"));
    assert!(!pacman.verify_package_signature("invalid_package.deb"));

    let mut fsync = CachyOsProtonWineNtsyncFsync12GovernorEngine::new();
    let fsync_msg = fsync.synchronize_win32_handles(16);
    assert!(fsync_msg.contains("16 Win32 handles"));

    let mut ananicy = CachyOsAnanicyRealtimeEbpfV12GovernorEngine::new();
    let ananicy_msg = ananicy.classify_process("cyberpunk2077.exe", 8888);
    assert!(ananicy_msg.contains("PID 8888"));

    let mut btrfs = CachyOsBtrfsZstdEbpfCompressorV12TunerEngine::new();
    assert_eq!(btrfs.tune_compression_ratio(7500), 1);
    assert_eq!(btrfs.tune_compression_ratio(1200), 3);

    let gamescope = CachyOsGamescopeHdr10VrrPassthroughPacerV12Engine::new();
    let frame_msg = gamescope.pace_frame(3600, 240);
    assert!(frame_msg.contains("Frame 3600"));
    assert!(frame_msg.contains("240 Hz"));

    let mut zram = CachyOsZramLz4ZstdDynamicPoolV12Engine::new();
    assert!(zram.allocate_compressed_page(2048));
    assert!(!zram.allocate_compressed_page(8192));

    let mut handheld = CachyOsHandheldPowerCapThermalV12GovernorEngine::new();
    let tdp_msg = handheld.set_tdp_cap(45);
    assert!(tdp_msg.contains("45 Watts"));
    assert_eq!(handheld.tdp_watts, 45);
}
