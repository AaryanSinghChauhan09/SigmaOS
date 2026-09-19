# AI Agent File System Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                      AI Agent Storage & File System Manager                     |
|        (SovereignMountManager, BtrfsEngine, ZfsZpool, Luks2CryptVolume)         |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                      Virtual File System (VFS) Router                           |
|       (Inode Lookup, Path Resolution, POSIX Locks, OpenBSD Unveil Policy)       |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
|  Btrfs CoW Engine     |   |  ZFS Zpool & ARC      |   |  LUKS2 AES-XTS Crypto |
| (Subvolumes, Snapper) |   | (vdevs, Datasets)     |   | (Volume Encryption)   |
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                         Storage Drivers & NVMe Controller                       |
|           (ModernNvmeDriver, AhciStorageDriver, VirtIO Block Backend)           |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Virtual File System (VFS) Layer**:
   - Manages mount points, inode tables, directory entry caches (`dcache`), and path resolution.
   - Enforces OpenBSD `securelevel` checks and `unveil()` path restriction tables for sandbox processes.

2. **Modern Filesystem Drivers**:
   - **Btrfs CoW Subvolume Engine**: Zero-allocation Btrfs subvolume tree management, extent allocation, and snapper transaction rollbacks.
   - **ZFS Zpool Engine**: ZFS vdev management, ARC cache hit/miss optimization, and dataset snapshot cloning.
   - **HAMMER2 Engine**: Multi-version inode tracking and PFS cluster consensus.

3. **LUKS2 Encryption Layer**:
   - Provides transparent AES-XTS block-level encryption for storage partitions.
   - Key material managed in locked memory buffers with automatic wiping on unmount.

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
