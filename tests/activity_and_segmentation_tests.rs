// Comprehensive Integration & Subsystem Tests for Activity Manager and Segmentation/Paging Subsystem

extern crate alloc;

#[path = "../src/klib/mod.rs"]
pub mod klib;
#[path = "../src/process/activity_manager.rs"]
mod activity_manager;
#[path = "../src/memory/segmentation_paging.rs"]
mod segmentation_paging;

use activity_manager::*;
use segmentation_paging::*;

#[test]
fn test_activity_manager_complete_suite() {
    let mut am = ActivityManager::new();

    // 1. Register processes
    am.register_process(1, 0, "systemd", -20);
    am.register_process(100, 1, "zenith_compositor", -5);
    am.register_process(200, 1, "firefox", 0);
    am.register_process(300, 1, "cargo_build", 10);

    // 2. Set foreground process
    am.set_foreground_process(200).unwrap();
    let ff = am.get_process_activity(200).unwrap();
    assert!(ff.is_foreground);
    assert_eq!(ff.state, ActivityState::Interactive);
    assert_eq!(ff.priority, -5); // Interactivity boost applied

    // 3. Update activity metrics
    am.update_activity_metrics(300, 95.0, 512 * 1024 * 1024, 1048576, 524288, 5000)
        .unwrap();
    let build = am.get_process_activity(300).unwrap();
    assert_eq!(build.state, ActivityState::Active);
    assert_eq!(build.cpu_usage_pct, 95.0);

    // 4. Capture register snapshots and address space bindings
    let snapshot = RegisterSnapshot {
        rax: 0xDEADBEEF,
        rsp: 0x0000_7FFF_FFFF_E000,
        rip: 0x0000_7FFF_0000_1000,
        cs: 0x2b,
        ..Default::default()
    };
    am.capture_register_snapshot(200, snapshot).unwrap();

    am.bind_address_space(
        200,
        "/usr/bin/firefox",
        0x0000_7FFF_0000_0000,
        0x500000,
        0x100000,
        true,
    )
    .unwrap();
    am.add_bound_library(200, "libgtk-3.so").unwrap();
    am.add_bound_library(200, "libwayland-client.so").unwrap();

    let ff_record = am.get_process_activity(200).unwrap();
    assert_eq!(ff_record.register_snapshot.unwrap().rax, 0xDEADBEEF);
    let binding = ff_record.address_binding.as_ref().unwrap();
    assert_eq!(binding.bound_libraries.len(), 2);

    // 5. Power throttling & idle background reclamation
    am.register_process(400, 1, "background_indexer", 5);
    am.set_power_throttling(400, true).unwrap();
    let indexer = am.get_process_activity(400).unwrap();
    assert_eq!(indexer.state, ActivityState::Throttled);

    am.update_activity_metrics(400, 0.0, 1024 * 1024, 0, 0, 1000)
        .unwrap();
    let reclaimed = am.reclaim_background_activity(10000, 3000);
    assert!(reclaimed.contains(&400));

    let summary = am.summary();
    assert!(summary.contains("System Activity Manager"));
}

#[test]
fn test_segmentation_paging_and_protection_suite() {
    let engine = SegmentationPagingEngine::new(SpaceProtectionFlags::strict_hardening());

    // 1. Logical to Linear Address Translation
    let ring0_cs = SegmentSelector::new(1, false, CpuRing::Ring0Kernel);
    let linear0 = engine
        .translate_logical_to_linear(ring0_cs, 0x4000, CpuRing::Ring0Kernel)
        .unwrap();
    assert_eq!(linear0, 0x4000);

    let ring3_cs = SegmentSelector::new(3, false, CpuRing::Ring3User);
    let linear3 = engine
        .translate_logical_to_linear(ring3_cs, 0x8000, CpuRing::Ring3User)
        .unwrap();
    assert_eq!(linear3, 0x8000);

    // GP Fault on DPL privilege violation
    let gp_fault = engine.translate_logical_to_linear(ring0_cs, 0x4000, CpuRing::Ring3User);
    assert!(gp_fault.is_err());

    // 2. Full Translation Walk & Hardware Space Protection Checks (SMEP, SMAP, W^X)
    let (lin, phys) = engine
        .full_address_translation_walk(
            ring3_cs,
            0x0000_0000_0001_0000,
            false,
            false,
            CpuRing::Ring3User,
        )
        .unwrap();

    assert_eq!(lin, 0x0000_0000_0001_0000);
    assert!(phys >= 0x2000_0000);

    // SMEP: Ring 0 Kernel executing User-space page -> Fault
    let smep_err = engine.translate_virtual_to_physical(
        0x0000_0000_0001_0000,
        false,
        true,
        CpuRing::Ring0Kernel,
    );
    assert!(smep_err.is_err());

    // SMAP: Ring 0 Kernel accessing User-space page data -> Fault
    let smap_err = engine.translate_virtual_to_physical(
        0x0000_0000_0001_0000,
        false,
        false,
        CpuRing::Ring0Kernel,
    );
    assert!(smap_err.is_err());

    // W^X: Simultaneous Write and Execute -> Security Violation
    let wx_err =
        engine.translate_virtual_to_physical(0x0000_7FFF_0000_0000, true, true, CpuRing::Ring3User);
    assert!(wx_err.is_err());

    // 3. ASLR Layout Entropy
    let config = AslrEntropyConfig::linux_default();
    let aslr_layout =
        RandomizedAddressSpace::compute_aslr_layout(0x0000_0000_0040_0000, config, 0xABCD1234);
    assert!(aslr_layout.text_base >= 0x0000_0000_0040_0000);
    assert!(aslr_layout.stack_top <= 0x0000_7FFF_FFFF_0000);
}
