# SigmaOS Filesystem Documentation

## Overview

SigmaOS implements a **Virtual Filesystem Switch (VFS)** layer that provides a unified interface to multiple filesystem backends. The VFS abstracts filesystem-specific operations and presents a POSIX-compatible interface to applications.

## Supported Filesystems

| Filesystem | Type | Status | Description |
|------------|------|--------|-------------|
| SigmaFS | Native | ✅ Beta | SigmaOS native journaled filesystem |
| ext4 | Linux-compat | ✅ | Extended filesystem 4 (read/write) |
| FAT32/exFAT | Portable | ✅ | FAT filesystem family |
| tmpfs | RAM | ✅ | In-memory temporary filesystem |
| procfs | Virtual | ✅ | Process information pseudo-filesystem |
| sysfs | Virtual | ✅ | Device/driver information |
| devtmpfs | Virtual | ✅ | Device nodes |
| OverlayFS | Union | ✅ | Container/live-CD overlay filesystem |
| Btrfs | CoW | ⬜ | B-tree filesystem with snapshots (planned) |
| ZFS | CoW | ⬜ | Zettabyte filesystem (planned) |
| NFS | Network | ⬜ | Network filesystem (planned) |

## VFS Architecture

```
Application (read/write/open/stat)
          ↓
  VFS Interface (src/filesystem/vfs.rs)
     ↙    ↓    ↘
 SigmaFS  ext4  tmpfs  ...
  (native) (compat) (ram)
          ↓
   Block Device Layer
   (src/driver/block/)
          ↓
  Hardware (NVMe/SATA/USB)
```

## SigmaFS — Native Filesystem

SigmaFS is designed specifically for SigmaOS:

### Design Goals
- **Journaling** — crash-consistent writes via write-ahead log
- **Copy-on-write** — atomic snapshot support
- **Inline data** — small files stored directly in inode
- **Extent-based allocation** — reduce fragmentation for large files
- **Checksums** — per-block CRC32c integrity verification

### Inode Structure
```rust
pub struct SigmaInode {
    pub magic: u32,           // 0x53494745 ("SIGE")
    pub ino: u64,             // Inode number
    pub mode: u32,            // File type + permissions
    pub uid: u32,             // Owner UID
    pub gid: u32,             // Owner GID
    pub size: u64,            // File size in bytes
    pub atime: u64,           // Access time (nanoseconds since epoch)
    pub mtime: u64,           // Modification time
    pub ctime: u64,           // Change time
    pub crtime: u64,          // Creation time
    pub nlinks: u32,          // Hard link count
    pub flags: InodeFlags,    // Extended flags
    pub extents: [Extent; 4], // Direct extents (inline)
    pub overflow_block: u64,  // Extent tree overflow block
    pub checksum: u32,        // CRC32c of inode data
}
```

### On-Disk Layout
```
Block 0:    Superblock
Block 1:    Backup superblock
Block 2-7:  Block group descriptor table
Block 8:    Journal superblock
Block 9+:   Journal blocks
Block N:    Inode bitmap
Block N+1:  Block bitmap
Block N+2+: Inode table
Block M+:   Data blocks
```

## Virtual Filesystem Interface

### Core VFS Operations

```rust
pub trait VfsNode: Send + Sync {
    fn read(&self, offset: u64, buf: &mut [u8]) -> Result<usize, VfsError>;
    fn write(&mut self, offset: u64, buf: &[u8]) -> Result<usize, VfsError>;
    fn stat(&self) -> Result<FileStat, VfsError>;
    fn truncate(&mut self, size: u64) -> Result<(), VfsError>;
    fn sync(&self) -> Result<(), VfsError>;
}

pub trait VfsDir: VfsNode {
    fn lookup(&self, name: &str) -> Result<VfsNodeRef, VfsError>;
    fn create(&mut self, name: &str, mode: u32) -> Result<VfsNodeRef, VfsError>;
    fn unlink(&mut self, name: &str) -> Result<(), VfsError>;
    fn readdir(&self, offset: u64) -> Result<Vec<DirEntry>, VfsError>;
    fn mkdir(&mut self, name: &str, mode: u32) -> Result<VfsNodeRef, VfsError>;
    fn rename(&mut self, old: &str, new_dir: &mut dyn VfsDir, new: &str) -> Result<(), VfsError>;
}
```

### Mount Operations

```rust
// Mount a filesystem
vfs.mount("/mnt/data", "ext4", "/dev/sda2", MountFlags::empty())?;

// Mount tmpfs at /tmp
vfs.mount("/tmp", "tmpfs", "tmpfs", MountFlags::NOEXEC | MountFlags::NOSUID)?;

// List mounts
for mount in vfs.list_mounts() {
    println!("{} on {} type {} ({})", mount.device, mount.path, mount.fstype, mount.options);
}

// Unmount
vfs.umount("/mnt/data")?;
```

## Path Resolution

SigmaOS path resolution handles:
- Absolute paths (`/usr/bin/ls`)
- Relative paths (`../lib/libsigma.so`)
- Symlinks (with cycle detection, max depth 40)
- `.` and `..` components
- Path traversal prevention (no `..` escaping chroot)

```rust
// Resolve a path with security validation
let node = vfs.resolve_path("/usr/bin/ls", resolve_flags)?;

// Resolve relative to a directory fd
let node = vfs.resolve_at(dirfd, "relative/path", resolve_flags)?;
```

## File Permissions

SigmaOS uses the standard Unix DAC model plus optional MAC enforcement:

```
Owner permissions: rwx (bits 8-6)
Group permissions: rwx (bits 5-3)
Other permissions: rwx (bits 2-0)
Special bits:
  SUID (bit 11): Run as owner
  SGID (bit 10): Run as group / new files inherit group
  Sticky (bit 9): Only owner can delete from directory
```

## Security: OverlayFS for Immutable Root

SigmaOS can run with an **immutable read-only root filesystem** using OverlayFS (ChromeOS-style):

```
OverlayFS
  ├── Upper layer: /overlay/upper (writable, RAM or encrypted disk)
  ├── Work dir:    /overlay/work
  └── Lower layer: /sigma/rootfs (read-only, signed)
```

This ensures the base system cannot be tampered with even if an attacker gains code execution.

## Performance Tuning

| Parameter | Description | Recommended |
|-----------|-------------|-------------|
| `read_ahead_kb` | Read-ahead window | 128-4096 KB |
| `dirty_ratio` | Writeback threshold % | 10-20% |
| `nr_requests` | I/O queue depth | 128-512 |
| `scheduler` | I/O scheduler | `mq-deadline` (HDD), `none` (NVMe) |

## References

- [Linux VFS Documentation](https://www.kernel.org/doc/html/latest/filesystems/vfs.html)
- [ext4 Disk Layout](https://ext4.wiki.kernel.org/index.php/Ext4_Disk_Layout)
- [OverlayFS](https://www.kernel.org/doc/html/latest/filesystems/overlayfs.html)
