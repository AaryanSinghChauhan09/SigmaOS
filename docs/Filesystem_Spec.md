# SigmaFS Filesystem Specification

## Overview

SigmaFS is SigmaOS's native filesystem. It is designed for correctness, security, and performance:
- **Copy-on-Write (CoW)** semantics for crash safety and snapshots
- **Per-block zstd compression** for transparent space savings
- **Per-volume encryption** (dm-crypt equivalent, AES-256-GCM)
- **Journaling** for metadata consistency
- **VFS integration** via the SigmaOS VFS layer

---

## On-Disk Format

### Superblock (Block 0, 4096 bytes)

```
Offset  Size  Field
0       8     Magic: 0x5369676D61465300 ("SigmaFS\0")
8       4     Version: 1
12      4     Block size: 4096
16      8     Total blocks
24      8     Free blocks
32      8     Inode table start block
40      8     Inode table size (blocks)
48      8     Journal start block
56      8     Journal size (blocks)
64      8     Data block bitmap start
72      32    Volume UUID
104     32    SHA-256 of this superblock (excluding this field)
136     64    Dilithium-5 signature of superblock SHA-256
200     3896  Reserved / future use
```

### Inode Table (starting at superblock.inode_table_start)

Each inode is 256 bytes:

```
Offset  Size  Field
0       4     Mode (file type + permissions)
4       4     UID
8       4     GID
12      8     File size (bytes)
20      8     Created timestamp (nanoseconds since epoch)
28      8     Modified timestamp
36      8     Accessed timestamp
44      4     Link count
48      4     Flags (compressed | encrypted | immutable | append-only)
52      4     Compression type (0=none, 1=zstd)
56      48    Direct block pointers (6 × 8 bytes)
104     8     Single indirect block pointer
112     8     Double indirect block pointer
120     8     Triple indirect block pointer
128     32    SHA-256 of file content (for integrity check)
160     96    Reserved
```

---

## CoW Semantics: Copy-on-Write for Snapshots

When a block is modified:
1. Allocate a new physical block.
2. Copy the original content to the new block.
3. Apply the modification to the new block.
4. Update the inode to point to the new block.
5. Release the old block reference count (free if count == 0).

This enables **snapshots**: freeze the inode table reference and create a new generation without copying any data.

```
Snapshot creation: O(1)
Snapshot read: O(1) (reads from frozen inode table)
Snapshot cleanup: O(dirty blocks) (deferred GC)
```

---

## Compression: Per-Block zstd

Each data block is independently compressed with zstd level 3 (default). The inode `compressed` flag indicates compressed blocks. Compression is transparent to applications.

```rust
// kernel/src/fs/sigma_fs/compress.rs (sketch)
use zstd::bulk::{compress, decompress};

pub fn compress_block(data: &[u8]) -> Vec<u8> {
    compress(data, 3).unwrap_or_else(|_| data.to_vec())
}

pub fn decompress_block(data: &[u8], original_size: usize) -> Vec<u8> {
    decompress(data, original_size).unwrap_or_else(|_| data.to_vec())
}
```

Compression ratio target: 2.0× for typical OS text/binary files.

---

## Encryption: Per-Volume AES-256-GCM

Volume encryption is applied at the block device level, below SigmaFS:

```
VFS syscall
  │
  ▼
SigmaFS block read/write
  │
  ▼
sigma-crypt block device (AES-256-GCM, 4096-byte granularity)
  │  key: unsealed from TPM2 PCR sealing
  ▼
Physical block device
```

Each 4096-byte block uses a unique 12-byte IV derived as `HKDF(master_key, block_number)`. The 16-byte GCM authentication tag is stored in a separate auth tag table (1 tag per block, contiguous at volume end).

---

## Journaling: Ordered Mode

SigmaFS uses **ordered journaling**: metadata is journaled, data is written to its final location before the journal commit.

Journal entry format:
```
[JournalEntry]
  type:       u8  (begin | commit | abort | block_write)
  block_num:  u64
  checksum:   u32 (CRC32C)
  data:       [u8; 4096] (for block_write entries)
```

On crash recovery, the journal is replayed from the last committed transaction.

---

## VFS Integration

SigmaFS registers with the SigmaOS VFS layer:

```rust
// kernel/src/fs/vfs.rs (registration)

pub trait Filesystem: Send + Sync {
    fn open(&self, path: &Path, flags: OpenFlags) -> Result<FileHandle, FsError>;
    fn read(&self, fh: &FileHandle, buf: &mut [u8], offset: u64) -> Result<usize, FsError>;
    fn write(&self, fh: &FileHandle, buf: &[u8], offset: u64) -> Result<usize, FsError>;
    fn stat(&self, path: &Path) -> Result<FileStat, FsError>;
    fn mkdir(&self, path: &Path, mode: u32) -> Result<(), FsError>;
    fn unlink(&self, path: &Path) -> Result<(), FsError>;
    fn snapshot(&self, name: &str) -> Result<SnapshotId, FsError>;
}
```

SigmaFS implements this trait. Other filesystems (tmpfs, devfs, procfs, 9P remote FS) implement the same trait and are mounted via the VFS mount table.
