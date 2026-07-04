# SigmaOS Storage Stack

---

## Storage Architecture

```
Application / VFS layer
    │  open() / read() / write() / ioctl()
    ▼
VFS (Virtual Filesystem Switch)
    │  inode / dentry / file operations
    ├── SigmaFS (native CoW)   ← Phase G
    ├── Ext4 + JBD2 journal    ← ✅ Done
    ├── FAT32                  ← ✅ Done
    ├── Tmpfs                  ← Phase G
    └── procfs / devfs         ← Partial
    ▼
Unified Buffer Cache (UBC)    ← Phase G
    │  LRU + read-ahead
    ▼
Block I/O layer
    │  I/O scheduler (deadline / CFQ)
    ▼
Block device drivers
    ├── NVMe    ✅ drivers/storage/sigma_nvme.cpp
    ├── AHCI    ⬜ Phase G
    └── VirtIO-blk ⬜ Phase G
```

---

## SigmaFS — Native Filesystem (Phase G)

SigmaFS is SigmaOS's native copy-on-write journaling filesystem:

**Design goals:**
- Atomic operations: crash → no partial writes
- Instant snapshots: O(1) snapshot creation via CoW
- Deduplication: content-addressed block storage
- Encryption: optional per-directory AES-256-GCM
- Compression: LZ4 (fast) or ZSTD (ratio) per-directory

**On-disk layout:**
```
Block 0:    Superblock
Block 1-N:  Journal (JBD2-compatible)
Block N+1:  Block group descriptors
Block ...:  Data blocks (CoW B-tree indexed)
```

**Status:** Early design — implementation is Phase G.

---

## Ext4 Support

- Read/write with JBD2 ordered journaling ✅
- Superblock parsing ✅
- Directory B-tree ✅
- Extents-based file layout ✅
- 64-bit block addresses ✅

```bash
# Mount an Ext4 partition
sigma-mount /dev/sda2 / -t ext4

# Check filesystem
sigma-fsck /dev/sda2

# Resize
sigma-resize2fs /dev/sda2 50G
```

---

## ZFS Integration (`tools/sigma_zfs.cpp`)

SigmaOS includes a ZFS compatibility shim for importing ZFS pools:

```bash
sigma-zpool import tank
sigma-zfs list
sigma-zfs snapshot tank/data@backup
sigma-zfs rollback tank/data@backup
```

Full ZFS kernel module is Phase H.

---

## Volume Management (`storage/vfs/`)

```bash
# List block devices
sigma-lsblk

# Partition a disk
sigma-fdisk /dev/sda

# Create a filesystem
sigma-mkfs.sigmafs /dev/sda2
sigma-mkfs.ext4 /dev/sda2

# Logical volumes (LVM equivalent)
sigma-lvm create --name data --size 100G /dev/sda3
sigma-lvm snapshot data pre-update
```

---

## Encryption

```bash
# Create an encrypted volume (CryptFS)
sigma-cryptsetup luksFormat /dev/sda3
sigma-cryptsetup luksOpen /dev/sda3 secure_data
sigma-mkfs.sigmafs /dev/mapper/secure_data

# TPM2-backed auto-unlock (Phase G — after CryptFS fix #1009)
sigma-cryptsetup tpm2-enroll /dev/sda3
```

---

## Source Files

| File | Purpose |
|------|---------|
| `fs/vfs.c` | VFS core |
| `fs/ext4.c` | Ext4 driver |
| `fs/ext4_journal.c` | JBD2 journaling |
| `fs/fat32.c` | FAT32 driver |
| `fs/procfs.c` | /proc filesystem |
| `fs/devfs.c` | /dev filesystem |
| `fs/sigmafs/` | Native SigmaFS (Phase G) |
| `fs/tmpfs.c` | RAM-backed tmpfs (Phase G) |
| `fs/fuse.c` | FUSE userspace FS bridge |
| `storage/README.md` | Storage subsystem docs |
| `tools/sigma_zfs.cpp` | ZFS compat shim |

---

*See also: [Kernel](Kernel) · [Architecture-Overview](Architecture-Overview) · [Recovery](Recovery)*
