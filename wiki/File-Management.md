# File Management

SigmaOS implements comprehensive file management with Linux and BSD-inspired features including VFS, file systems, file permissions, and file operations.

## Overview

File management provides:
- Virtual File System (VFS) layer for filesystem abstraction
- Support for multiple filesystems (ext4, XFS, Btrfs, ZFS, UFS, etc.)
- File permissions and access control (POSIX, ACL, capabilities)
- File operations (create, read, write, delete, copy, move)
- Directory operations (create, delete, list, traverse)
- File locking and advisory locks
- File descriptors and I/O operations
- Symbolic and hard links
- File attributes and extended attributes

## Implementation

### Virtual File System (VFS)
```rust
// src/fs/vfs.rs
pub struct VirtualFileSystem {
    pub mount_points: BTreeMap<String, Box<dyn Filesystem>>,
    pub current_directory: String,
    pub open_files: BTreeMap<fd, OpenFile>,
}

pub trait Filesystem {
    fn mount(&mut self, device: &str, mount_point: &str) -> Result<(), FsError>;
    fn unmount(&mut self, mount_point: &str) -> Result<(), FsError>;
    fn open(&mut self, path: &str, flags: OpenFlags) -> Result<fd, FsError>;
    fn close(&mut self, fd: fd) -> Result<(), FsError>;
    fn read(&mut self, fd: fd, buffer: &mut [u8]) -> Result<usize, FsError>;
    fn write(&mut self, fd: fd, buffer: &[u8]) -> Result<usize, FsError>;
    fn stat(&self, path: &str) -> Result<FileStat, FsError>;
    fn mkdir(&mut self, path: &str, mode: FileMode) -> Result<(), FsError>;
    fn rmdir(&mut self, path: &str) -> Result<(), FsError>;
    fn unlink(&mut self, path: &str) -> Result<(), FsError>;
}

#[derive(Debug, Clone)]
pub struct FileStat {
    pub size: u64,
    pub mode: FileMode,
    pub uid: u32,
    pub gid: u32,
    pub atime: SystemTime,
    pub mtime: SystemTime,
    pub ctime: SystemTime,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        VirtualFileSystem {
            mount_points: BTreeMap::new(),
            current_directory: "/".to_string(),
            open_files: BTreeMap::new(),
        }
    }

    pub fn mount(&mut self, fs: Box<dyn Filesystem>, mount_point: &str) -> Result<(), FsError> {
        self.mount_points.insert(mount_point.to_string(), fs);
        Ok(())
    }

    pub fn open(&mut self, path: &str, flags: OpenFlags) -> Result<fd, FsError> {
        let resolved_path = self.resolve_path(path)?;
        let mount_point = self.find_mount_point(&resolved_path)?;
        
        if let Some(fs) = self.mount_points.get_mut(mount_point) {
            let relative_path = resolved_path.strip_prefix(mount_point).unwrap_or(&resolved_path);
            let fd = fs.open(relative_path, flags)?;
            self.open_files.insert(fd, OpenFile::new(fd, path.to_string()));
            Ok(fd)
        } else {
            Err(FsError::NoFilesystem)
        }
    }

    fn resolve_path(&self, path: &str) -> Result<String, FsError> {
        if path.starts_with('/') {
            Ok(path.to_string())
        } else {
            Ok(format!("{}/{}", self.current_directory, path))
        }
    }

    fn find_mount_point(&self, path: &str) -> Result<&str, FsError> {
        let mut best_match = "";
        
        for mount_point in self.mount_points.keys() {
            if path.starts_with(mount_point) && mount_point.len() > best_match.len() {
                best_match = mount_point;
            }
        }
        
        if best_match.is_empty() {
            Err(FsError::NoFilesystem)
        } else {
            Ok(best_match)
        }
    }
}
```

### File Permissions
```rust
// src/fs/permissions.rs
pub struct FilePermissions {
    pub mode: FileMode,
    pub uid: u32,
    pub gid: u32,
    pub acl: Vec<AclEntry>,
}

#[derive(Debug, Clone, Copy)]
pub struct FileMode {
    pub user: PermissionBits,
    pub group: PermissionBits,
    pub other: PermissionBits,
    pub special: SpecialBits,
}

#[derive(Debug, Clone, Copy)]
pub struct PermissionBits {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SpecialBits {
    pub setuid: bool,
    pub setgid: bool,
    pub sticky: bool,
}

#[derive(Debug, Clone)]
pub struct AclEntry {
    pub entity: AclEntity,
    pub permissions: PermissionBits,
}

#[derive(Debug, Clone)]
pub enum AclEntity {
    User(u32),
    Group(u32),
    Other,
}

impl FilePermissions {
    pub fn new(mode: FileMode, uid: u32, gid: u32) -> Self {
        FilePermissions {
            mode,
            uid,
            gid,
            acl: Vec::new(),
        }
    }

    pub fn check_permission(&self, uid: u32, gid: u32, required: PermissionBits) -> bool {
        // Check if user is owner
        if uid == self.uid {
            return self.check_bits(self.mode.user, required);
        }
        
        // Check if user is in group
        if gid == self.gid {
            return self.check_bits(self.mode.group, required);
        }
        
        // Check other permissions
        self.check_bits(self.mode.other, required)
    }

    fn check_bits(&self, available: PermissionBits, required: PermissionBits) -> bool {
        (!required.read || available.read) &&
        (!required.write || available.write) &&
        (!required.execute || available.execute)
    }
}
```

### File Locking
```rust
// src/fs/locking.rs
pub struct FileLockManager {
    pub locks: BTreeMap<String, FileLock>,
}

#[derive(Debug, Clone)]
pub struct FileLock {
    pub path: String,
    pub lock_type: LockType,
    pub holder: u32,
    pub exclusive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockType {
    Read,
    Write,
}

impl FileLockManager {
    pub fn new() -> Self {
        FileLockManager {
            locks: BTreeMap::new(),
        }
    }

    pub fn acquire_lock(&mut self, path: &str, lock_type: LockType, holder: u32, exclusive: bool) -> Result<(), LockError> {
        if let Some(existing_lock) = self.locks.get(path) {
            if existing_lock.exclusive || (exclusive && existing_lock.holder != holder) {
                return Err(LockError::Locked);
            }
        }
        
        self.locks.insert(path.to_string(), FileLock {
            path: path.to_string(),
            lock_type,
            holder,
            exclusive,
        });
        
        Ok(())
    }

    pub fn release_lock(&mut self, path: &str, holder: u32) -> Result<(), LockError> {
        if let Some(lock) = self.locks.get(path) {
            if lock.holder == holder {
                self.locks.remove(path);
                Ok(())
            } else {
                Err(LockError::NotHolder)
            }
        } else {
            Err(LockError::NotLocked)
        }
    }
}
```

## Configuration

### File Management Configuration
```toml
# /etc/sigmaos/filesystem.toml
[vfs]
# VFS settings
enabled = true
mount_timeout_seconds = 30
open_file_limit = 1024

[permissions]
# Permission settings
default_umask = 022
acl_enabled = true
capabilities_enabled = true

[locking]
# File locking settings
enabled = true
lock_timeout_seconds = 60
```

### Runtime Control
```bash
# Show mount points
sigfs show-mounts

# Mount filesystem
sigfs mount /dev/sda1 /mnt/data

# Unmount filesystem
sigfs unmount /mnt/data

# Show file permissions
sigfs stat /path/to/file

# Change file permissions
sigfs chmod 755 /path/to/file

# Change file owner
sigfs chown user:group /path/to/file

# Acquire file lock
sigfs lock /path/to/file

# Release file lock
sigfs unlock /path/to/file

# Show open files
sigfs show-open-files
```

## Performance Optimization

### VFS Tuning
Optimize VFS for performance:
```bash
# Increase dentry cache
sigfs set-dentry-cache-size 10000

# Increase inode cache
sigfs set-inode-cache-size 10000

# Enable directory caching
sigfs enable-dir-cache

# Enable readahead
sigfs enable-readahead
```

### File Locking Optimization
Optimize file locking for performance:
```bash
# Reduce lock timeout
sigfs set-lock-timeout 30

# Enable optimistic locking
sigfs enable-optimistic-locking

# Enable lock aggregation
sigfs enable-lock-aggregation
```

## Troubleshooting

### Mount Fails
If mount fails:
1. Check filesystem type: `sigfs detect-fs /dev/sda1`
2. Check mount point: `sigfs check-mount-point /mnt/data`
3. Check device status
4. Check permissions
5. Check for existing mount

### Permission Denied
If permission denied:
1. Check file permissions: `sigfs stat /path/to/file`
2. Check user/group: `id`
3. Check ACLs: `sigfs get-acl /path/to/file`
4. Change permissions if necessary
5. Check capabilities

### Lock Timeout
If lock timeout occurs:
1. Check lock status: `sigfs show-locks`
2. Check lock holder
3. Increase lock timeout
4. Release stale locks
5. Check for deadlocks

### File Not Found
If file not found:
1. Check current directory: `pwd`
2. Check absolute path
3. Check mount points
4. Check filesystem status
5. Check file permissions

---

**[File Management](Category-File-Management)** | **[VFS](Category-VFS)** | **[Filesystems](Category-Filesystems)**
