// SPDX-License-Identifier: MIT
// Integration tests for FreeBSD Missing Components V71 PR Suite
// (`tests/test_freebsd_missing_components_v71_pr.rs`)

#[path = "../src/distro/freebsd_missing_components_v71_pr.rs"]
mod freebsd_missing_components_v71_pr;

use freebsd_missing_components_v71_pr::*;

#[test]
fn test_integration_freebsd_v71_master_suite() {
    let mut suite = FreeBsdMissingComponentsV71PrSuite::new();
    assert!(suite.evaluate_freebsd_v71_parity());
    assert_eq!(suite.proposals.len(), 4);
    for (id, p) in &suite.proposals {
        assert!(*id >= 7100);
        assert_eq!(p.status, FreeBsdPrStatusV71::Merged);
        assert!(!p.unified_diff.is_empty());
        assert!(!p.patch_summary.is_empty());
    }
}

#[test]
fn test_integration_freebsd_v71_all_engines() {
    // 1. bhyve ARM64 / x86_64 Virtio Multi-Queue & PCI ACS
    let mut bhyve = FreeBsdBhyveArm64HypercallPciAcsPrEngine::new();
    let d1 = bhyve.attach_pci_acs_multiqueue("vm_gateway_node", 7, 16);
    assert!(d1.contains("virtio-net,mq=16"));
    assert!(d1.contains("pci-acs=on"));
    assert!(bhyve.vm_instances.contains_key("vm_gateway_node"));

    // 2. OpenZFS 2.4+ RAID-Z Expansion & B-Tree Allocator
    let mut raidz = FreeBsdOpenZfsRaidzExpansionBtreePrEngine::new();
    let d2 = raidz.initiate_raidz_expansion("pool_tier0", "raidz2-0", "/dev/da10");
    assert!(d2.contains("zpool attach pool_tier0 raidz2-0 /dev/da10"));
    assert!(d2.contains("zfs set btree_alloc=on pool_tier0"));
    assert!(raidz.expansion_jobs.contains_key("pool_tier0"));

    // 3. kqueue Async Ring Timer Completion
    let mut kq = FreeBsdKqueueAsyncRingTimerCompletionPrEngine::new();
    let d3 = kq.create_async_ring(100, 16384, 25);
    assert!(d3.contains("kqueuex(KQUEUE_ASYNC_RING, depth=16384, timer_res_ns=25)"));
    assert!(kq.kqueue_rings.contains_key(&100));

    // 4. Casper cap_netlink Route Delegate
    let mut casper = FreeBsdCapsicumCasperCapNetlinkRoutePrEngine::new();
    let d4 = casper.configure_cap_netlink("routing_daemon", vec!["AF_INET".to_string(), "AF_INET6".to_string()], true);
    assert!(d4.contains("cap_service_open(cap_chan, \"cap_netlink\")"));
    assert!(casper.netlink_delegates.contains_key("routing_daemon"));

    // 5. Netgraph ng_bpf Filter Offload Node
    let mut bpf = FreeBsdNetgraphNgBpfFilterOffloadPrEngine::new();
    let d5 = bpf.setup_bpf_offload_node("bpf_ingress", "eth_in", "ip_fwd", 128);
    assert!(d5.contains("mkpeer . bpf bpf_ingress eth_in"));
    assert!(d5.contains("jit=1"));
    assert!(bpf.bpf_nodes.contains_key("bpf_ingress"));

    // 6. auditdistd Multi-Stream Zstandard Spool
    let mut audit = FreeBsdAuditdistdSpoolZstdMultiStreamPrEngine::new();
    let d6 = audit.attach_multistream_spool("pci_dss_stream", 19, 16, "collector.corp:9000");
    assert!(d6.contains("--stream pci_dss_stream --zstd-level 19 --workers 16"));
    assert!(audit.multi_streams.contains_key("pci_dss_stream"));

    // 7. rctl Storage Bandwidth QoS
    let mut qos = FreeBsdRctlStorageBandwidthQosPrEngine::new();
    let d7 = qos.configure_storage_qos("analytics_jail", 1073741824, 536870912, 100000);
    assert!(d7.contains("readbps:throttle:1073741824"));
    assert!(d7.contains("writebps:throttle:536870912"));
    assert!(qos.qos_rules.contains_key("analytics_jail"));

    // 8. pkgbase ZFS Boot Environment Staging
    let mut be = FreeBsdPkgBaseZfsBootEnvironmentStagingPrEngine::new();
    let d8_stage = be.stage_upgrade_environment("zroot", "16.1-RELEASE", "zroot/ROOT/default@pre_16_1");
    assert!(d8_stage.contains("bectl create -e zroot/ROOT/default@pre_16_1 16.1-RELEASE"));
    assert!(be.staged_environments.contains_key("16.1-RELEASE"));
    let d8_rb = be.rollback_staged_environment("16.0-RELEASE");
    assert!(d8_rb.contains("bectl activate 16.0-RELEASE"));

    // 9. DTrace kinst Instruction Trace Matrix
    let mut kinst = FreeBsdDtraceKinstInstructionTraceMatrixPrEngine::new();
    let d9 = kinst.trace_instruction_latency("tcp_do_segment", 0x30, 200);
    assert!(d9.contains("kinst::tcp_do_segment:0x30"));
    assert!(kinst.instruction_probes.contains_key("kinst::tcp_do_segment:0x30"));

    // 10. FreeBSD PQC Secure Boot Loader / Kexec Verifier
    let mut boot = FreeBsdPqcSecureBootKexecVerifierPrEngine::new();
    let d10 = boot.verify_and_authorize_kernel("/boot/kernel/kernel.v71", "d5_hex_sig_392", "sha3_root_trust");
    assert!(d10.contains("loader_pqc_verify --kernel /boot/kernel/kernel.v71"));
    assert!(boot.boot_verifications.contains_key("/boot/kernel/kernel.v71"));
}
