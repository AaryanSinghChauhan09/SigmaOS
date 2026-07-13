# OSS Absorption: Btrfs & ZFS — Advanced Filesystem Layer

> **Status**: 🔄 Active | **Source Projects**: Btrfs (Linux), OpenZFS | **Target Shard**: `SigmaOS sigma-fs Storage Layer`

---

## 1. Executive Summary

SigmaOS's native filesystem (`sigma-fs`) absorbs the most valuable features from both Btrfs and ZFS — the two most advanced production filesystems — while avoiding their respective weaknesses:

- **From Btrfs**: Copy-on-write snapshots, transparent compression, subvolumes, online defragmentation, send/receive for incremental backups
- **From ZFS**: Data integrity checksumming (SHA-256), RAIDZ storage pools, deduplication, ARC-style adaptive caching, scrubbing

`sigma-fs` is implemented in Rust with formal verification of critical data paths.

---

## 2. Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                        SIGMA-FS STACK                            │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │                  VFS INTERFACE                           │    │
│  │  POSIX: open, read, write, mmap, stat, readdir           │    │
│  └──────────────────────────┬───────────────────────────────┘    │
│                             │                                    │
│  ┌──────────────────────────▼───────────────────────────────┐    │
│  │                  SIGMA-FS CORE                           │    │
│  │                                                          │    │
│  │  ┌──────────────┐  ┌───────────┐  ┌──────────────────┐  │    │
│  │  │ Copy-on-Write│  │ Checksums │  │  Compression     │  │    │
│  │  │ (Btrfs-style)│  │ (SHA-256) │  │  (zstd, lz4)    │  │    │
│  │  └──────────────┘  └───────────┘  └──────────────────┘  │    │
│  │                                                          │    │
│  │  ┌──────────────┐  ┌───────────┐  ┌──────────────────┐  │    │
│  │  │  Snapshots   │  │ RAIDZ     │  │  Deduplication   │  │    │
│  │  │ (subvolumes) │  │ (parity)  │  │  (block-level)   │  │    │
│  │  └──────────────┘  └───────────┘  └──────────────────┘  │    │
│  │                                                          │    │
│  └──────────────────────────┬───────────────────────────────┘    │
│                             │                                    │
│  ┌──────────────────────────▼───────────────────────────────┐    │
│  │             BLOCK ALLOCATOR + DEVICE MANAGER             │    │
│  │  NVMe │ SATA SSD │ HDD │ USB │ Network (iSCSI)          │    │
│  └──────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 3. Key Features

### 3.1 Copy-on-Write Snapshots (Btrfs-inspired)

```rust
// kernel/fs/sigma_fs/snapshot.rs
// SPDX-License-Identifier: MIT

pub struct SnapshotManager {
    subvolumes: BTreeMap<SubvolId, Subvolume>,
}

pub struct Subvolume {
    pub id:       SubvolId,
    pub name:     String,
    pub parent:   Option<SubvolId>,
    pub readonly: bool,
    pub created:  SystemTime,
    pub size:     u64,   // Exclusive bytes (not shared with parent)
}

impl SnapshotManager {
    /// Create instant snapshot (O(1) — metadata-only, no data copy)
    pub fn create_snapshot(
        &mut self,
        source: SubvolId,
        name: &str,
        readonly: bool,
    ) -> Result<SubvolId> {
        let snap = self.subvolumes.get(&source)
            .ok_or(FsError::NotFound)?
            .clone_metadata(name, readonly)?;
        let id = snap.id;
        self.subvolumes.insert(id, snap);
        Ok(id)
    }

    /// Send incremental snapshot delta (Btrfs send/receive equivalent)
    pub fn send_delta(
        &self,
        from: SubvolId,
        to: SubvolId,
        writer: &mut dyn Write,
    ) -> Result<u64> {
        let changes = self.diff(from, to)?;
        let mut bytes_sent = 0;
        for change in changes {
            bytes_sent += writer.write(&change.to_binary())?;
        }
        Ok(bytes_sent)
    }
}
```

```bash
# Create snapshot
$ sigma fs snapshot create /home --name "before-upgrade"
Σ [FS] Snapshot created instantly (0ms, CoW — no data copied)

# List snapshots
$ sigma fs snapshot list /home
Σ [FS] Snapshots for /home:
  #1  before-upgrade     2025-11-01  (4.2GB exclusive)
  #2  auto-daily         2025-11-10  (120MB exclusive)
  #3  current            (active)

# Incremental backup via send/receive (like rsync but filesystem-level)
$ sigma fs snapshot send /home#1 /home#3 | ssh backup-server sigma fs snapshot receive /backup/
Σ [FS] Sending incremental delta #1 → #3: 320MB (vs 120GB full)
```

### 3.2 Data Integrity Checksumming (ZFS-inspired)

Every block written to `sigma-fs` gets a SHA-256 checksum. On read, the checksum is verified — **silent data corruption (bit rot) is detected and auto-repaired** from redundant copies.

```bash
# Run a data integrity scrub (verifies all checksums)
$ sigma fs scrub /
Σ [FS] Scrubbing / (500GB, ~3 hours estimated)...
  Verified: 12,847,293 blocks
  Errors:   0 (✅ all data intact)

# If corruption is found on RAID:
Σ [FS] Scrub found 2 corrupted blocks:
  /home/user/photos/2024/IMG_1234.jpg (block 0x1A2B3C — repaired from mirror)
  /var/log/sigma/journal.bin (block 0x4D5E6F — repaired from parity)
  All corruption auto-repaired. Data is safe.
```

### 3.3 Transparent Compression

```bash
# Enable transparent compression on a directory
$ sigma fs compress /home --algorithm zstd --level 3
Σ [FS] Compression enabled on /home:
  Algorithm : zstd (level 3 — balanced speed/ratio)
  Ratio     : 2.1x (120GB → 57GB actual disk usage)
  Throughput: 1.2GB/s compress, 3.8GB/s decompress

# Per-file compression info
$ sigma fs compress info /home/user/Documents/
Σ [FS] /home/user/Documents/:
  Files       : 4,218
  Logical size: 8.2GB
  On-disk     : 3.1GB (ratio: 2.6x)
  Algorithm   : zstd level 3
```

### 3.4 Storage Pools / RAIDZ (ZFS-inspired)

```bash
# Create a RAIDZ1 pool from 3 drives (1 drive parity — survives 1 failure)
$ sigma fs pool create tank --raidz1 /dev/sda /dev/sdb /dev/sdc
Σ [FS] Pool "tank" created:
  Layout   : RAIDZ1 (3 drives, 1 parity)
  Capacity : 2TB usable (3TB raw)
  Mounted  : /tank

# Add a cache drive (L2ARC equivalent)
$ sigma fs pool add-cache tank /dev/nvme0n1
Σ [FS] SSD cache added to pool "tank" (500GB NVMe)

# Check pool health
$ sigma fs pool status tank
Σ [FS] Pool "tank" — ONLINE:
  sda  ONLINE  0 read errors  0 write errors
  sdb  ONLINE  0 read errors  0 write errors
  sdc  ONLINE  0 read errors  0 write errors
  Cache: nvme0n1 — 500GB, 87% hit rate
```

---

## 4. Feature Comparison

| Feature | ext4 | Btrfs | ZFS | sigma-fs |
|:--------|:-----|:------|:----|:---------|
| Copy-on-Write | ❌ | ✅ | ✅ | ✅ |
| Snapshots | ❌ | ✅ | ✅ | ✅ |
| Checksumming | ❌ | CRC32C | SHA-256 | SHA-256 |
| Compression | ❌ | zstd/lzo | lz4/zstd | zstd/lz4 |
| RAID | mdadm | Built-in | RAIDZ | RAIDZ |
| Deduplication | ❌ | ❌ | ✅ | ✅ |
| Send/Receive | ❌ | ✅ | ✅ | ✅ |
| Formal verification | ❌ | ❌ | ❌ | Partial (Rust) |
| License | GPL-2.0 | GPL-2.0 | CDDL | MIT |

---

## 5. References & Standards

- Btrfs — `btrfs.readthedocs.io` (GPL-2.0)
- OpenZFS — `openzfs.org` (CDDL)
- zstd (compression) — `github.com/facebook/zstd` (BSD)
- POSIX filesystem semantics — IEEE Std 1003.1
