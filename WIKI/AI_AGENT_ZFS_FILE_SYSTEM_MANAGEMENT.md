# AI Agent ZFS File System Management Guidelines in SigmaOS

## Overview

SigmaOS implements a ZFS-inspired zpool and Copy-on-Write (CoW) storage subsystem in `src/filesystem/zfs_inspired.rs` and `src/filesystem/modern_fs.rs`. This document defines operational protocols and interfaces for autonomous AI Agents (Sentinel 🛡️, Bolt ⚡, and Palette 🎨) to inspect, manage, and optimize ZFS storage pools, datasets, Adaptive Replacement Cache (ARC/L2ARC), ZFS Intent Logs (ZIL/SLOG), Merkle checksum integrity verification, and dataset snapshot replication.

---

## Architecture & Subsystems

```
┌─────────────────────────────────────────────────────────────────┐
│              AI Agent ZFS Orchestration Manager                 │
├─────────────────────────────────────────────────────────────────┤
│  ZFS ARC/L2ARC Adaptive Agent   │  Zpool vdev Governor          │
│  Dataset & CoW Snapshot Manager │  Merkle Checksum & Scrubbing   │
└────────────────────────────────┬────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│               SigmaOS ZFS Core (`zfs_inspired.rs`)              │
├─────────────────────────────────────────────────────────────────┤
│  - StoragePool & VdevAllocation (Mirror, RaidZ1/2/3, SLOG, L2ARC)│
│  - ZfsArcCacheEngine (MRU vs MFU balance & vfs.zfs.arc_max)     │
│  - ZfsDatasetEngine (Quota, Reservation, Compression, Dedup)  │
│  - MerkleTreeVerification (fletcher4, sha256, xxhash)           │
│  - CoW Snapshot Timeline & Send/Recv Stream Replication         │
└─────────────────────────────────────────────────────────────────┘
```

---

## Key Operational Components

### 1. Zpool & Vdev Configuration Management (`StoragePool`)
- **Vdev Types Supported**: Single, Mirror, RaidZ1 (1 parity), RaidZ2 (2 parity), RaidZ3 (3 parity), Log (ZIL/SLOG intent log), Cache (L2ARC NVMe extension), Spare.
- **AI Agent Directives**:
  - Automatically balance ZIL writes to high-endurance NVMe SLOG vdevs under synchronous I/O spikes.
  - Dynamically attach L2ARC NVMe devices when page cache read hit ratios drop below 85%.

### 2. Adaptive Replacement Cache Engine (`ZfsArcCacheEngine`)
- **Dual-Tier Cache Management**:
  - **Most Recently Used (MRU)**: Stores recently accessed pages.
  - **Most Frequently Used (MFU)**: Stores repeatedly accessed pages.
- **Dynamic Tuning**:
  - Agents adjust `vfs.zfs.arc_max` dynamically based on real-time process memory pressure telemetry (`PSI` & DAMON).
  - Promotes metadata extents into L2ARC to accelerate VFS file lookup operations.

### 3. Dataset Configuration & Compression (`ZfsDatasetEngine`)
- **Compression Codecs**: `LZ4` (default low-latency), `ZSTD` (high-compression archival), `Gzip`, `ZLE`, `LZJB`.
- **Deduplication Engine**: Maintains ZFS-inspired block hash deduplication tables to eliminate duplicate data blocks across datasets.

### 4. Merkle Tree Checksumming & Data Scrubbing (`MerkleTreeVerification`)
- **Supported Checksums**: `fletcher4`, `sha256`, `xxhash`.
- **Self-Healing Mechanics**: During background scrub operations, if a data block checksum fails, the agent uses mirror/RaidZ parity blocks to repair damaged blocks automatically.

### 5. CoW Snapshot Timeline & Replication
- **Atomic Snapshots**: Before system livepatching or package updates, agents invoke `zfs snapshot` to establish recovery restore points.
- **Send/Recv Replication**: Streams incremental CoW snapshots across devices for cross-device mesh backup (`src/orchestration/cross_device.rs`).

---

## AI Agent Operating Boundaries

### ✅ Always Do:
- Generate an atomic ZFS dataset snapshot before executing package installations, livepatching, or live parameter tuning.
- Monitor S.M.A.R.T. disk telemetry before initiating zpool scrub cycles.
- Verify block checksums using `fletcher4` or `sha256` after raw disk reads.

### ⚠️ Ask First:
- Destroying ZFS pools (`zpool destroy`) or removing active vdevs.
- Altering ZFS pool feature flags or upgrading pool versions across non-backward-compatible releases.

### 🚫 Never Do:
- Disable checksum verification on production datasets.
- Force-import faulted zpools (`zpool import -f`) without prior health diagnostic scans.
- Allocate `vfs.zfs.arc_max` beyond physical NonPagedPool limits.

---

## Diagnostic CLI & Verification Commands

```bash
# Query ZFS pool status & vdev health
sigpool status

# Check ARC cache hit/miss statistics and MRU/MFU balance
sigarc stats

# Initiate background Merkle checksum scrub on zpool
sigpool scrub start rpool

# Create atomic CoW dataset snapshot
sigds snapshot rpool/ROOT@pre-update-2026

# Rollback dataset to stable snapshot
sigds rollback rpool/ROOT@pre-update-2026
```
