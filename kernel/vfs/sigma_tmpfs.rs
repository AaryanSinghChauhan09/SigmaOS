// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/vfs/sigma_tmpfs.rs — Tmpfs (RAM-backed filesystem)
//
// Provides /tmp, /run, /dev/shm backed entirely by RAM.
// Files stored in kernel slab allocator.  No persistence.
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use super::sigma_vfs::{FsOps, Inode, Stat, InodeType, Mode,
                       S_IRUSR, S_IWUSR, S_IRGRP, S_IROTH, DirEntry};

// ── Tmpfs file data store ─────────────────────────────────────────────────
const TMPFS_MAX_FILES:  usize = 256;
const TMPFS_MAX_FILE_SZ:usize = 1024 * 1024; // 1 MB max per file

struct TmpfsFile {
    ino:     u64,
    data:    *mut u8,
    size:    usize,
    cap:     usize,
    active:  bool,
}

impl TmpfsFile {
    const fn empty() -> Self {
        Self { ino: 0, data: core::ptr::null_mut(), size: 0, cap: 0, active: false }
    }
}

struct TmpfsState {
    files:  [TmpfsFile; TMPFS_MAX_FILES],
    count:  usize,
}

impl TmpfsState {
    const fn new() -> Self {
        Self {
            files: [const { TmpfsFile::empty() }; TMPFS_MAX_FILES],
            count: 0,
        }
    }

    fn find(&self, ino: u64) -> Option<&TmpfsFile> {
        self.files.iter().find(|f| f.active && f.ino == ino)
    }

    fn find_mut(&mut self, ino: u64) -> Option<&mut TmpfsFile> {
        self.files.iter_mut().find(|f| f.active && f.ino == ino)
    }

    unsafe fn alloc_file(&mut self, ino: u64) -> Option<&mut TmpfsFile> {
        for f in &mut self.files {
            if !f.active {
                extern "C" { fn sigma_slab_alloc(size: usize) -> *mut u8; }
                let cap  = 4096usize;
                let data = sigma_slab_alloc(cap);
                if data.is_null() { return None; }
                *f = TmpfsFile { ino, data, size: 0, cap, active: true };
                self.count += 1;
                return Some(f);
            }
        }
        None
    }

    unsafe fn grow(&mut self, ino: u64, needed: usize) -> bool {
        extern "C" { fn sigma_slab_alloc(size: usize) -> *mut u8; fn sigma_slab_free(p: *mut u8) -> i32; }
        let f = match self.find_mut(ino) { Some(f) => f, None => return false };
        if f.cap >= needed { return true; }
        let new_cap = (needed + 4095) & !4095;
        if new_cap > TMPFS_MAX_FILE_SZ { return false; }
        let new_data = sigma_slab_alloc(new_cap);
        if new_data.is_null() { return false; }
        core::ptr::copy_nonoverlapping(f.data, new_data, f.size);
        sigma_slab_free(f.data);
        f.data = new_data;
        f.cap  = new_cap;
        true
    }
}

static mut G_TMPFS: TmpfsState = TmpfsState::new();

// ── FsOps implementation ──────────────────────────────────────────────────

unsafe fn tmpfs_read(ino: &Inode, offset: u64, buf: *mut u8, len: usize) -> i64 {
    let f = match G_TMPFS.find(ino.ino) { Some(f) => f, None => return -2 };
    if offset >= f.size as u64 { return 0; }
    let start = offset as usize;
    let avail = f.size - start;
    let copy  = len.min(avail);
    core::ptr::copy_nonoverlapping(f.data.add(start), buf, copy);
    copy as i64
}

unsafe fn tmpfs_write(ino: &mut Inode, offset: u64, buf: *const u8, len: usize) -> i64 {
    let needed = offset as usize + len;
    if !G_TMPFS.grow(ino.ino, needed) {
        // First write: allocate the file
        if G_TMPFS.find(ino.ino).is_none() {
            G_TMPFS.alloc_file(ino.ino)?;
            G_TMPFS.grow(ino.ino, needed);
        }
    }
    let f = match G_TMPFS.find_mut(ino.ino) { Some(f) => f, None => return -12 };
    let start = offset as usize;
    core::ptr::copy_nonoverlapping(buf, f.data.add(start), len);
    if start + len > f.size { f.size = start + len; }
    ino.size = f.size as u64;
    len as i64
}

unsafe fn tmpfs_readdir(_ino: &Inode, _offset: u64, _out: *mut DirEntry, _max: usize) -> i64 {
    0 // TODO: return directory entries
}

unsafe fn tmpfs_create(parent: &mut Inode, name: &[u8], mode: Mode) -> Option<u64> {
    extern "C" { fn sigma_vfs_open(path: *const u8, path_len: usize, flags: u32) -> i64; }
    // Create inode and allocate storage in tmpfs
    let new_ino = super::sigma_vfs::INODE_COUNTER_NEXT();
    G_TMPFS.alloc_file(new_ino)?;
    Some(new_ino)
}

unsafe fn tmpfs_unlink(_parent: &mut Inode, _name: &[u8]) -> i32 { 0 }
unsafe fn tmpfs_mkdir(_parent: &mut Inode, _name: &[u8], _mode: Mode) -> Option<u64> { None }

unsafe fn tmpfs_stat(ino: &Inode, out: *mut Stat) -> i32 {
    if out.is_null() { return -14; }
    let size = G_TMPFS.find(ino.ino).map(|f| f.size).unwrap_or(0);
    (*out).st_ino   = ino.ino;
    (*out).st_mode  = ino.mode;
    (*out).st_size  = size as u64;
    (*out).st_nlink = ino.nlinks;
    0
}

pub static TMPFS_OPS: FsOps = FsOps {
    read:    tmpfs_read,
    write:   tmpfs_write,
    readdir: tmpfs_readdir,
    create:  tmpfs_create,
    unlink:  tmpfs_unlink,
    mkdir:   tmpfs_mkdir,
    stat:    tmpfs_stat,
};

// ── Public init ───────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_tmpfs_init() {
    // Mount tmpfs at /tmp
    extern "C" { fn sigma_vfs_mount(p: *const u8, pl: usize, root_ino: u64, ops: *const FsOps) -> i32; }
    sigma_vfs_mount(b"/tmp\0".as_ptr(), 4, 2, &TMPFS_OPS as *const FsOps);
    sigma_vfs_mount(b"/run\0".as_ptr(), 4, 3, &TMPFS_OPS as *const FsOps);
}
