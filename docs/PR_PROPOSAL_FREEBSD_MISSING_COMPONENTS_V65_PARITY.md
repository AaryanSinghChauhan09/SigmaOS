# FreeBSD Missing Components V65 Parity & Pull Request Gateway Specification

This document details the Pull Request (PR) proposals, architectures, and sub-engines implemented in `src/distro/freebsd_missing_components_v65_pr.rs` to bring SigmaOS into parity with FreeBSD 14.2 / 15.0+ core subsystems.

---

## 1. Subsystem Architecture Overview

The FreeBSD Parity Engine V65 is structured into 10 modular sub-engines coordinated by a unified Post-Quantum Cryptography (PQC) Dilithium-5 signed Pull Request Gateway (`FreeBsdMissingComponentsV65PrSuite`):

```
+-----------------------------------------------------------------------------------+
|               FreeBSD Missing Components V65 PR Gateway Engine                     |
|          (PQC Dilithium-5 Signed, Automated CI Gating, SAT Constraints)           |
+-----------------------+-------------------------------------------+---------------+
                        |                                           |
    +-------------------+-----------------------+   +---------------+-------------------+
    | Core FreeBSD Subsystem Engines            |   | Security & Administration         |
    | - NVMe-oF TCP Target & Pass-Through   |   | - Capsicum Casper cap_pwd/cap_grp |
    | - ipfw Stateful NAT64 & CLAT (RFC 6146) |   | - auditdistd Zstd Ring Buffer Stream|
    | - bhyve Virtio-FS Sandboxed Sharing   |   | - rctl GPU VRAM & Compute Quota  |
    | - OpenZFS Special Vdev & Fast Dedup   |   | - pkgbase Atomic Snapshot & Rollback|
    | - Netgraph ng_l2tp & ng_ipsec VPN     |   | - DTrace netisr/ipfw Latency Trace|
    +---------------------------------------+   +-----------------------------------+
```

---

## 2. Core Sub-Engine Capabilities

### 2.1 FreeBSD NVMe-oF Target & Host Pass-through Engine (`FreeBsdNvdNvmePassthroughPrEngine`)
- **NVMe-over-Fabrics Target**: Implements `nvd(4)` / `nvme(4)` NVMe-oF TCP & RDMA target subsystem daemon (`nvmf`).
- **PQC Session Signing**: Authenticates fabric connections with Dilithium-5 post-quantum signatures to prevent rogue controller attachments.
- **Kernel Pass-through**: Directly forwards block I/O requests between NVMe-oF network fabrics and target PCIe NVMe namespaces (`/dev/nvme0n1`).

### 2.2 FreeBSD ipfw Stateful NAT64 & CLAT IPv4-to-IPv6 Translation Engine (`FreeBsdIpfwInKernelNat64ClatPrEngine`)
- **CLAT Translation**: Customer-side Translator (RFC 6146 / RFC 6877) converting outbound IPv4 traffic from local applications to IPv6 packets (`clat0`).
- **Stateful NAT64**: Bridges dual-stack and IPv6-only VNET Jails with external IPv4 networks using dynamic session state tracking in kernel space.

### 2.3 FreeBSD bhyve Virtio-FS Sandboxed Host Filesystem Sharing Engine (`FreeBsdBhyveVirtioFsPrEngine`)
- **Virtio-FS Sharing**: Enables zero-copy host filesystem directory sharing with guest Virtual Machines (`bhyve -s virtio-fs`).
- **Capsicum Sandboxing**: Encapsulates host filesystem daemons (`virtiofsd`) inside Capsicum capability mode sandboxes using restricted UNIX domain sockets (`/var/run/virtiofsd_*.sock`).

### 2.4 FreeBSD Capsicum Casper cap_pwd & cap_grp Delegates Engine (`FreeBsdCapsicumCasperCapPwdGrpPrEngine`)
- **Sandboxed Database Lookups**: Extends Casper sandboxing daemon framework with `cap_pwd` and `cap_grp` service delegates.
- **Ambient Privilege Elimination**: Permits sandboxed applications to perform user/group ID lookups without ambient `/etc/passwd` or `/etc/group` file descriptor access.

### 2.5 FreeBSD OpenZFS Special Allocation Class Vdevs & Dedup Engine (`FreeBsdOpenZfsSpecialVdevMetadataDedupPrEngine`)
- **Special Allocation Vdevs**: Configures NVMe/SSD special allocation class vdevs for ZFS metadata and small block I/O offloading (`special_small_blocks`).
- **Fast Inline Deduplication**: Enables low-latency inline block cloning and deduplication (`zfs set dedup=fast`) on OpenZFS 2.3+ storage pools.

### 2.6 FreeBSD Netgraph ng_l2tp & ng_ipsec Kernel-Mode VPN Router Engine (`FreeBsdNetgraphL2tpIpsecVpnPrEngine`)
- **Netgraph VPN Graphing**: Constructs graph nodes (`ng_l2tp`, `ng_ipsec`) for in-kernel Layer 2 Tunneling Protocol over IPsec encryption.
- **PQC Hybrid Keying**: Combines ML-KEM-768 key encapsulation with ESP AES-GCM SA policies (`setkey`) for quantum-resistant VPN mesh topologies.

### 2.7 FreeBSD auditdistd Zstd Ring Buffer PQC Stream Engine (`FreeBsdAuditdistdZstdRingBufferPrEngine`)
- **Lockless Ring Buffering**: Collects kernel BSM audit trails into lockless high-throughput ring buffers (e.g. 512MB capacity).
- **Zstd Streaming & Dilithium-5**: Compresses audit trails using Zstd (level 9) and streams signed audit payloads over TLS to secure central log collectors.

### 2.8 FreeBSD rctl GPU Memory & Compute Quota Governor Engine (`FreeBsdRctlGpuMemoryComputeGovernorPrEngine`)
- **DRM GPU Quotas**: Enforces per-jail and per-process GPU compute time (`gputime`) and VRAM allocation (`vramuse`) quotas via `rctl(8)`.
- **Workload Isolation**: Prevents runaway machine learning or graphics jobs inside VNET Jails from starving host GPU render nodes (`/dev/dri/card0`).

### 2.9 FreeBSD pkgbase Atomic Upgrade & BE Rollback Engine (`FreeBsdPkgBaseAtomicSnapshotRollbackPrEngine`)
- **Zero-Downtime Base System Upgrade**: Manages FreeBSD base system release upgrades using atomic binary packages (`pkgbase`).
- **bectl ZFS Snapshot Rollback**: Automatically creates pre-upgrade ZFS boot environments (`bectl create`) and provides instant rollback capability if release verification fails.

### 2.10 FreeBSD DTrace netisr / ipfw Latency Profiler Engine (`FreeBsdDtraceEpfwNetisrLatencyProfilerPrEngine`)
- **Packet Pipeline Profiling**: Traces network soft-interrupt (`netisr`) and `ipfw` firewall packet processing latency using DTrace FBT probes.
- **CPU Thread Affinity**: Measures packet queue delays (`netisr_dispatch`) and tunes soft-interrupt thread CPU bindings (`swi1: netisr`) to minimize latency spikes.

---

## 3. Pull Request Proposals (#6500..#6502)

The `FreeBsdMissingComponentsV65PrSuite` pre-populates and validates FreeBSD PR proposals:

| PR ID | Title | Subsystem | Patch Summary |
|-------|-------|-----------|---------------|
| 6500  | Enable NVMe-oF TCP Target with Dilithium-5 authenticated sessions | `sys/dev/nvme` | NVMe-over-Fabrics target daemon and kernel pass-through |
| 6501  | Add OpenZFS Special Allocation Vdev for metadata and fast inline dedup | `sys/contrib/openzfs` | OpenZFS special allocation class vdev offload governor |
| 6502  | Enable pkgbase atomic release upgrades with bectl ZFS snapshot rollback | `usr.bin/bectl` | Zero-downtime base system update and ZFS boot environment governor |

---

## 4. Verification & Testing

The implementation is verified via standalone unit tests in `src/distro/freebsd_missing_components_v65_pr.rs`:

```bash
rustc --test src/distro/freebsd_missing_components_v65_pr.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/test_freebsd_v65_pr
./build/test_freebsd_v65_pr
```

All 4 test blocks pass 100%, validating all 10 FreeBSD engines and the master PR suite.
