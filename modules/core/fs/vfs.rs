/// SigmaOS — modules/core/fs/vfs.rs
/// Virtual Filesystem Switch (VFS).
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaI64   = i64;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ────────────────────────────────────────────────────────────────

pub const VFS_MAX_OPEN_FILES: SigmaUsize = 1024;
pub const VFS_MAX_MOUNTS:     SigmaUsize = 32;
pub const VFS_MAX_PATH_LEN:   SigmaUsize = 256;

// Flags
pub const O_RDONLY: SigmaU32 = 0x0000;
pub const O_WRONLY: SigmaU32 = 0x0001;
pub const O_RDWR:   SigmaU32 = 0x0002;
pub const O_CREAT:  SigmaU32 = 0x0040;
pub const O_TRUNC:  SigmaU32 = 0x0200;
pub const O_APPEND: SigmaU32 = 0x0400;

// ─── Data Structures ──────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NodeType {
    File      = 1,
    Directory = 2,
    Symlink   = 3,
    Device    = 4,
    Pipe      = 5,
    Socket    = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VfsNode {
    pub inode_num: SigmaU64,
    pub size:      SigmaU64,
    pub node_type: NodeType,
    pub dev_id:    SigmaU32,
    pub mode:      SigmaU32,
    pub uid:       SigmaU32,
    pub gid:       SigmaU32,
}

impl VfsNode {
    pub const fn empty() -> Self {
        VfsNode {
            inode_num: 0,
            size:      0,
            node_type: NodeType::File,
            dev_id:    0,
            mode:      0,
            uid:       0,
            gid:       0,
        }
    }
}

/// Function pointer table for filesystem drivers (Ext4, SigmaFS, FAT32).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FsOps {
    pub open:  Option<unsafe extern "C" fn(path: *const u8, flags: SigmaU32, mode: SigmaU32, out_node: *mut VfsNode) -> SigmaI32>,
    pub read:  Option<unsafe extern "C" fn(node: *mut VfsNode, offset: SigmaU64, buf: *mut u8, len: SigmaUsize) -> SigmaI64>,
    pub write: Option<unsafe extern "C" fn(node: *mut VfsNode, offset: SigmaU64, buf: *const u8, len: SigmaUsize) -> SigmaI64>,
    pub close: Option<unsafe extern "C" fn(node: *mut VfsNode) -> SigmaI32>,
    pub stat:  Option<unsafe extern "C" fn(node: *mut VfsNode, out_stat: *mut u8) -> SigmaI32>,
}

impl FsOps {
    pub const fn empty() -> Self {
        FsOps { open: None, read: None, write: None, close: None, stat: None }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MountPoint {
    pub path:    [u8; VFS_MAX_PATH_LEN],
    pub dev_id:  SigmaU32,
    pub ops:     FsOps,
    pub mounted: SigmaBool,
}

impl MountPoint {
    pub const fn empty() -> Self {
        MountPoint {
            path:    [0; VFS_MAX_PATH_LEN],
            dev_id:  0,
            ops:     FsOps::empty(),
            mounted: false,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FileDescriptor {
    pub node:      VfsNode,
    pub ops:       FsOps,
    pub offset:    SigmaU64,
    pub flags:     SigmaU32,
    pub allocated: SigmaBool,
}

impl FileDescriptor {
    pub const fn empty() -> Self {
        FileDescriptor {
            node:      VfsNode::empty(),
            ops:       FsOps::empty(),
            offset:    0,
            flags:     0,
            allocated: false,
        }
    }
}

// ─── Global State ─────────────────────────────────────────────────────────────

static mut MOUNT_TABLE: [MountPoint; VFS_MAX_MOUNTS] = [MountPoint::empty(); VFS_MAX_MOUNTS];
static mut FD_TABLE: [FileDescriptor; VFS_MAX_OPEN_FILES] = [FileDescriptor::empty(); VFS_MAX_OPEN_FILES];

// ─── Implementation ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn vfs_init() -> SigmaI32 {
    for m in MOUNT_TABLE.iter_mut() { m.mounted = false; }
    for fd in FD_TABLE.iter_mut() { fd.allocated = false; }
    0
}

/// Helper to string-compare null-terminated paths simply
unsafe fn path_starts_with(path: *const u8, prefix: &[u8]) -> bool {
    for i in 0..prefix.len() {
        if *path.add(i) == 0 || *path.add(i) != prefix[i] {
            return false;
        }
    }
    true
}

/// Find the appropriate mount point for a given path
unsafe fn find_mount(path: *const u8) -> Option<&'static MountPoint> {
    // In a real implementation this finds the longest prefix match.
    // For now we just return the first active mount point.
    for m in MOUNT_TABLE.iter() {
        if m.mounted { return Some(m); }
    }
    None
}

#[no_mangle]
pub unsafe extern "C" fn vfs_mount(
    path: *const u8,
    dev_id: SigmaU32,
    ops: *const FsOps,
) -> SigmaI32 {
    if path.is_null() || ops.is_null() { return -1; }
    
    for m in MOUNT_TABLE.iter_mut() {
        if !m.mounted {
            // Copy path
            let mut i = 0;
            while *path.add(i) != 0 && i < VFS_MAX_PATH_LEN - 1 {
                m.path[i] = *path.add(i);
                i += 1;
            }
            m.path[i] = 0;
            
            m.dev_id  = dev_id;
            m.ops     = *ops;
            m.mounted = true;
            return 0;
        }
    }
    -12 // ENOMEM (no mount slots)
}

#[no_mangle]
pub unsafe extern "C" fn vfs_open(path: *const u8, flags: SigmaU32, mode: SigmaU32) -> SigmaI32 {
    if path.is_null() { return -1; }
    
    let mount = match find_mount(path) {
        Some(m) => m,
        None => return -2, // ENOENT
    };
    
    if let Some(open_fn) = mount.ops.open {
        // Find free FD
        for (i, fd) in FD_TABLE.iter_mut().enumerate() {
            if !fd.allocated {
                let rc = open_fn(path, flags, mode, &mut fd.node);
                if rc == 0 {
                    fd.ops       = mount.ops;
                    fd.offset    = 0;
                    fd.flags     = flags;
                    fd.allocated = true;
                    return i as SigmaI32;
                }
                return rc;
            }
        }
        return -24; // EMFILE
    }
    -38 // ENOSYS
}

#[no_mangle]
pub unsafe extern "C" fn vfs_read(fd_idx: SigmaI32, buf: *mut u8, len: SigmaUsize) -> SigmaI64 {
    if fd_idx < 0 || fd_idx as usize >= VFS_MAX_OPEN_FILES { return -9; } // EBADF
    if buf.is_null() { return -1; }
    
    let fd = &mut FD_TABLE[fd_idx as usize];
    if !fd.allocated { return -9; }
    
    if let Some(read_fn) = fd.ops.read {
        let rc = read_fn(&mut fd.node, fd.offset, buf, len);
        if rc > 0 {
            fd.offset = fd.offset.wrapping_add(rc as SigmaU64);
        }
        return rc;
    }
    -38 // ENOSYS
}

#[no_mangle]
pub unsafe extern "C" fn vfs_write(fd_idx: SigmaI32, buf: *const u8, len: SigmaUsize) -> SigmaI64 {
    if fd_idx < 0 || fd_idx as usize >= VFS_MAX_OPEN_FILES { return -9; }
    if buf.is_null() { return -1; }
    
    let fd = &mut FD_TABLE[fd_idx as usize];
    if !fd.allocated { return -9; }
    
    if let Some(write_fn) = fd.ops.write {
        if (fd.flags & O_APPEND) != 0 {
            fd.offset = fd.node.size; // Append mode
        }
        
        let rc = write_fn(&mut fd.node, fd.offset, buf, len);
        if rc > 0 {
            fd.offset = fd.offset.wrapping_add(rc as SigmaU64);
            if fd.offset > fd.node.size {
                fd.node.size = fd.offset;
            }
        }
        return rc;
    }
    -38
}

#[no_mangle]
pub unsafe extern "C" fn vfs_close(fd_idx: SigmaI32) -> SigmaI32 {
    if fd_idx < 0 || fd_idx as usize >= VFS_MAX_OPEN_FILES { return -9; }
    
    let fd = &mut FD_TABLE[fd_idx as usize];
    if !fd.allocated { return -9; }
    
    let mut rc = 0;
    if let Some(close_fn) = fd.ops.close {
        rc = close_fn(&mut fd.node);
    }
    
    fd.allocated = false;
    rc
}
