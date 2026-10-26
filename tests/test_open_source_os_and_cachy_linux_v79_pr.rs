// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! Integration Test Suite for V79 Open Source OS Unimplemented Ideas & Cachy Linux Missing Components PR Engine
//! (`tests/test_open_source_os_and_cachy_linux_v79_pr.rs`)

#[path = "../src/distro/open_source_os_and_cachy_linux_v79_pr.rs"]
mod open_source_os_and_cachy_linux_v79_pr;

use open_source_os_and_cachy_linux_v79_pr::*;

#[test]
fn test_open_source_os_v79_engines_and_manifests() {
    let suite = OpenSourceOsUnimplementedIdeasV79PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 5);

    assert_eq!(manifests[0].pr_id, "PR-SIGMA-V79-OS-01");
    assert!(manifests[0].title.contains("Linux 6.33+"));
    assert_eq!(manifests[0].status, OpenSourceOsPrStatusV79::Validated);

    assert_eq!(manifests[1].pr_id, "PR-SIGMA-V79-OS-02");
    assert!(manifests[1].title.contains("FreeBSD 16.3+"));

    assert_eq!(manifests[2].pr_id, "PR-SIGMA-V79-OS-03");
    assert!(manifests[2].title.contains("OpenBSD 9.2+"));

    assert_eq!(manifests[3].pr_id, "PR-SIGMA-V79-OS-04");
    assert!(manifests[3].title.contains("DragonFly BSD 8.1+"));

    assert_eq!(manifests[4].pr_id, "PR-SIGMA-V79-OS-05");
    assert!(manifests[4].title.contains("NetBSD 12.8"));
}

#[test]
fn test_cachy_linux_v79_engines_and_manifests() {
    let suite = CachyLinuxMissingComponentsV79PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);

    assert_eq!(manifests[0].pr_number, 7901);
    assert_eq!(manifests[0].component_id, "cachyos-scx-central-lavd-v7-scheduler");
    assert_eq!(manifests[0].status, CachyOsPrStatusV79::Active);

    assert_eq!(manifests[1].pr_number, 7902);
    assert_eq!(manifests[1].component_id, "cachyos-kernel-autobolt-pgo-v7-pipeline");

    assert_eq!(manifests[2].pr_number, 7903);
    assert_eq!(manifests[2].component_id, "cachyos-chwd-vulkan-wayland-direct-scanout-v7");

    assert_eq!(manifests[3].pr_number, 7904);
    assert_eq!(manifests[3].component_id, "cachyos-pacman-v7-pqc-dilithium-verifier");

    assert_eq!(manifests[4].pr_number, 7905);
    assert_eq!(manifests[4].component_id, "cachyos-proton-wine-ntsync-fsync7-governor");

    assert_eq!(manifests[5].pr_number, 7906);
    assert_eq!(manifests[5].component_id, "cachyos-ananicy-realtime-ebpf-v7-governor");

    assert_eq!(manifests[6].pr_number, 7907);
    assert_eq!(manifests[6].component_id, "cachyos-btrfs-zstd-ebpf-compressor-v7-tuner");

    assert_eq!(manifests[7].pr_number, 7908);
    assert_eq!(manifests[7].component_id, "cachyos-gamescope-hdr10-vrr-passthrough-pacer-v7");

    assert_eq!(manifests[8].pr_number, 7909);
    assert_eq!(manifests[8].component_id, "cachyos-zram-lz4-zstd-dynamic-pool-v7");

    assert_eq!(manifests[9].pr_number, 7910);
    assert_eq!(manifests[9].component_id, "cachyos-handheld-power-cap-thermal-v7-governor");

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_components").unwrap(), "10");
    assert_eq!(summary.get("release_parity").unwrap(), "v79.0");
}

#[test]
fn test_cachy_linux_individual_engine_functionality() {
    let mut scx = CachyOsScxCentralLavdV7SchedulerPrEngine::new();
    let scx_msg = scx.schedule_gaming_thread(1024, 4);
    assert!(scx_msg.contains("scx_central_lavd_v7"));
    assert!(scx_msg.contains("1024"));

    let mut bolt = CachyOsKernelAutoBoltPgoV7PipelinePrEngine::new();
    assert!(bolt.optimize_kernel_binary());
    assert_eq!(bolt.pgo_profiles_processed, 257);

    let mut chwd = CachyOsChwdVulkanWaylandDirectScanoutV7Engine::new();
    let chwd_msg = chwd.bypass_compositor(42);
        assert!(chwd_msg.contains("surface 42"));

    let pacman = CachyOsPacmanV7PqcDilithiumVerifierEngine::new();
    assert!(pacman.verify_package_signature("linux-cachyos-6.33.pkg.tar.zst"));
    assert!(!pacman.verify_package_signature("invalid_package.deb"));

    let mut fsync = CachyOsProtonWineNtsyncFsync7GovernorEngine::new();
    let fsync_msg = fsync.synchronize_win32_handles(12);
    assert!(fsync_msg.contains("12 Win32 handles"));

    let mut ananicy = CachyOsAnanicyRealtimeEbpfV7GovernorEngine::new();
    let ananicy_msg = ananicy.classify_process("cyberpunk2077.exe", 9999);
    assert!(ananicy_msg.contains("PID 9999"));

    let mut btrfs = CachyOsBtrfsZstdEbpfCompressorV7TunerEngine::new();
    assert_eq!(btrfs.tune_compression_ratio(8000), 1);
    assert_eq!(btrfs.tune_compression_ratio(1000), 3);

    let gamescope = CachyOsGamescopeHdr10VrrPassthroughPacerV7Engine::new();
    let frame_msg = gamescope.pace_frame(1200, 144);
    assert!(frame_msg.contains("Frame 1200"));
    assert!(frame_msg.contains("144 Hz"));

    let mut zram = CachyOsZramLz4ZstdDynamicPoolV7Engine::new();
    assert!(zram.allocate_compressed_page(2048));
    assert!(!zram.allocate_compressed_page(8192));

    let mut handheld = CachyOsHandheldPowerCapThermalV7GovernorEngine::new();
    let tdp_msg = handheld.set_tdp_cap(30);
    assert!(tdp_msg.contains("30 Watts"));
    assert_eq!(handheld.tdp_watts, 30);
}
