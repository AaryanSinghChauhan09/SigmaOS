# FreeBSD Missing Components V68 Specification

Specifies the architecture and implementation details for `FreeBsdMissingComponentsV68PrSuite` in `src/distro/freebsd_missing_components_v68_pr.rs`.

## Architecture & Subsystems

1. **bhyve Intel GVT-g Virtual GPU**: Enables virtual GPU partitioning for PCI pass-through guest VMs in bhyve.
2. **OpenZFS dRAID**: Implements declustered RAID parity allocation and fast spare drive rebuilds.
3. **IPsec IKEv2 MOBIKE**: Implements RFC 4555 address migration for seamless roaming VPN connections.
4. **Capsicum Casper Netlink**: Delegates netlink socket control for sandboxed process contexts.
5. **Netgraph VXLAN Overlay**: Creates `ng_vxlan` netgraph nodes for L2 packet encapsulation across IP networks.
6. **auditdistd Post-Quantum Audit Trail**: Secures remote audit logging using TLS 1.3 ML-KEM-1024 / Dilithium-5.
7. **rctl Storage IOPS Throttle**: Enforces IOPS limits on jail processes.
8. **pkgbase Boot Environment Upgrade**: Performs safe ZFS boot environment updates via `bectl` and `pkg`.
9. **DTrace kinst Profiler**: Provides instruction-level tracing inside kernel routines.
10. **freebsd-update PQC Manifest**: Validates update manifests with Dilithium-5 signature fingerprints.
