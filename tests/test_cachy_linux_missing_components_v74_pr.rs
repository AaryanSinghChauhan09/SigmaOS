// SPDX-License-Identifier: MIT
// Integration Test Suite for CachyOS Missing Components V74 PR Engine
// (`tests/test_cachy_linux_missing_components_v74_pr.rs`)

#[path = "../src/distro/cachy_linux_missing_components_v74_pr.rs"]
mod cachy_linux_missing_components_v74_pr;

use cachy_linux_missing_components_v74_pr::*;

#[test]
fn test_cachy_linux_v74_pr_suite_full_cycle() {
    let suite = CachyLinuxMissingComponentsV74PrSuite::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 10);

    for (idx, manifest) in manifests.iter().enumerate() {
        assert_eq!(manifest.pr_number, (7401 + idx) as u32);
        assert_eq!(manifest.status, CachyOsPrStatusV74::Proposed);
        assert!(!manifest.title.is_empty());
        assert!(!manifest.component_id.is_empty());
        assert!(!manifest.patch_description.is_empty());
        assert!(!manifest.diff_summary.is_empty());
    }

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_components").unwrap(), "10");
    assert_eq!(summary.get("release_parity").unwrap(), "v74.0");

    // Test individual components
    let mut scx = CachyOsScxCentralLavdV6SchedulerPrEngine::new();
    let sched_res = scx.schedule_gaming_thread(101, 4);
    assert!(sched_res.contains("pinned to CPU core 4"));

    let mut bolt = CachyOsKernelAutoBoltPgoV6PipelinePrEngine::new();
    assert!(bolt.optimize_kernel_binary());

    let mut chwd = CachyOsChwdVulkanWaylandDirectScanoutV6Engine::new();
    let scanout_res = chwd.bypass_compositor(42);
    assert!(scanout_res.contains("Wayland surface 42"));

    let pacman = CachyOsPacmanV6PqcDilithiumVerifierEngine::new();
    assert!(pacman.verify_package_signature("kernel-cachyos-6.31.pkg.tar.zst"));

    let mut fsync = CachyOsProtonWineNtsyncFsync6GovernorEngine::new();
    let sync_res = fsync.synchronize_win32_handles(12);
    assert!(sync_res.contains("12 Win32 primitive handles"));

    let mut ananicy = CachyOsAnanicyRealtimeEbpfV6GovernorEngine::new();
    let cls_res = ananicy.classify_process("cyberpunk.exe", 9999);
    assert!(cls_res.contains("PID 9999"));

    let mut btrfs = CachyOsBtrfsZstdEbpfCompressorV6TunerEngine::new();
    assert_eq!(btrfs.tune_compression_ratio(8000), 1);

    let gamescope = CachyOsGamescopeHdr10VrrPassthroughPacerV6Engine::new();
    let pace_res = gamescope.pace_frame(1001, 144);
    assert!(pace_res.contains("144 Hz"));

    let mut zram = CachyOsZramLz4ZstdDynamicPoolV6Engine::new();
    assert!(zram.allocate_compressed_page(2048));

    let mut handheld = CachyOsHandheldPowerCapThermalV6GovernorEngine::new();
    let tdp_res = handheld.set_tdp_cap(25);
    assert!(tdp_res.contains("25 Watts"));
}
