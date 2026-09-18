# SigmaOS AI Agent File System Management Guidelines

## 1. Overview
SigmaOS incorporates modern, multi-filesystem storage architectures managed autonomously or interactively by AI agents (such as `SovereignMountManager`, `BtrfsEngine`, `ZfsZpoolManager`, `Luks2CryptVolumeEngine`, and `OpenBsdUnveilManager`). These guidelines define VFS mount point policies, Btrfs subvolume snapshotting, ZFS pool management, LUKS2 disk encryption, and path isolation for AI agents in SigmaOS.

## 2. Core File System Management Principles

### 2.1 Virtual File System (VFS) & Mount Management
- **Mount Management**: AI agents mounting or unmounting filesystems interface with `SovereignMountManager` (`src/kernel/vfs/vfs.rs`).
- **Extended Mount Flags**: Agents enforce mount flags (`MS_SYNCHRONOUS`, `MS_NOATIME`, `MS_NODIRATIME`, `MNT_FORCE`, `MNT_DETACH`) according to performance and security profiles.
- **OpenBSD Securelevel Checks**: Mount table modifications are rejected if the kernel `securelevel` is greater than 0.

### 2.2 Btrfs CoW Subvolumes & Automatic Snapshots
- **Copy-on-Write (CoW)**: Agents leverage `BtrfsEngine` (`src/filesystem/modern_fs.rs`) for zero-cost CoW subvolume creation and transaction rollbacks (`OpenSuseSnapperEngine`).
- **Pre-Update Snapshots**: Prior to package updates or system modifications, agents create atomic read-only Btrfs snapshots (`/@snapshots/pre-update-<id>`).

### 2.3 ZFS Storage Pools & HAMMER2 PFS
- **ZFS Zpools & Dataset Management**: Agents manage ZFS pools (`ZfsZpool`), ZFS ARC caches (`ZfsArcCacheEngine`), dataset quotas, and raidz/mirror vdev configurations.
- **DragonFly BSD HAMMER2 PFS**: Agents manage HAMMER2 Pseudo-Filesystems (PFS) for multi-version inode tracking and cluster consensus.

### 2.4 LUKS2 Disk Encryption & Memory Protection
- **AES-XTS Encryption**: Agents managing encrypted volumes interface with `Luks2CryptVolume` using AES-XTS 256/512-bit cipher suites.
- **Key Ring Protection**: Encryption keys are stored in kernel keyrings (`SimpleKeyring`) protected by memory-wiping routines.

### 2.5 Path Unveil & POSIX File Locking
- **OpenBSD Unveil Sandboxing**: AI agents restricted by `unveil()` can only access explicitly unveiled VFS paths.
- **Flock & Advisory Locks**: File operations adhere to POSIX `fcntl` / BSD `flock` file locking semantics to guarantee atomic concurrent updates.

---
*Maintained by the SigmaOS Storage & File System Steering Committee.*
