//! SigmaOS — Virtual File System (VFS)
//! A capability-based VFS layer handling inodes, dentries, and superblocks.
//! Pure no_std implementation.

#![no_std]
#![allow(dead_code)]

type U8    = u8;
type U16   = u16;
type U32   = u32;
type U64   = u64;
type I32   = i32;
type I64   = i64;
type Usize = usize;
type Bool  = bool;

// ── VFS Constants ─────────────────────────────────────────────────────────────
pub const MAX_PATH: Usize = 4096;
pub const MAX_NAME: Usize = 255;
pub const MAX_FDS:  Usize = 1024;
pub const MAX_INODES: Usize = 4096;
pub const MAX_SUPERBLOCKS: Usize = 16;

// ── VFS Data Structures ───────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FileType {
    Unknown   = 0,
    Regular   = 1,
    Directory = 2,
    CharDev   = 3,
    BlockDev  = 4,
    Fifo      = 5,
    Socket    = 6,
    Symlink   = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Inode {
    pub ino:        U64,
    pub dev_id:     U32,
    pub mode:       U16,
    pub file_type:  FileType,
    pub links:      U32,
    pub uid:        U32,
    pub gid:        U32,
    pub size:       U64,
    pub atime:      U64,
    pub mtime:      U64,
    pub ctime:      U64,
    // capability required to access this inode
    pub cap_token:  U64,
}

impl Inode {
    pub const fn zero() -> Self {
        Inode {
            ino: 0, dev_id: 0, mode: 0, file_type: FileType::Unknown,
            links: 0, uid: 0, gid: 0, size: 0,
            atime: 0, mtime: 0, ctime: 0, cap_token: 0,
        }
    }
}

/// A directory entry (Dentry) mapping a name to an inode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dentry {
    pub ino:       U64,
    pub parent_ino:U64,
    pub name:      [U8; MAX_NAME + 1],
    pub name_len:  U8,
}

impl Dentry {
    pub const fn zero() -> Self {
        Dentry { ino: 0, parent_ino: 0, name: [0; MAX_NAME + 1], name_len: 0 }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FileDesc {
    pub ino:      U64,
    pub offset:   U64,
    pub flags:    U32,
    pub refcount: U32,
}

impl FileDesc {
    pub const fn zero() -> Self {
        FileDesc { ino: 0, offset: 0, flags: 0, refcount: 0 }
    }
}

// ── VFS State ─────────────────────────────────────────────────────────────────
static mut INODES: [Inode; MAX_INODES] = [Inode::zero(); MAX_INODES];
static mut FDS:    [FileDesc; MAX_FDS] = [FileDesc::zero(); MAX_FDS];

/// Helper: C-string length
fn strlen(s: *const U8) -> Usize {
    let mut len = 0;
    unsafe {
        while *s.add(len) != 0 && len < MAX_PATH { len += 1; }
    }
    len
}

/// Helper: String equality
fn streq(s1: *const U8, s2: *const U8, len: Usize) -> Bool {
    unsafe {
        for i in 0..len {
            if *s1.add(i) != *s2.add(i) { return false; }
            if *s1.add(i) == 0 { break; }
        }
    }
    true
}

// ── VFS API ───────────────────────────────────────────────────────────────────

/// Initialize the VFS subsystem
#[no_mangle]
pub unsafe extern "C" fn vfs_init() {
    for i in 0..MAX_INODES { INODES[i] = Inode::zero(); }
    for i in 0..MAX_FDS { FDS[i] = FileDesc::zero(); }
}

/// Allocate a new inode. Returns index or U64::MAX on error.
#[no_mangle]
pub unsafe extern "C" fn vfs_alloc_inode(
    file_type: FileType, mode: U16, uid: U32, gid: U32, cap: U64,
) -> U64 {
    for (i, node) in INODES.iter_mut().enumerate().skip(1) {
        if node.links == 0 {
            *node = Inode {
                ino: i as U64, dev_id: 1, mode, file_type,
                links: 1, uid, gid, size: 0,
                atime: 0, mtime: 0, ctime: 0, cap_token: cap,
            };
            return i as U64;
        }
    }
    U64::MAX
}

/// Open a file descriptor for a given inode
#[no_mangle]
pub unsafe extern "C" fn vfs_open(ino: U64, flags: U32, cap: U64) -> I32 {
    if ino as Usize >= MAX_INODES { return -1; }
    let inode = &INODES[ino as Usize];
    if inode.links == 0 { return -1; }
    if inode.cap_token != 0 && inode.cap_token != cap { return -1; } // access denied

    for i in 0..MAX_FDS {
        if FDS[i].refcount == 0 {
            FDS[i].ino = ino;
            FDS[i].offset = if flags & 0x400 != 0 { inode.size } else { 0 }; // O_APPEND
            FDS[i].flags = flags;
            FDS[i].refcount = 1;
            return i as I32;
        }
    }
    -1 // EMFILE
}

/// Close a file descriptor
#[no_mangle]
pub unsafe extern "C" fn vfs_close(fd: I32) -> I32 {
    if fd < 0 || fd as Usize >= MAX_FDS { return -1; }
    let f = &mut FDS[fd as Usize];
    if f.refcount > 0 {
        f.refcount -= 1;
        if f.refcount == 0 { *f = FileDesc::zero(); }
        return 0;
    }
    -1 // EBADF
}

/// Read from a file descriptor. The underlying FS driver would be called here.
#[no_mangle]
pub unsafe extern "C" fn vfs_read(fd: I32, buf: *mut U8, count: Usize) -> I64 {
    if fd < 0 || fd as Usize >= MAX_FDS || buf.is_null() { return -1; }
    let f = &mut FDS[fd as Usize];
    if f.refcount == 0 { return -1; }
    let inode = &INODES[f.ino as Usize];

    if f.offset >= inode.size { return 0; } // EOF
    let to_read = count.min((inode.size - f.offset) as Usize);

    // In a real implementation, we would route this to the block device / fs driver
    // For now, just advance the offset as a stub
    f.offset += to_read as U64;
    to_read as I64
}

/// Write to a file descriptor.
#[no_mangle]
pub unsafe extern "C" fn vfs_write(fd: I32, buf: *const U8, count: Usize) -> I64 {
    if fd < 0 || fd as Usize >= MAX_FDS || buf.is_null() { return -1; }
    let f = &mut FDS[fd as Usize];
    if f.refcount == 0 { return -1; }
    if f.flags & 0x01 == 0 && f.flags & 0x02 == 0 { return -1; } // not open for writing

    let inode = &mut INODES[f.ino as Usize];
    // Route to FS driver...

    f.offset += count as U64;
    if f.offset > inode.size { inode.size = f.offset; }
    count as I64
}

/// Change file size
#[no_mangle]
pub unsafe extern "C" fn vfs_truncate(ino: U64, length: U64, cap: U64) -> I32 {
    if ino as Usize >= MAX_INODES { return -1; }
    let inode = &mut INODES[ino as Usize];
    if inode.links == 0 { return -1; }
    if inode.cap_token != 0 && inode.cap_token != cap { return -1; }
    inode.size = length;
    0
}
