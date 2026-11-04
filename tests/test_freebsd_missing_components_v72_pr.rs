// SPDX-License-Identifier: MIT
// Integration tests for FreeBSD Missing Components V72 PR Suite
// (`tests/test_freebsd_missing_components_v72_pr.rs`)

#[path = "../src/distro/freebsd_missing_components_v72_pr.rs"]
mod freebsd_missing_components_v72_pr;

use freebsd_missing_components_v72_pr::*;

#[test]
fn test_integration_freebsd_v72_master_suite() {
    let mut suite = FreeBsdMissingComponentsV72PrSuite::new();
    assert!(suite.evaluate_freebsd_v72_parity());
    assert_eq!(suite.proposals.len(), 4);
    for (id, p) in &suite.proposals {
        assert!(*id >= 7200);
        assert_eq!(p.status, FreeBsdPrStatusV72::Merged);
        assert!(!p.unified_diff.is_empty());
        assert!(!p.patch_summary.is_empty());
    }
}

#[test]
fn test_integration_freebsd_v72_all_engines() {
    // 1. bhyve NVMe-oF RDMA/TCP Namespace Passthrough
    let mut nvme = FreeBsdBhyveNvmeDirectOverFabricsPrEngine::new();
    let d1 = nvme.attach_nvme_over_fabrics(
        "vm_edge_compute",
        8,
        "rdma",
        "nqn.2026-04.org.freebsd:nvmf:cluster-vol1",
        "192.168.100.5:4420",
        64,
    );
    assert!(d1.contains("nvme-of,trtype=rdma"));
    assert!(d1.contains("addr=192.168.100.5:4420"));
    assert!(nvme.nvme_fabrics.contains_key("vm_edge_compute"));

    // 2. OpenZFS 2.4+ dRAID Distributed Spare Reconstruction
    let mut draid = FreeBsdOpenZfsDraidVdevExpansionPrEngine::new();
    let d2 = draid.configure_draid_rebuild("pool_fast", "draid2:8d:24c:2s-0", 8, 2, 2);
    assert!(d2.contains("zpool create pool_fast draid2:8d:2s"));
    assert!(d2.contains("zfs set sequential_rebuild=on pool_fast"));
    assert!(draid.draid_pools.contains_key("pool_fast"));

    // 3. kqueue Zero-Copy Ring Buffer I/O
    let mut kq = FreeBsdKqueueZeroCopyRingBufferIoPrEngine::new();
    let d3 = kq.register_zerocopy_ring(15, 16384, 128);
    assert!(d3.contains("kqueuex(KQUEUE_RING_ZEROCOPY, ring_size=16384, evfilt_user_slots=128)"));
    assert!(kq.rings.contains_key(&15));

    // 4. Casper cap_kvm Kernel VM Inspection Delegate
    let mut casper = FreeBsdCapsicumCasperCapKvmVmMiniJailPrEngine::new();
    let d4 = casper.configure_cap_kvm(
        "kvm_monitor_jail",
        false,
        true,
        vec!["CAP_KVM_READ_VMSTATS".to_string(), "CAP_KVM_READ_PAGECOUNTERS".to_string()],
    );
    assert!(d4.contains("cap_service_open(cap_chan, \"cap_kvm\")"));
    assert!(casper.kvm_delegates.contains_key("kvm_monitor_jail"));

    // 5. Netgraph ng_vxlan Hardware Inner Checksum Offload
    let mut vxlan = FreeBsdNetgraphNgVxlanHardwareOffloadPrEngine::new();
    let d5 = vxlan.setup_vxlan_offload("vxlan500", 500, "239.5.5.5", 4789);
    assert!(d5.contains("mkpeer . vxlan vxlan500 lower"));
    assert!(d5.contains("hw_offload=1"));
    assert!(vxlan.vxlan_nodes.contains_key("vxlan500"));

    // 6. auditdistd Multicast TLS PQC Dilithium-5 Spool
    let mut audit = FreeBsdAuditdistdMulticastTlsTlsPqcPrEngine::new();
    let d6 = audit.create_multicast_spool("global_audit_spool", "239.255.42.1:9443", 131072);
    assert!(d6.contains("--multicast-endpoint 239.255.42.1:9443"));
    assert!(d6.contains("--ring-buffer-kb 131072"));
    assert!(audit.multicast_channels.contains_key("global_audit_spool"));

    // 7. rctl NUMA Memory Bandwidth QoS
    let mut qos = FreeBsdRctlMemoryBandwidthNumaQosPrEngine::new();
    let d7 = qos.set_numa_memory_bandwidth_limit("hpc_node_jail", 2, 65536, 120);
    assert!(d7.contains("jail:hpc_node_jail:numa2:membw:throttle:65536"));
    assert!(d7.contains("jail:hpc_node_jail:numa2:latencyweight:120"));
    assert!(qos.numa_rules.contains_key("hpc_node_jail"));

    // 8. pkgbase A/B Partition Atomic Update Synchronization
    let mut ab = FreeBsdPkgBaseBootEnvironmentAbtSyncPrEngine::new();
    let d8_prep = ab.prepare_ab_atomic_update("node_alpha", "da0p2", "da0p3");
    assert!(d8_prep.contains("efibootmgr --bootnext standby"));
    assert!(d8_prep.contains("pkgbase upgrade --chroot da0p3"));
    assert!(ab.ab_slots.contains_key("node_alpha"));
    assert!(ab.commit_update("node_alpha"));

    // 9. DTrace kinst Branch Prediction Profiler Probe
    let mut kinst = FreeBsdDtraceKinstBranchPredictionSamplerPrEngine::new();
    let d9 = kinst.profile_branch_prediction("kinst::tcp_output:0x114", 0xffffffff80d92114, 8.2);
    assert!(d9.contains("kinst::tcp_output:0x114:branch-miss /arg0 == 0xffffffff80d92114/"));
    assert!(kinst.branch_probes.contains_key("kinst::tcp_output:0x114"));

    // 10. FreeBSD PQC Kexec Signed Kernel Chain of Trust
    let mut kexec = FreeBsdPqcKexecSignedKernelChainPrEngine::new();
    let d10 = kexec.verify_and_prepare_kexec(
        "/boot/kernel/kernel.current",
        "/boot/kernel/kernel.v72",
        "d5sig_f09328472910381023",
        "sha3_384:98234fedca9876",
    );
    assert!(d10.contains("kexec -l /boot/kernel/kernel.v72 --pqc-sig-dilithium5"));
    assert!(kexec.kernel_chains.contains_key("/boot/kernel/kernel.current"));
}
