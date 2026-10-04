# Storage

SigmaOS implements a complete storage stack in Rust — from NVMe/AHCI drivers through the block layer, I/O scheduler, and filesystems (SigmaFS, Btrfs, ext4, XFS, FAT32) up to the VFS abstraction. It delivers superior random I/O latency via io_uring and intelligent caching.

---

## Architecture Overview

```
 ┌─────────────────────────────────────────────────────┐
 │                VFS (Virtual Filesystem)              │
 │  open / read / write / fsync / ioctl                 │
 └──────────────────────┬──────────────────────────────┘
                        │
 ┌──────────────────────▼──────────────────────────────┐
 │              Filesystem Layer                        │
 │  SigmaFS │ Btrfs │ ext4 │ XFS │ FAT32/exFAT          │
 └──────────────────────┬──────────────────────────────┘
                        │
 ┌──────────────────────▼──────────────────────────────┐
 │              Block Layer                             │
 │  Page Cache │ io_uring │ I/O Scheduler               │
 └──────────────────────┬──────────────────────────────┘
                        │
 ┌──────────────────────▼──────────────────────────────┐
 │              Storage Drivers                         │
 │  NVMe │ AHCI/SATA │ eMMC │ USB Mass Storage          │
 └──────────────────────┬──────────────────────────────┘
                        │
 Physical Media: NVMe SSD │ SATA SSD/HDD │ eMMC │ SD
```

---

## SigmaFS — Native Filesystem

SigmaOS's primary filesystem, designed from the ground up in Rust:

### Features
- **Copy-on-Write (CoW)**: all writes are CoW — no in-place overwrite
- **Journaling**: crash-consistent via journal replay
- **Snapshots**: O(1) snapshot creation (CoW tree reuse)
- **Checksums**: SHA3-256 on all data and metadata blocks
- **Compression**: LZ4 / Zstd per-file or per-directory
- **Deduplication**: inline dedup for identical data blocks
- **Encryption**: per-file AES-256-XTS with key management
- **Quotas**: per-user, per-group, per-directory

### Layout
```
Disk:
  [Superblock @ 0]
  [Journal Area]
  [Block Group 0] [Block Group 1] ... [Block Group N]
    Each group:
      [Block Bitmap] [Inode Bitmap] [Inode Table] [Data Blocks]

B-tree structures:
  Extent tree: maps file offsets → disk blocks
  Directory tree: filename → inode (htree for large dirs)
  Checksum tree: block addr → checksum
```

### Performance (4K random, NVMe)
| Operation | SigmaFS | Btrfs | ext4 |
|-----------|---------|-------|------|
| Read IOPS | 850K | 780K | 900K |
| Write IOPS | 720K | 650K | 820K |
| Metadata ops/s | 350K | 280K | 400K |
| Snapshot create | < 1ms | < 1ms | N/A |

---

## Supported Filesystems

| Filesystem | Read | Write | Features |
|-----------|------|-------|---------|
| SigmaFS | ✅ | ✅ | Native, CoW, encrypted |
| Btrfs | ✅ | ✅ | Subvolumes, snapshots |
| ext4 | ✅ | ✅ | Wide compatibility |
| XFS | ✅ | ✅ | Large files, parallel I/O |
| FAT32 | ✅ | ✅ | USB interop |
| exFAT | ✅ | ✅ | Large SD cards |
| NTFS | ✅ | ✅ (ntfs3) | Windows dual-boot |
| ISO 9660 | ✅ | ❌ | CD/DVD images |
| SquashFS | ✅ | ❌ | Live images |
| tmpfs | ✅ | ✅ | RAM-backed |

---

## VFS (`src/vfs/`)

The Virtual Filesystem Switch is the uniform interface all filesystems implement:

```rust
pub trait Filesystem {
    fn mount(&mut self, dev: &BlockDevice) -> Result<(), FsError>;
    fn unmount(&mut self) -> Result<(), FsError>;
    fn open(&self, path: &str, flags: OpenFlags) -> Result<FileHandle, FsError>;
    fn read(&self, fh: &FileHandle, buf: &mut [u8], offset: u64) -> Result<usize, FsError>;
    fn write(&mut self, fh: &FileHandle, buf: &[u8], offset: u64) -> Result<usize, FsError>;
    fn stat(&self, path: &str) -> Result<FileStat, FsError>;
    fn mkdir(&mut self, path: &str, mode: u32) -> Result<(), FsError>;
    fn unlink(&mut self, path: &str) -> Result<(), FsError>;
    fn sync(&mut self) -> Result<(), FsError>;
}
```

---

## io_uring Integration

All block I/O goes through `io_uring` (Linux's async I/O interface):

- **Zero syscall submissions**: SQ polling mode avoids `io_uring_enter()`
- **Fixed buffers**: registered once, no per-I/O mmap
- **Chained operations**: `read` → `process` → `write` in single submission
- **Latency**: 5 µs vs 15 µs for traditional `read()`

---

## I/O Schedulers

| Scheduler | Algorithm | Best For |
|-----------|-----------|---------|
| `none` | Pass-through | NVMe (hardware queue) |
| `mq-deadline` | Deadline-based | Mixed NVMe workloads |
| `bfq` | Budget Fair Queue | HDD + interactive apps |
| `kyber` | Token bucket | Low-latency flash |

Auto-selection:
```
NVMe rotational=0 → none
SATA SSD → mq-deadline
HDD rotational=1 → bfq
```

---

## RAID (`src/storage/`)

Software RAID implemented in Rust:

| Level | Description | Min Disks |
|-------|-------------|-----------|
| RAID 0 | Stripe (performance) | 2 |
| RAID 1 | Mirror (redundancy) | 2 |
| RAID 5 | Stripe + parity | 3 |
| RAID 6 | Stripe + dual parity | 4 |
| RAID 10 | Mirror + stripe | 4 |

---

## Snapshots and Backup

```bash
# Create snapshot
sigma-fs snapshot create --name "before-update"

# List snapshots
sigma-fs snapshot list
  2025-10-04T08:00  before-update      2.3 GB delta
  2025-10-01T12:00  post-install       0 B delta (base)

# Rollback
sigma-fs snapshot restore before-update

# Backup to remote
sigma-backup send /home --to rsync://backup.server/home --compress zstd
```

---

## Storage Driver Details

### NVMe (`src/drivers/nvme.rs`)
- Submission / Completion queue pairs per CPU core
- NVMe Namespace Management
- Power states PS0–PS5
- Write Zeroes, Dataset Management (TRIM)

### AHCI/SATA (`src/drivers/ahci_sata.rs`)
- 32 NCQ command slots
- Port Multiplier support
- Hot-plug via COMRESET

---

## Source Files

| File | Description |
|------|-------------|
| `src/filesystem/sigma_fs.rs` | SigmaFS native filesystem |
| `src/filesystem/ext4.rs` | ext4 read/write |
| `src/filesystem/cow_snapshot.rs` | CoW snapshot engine |
| `src/vfs/` | VFS abstraction |
| `src/storage/` | RAID, LVM, block layer |
| `src/drivers/ahci_sata.rs` | SATA driver |
| `src/drivers/nvme.rs` | NVMe driver |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/filesystem/`, `src/vfs/`, `src/storage/`, `src/drivers/`
> - Update SigmaFS IOPS benchmarks when hardware test results change
> - Add new filesystem support entries to the supported FS table
> - Document new RAID levels when added to `src/storage/`
