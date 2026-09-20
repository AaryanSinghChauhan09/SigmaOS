# SigmaOS Filesystem & Storage Subsystem - Master Development Plan

## 1. Executive Summary & Vision

The **SigmaOS Virtual Filesystem (VFS) and Storage Subsystem** (`SigmaFS`) is built from the ground up in native, memory-safe Rust. It merges the enterprise reliability and Copy-on-Write (CoW) snapshotting of OpenZFS and DragonFly BSD HAMMER2, the flash-optimized and compressed efficiency of Linux EROFS and F2FS, the multi-device tiering of bcachefs, and the modular block provider abstraction of FreeBSD GEOM.

---

## 2. Inspirations from Linux & BSD Storage Ecosystems

| Ecosystem Origin | Feature & Storage Innovation Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **FreeBSD OpenZFS** | Adaptive Replacement Cache (ARC), Copy-on-Write dataset snapshots, ZPOOL storage pools, end-to-end checksum self-healing. | `src/filesystem/zfs_inspired.rs` |
| **DragonFly BSD HAMMER2** | Cluster pseudo-filesystems (pFS), multi-master Merkle-tree replication, fine-grained transaction group (`TXG`) commits. | `src/filesystem/bsd_linux_innovations.rs` |
| **Linux EROFS & Btrfs** | Compressed immutable system rootfs (EROFS), Btrfs subvolumes, transparent extent compression (`zstd`/`lz4`). | `src/filesystem/erofs.rs` & `btrfs_inspired.rs` |
| **Linux bcachefs & F2FS** | Multi-device NVMe/SSD/HDD storage tiering, flash-friendly log-structured zoned block garbage collection. | `src/filesystem/nextgen_filesystem_suite.rs` |
| **FreeBSD GEOM Framework** | Stackable block transformation providers (striping, mirroring, dm-crypt LUKS encryption, Ggate network storage). | `src/filesystem/geom.rs` |

---

## 3. 5-Layer Filesystem Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Time-Travel Snapshotting & Merkle Tree Self-Healing Engine   │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: Pseudo-Filesystems (`procfs`, `sysfs`, `devfs`, `configfs`)   │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: Next-Gen CoW Filesystems Engine (OpenZFS, HAMMER2, EROFS, F2FS)│
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: FreeBSD GEOM & Linux Block Layer (Providers, LUKS, Ggate)     │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: VFS Core & POSIX / BSD Syscall Parity (`open`, `read`, `stat`)│
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: VFS Core & Syscall Parity Layer
- **POSIX & BSD VFS Ops:** Standard filesystem operations (`open`, `read`, `write`, `stat`, `unlink`, `rename`, `mkdir`, `symlink`, `mount`).
- **Mount Namespaces:** Isolated filesystem hierarchy namespaces for unprivileged containers and sandboxed applications.

### Layer 2: FreeBSD GEOM & Linux Block Layer
- **GEOM Providers:** Stackable disk transformation layers (mirroring `gmirror`, striping `gstripe`, network block storage `ggate`).
- **Storage Encryption:** Hardware-accelerated LUKS2 / dm-crypt transparent volume encryption.

### Layer 3: Next-Gen CoW Filesystem Drivers
- **OpenZFS / Btrfs Engine:** Atomic Copy-on-Write extents, ARC cache management, dataset subvolumes.
- **DragonFly HAMMER2 Engine:** Cluster pseudo-filesystems, Merkle root tree commits, pFS replication.
- **EROFS Immutable Engine:** Fixed-output LZ4/ZSTD decompressed block mapping for tamper-proof system images.
- **bcachefs & F2FS:** Flash-optimized write allocation, SSD caching, zoned storage garbage collection.

### Layer 4: Pseudo-Filesystems & Telemetry
- **`procfs` (`/proc`):** Process state, memory maps, file descriptors, `cgroup` statistics.
- **`sysfs` (`/sys`):** Kernel device hierarchy, bus topology, power management telemetry.
- **`devfs` (`/dev`):** Dynamic device node creation, uevent hotplug triggers, `/dev/auditpipe`.

### Layer 5: Time-Travel Snapshotting & Self-Healing Engine
- **Point-in-Time Rollbacks:** Instantaneous zero-space snapshot creation and system state restoration.
- **Merkle Tree Integrity:** Automatic background scrubbing and self-healing data repair on bit rot detection.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | VFS Core | Implement VFS inode/dentry table, mount table, and POSIX syscall dispatchers. | Implemented |
| **Milestone 2** | Pseudo-Filesystems | Implement `procfs`, `sysfs`, `devfs`, and `tmpfs` kernel telemetry virtual filesystems. | Implemented |
| **Milestone 3** | CoW & EROFS Engines | Implement ZFS/Btrfs CoW snapshotting, ARC caching, and EROFS immutable image parsing. | Implemented |
| **Milestone 4** | Block & GEOM Layer | Implement GEOM provider stack, Ggate network block device, and LUKS encryption shims. | Implemented |
| **Milestone 5** | Time-Travel & Repair | Implement background Merkle tree scrubbing, self-healing, and point-in-time state rollbacks. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test suites in `src/filesystem/erofs.rs`, `src/filesystem/nextgen_filesystem_suite.rs`, and `src/filesystem/bsd_linux_innovations.rs`.
2. **FS Stress & Crash Recovery Testing:** Power-loss simulation tests verifying atomic Btree transaction log consistency.
3. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.
