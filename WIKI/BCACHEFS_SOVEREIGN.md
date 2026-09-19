# SigmaOS Sovereign bcachefs — Copy-on-Write Filesystem

## Overview

SigmaOS implements a **bcachefs-inspired copy-on-write filesystem** in 100% safe Rust (`src/fs/bcachefs_sovereign.rs`). bcachefs was developed by Kent Overstreet and merged into Linux 6.7 (November 2023). It is the newest major Linux filesystem and combines the best of btrfs, ZFS, and ext4.

## bcachefs Key Features (Implemented)

| Feature | Status | Notes |
|---------|--------|-------|
| Copy-on-Write (CoW) | ✅ | All writes non-destructive |
| Pure-Rust CRC32c checksums | ✅ | No hardware CRC instructions needed |
| Inline data | ✅ | Files < 3KB stored in inode |
| Extents (B-tree leaves) | ✅ | Large files stored in extents |
| Snapshots | ✅ | Writable and read-only |
| Reflinks (CoW clones) | ✅ | O(1) file copy, shared extents |
| Compression metadata | ✅ | lz4/gzip/zstd simulated ratios |
| UMEM checksumming | ✅ | `sovereign_crc32c()` pure-Rust |

## Architecture

### B-Tree Key Space

bcachefs organizes all metadata as keys in a B-tree:

```
B-tree key: (inode, offset, snapshot_id)
           → BcachefsExtent (physical device offset, checksum, compression)
```

### Extent Layout

```
Inode 1000, offset 0:
  ├── extent[0]: device_offset=0x10000 len=4096 crc32c=0xABCD compression=zstd
  ├── extent[1]: device_offset=0x11000 len=4096 crc32c=0xEF01 is_cow_shared=true
  └── extent[2]: device_offset=0x12000 len=2048 crc32c=0x2345
```

## Usage

```rust
// Create a 100GB volume
let mut vol = SovereignBcachefsVolume::new("sigmaos-root", 100);

// Create inodes
let ino = vol.create_inode(0, 0, 0o644); // uid=0 gid=0 mode=rw-r--r--

// Write data — small files use inline storage automatically
vol.write_inode(ino, 0, b"small file"); // inline (< 3KB)
vol.write_inode(ino, 0, &vec![0u8; 8192]); // extent-based (>= 3KB)

// Snapshots
let snap_id = vol.create_snapshot("before-update", false); // read-only
let writable_snap = vol.create_snapshot("working", true);

// Reflinks — O(1) file copy (like `cp --reflink=always`)
let src_ino = vol.create_inode(0, 0, 0o644);
let dst_ino = vol.create_inode(0, 0, 0o644);
vol.write_inode(src_ino, 0, &vec![0u8; 65536]);
vol.reflink(src_ino, dst_ino); // dst shares src's extents — zero copy
```

## CRC32c Checksum

Pure-Rust implementation — no `sse4.2` instruction required:

```rust
let data = b"verify me";
let checksum = sovereign_crc32c(data);
let extent = BcachefsExtent::new(ino, 0, data.len() as u32, data);
assert!(extent.verify(data));   // passes
assert!(!extent.verify(b"bad")); // fails — corruption detected
```

## Linux bcachefs Parity

| bcachefs Feature | SigmaOS Implementation |
|-----------------|------------------------|
| Unified B-tree | `Vec<BcachefsExtent>` per inode |
| `bcachefs format` | `SovereignBcachefsVolume::new()` |
| Inline data | `BcachefsInode::inline_data` (< 3KB) |
| Snapshots | `BcachefsSnapshot` with parent_id |
| Reflinks | `vol.reflink(src, dst)` → shared extents |
| CRC32c | `sovereign_crc32c()` pure Rust |
| Compression types | `CompressionType::{Lz4, Gzip, Zstd}` |
| `bcachefs show-super` | `SovereignBcachefsVolume::label / uuid` |

## Compression Comparison

| Algorithm | Simulated Ratio | Speed |
|-----------|----------------|-------|
| None | 100% (no reduction) | Fastest |
| LZ4 | 70% | Very fast |
| Gzip | 55% | Medium |
| Zstd | 45% | Fast + best ratio |

## Tests

6 unit tests, all passing:

- `test_crc32c_checksum_deterministic` — same data → same checksum, different data → different
- `test_extent_create_verify` — corrupted data fails verification
- `test_inline_inode_write` — small writes stored inline, zero extents
- `test_snapshot_creation` — multiple snapshots with unique IDs
- `test_reflink_cow` — reflinked extents marked `is_cow_shared=true`
- `test_compression_types` — zstd best ratio, ratios ordered correctly

## Source

[`src/fs/bcachefs_sovereign.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/fs/bcachefs_sovereign.rs)
