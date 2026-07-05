# Virtual File System (VFS)

The SigmaOS VFS (`kernel/vfs/sigma_vfs.rs`) provides a generic filesystem
abstraction that any filesystem implementation can plug into.

---

## Architecture

```
User process
    │  open("/tmp/file", O_RDWR)
    ▼
Syscall dispatch (syscall_dispatch.rs)
    │  sigma_vfs_open(path, flags)
    ▼
VFS core (sigma_vfs.rs)
    │  path_lookup → inode
    │  alloc fd slot
    │  return fd
    ▼
Filesystem ops vtable (FsOps)
    ├── tmpfs_read / tmpfs_write  (/tmp, /run)
    ├── ext4_read / ext4_write    (/etc, /usr, /home)
    └── procfs_read               (/proc)
```

---

## Key Data Structures

### Inode
Represents a file, directory, symlink, or device:
```rust
pub struct Inode {
    pub ino:    u64,         // Unique inode number
    pub kind:   InodeType,   // Regular/Directory/Symlink/CharDev/...
    pub mode:   Mode,        // rwxrwxrwx permission bits
    pub size:   u64,         // File size in bytes
    pub nlinks: u32,         // Hard link count
    pub uid:    u32,         // Owner UID
    pub mtime:  u64,         // Modification time (ns)
}
```

### Dentry (Directory Entry)
Links a filename to an inode:
```rust
pub struct Dentry {
    pub name:     [u8; 256],  // Filename
    pub ino:      u64,        // Inode number
    pub parent:   u64,        // Parent directory inode
}
```

### File Descriptor
Per-process open file handle:
```rust
pub struct FileDesc {
    pub ino:    u64,   // Inode
    pub offset: u64,   // Current read/write position
    pub flags:  u32,   // O_RDONLY, O_WRONLY, O_RDWR, O_APPEND, ...
}
```

---

## Filesystem Operations

Each filesystem provides an `FsOps` vtable:

```rust
pub struct FsOps {
    pub read:    unsafe fn(ino, offset, buf, len) -> i64,
    pub write:   unsafe fn(ino, offset, buf, len) -> i64,
    pub readdir: unsafe fn(ino, offset, out, max) -> i64,
    pub create:  unsafe fn(parent, name, mode) -> Option<u64>,
    pub unlink:  unsafe fn(parent, name) -> i32,
    pub mkdir:   unsafe fn(parent, name, mode) -> Option<u64>,
    pub stat:    unsafe fn(ino, out) -> i32,
}
```

---

## Mount Points

Up to 16 filesystems can be mounted simultaneously:

```c
// Mount tmpfs at /tmp
sigma_vfs_mount("/tmp", 4, root_ino, &TMPFS_OPS);

// Mount ext4 at /
sigma_vfs_mount("/", 1, 2, &EXT4_OPS);  // inode 2 = root dir in ext4

// Mount procfs at /proc
sigma_vfs_mount("/proc", 5, proc_root, &PROC_OPS);
```

---

## Path Resolution

`path_lookup("/usr/bin/ls")`:
1. Start at root inode (`/`)
2. Look up dentry `usr` → inode for `/usr`
3. Look up dentry `bin` → inode for `/usr/bin`
4. Look up dentry `ls` → inode for `/usr/bin/ls`
5. Return inode number

---

## Mounted Filesystems

| Path | Filesystem | Status |
|------|-----------|--------|
| `/tmp` | tmpfs (RAM) | ✅ |
| `/run` | tmpfs (RAM) | ✅ |
| `/proc` | procfs (shim) | ✅ |
| `/dev/urandom` | char device | ✅ |
| `/` | ext4 | 🔄 Phase B |
| `/home` | ext4 | ⬜ Phase B |

---

## Limits

| Limit | Value |
|-------|-------|
| Max inodes | 4096 |
| Max dentries | 1024 |
| Max open FDs | 256 |
| Max mount points | 16 |
| Max filename length | 255 |

---

*Source: `kernel/vfs/sigma_vfs.rs`, `kernel/vfs/sigma_tmpfs.rs`*
*See also: [Transactional Updates](Transactional-Updates) · [sigpkg](../docs/SIGPKG_SPEC.md)*
