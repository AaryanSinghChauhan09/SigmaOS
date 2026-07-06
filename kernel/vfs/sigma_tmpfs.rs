// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/vfs/sigma_tmpfs.rs — RAM-backed Temporary Filesystem
// Implements a simple in-memory filesystem for early boot and /tmp.
//
// Design:
//  - Fixed-size inode array (TMPFS_MAX_INODES)
//  - Each file: contiguous data buffer (TMPFS_MAX_FILE_SIZE)
//  - Directories: entry list (TMPFS_MAX_DIR_ENTRIES)
//  - No external allocator required; uses static arrays (no_std safe)

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ── Configuration ──────────────────────────────────────────────────────────
pub const TMPFS_MAX_INODES:      usize = 512;
pub const TMPFS_MAX_FILE_SIZE:   usize = 1024 * 1024; // 1 MB per file
pub const TMPFS_MAX_DIR_ENTRIES: usize = 64;
pub const TMPFS_NAME_LEN:        usize = 255;
pub const TMPFS_DATA_POOL_SIZE:  usize = 32 * 1024 * 1024; // 32 MB total

// ── Node types ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TmpNodeType { Free = 0, File = 1, Dir = 2, Symlink = 3 }

// ── Directory entry ────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct TmpDirEntry {
    pub valid: bool,
    pub inode: u64,
    pub name:  [u8; TMPFS_NAME_LEN + 1],
}

impl TmpDirEntry {
    const fn empty() -> Self {
        Self { valid: false, inode: 0, name: [0u8; TMPFS_NAME_LEN + 1] }
    }
}

// ── Inode ──────────────────────────────────────────────────────────────────
pub struct TmpInode {
    pub valid:     bool,
    pub inode_num: u64,
    pub node_type: TmpNodeType,
    pub size:      u64,
    pub mode:      u32,
    pub uid:       u32,
    pub gid:       u32,
    pub mtime:     i64,
    pub ctime:     i64,
    pub nlink:     u32,
    // File data: offset into DATA_POOL
    pub data_off:  usize,
    pub data_cap:  usize,
    // Directory entries
    pub entries:   [TmpDirEntry; TMPFS_MAX_DIR_ENTRIES],
    pub entry_count: usize,
}

impl TmpInode {
    const fn new() -> Self {
        Self {
            valid: false, inode_num: 0,
            node_type: TmpNodeType::Free,
            size: 0, mode: 0o644, uid: 0, gid: 0,
            mtime: 0, ctime: 0, nlink: 1,
            data_off: 0, data_cap: 0,
            entries: [TmpDirEntry::empty(); TMPFS_MAX_DIR_ENTRIES],
            entry_count: 0,
        }
    }
}

// ── Global state (static, no_std) ─────────────────────────────────────────
static mut INODES: [TmpInode; TMPFS_MAX_INODES] = {
    // Const-init trick
    const EMPTY: TmpInode = TmpInode::new();
    [EMPTY; TMPFS_MAX_INODES]
};

static mut DATA_POOL: [u8; TMPFS_DATA_POOL_SIZE] = [0u8; TMPFS_DATA_POOL_SIZE];
static DATA_CURSOR: AtomicUsize = AtomicUsize::new(0);
static INODE_COUNTER: AtomicU64 = AtomicU64::new(1);
static mut INODE_COUNT: usize = 0;

fn now_ts() -> i64 { crate::kernel::core::sigma_irq::jiffies() as i64 }

// ── Root directory init ────────────────────────────────────────────────────
static TMPFS_INITIALIZED: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(false);

pub fn tmpfs_init() {
    if TMPFS_INITIALIZED.swap(true, Ordering::SeqCst) { return; }
    unsafe {
        // Create root inode (inode 1)
        INODES[0] = TmpInode {
            valid: true, inode_num: 1,
            node_type: TmpNodeType::Dir,
            size: 0, mode: 0o755, uid: 0, gid: 0,
            mtime: 0, ctime: 0, nlink: 2,
            data_off: 0, data_cap: 0,
            entries: [TmpDirEntry::empty(); TMPFS_MAX_DIR_ENTRIES],
            entry_count: 0,
        };
        INODE_COUNT = 1;
    }
}

// ── Path helpers ───────────────────────────────────────────────────────────
fn path_to_bytes(path: *const u8) -> &'static [u8] {
    if path.is_null() { return &[]; }
    let mut len = 0usize;
    unsafe {
        while *path.add(len) != 0 && len < 4096 { len += 1; }
        core::slice::from_raw_parts(path, len)
    }
}

fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

/// Split "/foo/bar/baz" → (parent_path "/foo/bar", name "baz")
fn path_split(path: &[u8]) -> (&[u8], &[u8]) {
    let mut last_slash = 0usize;
    for (i, &c) in path.iter().enumerate() {
        if c == b'/' { last_slash = i; }
    }
    if last_slash == 0 {
        (&path[..0], &path[1..]) // root dir, component after /
    } else {
        (&path[..last_slash], &path[last_slash + 1..])
    }
}

// ── Inode lookup by path ───────────────────────────────────────────────────
pub fn tmpfs_lookup(path: *const u8) -> u64 {
    tmpfs_init();
    let path = path_to_bytes(path);
    if path.is_empty() || path == b"/" { return 1; } // root
    lookup_from_root(path)
}

fn lookup_from_root(path: &[u8]) -> u64 {
    // Strip leading /
    let path = if path.starts_with(b"/") { &path[1..] } else { path };
    let mut dir_inode = 1u64;
    for component in path.split(|&c| c == b'/') {
        if component.is_empty() { continue; }
        dir_inode = lookup_in_dir(dir_inode, component);
        if dir_inode == 0 { return 0; }
    }
    dir_inode
}

fn lookup_in_dir(dir_inode: u64, name: &[u8]) -> u64 {
    unsafe {
        for i in 0..INODE_COUNT {
            let node = &INODES[i];
            if !node.valid || node.inode_num != dir_inode { continue; }
            if node.node_type != TmpNodeType::Dir { return 0; }
            for j in 0..node.entry_count {
                let e = &node.entries[j];
                if !e.valid { continue; }
                let ename = cstr_slice(&e.name);
                if bytes_eq(ename, name) { return e.inode; }
            }
            return 0;
        }
    }
    0
}

unsafe fn cstr_slice(buf: &[u8]) -> &[u8] {
    let mut len = 0;
    while len < buf.len() && buf[len] != 0 { len += 1; }
    &buf[..len]
}

fn inode_find(inode: u64) -> Option<usize> {
    unsafe {
        for i in 0..INODE_COUNT {
            if INODES[i].valid && INODES[i].inode_num == inode {
                return Some(i);
            }
        }
    }
    None
}

// ── Allocate data region from pool ────────────────────────────────────────
fn alloc_data(size: usize) -> Option<usize> {
    let off = DATA_CURSOR.fetch_add(size, Ordering::SeqCst);
    if off + size > TMPFS_DATA_POOL_SIZE { return None; }
    Some(off)
}

// ── Create file ────────────────────────────────────────────────────────────
pub fn tmpfs_create(path: *const u8, mode: u32) -> i64 {
    tmpfs_init();
    let path_bytes = path_to_bytes(path);
    if path_bytes.is_empty() { return -22; }

    let (parent_path, name) = path_split(path_bytes);
    if name.is_empty() { return -22; }

    // Find parent directory
    let parent_inode = if parent_path.is_empty() { 1 } else {
        lookup_from_root(parent_path)
    };
    if parent_inode == 0 { return -2; } // ENOENT

    // Allocate new inode
    let new_inode_num = INODE_COUNTER.fetch_add(1, Ordering::SeqCst);
    unsafe {
        if INODE_COUNT >= TMPFS_MAX_INODES { return -28; } // ENOSPC
        let idx = INODE_COUNT;
        INODE_COUNT += 1;
        let off = alloc_data(TMPFS_MAX_FILE_SIZE).unwrap_or(0);
        INODES[idx] = TmpInode {
            valid: true, inode_num: new_inode_num,
            node_type: TmpNodeType::File,
            size: 0, mode, uid: 0, gid: 0,
            mtime: now_ts(), ctime: now_ts(), nlink: 1,
            data_off: off, data_cap: TMPFS_MAX_FILE_SIZE,
            entries: [TmpDirEntry::empty(); TMPFS_MAX_DIR_ENTRIES],
            entry_count: 0,
        };

        // Add entry in parent directory
        if let Some(parent_idx) = inode_find(parent_inode) {
            let parent = &mut INODES[parent_idx];
            if parent.entry_count < TMPFS_MAX_DIR_ENTRIES {
                let ei = parent.entry_count;
                parent.entries[ei].valid = true;
                parent.entries[ei].inode = new_inode_num;
                let nlen = name.len().min(TMPFS_NAME_LEN);
                parent.entries[ei].name[..nlen].copy_from_slice(&name[..nlen]);
                parent.entry_count += 1;
            }
        }
    }
    new_inode_num as i64
}

pub fn tmpfs_mkdir(path: *const u8, mode: u32) -> i64 {
    tmpfs_init();
    let path_bytes = path_to_bytes(path);
    if path_bytes.is_empty() { return -22; }
    let (parent_path, name) = path_split(path_bytes);
    let parent_inode = if parent_path.is_empty() { 1 } else {
        lookup_from_root(parent_path)
    };
    if parent_inode == 0 { return -2; }
    let new_inode_num = INODE_COUNTER.fetch_add(1, Ordering::SeqCst);
    unsafe {
        if INODE_COUNT >= TMPFS_MAX_INODES { return -28; }
        let idx = INODE_COUNT;
        INODE_COUNT += 1;
        INODES[idx] = TmpInode {
            valid: true, inode_num: new_inode_num,
            node_type: TmpNodeType::Dir,
            size: 0, mode: mode | 0o111, uid: 0, gid: 0,
            mtime: now_ts(), ctime: now_ts(), nlink: 2,
            data_off: 0, data_cap: 0,
            entries: [TmpDirEntry::empty(); TMPFS_MAX_DIR_ENTRIES],
            entry_count: 0,
        };
        if let Some(parent_idx) = inode_find(parent_inode) {
            let parent = &mut INODES[parent_idx];
            if parent.entry_count < TMPFS_MAX_DIR_ENTRIES {
                let ei = parent.entry_count;
                parent.entries[ei].valid = true;
                parent.entries[ei].inode = new_inode_num;
                let nlen = name.len().min(TMPFS_NAME_LEN);
                parent.entries[ei].name[..nlen].copy_from_slice(&name[..nlen]);
                parent.entry_count += 1;
            }
        }
    }
    0
}

// ── Read / Write ───────────────────────────────────────────────────────────
pub fn tmpfs_read(inode: u64, offset: u64, buf: *mut u8, len: usize) -> i64 {
    if buf.is_null() { return -14; }
    let idx = match inode_find(inode) { Some(i) => i, None => return -2 };
    unsafe {
        let node = &INODES[idx];
        if node.node_type != TmpNodeType::File { return -22; }
        let start = offset as usize;
        if start >= node.size as usize { return 0; } // EOF
        let avail = node.size as usize - start;
        let to_read = len.min(avail);
        let src = DATA_POOL.as_ptr().add(node.data_off + start);
        core::ptr::copy_nonoverlapping(src, buf, to_read);
        to_read as i64
    }
}

pub fn tmpfs_write(inode: u64, offset: u64, buf: *const u8, len: usize) -> i64 {
    if buf.is_null() { return -14; }
    let idx = match inode_find(inode) { Some(i) => i, None => return -2 };
    unsafe {
        let node = &mut INODES[idx];
        if node.node_type != TmpNodeType::File { return -22; }
        let start = offset as usize;
        let end   = start + len;
        if end > node.data_cap { return -28; } // ENOSPC
        let dst = DATA_POOL.as_mut_ptr().add(node.data_off + start);
        core::ptr::copy_nonoverlapping(buf, dst, len);
        if end as u64 > node.size { node.size = end as u64; }
        node.mtime = now_ts();
        len as i64
    }
}

// ── Stat ───────────────────────────────────────────────────────────────────
pub fn tmpfs_stat(inode: u64, out: *mut u8) -> i64 {
    if out.is_null() { return -14; }
    let idx = match inode_find(inode) { Some(i) => i, None => return -2 };
    unsafe {
        let node = &INODES[idx];
        let stat = &mut *(out as *mut crate::kernel::core::sigma_syscall_dispatch::FileStat);
        stat.st_ino   = node.inode_num;
        stat.st_mode  = node.mode;
        stat.st_nlink = node.nlink;
        stat.st_uid   = node.uid;
        stat.st_gid   = node.gid;
        stat.st_size  = node.size as i64;
        stat.st_mtime = node.mtime;
        stat.st_ctime = node.ctime;
        stat.st_atime = node.mtime;
    }
    0
}

// ── Remove / rename ────────────────────────────────────────────────────────
pub fn tmpfs_unlink(path: *const u8) -> i64 {
    let path_bytes = path_to_bytes(path);
    let (parent_path, name) = path_split(path_bytes);
    let parent_inode = if parent_path.is_empty() { 1 } else {
        lookup_from_root(parent_path)
    };
    if parent_inode == 0 { return -2; }
    unsafe {
        if let Some(parent_idx) = inode_find(parent_inode) {
            let parent = &mut INODES[parent_idx];
            for j in 0..parent.entry_count {
                let e = &mut parent.entries[j];
                if !e.valid { continue; }
                let ename = cstr_slice(&e.name);
                if bytes_eq(ename, name) {
                    e.valid = false;
                    return 0;
                }
            }
        }
    }
    -2 // ENOENT
}

pub fn tmpfs_rmdir(path: *const u8) -> i64 {
    tmpfs_unlink(path) // simplified: same as unlink for now
}

pub fn tmpfs_rename(old: *const u8, new: *const u8) -> i64 {
    // Read target name, update parent entry name
    let old_bytes = path_to_bytes(old);
    let new_bytes = path_to_bytes(new);
    let (old_parent, old_name) = path_split(old_bytes);
    let (_new_parent, new_name) = path_split(new_bytes);
    let parent_inode = if old_parent.is_empty() { 1 } else {
        lookup_from_root(old_parent)
    };
    if parent_inode == 0 { return -2; }
    unsafe {
        if let Some(parent_idx) = inode_find(parent_inode) {
            let parent = &mut INODES[parent_idx];
            for j in 0..parent.entry_count {
                let e = &mut parent.entries[j];
                if !e.valid { continue; }
                let ename = cstr_slice(&e.name);
                if bytes_eq(ename, old_name) {
                    e.name = [0u8; TMPFS_NAME_LEN + 1];
                    let nlen = new_name.len().min(TMPFS_NAME_LEN);
                    e.name[..nlen].copy_from_slice(&new_name[..nlen]);
                    return 0;
                }
            }
        }
    }
    -2
}
