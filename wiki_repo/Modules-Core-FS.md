# Σ core/fs — Sovereign Filesystem Layer

Abstracts storage handling into a **pluggable VFS layer** supporting multiple
filesystem backends without kernel recompilation.

## Architecture

```
User Process
   └─ sigma_vfs_open("/data/file.txt")
         └─ VFS Router (vfs.rs)
               ├─ SigmaFS  (sigmafs.rs)   ← default sovereign FS
               ├─ Ext4     (ext4.rs)       ← Linux compat layer
               ├─ FAT32    (fat32.rs)      ← removable media
               └─ Web3FS   (web3_persistence.rs) ← IPFS-backed
```

## Source Files

| File | Description |
|---|---|
| `vfs.rs` | Virtual Filesystem Switch — routes open/read/write to backends |
| `sigmafs.rs` | SovereignFS: CoW, journaling, BLAKE3 block integrity |
| `ext4.rs` | Read-compatible Ext4 driver (no journal write support yet) |
| `fat32.rs` | FAT32 r/w driver for removable media |
| `self_opt_fs.rs` | AI-driven self-optimising filesystem layout engine |
| `web3_persistence.rs` | IPFS/Filecoin-backed decentralised storage shim |

## API Interface

```c
// Mount a filesystem backend at a path
int sigma_vfs_mount(const char *path, const char *fs_type, uint32_t flags);

// Open a file — returns sovereign file descriptor
int sigma_vfs_open(const char *path, uint32_t flags, uint32_t mode);

// Read / Write — zero-copy where supported
ssize_t sigma_vfs_read(int fd, void *buf, size_t len);
ssize_t sigma_vfs_write(int fd, const void *buf, size_t len);

// Snapshot the current FS state (SovereignFS only)
int sigma_vfs_snapshot(const char *tag);

// Rollback to a named snapshot
int sigma_vfs_rollback(const char *tag);
```

## SovereignFS On-Disk Layout

```
[Superblock 4K] [Journal 64MB] [Inode Table] [Data Extents ...]
```

- **Copy-on-Write:** every write creates a new extent; old data preserved until
  explicitly pruned — enables instant snapshots.

- **Block Integrity:** each 4 KB block carries a BLAKE3 checksum; the kernel
  rejects tampered blocks at read time.

## Roadmap

- [x] VFS router (`vfs.rs`)

- [x] SovereignFS basic CoW layout (`sigmafs.rs`)

- [x] Ext4 read-only driver (`ext4.rs`)

- [x] FAT32 r/w driver (`fat32.rs`)

- [ ] SovereignFS journal format spec

- [ ] `sfs_mkfs` userland tool

- [ ] SPARK formal proofs for journal replay

- [ ] OverlayFS shim for container layers

- [ ] NVMe queue-depth optimisation in VFS

## Related Modules

- [`modules/core/kernel`](../kernel/README.md) — Kernel memory management

- [`modules/security/isolation`](../../security/isolation/README.md) — FS namespace isolation
