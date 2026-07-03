// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/vfs/sigma_vfs.rs — Virtual File System layer
//
// Implements the generic VFS: inode, dentry, file ops, mount table.
// Modelled on Linux VFS but significantly simplified for SigmaOS.
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Inode types ───────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum InodeType {
    Regular   = 1,
    Directory = 2,
    Symlink   = 3,
    CharDev   = 4,
    BlockDev  = 5,
    Fifo      = 6,
    Socket    = 7,
}

// ── File permission bits (rwxrwxrwx) ─────────────────────────────────────
pub type Mode = u16;
pub const S_IRUSR: Mode = 0o400;
pub const S_IWUSR: Mode = 0o200;
pub const S_IXUSR: Mode = 0o100;
pub const S_IRGRP: Mode = 0o040;
pub const S_IROTH: Mode = 0o004;
pub const S_IRWXU: Mode = 0o700;
pub const S_IRWXG: Mode = 0o070;
pub const S_IRWXO: Mode = 0o007;

// ── Inode ─────────────────────────────────────────────────────────────────
pub const MAX_INODES: usize = 4096;
pub const MAX_NAME:   usize = 256;
pub const MAX_DENTRIES: usize = 1024;
pub const MAX_FDS:    usize = 256;
pub const MAX_MOUNTS: usize = 16;

static INODE_COUNTER: AtomicU64 = AtomicU64::new(1);

fn next_ino() -> u64 { INODE_COUNTER.fetch_add(1, Ordering::Relaxed) }

#[repr(C)]
#[derive(Clone)]
pub struct Inode {
    pub ino:      u64,
    pub kind:     InodeType,
    pub mode:     Mode,
    pub uid:      u32,
    pub gid:      u32,
    pub size:     u64,
    pub nlinks:   u32,
    pub atime:    u64,
    pub mtime:    u64,
    pub ctime:    u64,
    pub dev_major:u32,
    pub dev_minor:u32,
    pub fs_data:  u64,   // filesystem-specific payload pointer
    pub active:   bool,
}

impl Inode {
    pub fn new_dir(mode: Mode) -> Self {
        let now = unsafe { sigma_clock_ns_vfs() };
        Self {
            ino: next_ino(), kind: InodeType::Directory, mode,
            uid: 0, gid: 0, size: 0, nlinks: 2,
            atime: now, mtime: now, ctime: now,
            dev_major: 0, dev_minor: 0, fs_data: 0, active: true,
        }
    }

    pub fn new_file(mode: Mode, size: u64) -> Self {
        let now = unsafe { sigma_clock_ns_vfs() };
        Self {
            ino: next_ino(), kind: InodeType::Regular, mode,
            uid: 0, gid: 0, size, nlinks: 1,
            atime: now, mtime: now, ctime: now,
            dev_major: 0, dev_minor: 0, fs_data: 0, active: true,
        }
    }
}

unsafe fn sigma_clock_ns_vfs() -> u64 {
    extern "C" { fn sigma_clock_ns() -> u64; }
    sigma_clock_ns()
}

// ── Directory entry ────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct Dentry {
    pub name:     [u8; MAX_NAME],
    pub name_len: usize,
    pub ino:      u64,
    pub parent:   u64,   // parent inode number (0 = root)
    pub active:   bool,
}

impl Dentry {
    pub fn new(name: &[u8], ino: u64, parent: u64) -> Self {
        let mut d = Self {
            name: [0u8; MAX_NAME],
            name_len: name.len().min(MAX_NAME - 1),
            ino, parent, active: true,
        };
        d.name[..d.name_len].copy_from_slice(&name[..d.name_len]);
        d
    }

    pub fn name_bytes(&self) -> &[u8] {
        &self.name[..self.name_len]
    }
}

// ── Open file descriptor ───────────────────────────────────────────────────
#[derive(Clone)]
pub struct FileDesc {
    pub ino:    u64,
    pub offset: u64,
    pub flags:  u32,
    pub active: bool,
}

pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR:   u32 = 2;
pub const O_CREAT:  u32 = 0o100;
pub const O_TRUNC:  u32 = 0o1000;
pub const O_APPEND: u32 = 0o2000;
pub const O_DIRECTORY: u32 = 0o200000;

// ── Filesystem ops vtable ─────────────────────────────────────────────────
pub struct FsOps {
    pub read:   unsafe fn(ino: &Inode, offset: u64, buf: *mut u8, len: usize) -> i64,
    pub write:  unsafe fn(ino: &mut Inode, offset: u64, buf: *const u8, len: usize) -> i64,
    pub readdir:unsafe fn(ino: &Inode, offset: u64, out: *mut DirEntry, max: usize) -> i64,
    pub create: unsafe fn(parent: &mut Inode, name: &[u8], mode: Mode) -> Option<u64>,
    pub unlink: unsafe fn(parent: &mut Inode, name: &[u8]) -> i32,
    pub mkdir:  unsafe fn(parent: &mut Inode, name: &[u8], mode: Mode) -> Option<u64>,
    pub stat:   unsafe fn(ino: &Inode, out: *mut Stat) -> i32,
}

#[repr(C)]
pub struct DirEntry {
    pub ino:       u64,
    pub off:       u64,
    pub reclen:    u16,
    pub file_type: u8,
    pub name:      [u8; MAX_NAME],
}

#[repr(C)]
pub struct Stat {
    pub st_dev:     u64,
    pub st_ino:     u64,
    pub st_mode:    Mode,
    pub st_nlink:   u32,
    pub st_uid:     u32,
    pub st_gid:     u32,
    pub st_rdev:    u64,
    pub st_size:    u64,
    pub st_blksize: u64,
    pub st_blocks:  u64,
    pub st_atime:   u64,
    pub st_mtime:   u64,
    pub st_ctime:   u64,
}

// ── Mount point ────────────────────────────────────────────────────────────
pub struct MountPoint {
    pub mount_path: [u8; MAX_NAME],
    pub path_len:   usize,
    pub root_ino:   u64,
    pub fs_ops:     *const FsOps,
    pub active:     bool,
}

// ── VFS core ──────────────────────────────────────────────────────────────
pub struct Vfs {
    pub inodes:   [Option<Inode>; MAX_INODES],
    pub dentries: [Option<Dentry>; MAX_DENTRIES],
    pub fds:      [Option<FileDesc>; MAX_FDS],
    pub mounts:   [Option<MountPoint>; MAX_MOUNTS],
    pub root_ino: u64,
    pub initialized: bool,
}

impl Vfs {
    pub const fn empty() -> Self {
        Self {
            inodes:  [const { None }; MAX_INODES],
            dentries:[const { None }; MAX_DENTRIES],
            fds:     [const { None }; MAX_FDS],
            mounts:  [const { None }; MAX_MOUNTS],
            root_ino: 0,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        // Create root inode
        let root = Inode::new_dir(S_IRWXU | S_IRGRP | S_IROTH);
        let root_ino = root.ino;
        self.insert_inode(root);

        // Create root dentry "/"
        let d = Dentry::new(b"/", root_ino, 0);
        self.insert_dentry(d);

        self.root_ino = root_ino;
        self.initialized = true;
    }

    fn insert_inode(&mut self, inode: Inode) -> usize {
        for i in 0..MAX_INODES {
            if self.inodes[i].is_none() {
                self.inodes[i] = Some(inode);
                return i;
            }
        }
        0
    }

    fn insert_dentry(&mut self, d: Dentry) -> usize {
        for i in 0..MAX_DENTRIES {
            if self.dentries[i].is_none() {
                self.dentries[i] = Some(d);
                return i;
            }
        }
        0
    }

    fn find_inode(&self, ino: u64) -> Option<&Inode> {
        for i in &self.inodes {
            if let Some(ref n) = i {
                if n.ino == ino && n.active { return Some(n); }
            }
        }
        None
    }

    fn find_inode_mut(&mut self, ino: u64) -> Option<&mut Inode> {
        for i in &mut self.inodes {
            if let Some(ref mut n) = i {
                if n.ino == ino && n.active { return Some(n); }
            }
        }
        None
    }

    fn lookup_dentry(&self, parent_ino: u64, name: &[u8]) -> Option<u64> {
        for d in &self.dentries {
            if let Some(ref de) = d {
                if de.active && de.parent == parent_ino && de.name_bytes() == name {
                    return Some(de.ino);
                }
            }
        }
        None
    }

    /// Resolve an absolute path to an inode number.
    pub fn path_lookup(&self, path: &[u8]) -> Option<u64> {
        if path.is_empty() || path[0] != b'/' {
            return None;
        }
        let mut cur_ino = self.root_ino;
        let mut components = path[1..].split(|&b| b == b'/');
        for comp in components {
            if comp.is_empty() { continue; } // double slash
            cur_ino = self.lookup_dentry(cur_ino, comp)?;
        }
        Some(cur_ino)
    }

    /// open() — allocate an fd for the given path
    pub fn open(&mut self, path: &[u8], flags: u32) -> i64 {
        // Check for mount point match first
        let ino = match self.path_lookup(path) {
            Some(i) => i,
            None => {
                if flags & O_CREAT == 0 { return -2; } // ENOENT
                // Create the file
                let (parent_path, fname) = split_path(path);
                let parent_ino = self.path_lookup(parent_path)?;
                let new_inode = Inode::new_file(S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH, 0);
                let new_ino = new_inode.ino;
                self.insert_inode(new_inode);
                let d = Dentry::new(fname, new_ino, parent_ino);
                self.insert_dentry(d);
                new_ino
            }
        };

        // Allocate fd
        for fd in 3..MAX_FDS {  // 0,1,2 = stdin,stdout,stderr
            if self.fds[fd].is_none() {
                self.fds[fd] = Some(FileDesc {
                    ino, offset: 0, flags, active: true,
                });
                return fd as i64;
            }
        }
        -24 // EMFILE
    }

    fn open_inner(&self, path: &[u8], flags: u32) -> Option<u64> { None }

    pub fn close(&mut self, fd: i32) -> i32 {
        if fd < 0 || fd as usize >= MAX_FDS { return -9; } // EBADF
        if self.fds[fd as usize].take().is_some() { 0 } else { -9 }
    }

    pub fn stat(&self, path: &[u8], out: *mut Stat) -> i32 {
        if out.is_null() { return -14; } // EFAULT
        let ino = match self.path_lookup(path) { Some(i) => i, None => return -2 };
        let inode = match self.find_inode(ino) { Some(i) => i, None => return -2 };
        unsafe {
            (*out).st_ino   = inode.ino;
            (*out).st_mode  = inode.mode;
            (*out).st_nlink = inode.nlinks;
            (*out).st_uid   = inode.uid;
            (*out).st_gid   = inode.gid;
            (*out).st_size  = inode.size;
            (*out).st_atime = inode.atime;
            (*out).st_mtime = inode.mtime;
            (*out).st_ctime = inode.ctime;
        }
        0
    }

    /// mkdir — create a directory
    pub fn mkdir(&mut self, path: &[u8], mode: Mode) -> i32 {
        if self.path_lookup(path).is_some() { return -17; } // EEXIST
        let (parent_path, dirname) = split_path(path);
        let parent_ino = match self.path_lookup(parent_path) { Some(i) => i, None => return -2 };
        let new_inode = Inode::new_dir(mode);
        let new_ino = new_inode.ino;
        self.insert_inode(new_inode);
        let d = Dentry::new(dirname, new_ino, parent_ino);
        self.insert_dentry(d);
        0
    }

    /// unlink — remove a file
    pub fn unlink(&mut self, path: &[u8]) -> i32 {
        let ino = match self.path_lookup(path) { Some(i) => i, None => return -2 };
        let (parent_path, fname) = split_path(path);
        let parent_ino = match self.path_lookup(parent_path) { Some(i) => i, None => return -2 };

        // Remove dentry
        for d in &mut self.dentries {
            if let Some(ref mut de) = d {
                if de.parent == parent_ino && de.name_bytes() == fname {
                    de.active = false;
                    break;
                }
            }
        }
        // Decrement nlink; free inode if 0
        if let Some(inode) = self.find_inode_mut(ino) {
            inode.nlinks = inode.nlinks.saturating_sub(1);
            if inode.nlinks == 0 { inode.active = false; }
        }
        0
    }

    /// Mount a filesystem at a path
    pub fn mount(&mut self, path: &[u8], root_ino: u64, ops: *const FsOps) -> i32 {
        for m in &mut self.mounts {
            if m.is_none() {
                let mut mp = MountPoint {
                    mount_path: [0u8; MAX_NAME],
                    path_len: path.len().min(MAX_NAME - 1),
                    root_ino, fs_ops: ops, active: true,
                };
                mp.mount_path[..mp.path_len].copy_from_slice(&path[..mp.path_len]);
                *m = Some(mp);
                return 0;
            }
        }
        -28 // ENOSPC
    }
}

fn split_path(path: &[u8]) -> (&[u8], &[u8]) {
    if let Some(pos) = path.iter().rposition(|&b| b == b'/') {
        let parent = if pos == 0 { b"/" as &[u8] } else { &path[..pos] };
        (&path[..pos.max(1)], &path[pos + 1..])
    } else {
        (b"/", path)
    }
}

// ── Global VFS instance ────────────────────────────────────────────────────
static mut G_VFS: Vfs = Vfs::empty();

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_init() {
    G_VFS.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_open(path: *const u8, path_len: usize, flags: u32) -> i64 {
    if path.is_null() { return -14; }
    let p = core::slice::from_raw_parts(path, path_len);
    G_VFS.open(p, flags)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_close(fd: i32) -> i32 {
    G_VFS.close(fd)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_stat(
    path: *const u8, path_len: usize, out: *mut Stat,
) -> i32 {
    if path.is_null() { return -14; }
    let p = core::slice::from_raw_parts(path, path_len);
    G_VFS.stat(p, out)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_mkdir(
    path: *const u8, path_len: usize, mode: Mode,
) -> i32 {
    if path.is_null() { return -14; }
    let p = core::slice::from_raw_parts(path, path_len);
    G_VFS.mkdir(p, mode)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_unlink(path: *const u8, path_len: usize) -> i32 {
    if path.is_null() { return -14; }
    let p = core::slice::from_raw_parts(path, path_len);
    G_VFS.unlink(p)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_mount(
    path: *const u8, path_len: usize, root_ino: u64, ops: *const FsOps,
) -> i32 {
    if path.is_null() { return -14; }
    let p = core::slice::from_raw_parts(path, path_len);
    G_VFS.mount(p, root_ino, ops)
}
