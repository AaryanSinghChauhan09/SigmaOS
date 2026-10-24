# FreeBSD Missing Components V65 Roadmap Specification

This document serves as the roadmap specification for FreeBSD 14.2 / 15.0+ missing components parity in SigmaOS (V65).

## Summary of Subsystem Engines
1. **FreeBsdNvdNvmePassthroughPrEngine**: NVMe-oF TCP/RDMA target daemon & kernel pass-through with Dilithium-5 session verification.
2. **FreeBsdIpfwInKernelNat64ClatPrEngine**: ipfw in-kernel stateful NAT64 and CLAT RFC 6146 translation governor.
3. **FreeBsdBhyveVirtioFsPrEngine**: bhyve Virtio-FS sandboxed host filesystem sharing daemon.
4. **FreeBsdCapsicumCasperCapPwdGrpPrEngine**: Capsicum casper `cap_pwd` and `cap_grp` delegates for sandboxed database lookups.
5. **FreeBsdOpenZfsSpecialVdevMetadataDedupPrEngine**: OpenZFS 2.3+ special allocation class vdevs for metadata offload and fast inline block deduplication.
6. **FreeBsdNetgraphL2tpIpsecVpnPrEngine**: Netgraph `ng_l2tp` and `ng_ipsec` kernel-mode VPN router with ML-KEM hybrid keying.
7. **FreeBsdAuditdistdZstdRingBufferPrEngine**: auditdistd lockless Zstd ring buffer exporter with Dilithium-5 signed audit trails.
8. **FreeBsdRctlGpuMemoryComputeGovernorPrEngine**: rctl DRM GPU compute time and VRAM quota governor for jails.
9. **FreeBsdPkgBaseAtomicSnapshotRollbackPrEngine**: pkgbase atomic release updates with bectl ZFS snapshot rollback integration.
10. **FreeBsdDtraceEpfwNetisrLatencyProfilerPrEngine**: DTrace netisr/ipfw packet pipeline latency profiler and soft-interrupt CPU affinity governor.
11. **FreeBsdMissingComponentsV65PrSuite**: Master FreeBSD Parity PR Gateway Suite V65.

## Integration
- Source Module: `src/distro/freebsd_missing_components_v65_pr.rs`
- Re-exported in: `src/distro/mod.rs` & `src/lib.rs`
- Test Runner: `run_sigma_tests.sh`
