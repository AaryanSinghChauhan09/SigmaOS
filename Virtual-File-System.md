# SigmaOS Virtual File System & Ext2 Driver

## Overview

SigmaOS uses a **Virtual File System (VFS)** abstraction layer that decouples kernel I/O from any specific disk format. Whether the backing storage is Ext2, FAT32, a RAMDisk, or `/proc` — the kernel uses the same unified `vfs_node_t` interface.

## VFS Node Structure

```c
struct vfs_node_t {
    char name[128];        // File/directory name
    sigma_u32 flags;       // File type flags
    sigma_u32 length;      // File size in bytes
    sigma_u32 inode;       // Filesystem-specific ID
    sigma_u32 impl_id;     // Driver ID (Ext2, FAT32, etc.)

    // Function pointers — the "virtual dispatch" mechanism
    vfs_read_func_t  read;
    vfs_write_func_t write;
    vfs_open_func_t  open;
    vfs_close_func_t close;
};
```

Any filesystem driver simply fills in these function pointers and registers itself as the root (or a mount point). The kernel never needs to know if it is talking to Ext2 or a virtual device.

## Ext2 Driver (Read-Only)

The Ext2 driver parses the **Superblock** at byte offset `1024` on disk:

| Field | Value |
|---|---|
| Magic | `0xEF53` |
| Block Size | `1024 << log_block_size` |
| Inode Count | From superblock |
| Block Groups | Calculated from total blocks |

### Initialization Flow

```
disk_read(sector=2) → Load 1KB → Parse Superblock
    │
    ├─ Magic == 0xEF53? → Valid Ext2 partition found
    └─ Register read handler on vfs_root
```

## Adding a New Filesystem

To plug in a new filesystem (e.g., FAT32), you only need to:
1. Parse its boot sector / metadata structures.
2. Implement functions matching `vfs_read_func_t`, `vfs_write_func_t`, etc.
3. Point `vfs_root->read = your_read_fn` and `vfs_root->impl_id = YOUR_FS_ID`.

The kernel immediately starts using your driver transparently.

## Relevant Source Files

- `include/kernel/fs/sigma_vfs.h` — VFS node structure and API
- `kernel/core/fs/ext2/sigma_ext2.cpp` — Ext2 superblock parser and read-only driver
