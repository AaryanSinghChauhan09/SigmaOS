# SigmaOS AI Agent Various File System Operation Management Guidelines

## 1. Executive Summary & Overview

SigmaOS supports a multi-filesystem storage architecture, offering native compatibility with key Linux, BSD, and modern storage filesystems (Ext4, Btrfs, ZFS, XFS, F2FS, HAMMER2, SquashFS, OverlayFS, and FAT32/exFAT). To perform filesystem management, formatting, mounting, snapshotting, and layer stacking safely, AI agents must understand the operational contracts, mount flags, and performance characteristics of each supported filesystem.

This document establishes the official guidelines and architectural standards for AI agents managing various file systems in SigmaOS.

---

## 2. Supported File Systems Matrix & Capabilities

| File System Type | Identifier | Primary Use Case | Key Engine / Module | Core Operational Features |
| :--- | :--- | :--- | :--- | :--- |
| **Ext4** | `ext4` | Standard rootfs, general Linux storage | `src/filesystem/` | JBD2 journaling, extent trees, delayed allocation |
| **Btrfs** | `btrfs` | CoW rootfs, atomic snapshots | `BtrfsEngine` | Copy-on-Write B-Trees, zero-cost subvolume snapshots |
| **ZFS** | `zfs` | Enterprise storage, raidz pools | `ZfsZpool` | Zpool vdev management, ARC cache, ZFS dataset quotas |
| **XFS** | `xfs` | High-throughput parallel I/O | `src/filesystem/` | Allocation Groups (AGs), direct I/O (`O_DIRECT`) |
| **F2FS** | `f2fs` | NVMe / SSD flash storage | `src/filesystem/` | Log-structured, wear-leveling, inline dentry layout |
| **HAMMER2** | `hammer2` | BSD multi-version clustering | `DragonFlyHammerFs` | Pseudo-Filesystems (PFS), multi-version inodes |
| **SquashFS** | `squashfs` | Immutable app/system layers | `SteamOsAtomicAbImageUpdateEngine` | High-ratio read-only zstd compression, A/B slots |
| **OverlayFS** | `overlay` | Ephemeral containers, immutable root | `src/package/declarative_app.rs` | Lowerdir/upperdir/workdir layer stacking |
| **FAT32/exFAT** | `vfat` | EFI System Partition (ESP) | `src/boot/bootloader.rs` | `/boot/efi` ESP bootloader stage-1/stage-2 storage |

---

## 3. File System Operations Protocol for AI Agents

### 3.1 VFS Mount & Flag Management (`SovereignMountManager`)

When mounting or remounting filesystems, AI agents interface with `SovereignMountManager`:

1. **Extended Mount Flags**:
   - `MS_SYNCHRONOUS`: Force synchronous metadata updates for critical recovery partitions.
   - `MS_NOATIME` / `MS_NODIRATIME`: Disable access time updates to reduce NVMe write amplification.
   - `MS_REMOUNT`: Dynamically alter filesystem permissions (e.g. remounting read-only `/` to `rw` during updates).
2. **OpenBSD Securelevel Lockout**:
   - Filesystem mount table alterations are strictly prohibited if `securelevel > 0`.

---

### 3.2 Btrfs Subvolumes & Atomic Snapshots

1. **Pre-Update Snapshots**:
   - Prior to system modifications, create read-only Btrfs subvolume snapshots (`/@snapshots/pre-update-<id>`).
2. **Transactional Rollback**:
   - If an update transaction fails, invoke `OpenSuseSnapperEngine` to atomically revert the default subvolume pointer to the pre-update snapshot.

---

### 3.3 ZFS Storage Pools & Dataset Quotas

1. **Pool Administration**:
   - Agents manage ZFS pools (`zpool create`, `zpool scrub`, `zpool status`) and monitor pool health.
2. **ARC Memory Adaptation**:
   - Adjust `ZfsArcCacheEngine` size dynamically depending on physical memory pressure to prevent OOM panics.

---

### 3.4 Immutable Layers & OverlayFS Stacking

1. **SquashFS Read-Only Slots**:
   - Package shards and A/B system rootfs layers are deployed as immutable zstd-compressed SquashFS images.
2. **OverlayFS Layer Assembly**:
   - Userland app containers combine read-only SquashFS `lowerdir` images with writable ephemeral `upperdir` and `workdir` mounts to isolate application changes.

---

## 4. Verification Protocol

AI agents managing or modifying filesystem operations must execute verification:

1. **Unit & Subsystem Testing**: Run `./run_sigma_tests.sh` to confirm filesystem driver stability.
2. **Stress Matrix**: Execute `tests/stress_and_fuzz_tests.rs` to validate CoW snapshotting, OverlayFS layer creation, and VFS mount lock safety under load.

---

*Approved by the SigmaOS Storage & File System Architecture Committee.*
