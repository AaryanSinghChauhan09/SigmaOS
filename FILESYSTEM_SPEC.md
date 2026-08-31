# 💾 SigmaFS: Sovereign High-Performance Filesystem Specification

This document details the architectural specification for **SigmaFS**, the native filesystem of SigmaOS.

---

## 🏗️ 1. Disk Layout Structure

SigmaFS splits a physical block device (or partition) into structured segments:

```
+---------------+----------------+----------------+----------------+
|  Superblock   |   Inode Table  |  Journal Log   |   Data Blocks  |
|   (Block 0)   |   (Blocks 1-8) |  (Blocks 9-16) |  (Blocks 17+)  |
+---------------+----------------+----------------+----------------+
```

### 1.1 Superblock (Block 0)
Contains overall metadata of the filesystem:
- **Magic Number:** `0x5349474D` (ASCII for `SIGM`)
- **Block Size:** Default `4096` bytes.
- **Total Inodes:** Maximum count of allocatable file entries.
- **Root Inode Index:** Block pointer to root directory inode.

---

## 🔒 2. Blockchain Audit Trails (AuditBlock)

To ensure tamper-proof secure system audits against security breaches:
- Every write, rename, or permission modification logs a transaction hash.
- Log entries are chained together using a cryptographic SHA-256 block hash (`AuditBlock`), matching enterprise blockchain architectures.
- Tampering with historical file state invalidates the chain, triggering a self-healing rollback to a previous snapshot.

---

## 🚀 3. Extent-Based Data Allocation

To avoid fragmentation common in older FAT32 or Ext2 block mapping, SigmaFS allocates sequential files using **extents**:
- An extent represents a range of contiguous physical blocks: `(start_block_id, block_count)`.
- Each inode supports up to 4 direct extents, enabling multi-gigabyte sequential NVMe writes with near-zero pointer overhead.
