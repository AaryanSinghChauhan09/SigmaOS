// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS sigma_tmpfs — Sovereign RAM Filesystem
//! Memory-backed filesystem with inode management and size limits.
//! no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaI64   = i64;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ──────────────────────────────────────────────────────────────
pub const TMPFS_MAX_INODES:    usize = 1024;
pub const TMPFS_MAX_CHILDREN:  usize = 64;
pub const TMPFS_NAME_LEN:      usize = 64;
pub const TMPFS_DATA_INLINE:   usize = 4096; // inline data for small files
pub const TMPFS_MAX_MOUNTS:    usize = 8;

// ─── Inode Types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum TmpfsInodeType {
    File      = 0,
    Directory = 1,
    SymLink   = 2,
    Fifo      = 3,   // named pipe
    Socket    = 4,   // Unix domain socket
    CharDev   = 5,
    BlockDev  = 6,
}

/// File permissions (Unix-style octal)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TmpfsPermissions {
    pub mode:  SigmaU32,   // e.g., 0o755
    pub uid:   SigmaU32,
    pub gid:   SigmaU32,
}

/// Timestamps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TmpfsTimestamps {
    pub created:  SigmaU64,   // seconds since epoch
    pub modified: SigmaU64,
    pub accessed: SigmaU64,
}

/// A single inode in the tmpfs
#[repr(C)]
#[derive(Clone)]
pub struct TmpfsInode {
    pub ino:        SigmaU64,          // inode number
    pub itype:      TmpfsInodeType,
    pub perms:      TmpfsPermissions,
    pub times:      TmpfsTimestamps,
    pub name:       [u8; TMPFS_NAME_LEN],
    pub parent_ino: SigmaI64,          // -1 = root
    // Directory: children
    pub children:    [SigmaU64; TMPFS_MAX_CHILDREN],
    pub child_count: SigmaU32,
    // File: inline data
    pub data:        [u8; TMPFS_DATA_INLINE],
    pub data_len:    SigmaU32,
    // SymLink: target path
    pub link_target: [u8; TMPFS_NAME_LEN],
    // Device: major/minor
    pub dev_major:   SigmaU32,
    pub dev_minor:   SigmaU32,
    // Link count
    pub nlink:       SigmaU32,
    pub active:      SigmaBool,
}

impl TmpfsInode {
    pub const fn empty() -> Self {
        Self {
            ino: 0,
            itype: TmpfsInodeType::File,
            perms: TmpfsPermissions { mode: 0o644, uid: 0, gid: 0 },
            times: TmpfsTimestamps { created: 0, modified: 0, accessed: 0 },
            name: [0u8; TMPFS_NAME_LEN],
            parent_ino: -1,
            children: [0u64; TMPFS_MAX_CHILDREN],
            child_count: 0,
            data: [0u8; TMPFS_DATA_INLINE],
            data_len: 0,
            link_target: [0u8; TMPFS_NAME_LEN],
            dev_major: 0,
            dev_minor: 0,
            nlink: 1,
            active: false,
        }
    }
}

/// Mount options for a tmpfs instance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TmpfsMountOpts {
    pub size_limit_kb:   SigmaU64,   // max total size in KB (0 = unlimited)
    pub inode_limit:     SigmaU32,   // max inodes (0 = default)
    pub default_mode:    SigmaU32,   // default file mode
    pub default_uid:     SigmaU32,
    pub default_gid:     SigmaU32,
    pub noexec:          SigmaBool,  // disallow execution
    pub nosuid:          SigmaBool,  // disallow setuid
    pub nodev:           SigmaBool,  // disallow device nodes
}

impl TmpfsMountOpts {
    pub const fn default() -> Self {
        Self {
            size_limit_kb: 0,
            inode_limit:   0,
            default_mode:  0o1777,   // sticky bit like /tmp
            default_uid:   0,
            default_gid:   0,
            noexec:        false,
            nosuid:        true,
            nodev:         true,
        }
    }
}

/// A single tmpfs mount instance
#[repr(C)]
pub struct TmpfsMount {
    pub mount_point: [u8; TMPFS_NAME_LEN],
    pub opts:        TmpfsMountOpts,
    pub inodes:      [TmpfsInode; TMPFS_MAX_INODES],
    pub inode_count: SigmaU32,
    pub next_ino:    SigmaU64,
    pub total_bytes: SigmaU64,
    pub active:      SigmaBool,
}

// ─── Global State ───────────────────────────────────────────────────────────

// We can only use a single mount instance as a static due to the enormous
// size of TmpfsInode array. In a real kernel, mounts would use page allocator.
static mut TMPFS_INSTANCE: TmpfsMount = TmpfsMount {
    mount_point: [0u8; TMPFS_NAME_LEN],
    opts:        TmpfsMountOpts::default(),
    inodes:      [TmpfsInode::empty(); TMPFS_MAX_INODES],
    inode_count: 0,
    next_ino:    2,  // 1 = root inode
    total_bytes: 0,
    active:      false,
};

// ─── Helpers ────────────────────────────────────────────────────────────────

unsafe fn tmpfs_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n { let b = *src.add(i); *dst.add(i) = b; if b == 0 { return; } i += 1; }
    if n > 0 { *dst.add(n - 1) = 0; }
}

unsafe fn tmpfs_strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0usize;
    loop {
        let ca = *a.add(i); let cb = *b.add(i);
        if ca != cb { return ca as i32 - cb as i32; }
        if ca == 0  { return 0; }
        i += 1;
    }
}

unsafe fn tmpfs_find_free_slot() -> SigmaI32 {
    let t = &TMPFS_INSTANCE;
    for i in 0..TMPFS_MAX_INODES {
        if !t.inodes[i].active { return i as SigmaI32; }
    }
    -1
}

unsafe fn tmpfs_find_by_ino(ino: SigmaU64) -> SigmaI32 {
    let t = &TMPFS_INSTANCE;
    for i in 0..t.inode_count as usize {
        if t.inodes[i].active && t.inodes[i].ino == ino {
            return i as SigmaI32;
        }
    }
    // Also search beyond inode_count in case of gaps
    for i in t.inode_count as usize..TMPFS_MAX_INODES {
        if t.inodes[i].active && t.inodes[i].ino == ino {
            return i as SigmaI32;
        }
    }
    -1
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

/// Mount a new tmpfs instance
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_mount(
    mount_point: *const u8,
    opts:        *const TmpfsMountOpts,
) -> SigmaI32 {
    let t = &mut TMPFS_INSTANCE;
    if t.active { return -1; } // already mounted

    tmpfs_strncpy(t.mount_point.as_mut_ptr(), mount_point, TMPFS_NAME_LEN);
    t.opts = if opts.is_null() { TmpfsMountOpts::default() } else { *opts };
    t.inode_count = 0;
    t.next_ino    = 2;
    t.total_bytes = 0;
    t.active      = true;

    // Create root directory (inode 1)
    let root = &mut t.inodes[0];
    root.ino        = 1;
    root.itype      = TmpfsInodeType::Directory;
    root.perms.mode = t.opts.default_mode;
    root.perms.uid  = t.opts.default_uid;
    root.perms.gid  = t.opts.default_gid;
    root.parent_ino = -1;
    root.nlink      = 2; // . and parent
    root.active     = true;
    let name = b"/\0";
    tmpfs_strncpy(root.name.as_mut_ptr(), name.as_ptr(), TMPFS_NAME_LEN);
    t.inode_count = 1;

    0
}

/// Unmount tmpfs
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_umount() -> SigmaI32 {
    let t = &mut TMPFS_INSTANCE;
    if !t.active { return -1; }
    // Deactivate all inodes
    for i in 0..TMPFS_MAX_INODES {
        t.inodes[i].active = false;
    }
    t.inode_count = 0;
    t.total_bytes = 0;
    t.active      = false;
    0
}

/// Create a file or directory
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_create(
    parent_ino: SigmaU64,
    name:       *const u8,
    itype:      TmpfsInodeType,
    mode:       SigmaU32,
) -> SigmaI64 {
    let t = &mut TMPFS_INSTANCE;
    if !t.active { return -1; }

    // Check inode limit
    if t.opts.inode_limit > 0 && t.inode_count >= t.opts.inode_limit { return -2; }

    let parent_slot = tmpfs_find_by_ino(parent_ino);
    if parent_slot < 0 { return -3; }

    let slot = tmpfs_find_free_slot();
    if slot < 0 { return -4; }

    // Check parent has room for children
    if t.inodes[parent_slot as usize].child_count as usize >= TMPFS_MAX_CHILDREN { return -5; }

    let ino = t.next_ino;
    t.next_ino += 1;

    let node = &mut t.inodes[slot as usize];
    *node = TmpfsInode::empty();
    node.ino        = ino;
    node.itype      = itype;
    node.perms.mode = mode;
    node.parent_ino = parent_ino as SigmaI64;
    node.nlink      = if itype == TmpfsInodeType::Directory { 2 } else { 1 };
    node.active     = true;
    tmpfs_strncpy(node.name.as_mut_ptr(), name, TMPFS_NAME_LEN);

    // Link to parent
    let p = &mut t.inodes[parent_slot as usize];
    p.children[p.child_count as usize] = ino;
    p.child_count += 1;
    if itype == TmpfsInodeType::Directory { p.nlink += 1; }

    t.inode_count += 1;
    ino as SigmaI64
}

/// Write data to a file inode
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_write(
    ino:    SigmaU64,
    offset: SigmaU32,
    data:   *const u8,
    len:    SigmaU32,
) -> SigmaI32 {
    let t = &mut TMPFS_INSTANCE;
    let slot = tmpfs_find_by_ino(ino);
    if slot < 0 { return -1; }

    let node = &mut t.inodes[slot as usize];
    if node.itype != TmpfsInodeType::File { return -2; }

    // Check size limit
    let end = offset as SigmaU64 + len as SigmaU64;
    if end > TMPFS_DATA_INLINE as SigmaU64 { return -3; } // exceeds inline limit

    if t.opts.size_limit_kb > 0 {
        let new_total = t.total_bytes + len as SigmaU64;
        if new_total > t.opts.size_limit_kb * 1024 { return -4; }
    }

    // Check noexec
    if t.opts.noexec && node.perms.mode & 0o111 != 0 {
        // Strip execute bits if noexec
        node.perms.mode &= !0o111u32;
    }

    for i in 0..len as usize {
        node.data[offset as usize + i] = *data.add(i);
    }
    if end as SigmaU32 > node.data_len {
        let delta = end as SigmaU64 - node.data_len as SigmaU64;
        t.total_bytes += delta;
        node.data_len = end as SigmaU32;
    }

    len as SigmaI32
}

/// Read data from a file inode
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_read(
    ino:    SigmaU64,
    offset: SigmaU32,
    buf:    *mut u8,
    len:    SigmaU32,
) -> SigmaI32 {
    let t = &TMPFS_INSTANCE;
    let slot = tmpfs_find_by_ino(ino);
    if slot < 0 { return -1; }

    let node = &t.inodes[slot as usize];
    if node.itype != TmpfsInodeType::File { return -2; }

    let avail = if offset >= node.data_len { 0 } else { node.data_len - offset };
    let copy_len = if len < avail { len } else { avail };

    for i in 0..copy_len as usize {
        *buf.add(i) = node.data[offset as usize + i];
    }

    copy_len as SigmaI32
}

/// Unlink (delete) a file or empty directory
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_unlink(parent_ino: SigmaU64, name: *const u8) -> SigmaI32 {
    let t = &mut TMPFS_INSTANCE;
    let parent_slot = tmpfs_find_by_ino(parent_ino);
    if parent_slot < 0 { return -1; }

    let p = &t.inodes[parent_slot as usize];
    let mut target_ino: SigmaU64 = 0;
    let mut child_idx: usize = 0;
    let mut found = false;

    for i in 0..p.child_count as usize {
        let ci = p.children[i];
        let cs = tmpfs_find_by_ino(ci);
        if cs >= 0 && tmpfs_strcmp(t.inodes[cs as usize].name.as_ptr(), name) == 0 {
            target_ino = ci;
            child_idx  = i;
            found = true;
            break;
        }
    }
    if !found { return -2; }

    let target_slot = tmpfs_find_by_ino(target_ino);
    if target_slot < 0 { return -3; }

    let target = &t.inodes[target_slot as usize];
    // Can't unlink non-empty directories
    if target.itype == TmpfsInodeType::Directory && target.child_count > 0 {
        return -4; // ENOTEMPTY
    }

    // Free data bytes
    if target.itype == TmpfsInodeType::File {
        t.total_bytes = t.total_bytes.saturating_sub(target.data_len as SigmaU64);
    }

    // Deactivate
    t.inodes[target_slot as usize].active = false;

    // Remove from parent's children array (shift left)
    let p_mut = &mut t.inodes[parent_slot as usize];
    for i in child_idx..(p_mut.child_count as usize - 1) {
        p_mut.children[i] = p_mut.children[i + 1];
    }
    p_mut.child_count -= 1;
    if t.inodes[target_slot as usize].itype == TmpfsInodeType::Directory {
        p_mut.nlink = p_mut.nlink.saturating_sub(1);
    }
    t.inode_count = t.inode_count.saturating_sub(1);

    0
}

/// List directory entries
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_readdir(
    dir_ino:  SigmaU64,
    out_inos: *mut SigmaU64,
    max:      SigmaU32,
) -> SigmaU32 {
    let t = &TMPFS_INSTANCE;
    let slot = tmpfs_find_by_ino(dir_ino);
    if slot < 0 { return 0; }

    let node = &t.inodes[slot as usize];
    if node.itype != TmpfsInodeType::Directory { return 0; }

    let count = if node.child_count < max { node.child_count } else { max };
    for i in 0..count as usize {
        *out_inos.add(i) = node.children[i];
    }
    count
}

/// Create a symbolic link
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_symlink(
    parent_ino: SigmaU64,
    name:       *const u8,
    target:     *const u8,
) -> SigmaI64 {
    let ino = sigma_tmpfs_create(parent_ino, name, TmpfsInodeType::SymLink, 0o777);
    if ino < 0 { return ino; }

    let t = &mut TMPFS_INSTANCE;
    let slot = tmpfs_find_by_ino(ino as SigmaU64);
    if slot >= 0 {
        tmpfs_strncpy(t.inodes[slot as usize].link_target.as_mut_ptr(), target, TMPFS_NAME_LEN);
    }
    ino
}

/// Stat an inode (get metadata)
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_stat(ino: SigmaU64, out: *mut TmpfsInode) -> SigmaBool {
    let t = &TMPFS_INSTANCE;
    let slot = tmpfs_find_by_ino(ino);
    if slot < 0 { return false; }
    if !t.inodes[slot as usize].active { return false; }
    *out = t.inodes[slot as usize].clone();
    true
}

/// Rename a file/directory
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_rename(
    ino:      SigmaU64,
    new_name: *const u8,
) -> SigmaI32 {
    let t = &mut TMPFS_INSTANCE;
    let slot = tmpfs_find_by_ino(ino);
    if slot < 0 { return -1; }
    tmpfs_strncpy(t.inodes[slot as usize].name.as_mut_ptr(), new_name, TMPFS_NAME_LEN);
    0
}

/// Get filesystem stats
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_statfs(
    total_bytes: *mut SigmaU64,
    used_bytes:  *mut SigmaU64,
    inode_count: *mut SigmaU32,
    inode_limit: *mut SigmaU32,
) {
    let t = &TMPFS_INSTANCE;
    *total_bytes = if t.opts.size_limit_kb > 0 { t.opts.size_limit_kb * 1024 } else { SigmaU64::MAX };
    *used_bytes  = t.total_bytes;
    *inode_count = t.inode_count;
    *inode_limit = if t.opts.inode_limit > 0 { t.opts.inode_limit } else { TMPFS_MAX_INODES as SigmaU32 };
}
