// SPDX-License-Identifier: MIT
// Integration tests for DragonFly BSD Missing Components V70 PR Suite
// (`tests/test_dragonflybsd_missing_components_v70_pr.rs`)

extern crate alloc;

#[path = "../src/distro/dragonflybsd_missing_components_v70_pr.rs"]
mod dragonflybsd_missing_components_v70_pr;

use dragonflybsd_missing_components_v70_pr::*;

#[test]
fn test_integration_dragonflybsd_v70_master_suite() {
    let mut suite = DragonFlyBsdMissingComponentsV70PrSuite::new();
    assert!(suite.run_parity_checks());
    let manifest = suite.generate_manifest();
    assert_eq!(manifest.len(), 10);
    for pr in &manifest {
        assert_eq!(pr.status, PrProposalStatusV70::Approved);
        assert!(pr.pr_id >= 7001 && pr.pr_id <= 7010);
        assert!(!pr.title.is_empty());
        assert!(!pr.unified_diff.is_empty());
    }
}

#[test]
fn test_integration_dragonflybsd_v70_engines() {
    // 1. HAMMER2 Dedup & Quorum
    let mut h2 = DragonFlyHammer2DistributedDeduplicationConsensusV70PrEngine::new(4);
    h2.register_pfs_node(1, "pfs-alpha", 2);
    h2.register_pfs_node(2, "pfs-beta", 2);
    let hash = [0x77; 32];
    assert!(!h2.store_or_dedup_block(1, hash, 1000));
    assert!(h2.store_or_dedup_block(2, hash, 1001));
    assert!(h2.propose_txg_commit(7050));
    assert_eq!(h2.current_txg, 7050);

    // 2. LWKT Affinity Scheduler
    let mut lwkt = DragonFlyLwktAffinityAwareStealingSchedulerV70PrEngine::new(4);
    lwkt.enqueue_thread(11, 0, 10, true);
    lwkt.enqueue_thread(12, 0, 20, true);
    let stolen = lwkt.steal_work_for_cpu(2);
    assert_eq!(stolen, Some(11));
    assert!(lwkt.tick_cpu(2));

    // 3. VKernel NUMA IOV Hypercall
    let mut vkernel = DragonFlyVKernelNumaDirectIovHypercallVfsV70PrEngine::new(4, 4096);
    assert!(vkernel.map_iov_buffer(100, 1, 0x8000_0000, 2 * 1024 * 1024));
    assert!(vkernel.dispatch_vfs_hypercall(1, 0xFE, 100));

    // 4. Dsynth PQC Builder
    let mut dsynth = DragonFlyDsynthPqcDilithium5DeterministicBuilderV70PrEngine::new();
    assert!(dsynth.build_and_sign_reproducible_package(42, "compiler-rt", "18.1.0", [0xCC; 32]));
    assert!(dsynth.verify_package(42));

    // 5. Varsyms Hierarchical Scope
    let mut varsyms = DragonFlyVarsymsHierarchicalProcessGroupScopeV70PrEngine::new();
    varsyms.set_sys_default("MACHINE", "x86_64");
    varsyms.register_group(100, None);
    varsyms.register_group(101, Some(100));
    varsyms.set_group_var(100, "ZONE", "eu-central");
    varsyms.set_group_var(101, "POD", "frontend-1");
    assert_eq!(
        varsyms.expand_path(101, "/cluster/$ZONE/$MACHINE/$POD/state"),
        "/cluster/eu-central/x86_64/frontend-1/state"
    );

    // 6. Swapcache NVMe-oF
    let mut swap = DragonFlySwapcacheNvmeOverFabricsAdaptiveCompressionV70PrEngine::new(0.5);
    swap.stage_block(200, 8192);
    assert!(swap.flush_to_nvme_over_fabrics(200));
    assert_eq!(swap.get_tier(200), Some(1));

    // 7. Per-CPU Namecache
    let mut nc = DragonFlyPerCpuGenerationNamecacheLockfreeV70PrEngine::new(4);
    nc.insert_entry(1, 0xDEADBEEF, 999);
    assert_eq!(nc.resolve(1, 0xDEADBEEF), Some(999));
    nc.advance_epoch_invalidate_all();
    assert_eq!(nc.resolve(1, 0xDEADBEEF), None);

    // 8. Devctl Multi-Queue
    let mut devctl = DragonFlyDevctlMultiQueueKqueuePriorityRouterV70PrEngine::new();
    devctl.enqueue_event("nvme", "online", "/dev/nvme0", 5);
    devctl.enqueue_event("kernel", "panic_trap", "/dev/kmem", 20);
    let top = devctl.dequeue_event().unwrap();
    assert_eq!(top.subsystem, "kernel");
    assert_eq!(top.priority, 20);

    // 9. Firmlink Quota Guard
    let mut firmlink = DragonFlyFirmlinkSecurityAuditedCrossMountQuotaGuardV70PrEngine::new();
    firmlink.register_rule(10, "/export/data", "/jail/data", 2, true);
    assert!(firmlink.request_firmlink("/export/data/file1", "/jail/data/file1", false));
    assert!(firmlink.request_firmlink("/export/data/file2", "/jail/data/file2", false));
    assert!(!firmlink.request_firmlink("/export/data/file3", "/jail/data/file3", false)); // Quota reached
    assert!(!firmlink.request_firmlink("/export/data/file4", "/jail/data/file4", true)); // RO enforced

    // 10. SMP TSC Governor
    let mut tsc = DragonFlySmpTscPhaseLockEqualizerDriftGovernorV70PrEngine::new(8, 20);
    tsc.record_core_skew(2, 45);
    assert!(!tsc.is_all_phase_locked());
    assert!(tsc.run_drift_equalization());
    assert!(tsc.is_all_phase_locked());
}
