# SigmaOS Filesystem Subsystem: Gap Analysis & Strategic Action Plan

## Executive Overview
This document provides a comprehensive comparative gap analysis between SigmaOS's virtual filesystem (VFS) and block storage capabilities against production Linux (v6.8+ LTS) and BSD (FreeBSD 14, OpenBSD 7.4, DragonflyBSD 6.4) standards. It defines a 4-phase strategic action plan to elevate SigmaOS to parity with enterprise-grade operating systems.

---

## 1. Subsystem Gap Analysis

| Feature / Subsystem | Linux Standard (v6.8+) | BSD Standard (FreeBSD/OpenBSD) | SigmaOS Current State | Priority Gap |
| :--- | :--- | :--- | :--- | :--- |
| **Virtual Filesystem (VFS)** | Lockless `dcache` RCU lookup, `path_lookupat` | VFS vnode operations (`vop_lookup`), namecache | Synchronous mutex VFS (`VirtualFilesystem`) | **High** — Single lock contention under parallel I/O |
| **Copy-on-Write (CoW) FS** | Btrfs, Bcachefs, OverlayFS lowerdir/upperdir | FreeBSD ZFS / Dragonfly HAMMER2 | `CowSnapshotManager` Btrfs-inspired subvolumes | **Medium** — Needs transactional Bcachefs multi-tier caching |
| **Read-Only / Flash FS** | EROFS (Enhanced Read-Only FS), F2FS | OpenBSD ffs2 / softdep | Basic EROFS superblock parsing | **Medium** — Needs LZ4/ZSTD block decompression support |
| **Process / System FS** | `procfs` (`/proc`), `sysfs` (`/sys`), `configfs` | FreeBSD `procfs`, `devfs` | Basic `procfs` and `sysfs` stubs | **High** — Missing rich process telemetry endpoints |
| **Extended Attributes & ACLs** | POSIX.1e ACLs, xattr namespaces (`user`, `security`) | FreeBSD `extattr`, NFSv4 ACLs | Minimal security attribute stubs | **High** — Essential for Landlock & SELinux contexts |

---

## 2. Strategic 4-Phase Roadmap

### Phase 1: Lockless VFS Path Resolution (Months 1–2)
* Transition `VirtualFilesystem` from global mutex locking to lockless Read-Copy-Update (RCU) path lookups (`dcache_rcu_lookup`).
* Implement per-cpu mount namespace cache lines to eliminate lock contention on `/proc`, `/sys`, and `/dev`.

### Phase 2: Enhanced Flash & Read-Only Storage (Months 3–4)
* Implement block-level LZ4 and ZSTD decompression pipelines for EROFS system image mounting.
* Add F2FS flash-friendly direct allocation logging for NVMe and eMMC storage media.

### Phase 3: Enterprise CoW & Multi-Tier Caching (Months 5–6)
* Integrate Bcachefs-inspired multi-tier caching (RAM -> NVMe SSD -> HDD) into `CowSnapshotManager`.
* Implement ZFS-style ARC (Adaptive Replacement Cache) for page cache management.

### Phase 4: Extended Security Attributes & ACL Enforcement (Months 7–8)
* Implement full POSIX.1e extended attribute (`xattr`) support across all VFS inodes.
* Integrate security attribute namespaces (`security.selinux`, `security.landlock`) directly into VFS lookup paths.
