// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/vfs/sigma_vfs.rs — Virtual Filesystem Switch
// Implements VFS layer: open/read/write/close/stat/mkdir/rmdir/
// readdir/rename/chdir/getcwd/ioctl/lseek.
//
// Architecture:
//  - FD table per-process (MAX_FD open files)
//  - VfsNode: inode-like abstraction with vtable of fn pointers
//  - Filesystem drivers register via vfs_register_fs()
//  - Supports: tmpfs, sigmafs, ext4 (ro), fat32

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};

// ── Limits ─────────────────────────────────────────────────────────────────
pub const MAX_FD:         usize = 256;
pub const MAX_OPEN_FILES: usize = 4096;
pub const MAX_FS_TYPES:   usize = 16;
pub const PATH_MAX:       usize = 4096;
pub const NAME_MAX:       usize = 255;

// ── Flags ──────────────────────────────────────────────────────────────────
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR:   u32 = 2;
pub const O_CREAT:  u32 = 0x40;
pub const O_TRUNC:  u32 = 0x200;
pub const O_APPEND: u32 = 0x400;
pub const O_NONBLOCK: u32 = 0x800;

pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;

// ── Node types ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NodeType {
    Unknown   = 0,
    Regular   = 1,
    Directory = 2,
    Symlink   = 3,
    Char      = 4,
    Block     = 5,
    Fifo      = 6,
    Socket    = 7,
}

// ── VFS node (inode-equivalent) ────────────────────────────────────────────
#[repr(C)]
pub struct VfsNode {
    pub inode:    u64,
    pub size:     u64,
    pub mode:     u32,
    pub uid:      u32,
    pub gid:      u32,
    pub atime:    i64,
    pub mtime:    i64,
    pub ctime:    i64,
    pub nlink:    u32,
    pub node_type: NodeType,
    pub fs_id:    u8,   // which filesystem owns this node
    pub ops:      &'static VfsOps,
    pub private:  u64,  // fs-private pointer (cast to fs-specific struct)
}

/// Filesystem operations vtable
#[repr(C)]
pub struct VfsOps {
    pub read:    fn(node: &VfsNode, offset: u64, buf: *mut u8, len: usize) -> i64,
    pub write:   fn(node: &VfsNode, offset: u64, buf: *const u8, len: usize) -> i64,
    pub readdir: fn(node: &VfsNode, offset: u64, out: *mut DirEntry, max: usize) -> i64,
    pub lookup:  fn(parent: &VfsNode, name: &[u8]) -> Option<u64>, // → inode
    pub create:  fn(parent: &VfsNode, name: &[u8], mode: u32) -> i64,
    pub mkdir:   fn(parent: &VfsNode, name: &[u8], mode: u32) -> i64,
    pub unlink:  fn(parent: &VfsNode, name: &[u8]) -> i64,
    pub rmdir:   fn(parent: &VfsNode, name: &[u8]) -> i64,
    pub rename:  fn(old_parent: &VfsNode, old: &[u8], new_parent: &VfsNode, new: &[u8]) -> i64,
    pub truncate: fn(node: &VfsNode, size: u64) -> i64,
    pub ioctl:   fn(node: &VfsNode, req: u64, arg: u64) -> i64,
    pub sync:    fn(node: &VfsNode) -> i64,
}

// ── Directory entry ────────────────────────────────────────────────────────
#[repr(C)]
pub struct DirEntry {
    pub inode:  u64,
    pub offset: u64,
    pub reclen: u16,
    pub ftype:  NodeType,
    pub name:   [u8; NAME_MAX + 1],
}

// ── Open file descriptor ───────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct FileDesc {
    pub valid:  bool,
    pub inode:  u64,
    pub offset: u64,
    pub flags:  u32,
    pub fs_id:  u8,
}

// ── Per-process FD table (global for single address space in early boot) ───
static mut FD_TABLE: [FileDesc; MAX_FD] = [FileDesc {
    valid: false, inode: 0, offset: 0, flags: 0, fs_id: 0,
}; MAX_FD];
static NEXT_FD: AtomicUsize = AtomicUsize::new(3); // 0,1,2 = stdin/stdout/stderr

// ── Filesystem registry ────────────────────────────────────────────────────
pub struct FsType {
    pub name:    &'static str,
    pub fs_id:   u8,
    pub mount:   fn(device: u64, flags: u32) -> i64,
    pub unmount: fn(fs_id: u8) -> i64,
}

static mut FS_REGISTRY: [Option<FsType>; MAX_FS_TYPES] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
];
static FS_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn vfs_register_fs(fs: FsType) -> i64 {
    let idx = FS_COUNT.fetch_add(1, Ordering::SeqCst);
    if idx >= MAX_FS_TYPES { return -1; }
    unsafe { FS_REGISTRY[idx] = Some(fs); }
    0
}

// ── Inode cache (simple array; production would use hash table) ────────────
static mut INODE_CACHE: [Option<VfsNode>; 1024] = [None; 1024];
static INODE_CACHE_LEN: AtomicUsize = AtomicUsize::new(0);

fn inode_get(inode: u64) -> Option<&'static VfsNode> {
    let len = INODE_CACHE_LEN.load(Ordering::Relaxed);
    for i in 0..len {
        unsafe {
            if let Some(ref node) = INODE_CACHE[i] {
                if node.inode == inode { return Some(node); }
            }
        }
    }
    None
}

fn inode_insert(node: VfsNode) {
    let idx = INODE_CACHE_LEN.fetch_add(1, Ordering::SeqCst);
    if idx < 1024 {
        unsafe { INODE_CACHE[idx] = Some(node); }
    }
}

// ── VFS interface ──────────────────────────────────────────────────────────

/// Allocate a new file descriptor slot.
fn alloc_fd() -> Option<i32> {
    for fd in 3..MAX_FD {
        let valid = unsafe { FD_TABLE[fd].valid };
        if !valid {
            return Some(fd as i32);
        }
    }
    None
}

pub fn vfs_open(path: *const u8, flags: u32, _mode: u32) -> i64 {
    if path.is_null() { return -14; } // EFAULT
    let fd = match alloc_fd() {
        Some(fd) => fd,
        None => return -12, // ENOMEM (too many open files)
    };
    // Resolve path through mounted filesystems
    let inode = vfs_path_resolve(path);
    if inode == 0 {
        if flags & O_CREAT == 0 { return -2; } // ENOENT
        // Create in tmpfs (default fs)
        let new_inode = tmpfs_create(path, _mode);
        if new_inode < 0 { return new_inode; }
        unsafe {
            FD_TABLE[fd as usize] = FileDesc {
                valid: true, inode: new_inode as u64,
                offset: 0, flags, fs_id: 0,
            };
        }
        return fd as i64;
    }
    unsafe {
        FD_TABLE[fd as usize] = FileDesc {
            valid: true, inode, offset: 0, flags, fs_id: 0,
        };
    }
    fd as i64
}

pub fn vfs_close(fd: i32) -> i64 {
    if fd < 0 || fd as usize >= MAX_FD { return -9; } // EBADF
    unsafe {
        if !FD_TABLE[fd as usize].valid { return -9; }
        // Sync before close
        if let Some(node) = inode_get(FD_TABLE[fd as usize].inode) {
            (node.ops.sync)(node);
        }
        FD_TABLE[fd as usize].valid = false;
    }
    0
}

pub fn vfs_read(fd: i32, buf: *mut u8, count: usize) -> i64 {
    if fd < 0 || fd as usize >= MAX_FD { return -9; }
    let (inode, offset, flags) = unsafe {
        let f = FD_TABLE[fd as usize];
        if !f.valid { return -9; }
        if flags_write_only(f.flags) { return -13; } // EACCES
        (f.inode, f.offset, f.flags)
    };
    let node = match inode_get(inode) {
        Some(n) => n,
        None => return tmpfs_read(inode, offset, buf, count),
    };
    let n = (node.ops.read)(node, offset, buf, count);
    if n > 0 {
        unsafe { FD_TABLE[fd as usize].offset += n as u64; }
    }
    n
}

pub fn vfs_write(fd: i32, buf: *const u8, count: usize) -> i64 {
    if fd < 0 || fd as usize >= MAX_FD { return -9; }
    let (inode, offset, flags) = unsafe {
        let f = FD_TABLE[fd as usize];
        if !f.valid { return -9; }
        if flags & O_RDONLY == O_RDONLY && flags & O_RDWR == 0 { return -13; }
        (f.inode, f.offset, f.flags)
    };
    // fd=1 (stdout) → serial console write
    if fd == 1 || fd == 2 {
        return console_write(buf, count);
    }
    let node = match inode_get(inode) {
        Some(n) => n,
        None => return tmpfs_write(inode, offset, buf, count),
    };
    let n = (node.ops.write)(node, offset, buf, count);
    if n > 0 {
        unsafe {
            let off = if flags & O_APPEND != 0 { u64::MAX } else { offset };
            FD_TABLE[fd as usize].offset = off + n as u64;
        }
    }
    n
}

pub fn vfs_lseek(fd: i32, offset: i64, whence: i32) -> i64 {
    if fd < 0 || fd as usize >= MAX_FD { return -9; }
    unsafe {
        let f = &mut FD_TABLE[fd as usize];
        if !f.valid { return -9; }
        let new_off: i64 = match whence {
            SEEK_SET => offset,
            SEEK_CUR => f.offset as i64 + offset,
            SEEK_END => {
                if let Some(node) = inode_get(f.inode) {
                    node.size as i64 + offset
                } else { f.offset as i64 + offset }
            }
            _ => return -22, // EINVAL
        };
        if new_off < 0 { return -22; }
        f.offset = new_off as u64;
        new_off
    }
}

pub fn vfs_stat(path: *const u8, out: *mut u8) -> i64 {
    if path.is_null() || out.is_null() { return -14; }
    let inode = vfs_path_resolve(path);
    if inode == 0 { return -2; }
    if let Some(node) = inode_get(inode) {
        fill_stat(node, out);
        0
    } else {
        tmpfs_stat(inode, out)
    }
}

pub fn vfs_fstat(fd: i32, out: *mut u8) -> i64 {
    if fd < 0 || fd as usize >= MAX_FD { return -9; }
    let inode = unsafe {
        let f = FD_TABLE[fd as usize];
        if !f.valid { return -9; }
        f.inode
    };
    if let Some(node) = inode_get(inode) {
        fill_stat(node, out);
        0
    } else {
        tmpfs_stat(inode, out)
    }
}

pub fn vfs_ioctl(fd: i32, request: u64, arg: u64) -> i64 {
    if fd < 0 || fd as usize >= MAX_FD { return -9; }
    let inode = unsafe {
        let f = FD_TABLE[fd as usize];
        if !f.valid { return -9; }
        f.inode
    };
    if let Some(node) = inode_get(inode) {
        return (node.ops.ioctl)(node, request, arg);
    }
    -25 // ENOTTY
}

pub fn vfs_mkdir(path: *const u8, mode: u32) -> i64 {
    if path.is_null() { return -14; }
    tmpfs_mkdir(path, mode)
}

pub fn vfs_rmdir(path: *const u8) -> i64 {
    if path.is_null() { return -14; }
    tmpfs_rmdir(path)
}

pub fn vfs_unlink(path: *const u8) -> i64 {
    if path.is_null() { return -14; }
    tmpfs_unlink(path)
}

pub fn vfs_rename(old: *const u8, new: *const u8) -> i64 {
    if old.is_null() || new.is_null() { return -14; }
    tmpfs_rename(old, new)
}

pub fn vfs_chdir(path: *const u8) -> i64 {
    if path.is_null() { return -14; }
    // Verify directory exists
    let inode = vfs_path_resolve(path);
    if inode == 0 { return -2; }
    // Update CWD for current process
    crate::kernel::proc::proc_set_cwd(path);
    0
}

pub fn vfs_getcwd(buf: *mut u8, size: usize) -> i64 {
    if buf.is_null() || size == 0 { return -14; }
    crate::kernel::proc::proc_get_cwd(buf, size)
}

// ── Path resolution ────────────────────────────────────────────────────────
fn vfs_path_resolve(_path: *const u8) -> u64 {
    // Walk mount table, then delegate to filesystem lookup
    // Returns inode number or 0 on not-found
    // Simplified: always try tmpfs first
    tmpfs_lookup(_path)
}

// ── Fill stat structure ────────────────────────────────────────────────────
fn fill_stat(node: &VfsNode, out: *mut u8) {
    // Cast to FileStat and fill — safe because caller guarantees size
    unsafe {
        let stat = &mut *(out as *mut super::sigma_syscall_dispatch::FileStat);
        stat.st_ino   = node.inode;
        stat.st_mode  = node.mode;
        stat.st_uid   = node.uid;
        stat.st_gid   = node.gid;
        stat.st_size  = node.size as i64;
        stat.st_nlink = node.nlink;
        stat.st_atime = node.atime;
        stat.st_mtime = node.mtime;
        stat.st_ctime = node.ctime;
    }
}

// ── Console write (fd 1/2) ─────────────────────────────────────────────────
fn console_write(buf: *const u8, count: usize) -> i64 {
    for i in 0..count {
        let c = unsafe { *buf.add(i) };
        // Write to serial port 0x3F8 (COM1)
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3F8u16,
                in("al") c
            );
        }
    }
    count as i64
}

// ── Flags helpers ──────────────────────────────────────────────────────────
fn flags_write_only(flags: u32) -> bool {
    flags & 0x3 == O_WRONLY
}

// ── Tmpfs stubs (forward to sigma_tmpfs module) ────────────────────────────
fn tmpfs_create(path: *const u8, mode: u32) -> i64 {
    crate::kernel::vfs::tmpfs::tmpfs_create(path, mode)
}
fn tmpfs_read(inode: u64, offset: u64, buf: *mut u8, len: usize) -> i64 {
    crate::kernel::vfs::tmpfs::tmpfs_read(inode, offset, buf, len)
}
fn tmpfs_write(inode: u64, offset: u64, buf: *const u8, len: usize) -> i64 {
    crate::kernel::vfs::tmpfs::tmpfs_write(inode, offset, buf, len)
}
fn tmpfs_stat(inode: u64, out: *mut u8) -> i64 {
    crate::kernel::vfs::tmpfs::tmpfs_stat(inode, out)
}
fn tmpfs_mkdir(path: *const u8, mode: u32) -> i64 {
    crate::kernel::vfs::tmpfs::tmpfs_mkdir(path, mode)
}
fn tmpfs_rmdir(path: *const u8) -> i64 {
    crate::kernel::vfs::tmpfs::tmpfs_rmdir(path)
}
fn tmpfs_unlink(path: *const u8) -> i64 {
    crate::kernel::vfs::tmpfs::tmpfs_unlink(path)
}
fn tmpfs_rename(old: *const u8, new: *const u8) -> i64 {
    crate::kernel::vfs::tmpfs::tmpfs_rename(old, new)
}
fn tmpfs_lookup(path: *const u8) -> u64 {
    crate::kernel::vfs::tmpfs::tmpfs_lookup(path)
}
