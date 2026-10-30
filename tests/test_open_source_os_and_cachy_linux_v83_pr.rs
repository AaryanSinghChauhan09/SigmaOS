// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! Integration Test Suite for V83 Open Source OS Unimplemented Ideas & Cachy Linux Missing Components PR Engine
//! (`tests/test_open_source_os_and_cachy_linux_v83_pr.rs`)

use sigmaos::distro::cachy_linux_missing_components_v83_pr::*;
use sigmaos::distro::open_source_os_and_cachy_linux_v83_pr::*;
use sigmaos::distro::open_source_os_unimplemented_ideas_v83_pr::*;

#[test]
fn test_open_source_os_v83_engines_and_manifests() {
    let suite = SovereignUnimplementedIdeasSuiteV83::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 9);

    assert_eq!(manifests[0].pr_id, "PR-SIGMA-V83-01-LINUX-637");
    assert!(manifests[0].title.contains("Linux 6.37+"));
    assert_eq!(manifests[0].status, OsPrStatusV83::Validated);

    assert_eq!(manifests[1].pr_id, "PR-SIGMA-V83-02-FREEBSD-167");
    assert!(manifests[1].title.contains("FreeBSD 16.7+"));

    assert_eq!(manifests[2].pr_id, "PR-SIGMA-V83-03-OPENBSD-96");
    assert!(manifests[2].title.contains("OpenBSD 9.6+"));

    assert_eq!(manifests[3].pr_id, "PR-SIGMA-V83-04-DRAGONFLY-85");
    assert!(manifests[3].title.contains("DragonFly BSD 8.5+"));

    assert_eq!(manifests[4].pr_id, "PR-SIGMA-V83-05-NETBSD-132");
    assert!(manifests[4].title.contains("NetBSD 13.2"));

    assert_eq!(manifests[5].pr_id, "PR-SIGMA-V83-06-ILLUMOS-DTRACE");
    assert!(manifests[5].title.contains("Illumos"));

    assert_eq!(manifests[6].pr_id, "PR-SIGMA-V83-07-MICROKERNEL-SCHEMES");
    assert!(manifests[6].title.contains("Haiku BFS"));

    assert_eq!(manifests[7].pr_id, "PR-SIGMA-V83-08-TECH-MEDIA");
    assert!(manifests[7].title.contains("Global Tech Media"));

    assert_eq!(manifests[8].pr_id, "PR-SIGMA-V83-09-OMARCHY-NAVIGATOR");
    assert!(manifests[8].title.contains("Omarchy Hyprland"));
}

#[test]
fn test_cachy_linux_v83_engines_and_manifests() {
    let suite = CachyLinuxMissingComponentsV83PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);

    assert_eq!(manifests[0].pr_number, 8301);
    assert_eq!(
        manifests[0].component_id,
        "cachyos-scx-central-lavd-v9-scheduler"
    );
    assert_eq!(manifests[0].status, CachyOsPrStatusV83::Active);

    assert_eq!(manifests[1].pr_number, 8302);
    assert_eq!(
        manifests[1].component_id,
        "cachyos-kernel-autobolt-pgo-v9-pipeline"
    );

    assert_eq!(manifests[2].pr_number, 8303);
    assert_eq!(
        manifests[2].component_id,
        "cachyos-chwd-vulkan-wayland-direct-scanout-v9"
    );

    assert_eq!(manifests[3].pr_number, 8304);
    assert_eq!(
        manifests[3].component_id,
        "cachyos-pacman-v9-pqc-dilithium-verifier"
    );

    assert_eq!(manifests[4].pr_number, 8305);
    assert_eq!(
        manifests[4].component_id,
        "cachyos-proton-wine-ntsync-fsync9-governor"
    );

    assert_eq!(manifests[5].pr_number, 8306);
    assert_eq!(
        manifests[5].component_id,
        "cachyos-ananicy-realtime-ebpf-v9-governor"
    );

    assert_eq!(manifests[6].pr_number, 8307);
    assert_eq!(
        manifests[6].component_id,
        "cachyos-btrfs-zstd-ebpf-compressor-v9-tuner"
    );

    assert_eq!(manifests[7].pr_number, 8308);
    assert_eq!(
        manifests[7].component_id,
        "cachyos-gamescope-hdr10-vrr-passthrough-pacer-v9"
    );

    assert_eq!(manifests[8].pr_number, 8309);
    assert_eq!(
        manifests[8].component_id,
        "cachyos-zram-lz4-zstd-dynamic-pool-v9"
    );

    assert_eq!(manifests[9].pr_number, 8310);
    assert_eq!(
        manifests[9].component_id,
        "cachyos-handheld-power-cap-thermal-v9-governor"
    );

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_components").unwrap(), "10");
    assert_eq!(summary.get("release_parity").unwrap(), "v83.0");
}

#[test]
fn test_cachy_linux_individual_v83_engine_functionality() {
    let mut scx = CachyOsScxCentralLavdV9SchedulerPrEngine::new();
    let scx_msg = scx.schedule_gaming_thread(1024, 4);
    assert!(scx_msg.contains("scx_central_lavd_v9"));
    assert!(scx_msg.contains("1024"));

    let mut bolt = CachyOsKernelAutoBoltPgoV9PipelinePrEngine::new();
    assert!(bolt.optimize_kernel_binary());
    assert_eq!(bolt.pgo_profiles_processed, 513);

    let mut chwd = CachyOsChwdVulkanWaylandDirectScanoutV9Engine::new();
    let chwd_msg = chwd.bypass_compositor(42);
    assert!(chwd_msg.contains("surface 42"));

    let pacman = CachyOsPacmanV9PqcDilithiumVerifierEngine::new();
    assert!(pacman.verify_package_signature("linux-cachyos-6.37.pkg.tar.zst"));
    assert!(!pacman.verify_package_signature("invalid_package.deb"));

    let mut fsync = CachyOsProtonWineNtsyncFsync9GovernorEngine::new();
    let fsync_msg = fsync.synchronize_win32_handles(12);
    assert!(fsync_msg.contains("12 Win32 handles"));

    let mut ananicy = CachyOsAnanicyRealtimeEbpfV9GovernorEngine::new();
    let ananicy_msg = ananicy.classify_process("cyberpunk2077.exe", 9999);
    assert!(ananicy_msg.contains("PID 9999"));

    let mut btrfs = CachyOsBtrfsZstdEbpfCompressorV9TunerEngine::new();
    assert_eq!(btrfs.tune_compression_ratio(8000), 1);
    assert_eq!(btrfs.tune_compression_ratio(1000), 3);

    let gamescope = CachyOsGamescopeHdr10VrrPassthroughPacerV9Engine::new();
    let frame_msg = gamescope.pace_frame(1200, 144);
    assert!(frame_msg.contains("Frame 1200"));
    assert!(frame_msg.contains("144 Hz"));

    let mut zram = CachyOsZramLz4ZstdDynamicPoolV9Engine::new();
    assert!(zram.allocate_compressed_page(2048));
    assert!(!zram.allocate_compressed_page(8192));

    let mut handheld = CachyOsHandheldPowerCapThermalV9GovernorEngine::new();
    let tdp_msg = handheld.set_tdp_cap(30);
    assert!(tdp_msg.contains("30 Watts"));
    assert_eq!(handheld.tdp_watts, 30);
}

#[test]
fn test_v83_gateway_execution() {
    let gateway = SovereignOpenSourceOsAndCachyLinuxV83Gateway::new();
    assert!(gateway.execute_all_v83_validations());
}
