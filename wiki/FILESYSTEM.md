# SigmaOS Filesystem

## Overview

SigmaOS implements a Virtual Filesystem (VFS) layer with support for multiple filesystem types. This document describes the filesystem architecture and implementation.

## Filesystem Architecture

### VFS Layer

The Virtual Filesystem (VFS) layer provides a unified interface for different filesystem types:

```
┌─────────────────────────────────────┐
│         Userland Applications       │
└─────────────────────────────────────┘
              │
┌─────────────────────────────────────┐
│         System Call Interface       │
└─────────────────────────────────────┘
              │
┌─────────────────────────────────────┐
│         VFS Layer                   │
│  ┌──────┐ ┌──────┐ ┌──────┐       │
│  │Ext2  │ │FAT32 │ │CryptFS│       │
│  └──────┘ └──────┘ └──────┘       │
└─────────────────────────────────────┘
              │
┌─────────────────────────────────────┐
│         Block Device Layer          │
└─────────────────────────────────────┘
              │
┌─────────────────────────────────────┐
│         Storage Drivers             │
└─────────────────────────────────────┘
```

### VFS Interface

```rust
pub trait Filesystem {
    fn name(&self) -> &str;
    fn mount(&mut self, device: &str) -> Result<(), FsError>;
    fn unmount(&mut self) -> Result<(), FsError>;
    fn open(&mut self, path: &str, flags: u32) -> Result<FileHandle, FsError>;
    fn close(&mut self, handle: FileHandle) -> Result<(), FsError>;
    fn read(&mut self, handle: FileHandle, buf: &mut [u8]) -> Result<usize, FsError>;
    fn write(&mut self, handle: FileHandle, buf: &[u8]) -> Result<usize, FsError>;
    fn stat(&mut self, path: &str) -> Result<FileInfo, FsError>;
    fn mkdir(&mut self, path: &str) -> Result<(), FsError>;
    fn rmdir(&mut self, path: &str) -> Result<(), FsError>;
    fn unlink(&mut self, path: &str) -> Result<(), FsError>;
}
```

## Filesystem Types

### Ext2/Ext3

**Location**: `kernel/fs/ext2.rs`

**Features**:
- Linux-compatible filesystem
- Journaling (Ext3)
- Extended attributes
- Access control lists

**Structure**:
```rust
pub struct Ext2Filesystem {
    device: BlockDevice,
    superblock: Ext2Superblock,
    block_groups: Vec<Ext2BlockGroup>,
    mounted: bool,
}

pub struct Ext2Superblock {
    pub inodes_count: u32,
    pub blocks_count: u32,
    pub free_blocks_count: u32,
    pub free_inodes_count: u32,
    pub block_size: u32,
    pub inode_size: u32,
    // ... more fields
}
```

### FAT32

**Location**: `kernel/fs/fat32.rs`

**Features**:
- Windows-compatible filesystem
- Long filename support
- Removable media support

**Structure**:
```rust
pub struct Fat32Filesystem {
    device: BlockDevice,
    boot_sector: Fat32BootSector,
    fat_tables: Vec<Vec<u32>>,
    mounted: bool,
}

pub struct Fat32BootSector {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub fat_count: u8,
    pub root_dir_cluster: u32,
    // ... more fields
}
```

### CryptFS

**Location**: `kernel/fs/cryptfs.rs`

**Features**:
- Encrypted filesystem
- Argon2id key derivation
- AES-256-GCM encryption
- Plausible deniability

**Structure**:
```rust
pub struct CryptFilesystem {
    device: BlockDevice,
    master_key: [u8; 32],
    cipher: Aes256Gcm,
    mounted: bool,
}

pub struct CryptHeader {
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
    pub key_derivation: Argon2idParams,
    // ... more fields
}
```

## File Operations

### Open File

```rust
pub unsafe fn sys_open(path: *const u8, flags: i32, mode: u32) -> i32 {
    let path_str = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    let vfs = get_current_vfs();
    
    match vfs.open(path_str, flags as u32) {
        Ok(handle) => handle as i32,
        Err(e) => -e as i32,
    }
}
```

### Read File

```rust
pub unsafe fn sys_read(fd: i32, buf: *mut u8, count: usize) -> isize {
    let vfs = get_current_vfs();
    let handle = fd as FileHandle;
    
    match vfs.read(handle, unsafe { slice_from_raw_parts_mut(buf, count) }) {
        Ok(n) => n as isize,
        Err(e) => -(e as isize),
    }
}
```

### Write File

```rust
pub unsafe fn sys_write(fd: i32, buf: *const u8, count: usize) -> isize {
    let vfs = get_current_vfs();
    let handle = fd as FileHandle;
    
    match vfs.write(handle, unsafe { slice_from_raw_parts(buf, count) }) {
        Ok(n) => n as isize,
        Err(e) => -(e as isize),
    }
}
```

## Directory Operations

### Create Directory

```rust
pub unsafe fn sys_mkdir(path: *const u8, mode: u32) -> i32 {
    let path_str = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    let vfs = get_current_vfs();
    
    match vfs.mkdir(path_str) {
        Ok(_) => 0,
        Err(e) => -e as i32,
    }
}
```

### Remove Directory

```rust
pub unsafe fn sys_rmdir(path: *const u8) -> i32 {
    let path_str = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    let vfs = get_current_vfs();
    
    match vfs.rmdir(path_str) {
        Ok(_) => 0,
        Err(e) => -e as i32,
    }
}
```

## File Attributes

### File Information

```rust
pub struct FileInfo {
    pub inode: u64,
    pub size: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
}

pub unsafe fn sys_stat(path: *const u8, stat: *mut FileInfo) -> i32 {
    let path_str = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    let vfs = get_current_vfs();
    
    match vfs.stat(path_str) {
        Ok(info) => {
            unsafe { *stat = info };
            0
        }
        Err(e) => -e as i32,
    }
}
```

## Mount Points

### Mount Filesystem

```rust
pub unsafe fn sys_mount(source: *const u8, target: *const u8, fstype: *const u8, flags: u32) -> i32 {
    let source_str = unsafe { CStr::from_ptr(source) }.to_str().unwrap();
    let target_str = unsafe { CStr::from_ptr(target) }.to_str().unwrap();
    let fstype_str = unsafe { CStr::from_ptr(fstype) }.to_str().unwrap();
    
    let vfs = get_vfs_manager();
    match vfs.mount(source_str, target_str, fstype_str, flags) {
        Ok(_) => 0,
        Err(e) => -e as i32,
    }
}
```

### Unmount Filesystem

```rust
pub unsafe fn sys_umount(target: *const u8) -> i32 {
    let target_str = unsafe { CStr::from_ptr(target) }.to_str().unwrap();
    let vfs = get_vfs_manager();
    
    match vfs.unmount(target_str) {
        Ok(_) => 0,
        Err(e) => -e as i32,
    }
}
```

## Path Resolution

### Path Resolution Algorithm

1. **Absolute path**: Start from root directory
2. **Relative path**: Start from current working directory
3. **Component traversal**: Process each path component
4. **Symbolic links**: Follow links (with cycle detection)
5. **Mount points**: Cross mount boundaries

### Current Working Directory

```rust
thread_local! {
    static CWD: RefCell<PathBuf> = RefCell::new(PathBuf::from("/"));
}

pub fn get_cwd() -> PathBuf {
    CWD.with(|cwd| cwd.borrow().clone())
}

pub fn set_cwd(path: &Path) -> Result<(), FsError> {
    CWD.with(|cwd| {
        *cwd.borrow_mut() = path.to_path_buf();
        Ok(())
    })
}
```

## File Descriptors

### File Descriptor Table

```rust
pub struct FileDescriptorTable {
    descriptors: Vec<Option<FileDescriptor>>,
    next_fd: usize,
}

pub struct FileDescriptor {
    pub handle: FileHandle,
    pub flags: u32,
    pub offset: u64,
}
```

### File Descriptor Allocation

```rust
pub fn alloc_fd(table: &mut FileDescriptorTable, handle: FileHandle, flags: u32) -> usize {
    let fd = table.next_fd;
    table.descriptors.push(Some(FileDescriptor {
        handle,
        flags,
        offset: 0,
    }));
    table.next_fd += 1;
    fd
}
```

## Caching

### Page Cache

```rust
pub struct PageCache {
    pages: HashMap<u64, CachedPage>,
    max_pages: usize,
}

pub struct CachedPage {
    pub data: [u8; 4096],
    pub dirty: bool,
    pub last_access: u64,
}
```

### Buffer Cache

```rust
pub struct BufferCache {
    buffers: HashMap<u64, CachedBuffer>,
    max_buffers: usize,
}

pub struct CachedBuffer {
    pub data: Vec<u8>,
    pub dirty: bool,
    pub last_access: u64,
}
```

## Security

### File Permissions

```rust
pub fn check_permissions(mode: u32, uid: u32, gid: u32, required: u32) -> bool {
    let current_uid = get_current_uid();
    let current_gid = get_current_gid();
    
    if current_uid == uid {
        // Owner permissions
        (mode & (required << 6)) != 0
    } else if current_gid == gid {
        // Group permissions
        (mode & (required << 3)) != 0
    } else {
        // Other permissions
        (mode & required) != 0
    }
}
```

### Capability Checks

```rust
pub fn check_file_capability(cap: u64, path: &str) -> bool {
    let process = get_current_process();
    if !process.has_capability(cap) {
        return false;
    }
    
    // Additional path-specific checks
    true
}
```

## Performance Optimization

### Read-Ahead

```rust
pub fn read_ahead(file: &mut File, offset: u64, size: usize) {
    let cache = get_page_cache();
    for i in 0..(size / 4096) {
        let page_offset = offset + (i * 4096) as u64;
        cache.prefetch(file.inode, page_offset);
    }
}
```

### Write-Back

```rust
pub fn write_back(cache: &mut PageCache) {
    for (_, page) in cache.pages.iter() {
        if page.dirty {
            write_page_to_disk(page);
            page.dirty = false;
        }
    }
}
```

## Future Enhancements

### Planned Features

1. **ZFS-style filesystem**: Advanced features like snapshots
2. **Btrfs**: Copy-on-write filesystem
3. **Network filesystems**: NFS, SMB support
4. **Distributed filesystems**: Ceph, GlusterFS
5. **User-space filesystems**: FUSE support

### Research Areas

1. **Persistent memory**: NVDIMM filesystems
2. **Erasure coding**: Distributed storage
3. **Deduplication**: Block-level deduplication
4. **Compression**: Transparent compression

## Best Practices

### For Kernel Developers

1. Use VFS layer for filesystem operations
2. Implement proper error handling
3. Support standard Unix semantics
4. Implement proper locking
5. Test with various filesystems

### For Userland Developers

1. Use standard file operations
2. Check return values
3. Close file descriptors
4. Use appropriate permissions
5. Consider using mmap for large files

## Troubleshooting

### Filesystem Corruption

**Symptoms**: Files not accessible, errors on read/write

**Solutions**:
1. Run filesystem check
2. Check for hardware errors
3. Review driver logs
4. Restore from backup

### Performance Issues

**Symptoms**: Slow file operations

**Solutions**:
1. Check cache settings
2. Verify disk health
3. Review I/O patterns
4. Consider different filesystem

### Mount Failures

**Symptoms**: Cannot mount filesystem

**Solutions**:
1. Check filesystem type
2. Verify device is accessible
3. Check for corruption
4. Review mount options

## References

- [Linux VFS](https://www.kernel.org/doc/html/latest/filesystems/vfs.html)
- [Ext2 Specification](https://www.kernel.org/doc/Documentation/filesystems/ext2.txt)
- [FAT32 Specification](https://www.microsoft.com/en-us/download/details.aspx?id=10038)
