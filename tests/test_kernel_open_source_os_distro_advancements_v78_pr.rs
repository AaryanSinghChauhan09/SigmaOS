// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Kernel Open Source OS & Distro Advancements V78 PR Gateway
// (`tests/test_kernel_open_source_os_distro_advancements_v78_pr.rs`)

#[path = "../src/kernel/sovereign_kernel_open_source_os_distro_advancements_v78_pr.rs"]
mod sovereign_kernel_open_source_os_distro_advancements_v78_pr;

use sovereign_kernel_open_source_os_distro_advancements_v78_pr::*;

#[test]
fn test_kernel_open_source_os_distro_advancements_v78_pr_full_cycle() {
    let mut gateway = SovereignKernelOpenSourceOsDistroAdvancementsV78PrGateway::new();

    // 1. Linux 6.32+ BPF sched_ext EEVDF & BORE
    gateway.eevdf_bore.register_task(9001, "audio_daemon", 98);
    gateway.eevdf_bore.register_task(9002, "compilation_worker", 10);
    let burst_a = gateway.eevdf_bore.update_task_burst(9001, 100_000);
    let burst_b = gateway.eevdf_bore.update_task_burst(9002, 100_000);
    assert!(burst_a > 0);
    assert!(burst_b > burst_a);
    let chosen_pid = gateway.eevdf_bore.select_next_eligible_task();
    assert!(chosen_pid.is_some());

    // 2. FreeBSD 16.2+ OpenZFS ARC & GEOM Async VDEV
    assert!(gateway.zfs_arc_vdev.allocate_arc_buffer(101, 2048 * 1024));
    let flushes = gateway.zfs_arc_vdev.flush_async_vdev_queue();
    assert_eq!(flushes, 1);

    // 3. OpenBSD 9.1+ Pinsyscall KARL FineIBT & Retguard
    gateway.pinsyscall_karl.pin_syscall_entry(0xFFFFFFFF8000A000);
    let ret_addr = 0xFFFFFFFF8000A000;
    let canary = ret_addr ^ 0x2026_8008_1337_0001;
    assert!(gateway.pinsyscall_karl.verify_retguard_canary(ret_addr, canary));
    gateway.pinsyscall_karl.set_pledge_promises(0x1F);
    assert_eq!(gateway.pinsyscall_karl.pledge_promises, 0x1F);

    // 4. DragonFly BSD 8.0+ HAMMER3 CoW
    let gen = gateway.hammer3_cow.create_pfs_snapshot(5, "pfs_data_v78", 50005);
    assert!(gateway.hammer3_cow.perform_atomic_rollback(5, gen));

    // 5. NetBSD 12.7 Rump Kernel & Veriexec V4
    let hash = [0xC2u8; 32];
    gateway.rump_veriexec.register_rump_driver("rumpdev_virtio");
    gateway.rump_veriexec.register_veriexec_hash("/bin/sh", hash);
    assert!(gateway.rump_veriexec.verify_and_exec_hypercall("rumpdev_virtio", &hash));

    // 6. Illumos Crossbow VNIC & Zone
    gateway.crossbow_zone.register_dtrace_probe();
    assert!(gateway.crossbow_zone.evaluate_quota(1024 * 1024, 1000));

    // 7. Haiku / Redox OS Scheme Capabilities
    gateway.scheme_caps.register_scheme("file_v78", 0b0111);
    assert!(gateway.scheme_caps.dispatch_scheme_request("file_v78", 0b0011));

    // 8. Plan 9 / NixOS 9P2000.L & Merkle CAS Store
    let merkle_hash = [0xDDu8; 32];
    gateway.plan9_merkle.mount_9p_channel("plan9_chan");
    gateway.plan9_merkle.store_merkle_closure("/nix/store/pkg-v78", merkle_hash);
    assert!(gateway.plan9_merkle.verify_and_fetch("plan9_chan", &merkle_hash));

    // 9. Master PR Proposal Manifest
    let manifest = gateway.generate_advancements_v78_pr_manifest();
    assert_eq!(manifest.pr_id, "PR-KERNEL-OPEN-SOURCE-OS-DISTRO-ADVANCEMENTS-V78");
    assert_eq!(manifest.subsystems_affected.len(), 8);
    assert_eq!(manifest.status, "Approved and Ready for Merge");
    assert!(manifest.pqc_signature.contains("DILITHIUM5_SIG"));
}
