# SigmaOS VFS Layer

## Overview

The SigmaOS Virtual Filesystem Layer (`src/fs/sigma_vfs.rs`) abstracts all concrete filesystem implementations behind a common trait-based API.  User space syscalls (`open`, `read`, `write`, `stat`, `readdir`) dispatch through this layer to the appropriate filesystem driver.

---

## Architecture

```
 User-space syscalls
        │
 ┌──────▼──────────────────────────────────────────────────────┐
 │                      VFS Layer                               │
 │                                                              │
 │  VfsContext ──► mount table ──► VfsMount                    │
 │  VfsPath ──► VfsDentry ──► VfsInode ──► VfsNode (trait)    │
 └──────┬──────────────────────────────────────────────────────┘
        │  (dynamic dispatch via Box<dyn VfsNode>)
   ┌────▼──────┐ ┌──────────────┐ ┌─────────┐ ┌──────┐ ┌──────┐
   │ Sigma-EXT │ │ Sigma-ZFS-  │ │OverlayFS│ │tmpfs │ │procfs│
   │           │ │   parity     │ │         │ │      │ │/sysfs│
   └───────────┘ └──────────────┘ └─────────┘ └──────┘ └──────┘
```

---

## Inode Model

An **inode** (`VfsInode`) is the fundamental unit of a SigmaOS filesystem.  It holds:

| Field | Type | Description |
|-------|------|-------------|
| `ino` | `u64` | Unique number within filesystem |
| `inode_type` | `InodeType` | File / Dir / Symlink / Device / … |
| `ref_count` | `AtomicUsize` | Reference count (`iget` / `iput`) |
| `node` | `Box<dyn VfsNode>` | Filesystem-specific implementation |

### Inode types

| `InodeType` | Unix equivalent |
|-------------|----------------|
| `File` | Regular file |
| `Directory` | Directory |
| `Symlink` | Symbolic link |
| `CharDevice` | Character device (`/dev/null`, …) |
| `BlockDevice` | Block device (`/dev/sda`, …) |
| `Fifo` | Named pipe |
| `Socket` | Unix-domain socket |

---

## Dentry Cache

`VfsDentry` maps a `(name, parent)` tuple to a `VfsInode`.  The dcache avoids repeated filesystem lookups for commonly-accessed paths:

```
lookup("/usr/lib/libc.so.6"):
  dcache hit  → return cached VfsInode   (O(1))
  dcache miss → ask filesystem driver → insert into dcache
```

Each `VfsDentry` carries a reference count (`get` / `put`) so that inodes are not freed while dentries point to them.

---

## Mount Table

`VfsContext` owns the mount table (`Vec<VfsMount>`).  Mounts are sorted **longest-prefix first** so that the correct filesystem is selected for nested mounts.

```
/proc            → procfs
/sys             → sysfs
/tmp             → tmpfs
/                → Sigma-EXT root
```

A lookup for `/proc/1/status` matches `/proc` (longest prefix) rather than `/`.

### Mount / unmount

```rust
ctx.register_filesystem(Box::new(TmpfsSuperblock));
ctx.mount("tmpfs", "", "/tmp", MountFlags::default())?;

ctx.umount("/tmp")?;
```

### `MountFlags`

| Flag | Meaning |
|------|---------|
| `read_only` | No write operations |
| `no_suid` | Ignore setuid bits |
| `no_dev` | Ignore device files |
| `no_exec` | Deny execution |
| `no_atime` | Skip access-time updates |

---

## `VfsNode` Trait

Every filesystem object implements `VfsNode`:

```rust
pub trait VfsNode: Send + Sync {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize, VfsError>;
    fn write(&self, data: &[u8], offset: u64) -> Result<usize, VfsError>;
    fn stat(&self) -> Result<VfsStat, VfsError>;
    fn readdir(&self) -> Result<Vec<DirEntry>, VfsError>;
    fn readlink(&self) -> Result<String, VfsError>;  // symlinks only
    fn inode_type(&self) -> InodeType;
}
```

---

## `VfsSuperblock` Trait

Filesystem drivers register with the VFS by implementing `VfsSuperblock`:

```rust
pub trait VfsSuperblock: Send + Sync {
    fn fs_type(&self) -> &str;
    fn mount(&self, source: &str, flags: MountFlags) -> Result<Arc<dyn VfsNode>, VfsError>;
    fn umount(&self) -> Result<(), VfsError>;
    fn statfs(&self) -> FsStats;
}
```

---

## Path Resolution

`VfsPath` resolves a string path into components and drives the lookup:

```rust
let p = VfsPath::new("/usr/local/bin/sigma");
// components: ["usr", "local", "bin", "sigma"]
// parent: /usr/local/bin
// file_name: "sigma"
```

Symlink following: the caller detects `InodeType::Symlink`, calls `readlink()`, and re-resolves the target (up to a maximum of 40 symlink levels — `VfsError::TooManySymlinks`).

---

## Supported Filesystems

### Sigma-EXT (native journaling FS)

Inspired by ext4.  Features:
- 64-bit inode numbers
- Extent-based block allocation
- Journal (metadata + data modes)
- Inline data for small files

### Sigma-ZFS-parity

ZFS-inspired features ported to SigmaOS:
- Copy-on-Write (CoW) transactions
- Checksummed data blocks (SHA-256)
- Built-in RAID-Z equivalent
- Dataset / snapshot model

### OverlayFS

Union filesystem for container layers:
- **Upper** layer: read-write overlay
- **Lower** layers: read-only base images
- **Work** directory: atomic renames
- Used by SigmaOS container runtime

### tmpfs

Fully in-memory filesystem backed by kernel heap:
- Files exist only until unmount or reboot
- Used for `/tmp`, `/run`, shared memory
- Included as `TmpfsSuperblock` in the sovereign VFS implementation

### procfs

Virtual filesystem exposing kernel process state:
- `/proc/<pid>/status` — process metadata
- `/proc/<pid>/maps` — virtual memory map
- `/proc/meminfo` — system memory statistics
- Nodes are synthesized on read, no on-disk storage

### sysfs

Exposes kernel object model (devices, drivers, buses):
- `/sys/class/net/` — network interfaces
- `/sys/block/` — block devices
- `/sys/module/` — loaded kernel modules
- Writeable attributes allow runtime tuning

---

## Error Reference

| `VfsError` | POSIX errno equivalent |
|-----------|----------------------|
| `NotFound(path)` | `ENOENT` |
| `NotSupported` | `ENOSYS` / `EOPNOTSUPP` |
| `PermissionDenied` | `EACCES` / `EPERM` |
| `TooManySymlinks` | `ELOOP` |
| `NotADirectory` | `ENOTDIR` |
| `IsADirectory` | `EISDIR` |
| `Io(msg)` | `EIO` |
| `ReadOnly` | `EROFS` |
| `AlreadyMounted` | `EBUSY` |

---

## See Also

- [`SIGMA_IPC_PIPES.md`](SIGMA_IPC_PIPES.md) — named pipes live in VFS
- [`SIGMA_RCU_SYNCHRONIZATION.md`](SIGMA_RCU_SYNCHRONIZATION.md) — mount table protected by RCU
- [`SIGMA_CONCURRENCY_PRIMITIVES.md`](SIGMA_CONCURRENCY_PRIMITIVES.md) — inode reference count uses atomics
