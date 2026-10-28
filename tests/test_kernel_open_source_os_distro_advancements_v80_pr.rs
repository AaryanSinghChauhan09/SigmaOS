//! Test Suite for Sovereign Kernel Open Source OS & Distro Advancements V80 PR Engine

#[path = "../src/kernel/sovereign_kernel_open_source_os_distro_advancements_v80_pr.rs"]
mod sovereign_kernel_open_source_os_distro_advancements_v80_pr;

use sovereign_kernel_open_source_os_distro_advancements_v80_pr::*;

#[test]
fn test_v80_kernel_pr_gateway_manifest() {
    let mut gateway = SovereignKernelOpenSourceOsDistroAdvancementsV80PrGateway::new();
    let manifest = gateway.generate_advancements_v80_pr_manifest();

    assert_eq!(manifest.pr_id, "PR-KERNEL-OPEN-SOURCE-OS-DISTRO-ADVANCEMENTS-V80");
    assert!(manifest.title.contains("V80 Engine"));
    assert_eq!(manifest.subsystems_affected.len(), 8);
    assert_eq!(manifest.pqc_signature, "DILITHIUM5_SIG_V80_KERNEL_OPEN_SOURCE_DISTRO_ADVANCEMENTS_SUCCESS");
    assert_eq!(manifest.status, "Approved and Ready for Merge");
}

#[test]
fn test_v80_eevdf_bore_scheduler() {
    let mut sched = LinuxSchedExtEevdfBoreEngineV80::new();
    sched.register_task(201, "zenith-compositor", 90);
    sched.register_task(202, "background-indexer", 10);

    let b1 = sched.update_task_burst(201, 100_000);
    let b2 = sched.update_task_burst(202, 100_000);
    assert!(b2 > b1);

    let chosen = sched.select_next_eligible_task();
    assert_eq!(chosen, Some(201));
}

#[test]
fn test_v80_freebsd_arc_vdev() {
    let mut arc = FreeBsdZfsArcGeomAsyncEngineV80::new(5000);
    assert!(arc.allocate_arc_buffer(10, 3000));
    assert!(arc.allocate_arc_buffer(11, 3000));
    assert!(arc.current_arc_bytes <= 5000);
    assert_eq!(arc.flush_async_vdev_queue(), 1);
}

#[test]
fn test_v80_openbsd_retguard() {
    let mut retguard = OpenBsdPinsyscallKarlRetguardEngineV80::new(0xABCD, 0x1234);
    retguard.pin_syscall_entry(0x8000);
    let ret = 0x8000;
    let cookie = ret ^ 0x1234;
    assert!(retguard.verify_retguard_canary(ret, cookie));
    assert!(!retguard.verify_retguard_canary(ret, 0x9999));
}

#[test]
fn test_v80_dragonfly_hammer3() {
    let mut hammer = DragonFlyHammer3EngineV80::new();
    let gen = hammer.create_pfs_snapshot(1, "snap_root", 1000);
    assert_eq!(gen, 1);
    assert!(hammer.perform_atomic_rollback(1, gen));
}

#[test]
fn test_v80_netbsd_rump_veriexec() {
    let mut rump = NetBsdRumpVeriexecEngineV80::new();
    let hash = [0x55u8; 32];
    rump.register_rump_driver("rumpdev_virtio");
    rump.register_veriexec_hash("/bin/sh", hash);
    assert!(rump.verify_and_exec_hypercall("rumpdev_virtio", &hash));
}

#[test]
fn test_v80_illumos_crossbow() {
    let mut illumos = IllumosCrossbowZoneEngineV80::new("zone_v80", 10000, 1024 * 1024, 1024);
    illumos.register_dtrace_probe();
    assert!(illumos.evaluate_quota(512 * 1024, 5000));
}

#[test]
fn test_v80_haiku_redox_scheme() {
    let mut scheme = HaikuRedoxSchemeEngineV80::new();
    scheme.register_scheme("v80_ipc", 0b111);
    assert!(scheme.dispatch_scheme_request("v80_ipc", 0b011));
}

#[test]
fn test_v80_plan9_nixos_merkle() {
    let mut merkle = Plan9NixOsMerkleEngineV80::new();
    let hash = [0x77u8; 32];
    merkle.mount_9p_channel("channel_v80");
    merkle.store_merkle_closure("/store/item", hash);
    assert!(merkle.verify_and_fetch("channel_v80", &hash));
}
