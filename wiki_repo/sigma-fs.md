# sigma-fs — Filesystem Layer Specification

**Status:** Draft · Target: v0.2 (VFS + Ext4) / v0.5 (SigmaFS native)
**Owner:** fs team
**Canonical source:** `fs/`, `kernel/fs/`

---

## Overview

sigma-fs defines the VFS abstraction layer and all filesystem implementations for SigmaOS. The VFS presents a uniform inode/dentry/file ops interface to the kernel. Concrete implementations include SigmaFS (native CoW B-tree), Ext4, FAT32, and Tmpfs.

## Goals

- Single VFS API: all filesystems expose identical `inode_ops` / `file_ops` / `sb_ops` vtables

- SigmaFS: copy-on-write B-tree, no fsck needed, O(log n) lookup

- Ext4: full read/write with ordered journaling via JBD2

- FAT32: for EFI System Partition (read/write)

- Tmpfs: RAM-backed, ephemeral, used for initramfs and `/tmp`

- dm-verity: block-level integrity for read-only mounts (cloud/container profiles)

- OSTree A/B: atomic OS update overlay mount

---

## VFS Layer

### Core Objects

```c
struct sigma_inode {
    uint64_t       ino;
    uint32_t       mode;        // S_IFREG / S_IFDIR / S_IFLNK ...
    uint32_t       uid, gid;
    uint64_t       size;
    uint64_t       atime, mtime, ctime; // nanoseconds since epoch
    uint32_t       nlink;
    struct inode_ops  *ops;
    struct superblock *sb;
    void          *fs_private;  // per-fs inode data
};

struct inode_ops {
    int  (*lookup)(struct sigma_inode *dir, const char *name,
                   struct sigma_inode **out);
    int  (*create)(struct sigma_inode *dir, const char *name, uint32_t mode);
    int  (*mkdir) (struct sigma_inode *dir, const char *name, uint32_t mode);
    int  (*unlink)(struct sigma_inode *dir, const char *name);
    int  (*rename)(struct sigma_inode *old_dir, const char *old_name,
                   struct sigma_inode *new_dir, const char *new_name);
    int  (*symlink)(struct sigma_inode *dir, const char *name, const char *target);
    int  (*readlink)(struct sigma_inode *inode, char *buf, size_t len);
    int  (*getattr)(struct sigma_inode *inode, struct sigma_stat *stat);
    int  (*setattr)(struct sigma_inode *inode, struct sigma_stat *stat);
    int  (*truncate)(struct sigma_inode *inode, uint64_t new_size);
};

struct file_ops {
    ssize_t (*read) (struct sigma_file *, void *buf, size_t len, uint64_t *off);
    ssize_t (*write)(struct sigma_file *, const void *buf, size_t len, uint64_t *off);
    int     (*readdir)(struct sigma_file *, struct sigma_dirent *buf, size_t count);
    int     (*fsync)(struct sigma_file *);
    int     (*mmap) (struct sigma_file *, struct vm_area *);
    int     (*ioctl)(struct sigma_file *, uint32_t cmd, uintptr_t arg);
    int     (*open) (struct sigma_inode *, struct sigma_file *);
    int     (*release)(struct sigma_file *);
};
```

---

## SigmaFS On-Disk Format (Native CoW B-tree)

```
Block 0:    Superblock (4 KB)
Block 1–N:  B-tree node pool (variable)
Block N+1+: Data extent pool
```

### Superblock (4 KB)

```
magic[8]        = "SIGMAFS\0"
version[4]      = 0x00000001
block_size[4]   = 4096
total_blocks[8]
free_blocks[8]
root_tree_root[8]   // block address of B-tree root node
extent_bitmap[8]    // block address of free extent bitmap
checksum[32]        // SHA-3-256 of all above fields
```

### B-tree Nodes

- Order-128 B-tree (up to 255 keys per node)

- Key: 64-bit inode number (for inode tree) or 128-bit (dir_ino, name_hash) for dentry tree

- Leaf value: inode record or extent map

- CoW: writes clone the path from root to leaf; old root freed after journal commit

### Journal (WAL)

- Circular log at fixed offset; 64 MB default

- Each transaction: begin marker, sequence of block writes, commit marker with CRC32C

- Recovery: replay from last committed transaction on mount

---

## Ext4 Implementation

- Block groups with inode tables, block bitmaps as per Linux ext4 on-disk format

- Journal via JBD2 ordered mode (metadata journaled; data written before commit)

- Extent tree for large files (replaces block map for >12 direct blocks)

- dir_hash_tree (htree) for large directories

- Mount options: `ro`, `rw`, `noatime`, `data=ordered`

---

## FAT32 Implementation

- BPB parsing, FAT chain traversal

- Long File Name (LFN) VFAT extension support

- Used for: EFI System Partition read/write, USB sticks

- No journaling; fsck recommended after unclean unmount

---

## Tmpfs

- All data in kernel page cache (anonymous pages)

- No on-disk format; survives only while mounted

- Uses: `/tmp`, `/run`, initramfs extraction target, container overlay upper dir

- Size limit: configurable mount option `size=512M`

---

## initramfs cpio Extraction

1. sigma-boot passes initramfs cpio.gz physical address in `SigmaBootInfo`

2. Kernel decompresses gzip stream into Tmpfs mount at `/`

3. Parses newc cpio format: header magic `070701`, filename, data

4. Runs `/init` pivot_root after hardware init

---

## dm-verity Integration

- Block device layer below filesystem; computes SHA-256 tree over 4 KB blocks

- Root hash stored in `SigmaBootInfo` (measured by sigma-boot TPM2 PCR extend)

- Read failure if block hash mismatches → kernel panic or read-only error depending on policy

- Used by: `release/cloud` profile (immutable root), `release/container` profile

---

## OSTree A/B Layout

```
/ostree/
  deploy/
    sigmaos/
      deploy/<checksum>/      ← active deployment
      deploy/<prev-checksum>/ ← previous (rollback target)
  repo/                       ← OSTree object store
```

Bind-mounts overlay `/usr`, `/lib`, `/bin` from active deployment. `/etc`, `/var` remain mutable. Atomic update: write new deployment → update symlink → reboot.

---

## Implementation Plan

- [ ] 1. VFS inode/dentry/file ops abstraction (`fs/vfs.c`, `include/fs/vfs.h`)

- [ ] 2. Path resolution (namei) with dentry cache (`fs/namei.c`)

- [ ] 3. Page cache (file-backed page mapping) (`kernel/page_cache.c`)

- [ ] 4. Tmpfs (`fs/tmpfs.c`)

- [ ] 5. initramfs cpio extractor (`fs/initramfs.c`)

- [ ] 6. FAT32 (`fs/fat32.c`)

- [ ] 7. Ext4 read-only (`fs/ext4_ro.c`)

- [ ] 8. Ext4 read-write + JBD2 journal (`fs/ext4_rw.c`)

- [ ] 9. SigmaFS superblock + B-tree (`fs/sigmafs/btree.c`)

- [ ] 10. SigmaFS CoW write path + WAL journal (`fs/sigmafs/journal.c`)

- [ ] 11. dm-verity block layer (`kernel/dm_verity.c`)

- [ ] 12. OSTree mount overlay support

- [ ] 13. Tests: VFS ops on all filesystems, CoW semantics, journal replay

---

## Status

| Feature | State |
|---------|-------|
| VFS layer | ⬜ Not started |
| Tmpfs | ⬜ Not started |
| FAT32 | ⬜ Not started |
| Ext4 read-only | ⬜ Not started |
| Ext4 read-write | ⬜ Not started |
| SigmaFS | ⬜ Not started |
| dm-verity | ⬜ Not started |
| OSTree A/B | ⬜ Not started |
