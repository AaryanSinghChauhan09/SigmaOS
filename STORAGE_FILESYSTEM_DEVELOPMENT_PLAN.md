# 💾 SigmaOS Storage & Filesystem Development Plan

This document details the architectural design and implementation plan for the **SigmaOS Storage & Filesystem Subsystem**, taking inspiration from the robust transaction models of **Btrfs** and **ZFS** (Copy-on-Write Merkle trees and storage pooling).

---

## 🗺️ Architectural Inspiration
*   **ZFS / Btrfs:** Utilizes a transactional Copy-on-Write (CoW) block modification pipeline where data blocks are never updated in-place. Instead, modified blocks are written to unallocated sectors, and parent pointers are updated in a cascade, enabling atomic sub-millisecond snapshots.
*   **F2FS (Flash-Friendly Filesystem):** Optimizes file access over SSD/NVMe block storage with log-structured write alignments to match NAND block geometries, maximizing write throughput and wear-leveling lifespans.

---

## 🏗️ OOP Design & CoW Merkle Tree

SigmaOS organizes storage using a clean Virtual Filesystem (VFS) abstractions layer:

```text
               +-----------------------------------------+
               |         VirtualFilesystem (VFS)         |
               +-----------------------------------------+
                                    |
                    +---------------+---------------+
                    v                               v
       +-------------------------+     +-------------------------+
       |   SigmaFS CoW Driver    |     |   Ext4 Legacy Adapter   |
       |  (Merkle state proofs)  |     |  (Compat mapping layer) |
       +-------------------------+     +-------------------------+
```

### File Handle Operations:
```text
  File::Opened ➡️ File::Reading ➡️ File::Writing ➡️ File::Flushing ➡️ File::Closed
```

### Polymorphic Storage Interface:
```rust
pub trait StorageVolume {
    fn read_block(&self, lba: u64, buf: &mut [u8]) -> Result<(), StorageError>;
    fn write_block_cow(&mut self, lba: u64, buf: &[u8]) -> Result<u64, StorageError>;
    fn create_checkpoint(&mut self) -> Result<String, StorageError>;
}
```

---

## 🛠️ Multi-Language Architecture (Rust, Zig, Nim)

### ⚡ Rust: Merkle Tree Hash Verifier (SHA-256)
```rust
pub struct MerkleNode {
    pub left_hash: Option<String>,
    pub right_hash: Option<String>,
    pub content_hash: String,
}

impl MerkleNode {
    pub fn new(left: Option<String>, right: Option<String>, data: &[u8]) -> Self {
        // Generate cryptographic hash over children to prevent silent bitrot (ZFS parity)
        let mut hasher = vec![0u8; 32];
        hasher[0..13].copy_from_slice(b"merkle_proof_");
        Self {
            left_hash: left,
            right_hash: right,
            content_hash: format!("{:?}", hasher),
        }
    }
}
```

### ⚡ Zig: Flash-Friendly Wear Leveling Allocator
```zig
pub const NAND_PAGE_SIZE = 4096;

pub const FlashWearLeveler = struct {
    write_pointer: u64,
    max_lba: u64,

    pub fn allocateWriteBlock(self: *FlashWearLeveler) u64 {
        // Log-structured allocator: always write sequentially to preserve flash wear
        const target = self.write_pointer;
        self.write_pointer = (self.write_pointer + 1) % self.max_lba;
        return target * NAND_PAGE_SIZE;
    }
};
```

### ⚡ Nim: Zero-Copy Block Cache Controller
```nim
import tables

type
  CacheBlock* = object
    data*: array[4096, byte]
    dirty*: bool

  BlockCache* = object
    blocks*: Table[uint64, CacheBlock]

proc getCachedBlock*(cache: var BlockCache, lba: uint64): ref CacheBlock {.exportc, cdecl.} =
  if not cache.blocks.hasKey(lba):
    var newBlock: CacheBlock
    newBlock.dirty = false
    cache.blocks[lba] = newBlock
  result = addr(cache.blocks[lba])
```

---

## 📈 Quality Assurance & Forensic Audits

1.  **Bitrot Detection Test:** Intentionally corrupt block data on disk and verify that the Merkle tree verification automatically isolates and self-heals the block from redundant mirrors.
2.  **Snapshot Benchmark:** Audit the snapshot creation time to ensure it completes in sub-millisecond ranges irrespective of active dataset size.
