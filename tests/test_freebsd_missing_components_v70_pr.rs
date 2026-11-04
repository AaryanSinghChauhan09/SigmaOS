// SPDX-License-Identifier: MIT
// Integration tests for FreeBSD Missing Components V70 PR Suite
// (`tests/test_freebsd_missing_components_v70_pr.rs`)

#[path = "../src/distro/freebsd_missing_components_v70_pr.rs"]
mod freebsd_missing_components_v70_pr;

use freebsd_missing_components_v70_pr::*;

#[test]
fn test_integration_freebsd_v70_master_suite() {
    let mut suite = FreeBsdMissingComponentsV70PrSuite::new();
    assert!(suite.evaluate_freebsd_v70_parity());
    assert_eq!(suite.proposals.len(), 4);
    for (id, p) in &suite.proposals {
        assert!(*id >= 7000);
        assert_eq!(p.status, FreeBsdPrStatusV70::Merged);
        assert!(!p.unified_diff.is_empty());
        assert!(!p.patch_summary.is_empty());
    }
}

#[test]
fn test_integration_freebsd_v70_all_engines() {
    // 1. Virtio-GPU Vulkan Compute
    let mut gpu = FreeBsdBhyveVirtioGpuVulkanComputePrEngine::new();
    let d1 = gpu.attach_vulkan_compute("vm_k8s_worker", 5, "1.3.280");
    assert!(d1.contains("virtio-gpu,vulkan=1,virgl=1"));
    assert!(gpu.gpu_instances.contains_key("vm_k8s_worker"));

    // 2. OpenZFS Compressed ARC Direct-IO
    let mut arc = FreeBsdOpenZfsCompressedArcDirectIoPrEngine::new();
    let d2 = arc.configure_direct_io("tank", "tank/analytics", 4194304, "zstd-1");
    assert!(d2.contains("zfs set direct=always"));
    assert!(arc.direct_io_configs.contains_key("tank/analytics"));

    // 3. Wireguard Kernel Kmod
    let mut wg = FreeBsdIpsecWireguardKernelKmodPrEngine::new();
    let d3 = wg.configure_wireguard_interface("wg_corp", 51820, "ml-kem-1024");
    assert!(d3.contains("crypto-offload on"));
    assert!(wg.wg_interfaces.contains_key("wg_corp"));

    // 4. Casper cap_dns
    let mut dns = FreeBsdCapsicumCasperCapDnsPrEngine::new();
    let d4 = dns.configure_cap_dns("unbound_jail", true, true);
    assert!(d4.contains("cap_dns_limit(cap_d, dnssec=true, doh=true)"));
    assert!(dns.dns_delegates.contains_key("unbound_jail"));

    // 5. Netgraph ng_nat64
    let mut nat64 = FreeBsdNetgraphNgNat64StatefulPrEngine::new();
    let d5 = nat64.setup_nat64_node("nat64_edge", "64:ff9b::/96", "203.0.113.1", "203.0.113.100");
    assert!(d5.contains("mkpeer . nat64 nat64_edge in"));
    assert!(nat64.nat64_nodes.contains_key("nat64_edge"));

    // 6. Auditdistd Zstd
    let mut audit = FreeBsdAuditdistdSpoolZstdCompressedPrEngine::new();
    let d6 = audit.attach_zstd_stream("stream_pci_dss", 19, "siem.internal");
    assert!(d6.contains("--zstd-level 19"));
    assert!(audit.zstd_streams.contains_key("stream_pci_dss"));

    // 7. Rctl Memory Tiering Swapcache
    let mut rctl = FreeBsdRctlMemoryTieringSwapcacheGovernorPrEngine::new();
    let d7 = rctl.set_tiering_rule("cache_jail", 17179869184, 68719476736);
    assert!(d7.contains("swapcache:limit"));
    assert!(rctl.tiering_rules.contains_key("cache_jail"));

    // 8. PkgBase ZFS Checkpoint
    let mut chk = FreeBsdPkgBaseZfsCheckpointRollbackPrEngine::new();
    let d8_create = chk.create_upgrade_checkpoint("zroot", "pre_16_0_upgrade");
    assert!(d8_create.contains("zpool checkpoint zroot"));
    let d8_rb = chk.rollback_checkpoint("zroot");
    assert!(d8_rb.contains("zpool import --rewind-to-checkpoint zroot"));

    // 9. DTrace Kinst Branch Tracing
    let mut kinst = FreeBsdDtraceKinstBranchTracingPrEngine::new();
    let d9 = kinst.trace_kernel_branches("tcp_input", "prediction_miss");
    assert!(d9.contains("kinst::tcp_input:branch"));
    assert!(kinst.branch_probes.contains_key("kinst::tcp_input:branch"));

    // 10. FreeBSD Update Dilithium-5
    let mut updater = FreeBsdFreebsdUpdateDilithium5PqcPackageSignerEngine::new();
    let d10 = updater.sign_and_verify_package_manifest("16.0-RELEASE", "sha256_hash", "pqc_dilithium5_pubkey");
    assert!(d10.contains("--verify-dilithium5"));
    assert!(updater.signature_registry.contains_key("16.0-RELEASE"));
}
