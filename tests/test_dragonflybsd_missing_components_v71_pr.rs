// SPDX-License-Identifier: MIT
// Integration tests for DragonFly BSD Missing Components V71 PR Suite
// (`tests/test_dragonflybsd_missing_components_v71_pr.rs`)

extern crate alloc;

#[path = "../src/distro/dragonflybsd_missing_components_v71_pr.rs"]
mod dragonflybsd_missing_components_v71_pr;

use alloc::vec::Vec;
use dragonflybsd_missing_components_v71_pr::*;

#[test]
fn test_integration_dragonflybsd_v71_master_suite() {
    let mut suite = DragonFlyBsdMissingComponentsV71PrSuite::new();
    assert!(suite.run_parity_checks());
    let manifest: Vec<DragonFlyComponentPrSpecV71> = suite.generate_manifest();
    assert_eq!(manifest.len(), 10);
    for pr in &manifest {
        assert!(pr.pr_id >= 7101 && pr.pr_id <= 7110);
        assert_eq!(pr.status, PrProposalStatusV71::Approved);
        assert!(!pr.unified_diff.is_empty());
    }
}

#[test]
fn test_integration_dragonflybsd_v71_engines() {
    // 1. HAMMER2 Deduplication
    let mut hammer2 = DragonFlyHammer2DistributedDeduplicationConsensusV71PrEngine::new(2);
    let block_data = b"sigmaos_hammer2_integration_test_block";
    let (dedup1, off1) = hammer2.register_or_dedup_block(block_data, 0x1000);
    assert!(!dedup1);
    assert_eq!(off1, 0x1000);
    let (dedup2, off2) = hammer2.register_or_dedup_block(block_data, 0x2000);
    assert!(dedup2);
    assert_eq!(off2, 0x1000);

    // 2. LWKT Scheduler
    let mut lwkt = DragonFlyLwktAffinityAwareStealingSchedulerV71PrEngine::new(4, 2);
    sched_test_helper(&mut lwkt);

    // 3. VKernel Hypercall
    let mut vkernel = DragonFlyVKernelNumaDirectIovHypercallVfsV71PrEngine::new(2);
    assert!(vkernel.create_instance(0, 0));
    let seg = vkernel.map_iov_buffer(0x2000, 512);
    assert_eq!(seg.guest_addr, 0x2000);
    let res = vkernel.execute_hypercall(0, 2, &[5, 15]).unwrap();
    assert_eq!(res, 20);

    // 4. Dsynth PQC Builder
    let mut dsynth = DragonFlyDsynthPqcDilithium5DeterministicBuilderV71PrEngine::new();
    let digest = [0x7C; 32];
    let art = dsynth.build_deterministic("pkg-tools", "2.1", &digest);
    assert_eq!(art.package_name, "pkg-tools");
    assert!(dsynth.verify_package_signature("pkg-tools"));

    // 5. Varsyms
    let mut varsyms = DragonFlyVarsymsHierarchicalProcessGroupScopeV71PrEngine::new();
    varsyms.set_global("OS", "DragonFly");
    varsyms.set_scoped(10, 1, 100, "TARGET", "x86_64");
    assert_eq!(
        varsyms.resolve_path(10, "/usr/${OS}/${TARGET}/bin"),
        "/usr/DragonFly/x86_64/bin"
    );

    // 6. Swapcache
    let mut swapcache = DragonFlySwapcacheNvmeOverFabricsAdaptiveCompressionV71PrEngine::new(0.8);
    let sample_data = [0x22; 256];
    assert!(swapcache.ingest_block(200, &sample_data));
    assert!(swapcache.flush_to_nvme_over_fabrics(200));

    // 7. Per-CPU Namecache
    let mut nc = DragonFlyPerCpuGenerationNamecacheLockfreeV71PrEngine::new(4);
    nc.insert_entry(0, 10, 0xABCD, 1234);
    assert_eq!(nc.lookup(0, 10, 0xABCD), Some(1234));
    nc.bump_generation_invalidate_all();
    assert_eq!(nc.lookup(0, 10, 0xABCD), None);

    // 8. Devctl
    let mut devctl = DragonFlyDevctlMultiQueueKqueuePriorityRouterV71PrEngine::new();
    devctl.post_event("nvme", "/dev/nvme0", "alert", 25, 100);
    let ev = devctl.poll_highest_priority_event().unwrap();
    assert_eq!(ev.priority, 25);
    assert_eq!(ev.subsystem, "nvme");

    // 9. Firmlink
    let mut firmlink = DragonFlyFirmlinkSecurityAuditedCrossMountQuotaGuardV71PrEngine::new();
    firmlink.register_policy(1, "/mnt/data", "/jail/data", 1, false);
    assert!(firmlink.request_link_creation("/mnt/data/a", "/jail/data/a", false));
    assert!(!firmlink.request_link_creation("/mnt/data/b", "/jail/data/b", false)); // Quota reached

    // 10. TSC Governor
    let mut tsc = DragonFlySmpTscPhaseLockEqualizerDriftGovernorV71PrEngine::new(2, 8);
    tsc.record_core_offset(0, 2);
    tsc.record_core_offset(1, 4);
    assert!(tsc.is_drift_acceptable());
}

fn sched_test_helper(sched: &mut DragonFlyLwktAffinityAwareStealingSchedulerV71PrEngine) {
    sched.enqueue_thread(LwktThreadV71 {
        tid: 10,
        priority: 5,
        preferred_numa_node: 0,
        pinned_cpu: Some(0),
        run_ticks: 0,
    });
    sched.enqueue_thread(LwktThreadV71 {
        tid: 20,
        priority: 8,
        preferred_numa_node: 0,
        pinned_cpu: None,
        run_ticks: 0,
    });
    // CPU 2 is in NUMA node 0 (2 % 2 = 0) and will steal unpinned thread from CPU 0
    let stolen = sched.pop_next_thread(2);
    assert_eq!(stolen.map(|t| t.tid), Some(20));
}
