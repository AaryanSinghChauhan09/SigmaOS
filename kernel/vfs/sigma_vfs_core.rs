//! SigmaOS — Virtual File System (VFS) Core
//! Provides a unified filesystem interface with mount points, inodes, and file handles.
//! No std, no allocator — fixed-size inode/mount tables.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I64 = i64;
type Usize = usize;

// ── Constants ───────────────────────────────────────────────────────────────
const MAX_PATH_LEN:     usize = 256;
const MAX_NAME_LEN:     usize = 64;
const MAX_INODES:       usize = 1024;
const MAX_OPEN_FILES:   usize = 256;
const MAX_MOUNT_POINTS: usize = 16;
const MAX_DENTRIES:     usize = 512;
const MAX_DATA_SIZE:    usize = 4096; // Per-file inline data

// ── Inode Types ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum InodeType {
    Free       = 0,
    RegularFile = 1,
    Directory  = 2,
    Symlink    = 3,
    CharDevice = 4,
    BlockDevice = 5,
    Fifo       = 6,
    Socket     = 7,
}

// ── File Permissions ────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct FilePermissions {
    pub mode: U16,   // Unix-style mode bits (rwxrwxrwx)
    pub uid:  U32,
    pub gid:  U32,
}

impl FilePermissions {
    pub const fn default_file() -> Self {
        FilePermissions { mode: 0o644, uid: 0, gid: 0 }
    }
    pub const fn default_dir() -> Self {
        FilePermissions { mode: 0o755, uid: 0, gid: 0 }
    }
}

// ── Timestamps ──────────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct Timestamps {
    pub created:  U64,  // nanoseconds since boot
    pub modified: U64,
    pub accessed: U64,
}

impl Timestamps {
    pub const fn zero() -> Self {
        Timestamps { created: 0, modified: 0, accessed: 0 }
    }
}

// ── Inode ───────────────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct Inode {
    pub ino:         U32,
    pub itype:       InodeType,
    pub perms:       FilePermissions,
    pub timestamps:  Timestamps,
    pub size:        U64,
    pub link_count:  U16,
    pub fs_id:       U16,          // Which filesystem owns this
    pub parent_ino:  U32,
    pub data:        [U8; MAX_DATA_SIZE],
    pub data_len:    usize,
}

impl Inode {
    pub const fn empty() -> Self {
        Inode {
            ino: 0, itype: InodeType::Free,
            perms: FilePermissions::default_file(),
            timestamps: Timestamps::zero(),
            size: 0, link_count: 0, fs_id: 0, parent_ino: 0,
            data: [0u8; MAX_DATA_SIZE], data_len: 0,
        }
    }
}

// ── Directory Entry ─────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct Dentry {
    pub name:      [U8; MAX_NAME_LEN],
    pub name_len:  usize,
    pub ino:       U32,
    pub parent_ino: U32,
    pub itype:     InodeType,
    pub active:    bool,
}

impl Dentry {
    pub const fn empty() -> Self {
        Dentry {
            name: [0u8; MAX_NAME_LEN], name_len: 0,
            ino: 0, parent_ino: 0, itype: InodeType::Free, active: false,
        }
    }
}

// ── File Handle ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OpenMode {
    Closed    = 0,
    ReadOnly  = 1,
    WriteOnly = 2,
    ReadWrite = 3,
    Append    = 4,
}

#[derive(Copy, Clone)]
pub struct FileHandle {
    pub fd:       i32,
    pub ino:      U32,
    pub mode:     OpenMode,
    pub offset:   U64,
    pub flags:    U32,
    pub pid:      U32,
    pub active:   bool,
}

impl FileHandle {
    pub const fn empty() -> Self {
        FileHandle {
            fd: -1, ino: 0, mode: OpenMode::Closed,
            offset: 0, flags: 0, pid: 0, active: false,
        }
    }
}

// ── Mount Point ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum FsType {
    None    = 0,
    Tmpfs   = 1,
    SigmaFs = 2,
    Ext4    = 3,
    ProcFs  = 4,
    SysFs   = 5,
    DevFs   = 6,
}

#[derive(Copy, Clone)]
pub struct MountPoint {
    pub path:    [U8; MAX_PATH_LEN],
    pub path_len: usize,
    pub fs_type: FsType,
    pub fs_id:   U16,
    pub root_ino: U32,
    pub flags:   U32,    // MS_RDONLY, etc.
    pub active:  bool,
}

impl MountPoint {
    pub const fn empty() -> Self {
        MountPoint {
            path: [0u8; MAX_PATH_LEN], path_len: 0,
            fs_type: FsType::None, fs_id: 0, root_ino: 0,
            flags: 0, active: false,
        }
    }
}

// ── VFS Global State ────────────────────────────────────────────────────────
pub struct Vfs {
    pub inodes:   [Inode; MAX_INODES],
    pub dentries: [Dentry; MAX_DENTRIES],
    pub files:    [FileHandle; MAX_OPEN_FILES],
    pub mounts:   [MountPoint; MAX_MOUNT_POINTS],
    pub inode_count:  usize,
    pub dentry_count: usize,
    pub file_count:   usize,
    pub mount_count:  usize,
    pub next_ino:     U32,
    pub next_fd:      i32,
}

static mut VFS: Vfs = Vfs {
    inodes:   [Inode::empty(); MAX_INODES],
    dentries: [Dentry::empty(); MAX_DENTRIES],
    files:    [FileHandle::empty(); MAX_OPEN_FILES],
    mounts:   [MountPoint::empty(); MAX_MOUNT_POINTS],
    inode_count: 0, dentry_count: 0, file_count: 0, mount_count: 0,
    next_ino: 1, next_fd: 3, // 0=stdin, 1=stdout, 2=stderr
};

// ── Helpers ─────────────────────────────────────────────────────────────────
fn bytes_eq(a: &[U8], al: usize, b: &[U8], bl: usize) -> bool {
    if al != bl { return false; }
    for i in 0..al {
        if a[i] != b[i] { return false; }
    }
    true
}

unsafe fn alloc_inode() -> Option<usize> {
    for i in 0..MAX_INODES {
        if VFS.inodes[i].itype == InodeType::Free {
            VFS.inodes[i].ino = VFS.next_ino;
            VFS.next_ino += 1;
            VFS.inode_count += 1;
            return Some(i);
        }
    }
    None
}

unsafe fn alloc_fd() -> Option<usize> {
    for i in 0..MAX_OPEN_FILES {
        if !VFS.files[i].active {
            return Some(i);
        }
    }
    None
}

unsafe fn find_dentry(parent_ino: U32, name: &[U8], name_len: usize) -> Option<usize> {
    for i in 0..MAX_DENTRIES {
        if VFS.dentries[i].active
            && VFS.dentries[i].parent_ino == parent_ino
            && bytes_eq(&VFS.dentries[i].name, VFS.dentries[i].name_len, name, name_len)
        {
            return Some(i);
        }
    }
    None
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize the VFS with a root directory.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_init() -> i32 {
    // Create root inode (ino = 1)
    let idx = match alloc_inode() {
        Some(i) => i,
        None => return -1,
    };
    let root = &mut VFS.inodes[idx];
    root.itype = InodeType::Directory;
    root.perms = FilePermissions::default_dir();
    root.link_count = 2; // . and parent

    // Create root mount point
    VFS.mounts[0].path[0] = b'/';
    VFS.mounts[0].path_len = 1;
    VFS.mounts[0].fs_type = FsType::Tmpfs;
    VFS.mounts[0].fs_id = 0;
    VFS.mounts[0].root_ino = root.ino;
    VFS.mounts[0].active = true;
    VFS.mount_count = 1;

    0
}

/// Mount a filesystem at the given path.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_mount(
    path: *const U8, path_len: usize,
    fs_type: U8,
) -> i32 {
    if VFS.mount_count >= MAX_MOUNT_POINTS { return -1; }
    if path.is_null() || path_len == 0 || path_len > MAX_PATH_LEN { return -2; }

    let idx = VFS.mount_count;
    let src = core::slice::from_raw_parts(path, path_len);
    for i in 0..path_len { VFS.mounts[idx].path[i] = src[i]; }
    VFS.mounts[idx].path_len = path_len;
    VFS.mounts[idx].fs_type = match fs_type {
        1 => FsType::Tmpfs,
        2 => FsType::SigmaFs,
        3 => FsType::Ext4,
        4 => FsType::ProcFs,
        5 => FsType::SysFs,
        6 => FsType::DevFs,
        _ => return -3,
    };

    // Create root inode for this mount
    let inode_idx = match alloc_inode() {
        Some(i) => i,
        None => return -4,
    };
    VFS.inodes[inode_idx].itype = InodeType::Directory;
    VFS.inodes[inode_idx].perms = FilePermissions::default_dir();
    VFS.mounts[idx].root_ino = VFS.inodes[inode_idx].ino;
    VFS.mounts[idx].fs_id = idx as U16;
    VFS.mounts[idx].active = true;
    VFS.mount_count += 1;

    0
}

/// Create a file or directory. Returns inode number or negative error.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_create(
    parent_ino: U32,
    name: *const U8, name_len: usize,
    itype: U8,
) -> i32 {
    if name.is_null() || name_len == 0 || name_len > MAX_NAME_LEN { return -1; }
    let name_slice = core::slice::from_raw_parts(name, name_len);

    // Check if name already exists
    if find_dentry(parent_ino, name_slice, name_len).is_some() {
        return -17; // EEXIST
    }

    // Allocate inode
    let idx = match alloc_inode() {
        Some(i) => i,
        None => return -28, // ENOSPC
    };

    let inode = &mut VFS.inodes[idx];
    inode.itype = match itype {
        1 => InodeType::RegularFile,
        2 => InodeType::Directory,
        3 => InodeType::Symlink,
        _ => InodeType::RegularFile,
    };
    inode.parent_ino = parent_ino;
    inode.perms = if itype == 2 {
        FilePermissions::default_dir()
    } else {
        FilePermissions::default_file()
    };

    // Create dentry
    for d in 0..MAX_DENTRIES {
        if !VFS.dentries[d].active {
            for i in 0..name_len {
                VFS.dentries[d].name[i] = name_slice[i];
            }
            VFS.dentries[d].name_len = name_len;
            VFS.dentries[d].ino = inode.ino;
            VFS.dentries[d].parent_ino = parent_ino;
            VFS.dentries[d].itype = inode.itype;
            VFS.dentries[d].active = true;
            VFS.dentry_count += 1;
            break;
        }
    }

    inode.ino as i32
}

/// Open a file by inode number. Returns file descriptor or negative error.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_open(ino: U32, mode: U8) -> i32 {
    // Find inode
    let mut found = false;
    for i in 0..MAX_INODES {
        if VFS.inodes[i].ino == ino && VFS.inodes[i].itype != InodeType::Free {
            found = true;
            break;
        }
    }
    if !found { return -2; } // ENOENT

    let fd_idx = match alloc_fd() {
        Some(i) => i,
        None => return -24, // EMFILE
    };

    let fd = VFS.next_fd;
    VFS.next_fd += 1;

    VFS.files[fd_idx].fd = fd;
    VFS.files[fd_idx].ino = ino;
    VFS.files[fd_idx].mode = match mode {
        1 => OpenMode::ReadOnly,
        2 => OpenMode::WriteOnly,
        3 => OpenMode::ReadWrite,
        4 => OpenMode::Append,
        _ => OpenMode::ReadOnly,
    };
    VFS.files[fd_idx].offset = 0;
    VFS.files[fd_idx].active = true;
    VFS.file_count += 1;

    fd
}

/// Read from an open file. Returns bytes read or negative error.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_read(fd: i32, buf: *mut U8, count: U32) -> i32 {
    if buf.is_null() { return -1; }

    // Find file handle
    let mut fh: Option<usize> = None;
    for i in 0..MAX_OPEN_FILES {
        if VFS.files[i].active && VFS.files[i].fd == fd {
            fh = Some(i);
            break;
        }
    }
    let fi = match fh { Some(i) => i, None => return -9 }; // EBADF

    // Find inode
    let ino = VFS.files[fi].ino;
    let mut inode_idx: Option<usize> = None;
    for i in 0..MAX_INODES {
        if VFS.inodes[i].ino == ino {
            inode_idx = Some(i);
            break;
        }
    }
    let ii = match inode_idx { Some(i) => i, None => return -2 };

    let offset = VFS.files[fi].offset as usize;
    let data_len = VFS.inodes[ii].data_len;
    if offset >= data_len { return 0; } // EOF

    let avail = data_len - offset;
    let to_read = (count as usize).min(avail);
    let dst = core::slice::from_raw_parts_mut(buf, to_read);
    for i in 0..to_read {
        dst[i] = VFS.inodes[ii].data[offset + i];
    }
    VFS.files[fi].offset += to_read as U64;

    to_read as i32
}

/// Write to an open file. Returns bytes written or negative error.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_write(fd: i32, buf: *const U8, count: U32) -> i32 {
    if buf.is_null() { return -1; }

    let mut fh: Option<usize> = None;
    for i in 0..MAX_OPEN_FILES {
        if VFS.files[i].active && VFS.files[i].fd == fd {
            fh = Some(i);
            break;
        }
    }
    let fi = match fh { Some(i) => i, None => return -9 };

    let ino = VFS.files[fi].ino;
    let mut inode_idx: Option<usize> = None;
    for i in 0..MAX_INODES {
        if VFS.inodes[i].ino == ino {
            inode_idx = Some(i);
            break;
        }
    }
    let ii = match inode_idx { Some(i) => i, None => return -2 };

    let offset = if VFS.files[fi].mode == OpenMode::Append {
        VFS.inodes[ii].data_len
    } else {
        VFS.files[fi].offset as usize
    };

    let src = core::slice::from_raw_parts(buf, count as usize);
    let space = MAX_DATA_SIZE - offset;
    let to_write = (count as usize).min(space);
    for i in 0..to_write {
        VFS.inodes[ii].data[offset + i] = src[i];
    }
    let new_end = offset + to_write;
    if new_end > VFS.inodes[ii].data_len {
        VFS.inodes[ii].data_len = new_end;
        VFS.inodes[ii].size = new_end as U64;
    }
    VFS.files[fi].offset = new_end as U64;

    to_write as i32
}

/// Close a file descriptor.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_close(fd: i32) -> i32 {
    for i in 0..MAX_OPEN_FILES {
        if VFS.files[i].active && VFS.files[i].fd == fd {
            VFS.files[i].active = false;
            VFS.files[i].mode = OpenMode::Closed;
            VFS.file_count -= 1;
            return 0;
        }
    }
    -9 // EBADF
}

/// Seek within a file. Returns new offset or negative error.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_seek(fd: i32, offset: I64, whence: U8) -> I64 {
    let mut fh: Option<usize> = None;
    for i in 0..MAX_OPEN_FILES {
        if VFS.files[i].active && VFS.files[i].fd == fd {
            fh = Some(i);
            break;
        }
    }
    let fi = match fh { Some(i) => i, None => return -9 };

    let ino = VFS.files[fi].ino;
    let mut file_size: U64 = 0;
    for i in 0..MAX_INODES {
        if VFS.inodes[i].ino == ino {
            file_size = VFS.inodes[i].size;
            break;
        }
    }

    let new_offset: I64 = match whence {
        0 => offset,                                    // SEEK_SET
        1 => VFS.files[fi].offset as I64 + offset,    // SEEK_CUR
        2 => file_size as I64 + offset,                // SEEK_END
        _ => return -22, // EINVAL
    };

    if new_offset < 0 { return -22; }
    VFS.files[fi].offset = new_offset as U64;
    new_offset
}

/// Delete a file by name under a parent directory.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_unlink(parent_ino: U32, name: *const U8, name_len: usize) -> i32 {
    if name.is_null() || name_len == 0 { return -1; }
    let name_slice = core::slice::from_raw_parts(name, name_len);

    let dentry_idx = match find_dentry(parent_ino, name_slice, name_len) {
        Some(i) => i,
        None => return -2, // ENOENT
    };

    let ino = VFS.dentries[dentry_idx].ino;

    // Remove dentry
    VFS.dentries[dentry_idx].active = false;
    VFS.dentry_count -= 1;

    // Free inode
    for i in 0..MAX_INODES {
        if VFS.inodes[i].ino == ino {
            VFS.inodes[i].itype = InodeType::Free;
            VFS.inodes[i].data_len = 0;
            VFS.inodes[i].size = 0;
            VFS.inode_count -= 1;
            break;
        }
    }

    0
}

/// Get total inode count.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_inode_count() -> U32 { VFS.inode_count as U32 }

/// Get total open file count.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_open_files() -> U32 { VFS.file_count as U32 }

/// Get total mount count.
#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_mount_count() -> U32 { VFS.mount_count as U32 }
