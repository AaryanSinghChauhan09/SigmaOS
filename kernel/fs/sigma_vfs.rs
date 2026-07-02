// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/fs/sigma_vfs.rs — Virtual Filesystem Layer
// Replaces: SovereignVFS.cpp, sigma_vfs.cpp (C++ stubs, removed)
//
// Language: Rust #![no_std]
// Pattern: OOP via FileSystem + File traits, concrete implementations

#![no_std]

use core::fmt;

// ── Constants ────────────────────────────────────────────────────────────────

pub const MAX_PATH_LEN:   usize = 256;
pub const MAX_FD:         usize = 64;
pub const MAX_FILESYSTEMS: usize = 8;
pub const MAX_MOUNTS:     usize = 16;

// ── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VfsError {
    NotFound,
    PermissionDenied,
    InvalidPath,
    NotADirectory,
    IsADirectory,
    NoSpace,
    AlreadyExists,
    NotSupported,
    IoError,
    TooManyOpen,
    BadFileDescriptor,
}

pub type VfsResult<T> = Result<T, VfsError>;

// ── Open Flags ───────────────────────────────────────────────────────────────

pub struct OpenFlags(pub u32);
impl OpenFlags {
    pub const RDONLY:  u32 = 0x0000;
    pub const WRONLY:  u32 = 0x0001;
    pub const RDWR:    u32 = 0x0002;
    pub const CREAT:   u32 = 0x0040;
    pub const EXCL:    u32 = 0x0080;
    pub const TRUNC:   u32 = 0x0200;
    pub const APPEND:  u32 = 0x0400;

    pub fn is_readable(&self)  -> bool { (self.0 & 0x3) != Self::WRONLY }
    pub fn is_writable(&self)  -> bool { (self.0 & 0x3) != Self::RDONLY }
    pub fn is_create(&self)    -> bool { self.0 & Self::CREAT != 0 }
}

// ── Inode ────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InodeKind { File, Directory, Symlink, Device }

#[derive(Clone, Copy)]
pub struct InodeMeta {
    pub kind:    InodeKind,
    pub size:    u64,
    pub inode:   u64,
    pub nlinks:  u32,
    pub uid:     u32,
    pub gid:     u32,
    pub mode:    u16,
}

// ── FileSystem Trait (OOP interface) ─────────────────────────────────────────

pub trait FileSystem: Send + Sync {
    fn name(&self) -> &'static str;
    fn mount(&mut self, device: usize) -> VfsResult<()>;
    fn umount(&mut self) -> VfsResult<()>;

    fn lookup(&self, path: &[u8]) -> VfsResult<InodeMeta>;
    fn open   (&mut self, path: &[u8], flags: OpenFlags) -> VfsResult<u64>; // returns file_handle
    fn close  (&mut self, handle: u64) -> VfsResult<()>;
    fn read   (&mut self, handle: u64, buf: &mut [u8], offset: u64) -> VfsResult<usize>;
    fn write  (&mut self, handle: u64, buf: &[u8],     offset: u64) -> VfsResult<usize>;
    fn mkdir  (&mut self, path: &[u8]) -> VfsResult<()>;
    fn unlink (&mut self, path: &[u8]) -> VfsResult<()>;
    fn rename (&mut self, old: &[u8], new: &[u8]) -> VfsResult<()>;
    fn readdir(&self, path: &[u8], cb: &mut dyn FnMut(&[u8], InodeKind)) -> VfsResult<()>;
    fn stat   (&self, path: &[u8]) -> VfsResult<InodeMeta>;
}

// ── Mount Table ──────────────────────────────────────────────────────────────

struct MountEntry {
    mountpoint: [u8; MAX_PATH_LEN],
    mp_len:     usize,
    fs_index:   usize,
}

// ── VFS Layer ────────────────────────────────────────────────────────────────

pub struct Vfs {
    filesystems: [Option<*mut dyn FileSystem>; MAX_FILESYSTEMS],
    mounts:      [Option<MountEntry>;          MAX_MOUNTS],
    fs_count:    usize,
    mt_count:    usize,
}

impl Vfs {
    pub const fn new() -> Self {
        Self {
            filesystems: [const { None }; MAX_FILESYSTEMS],
            mounts:      [const { None }; MAX_MOUNTS],
            fs_count:    0,
            mt_count:    0,
        }
    }

    /// Register a filesystem driver
    pub fn register(&mut self, fs: *mut dyn FileSystem) -> VfsResult<()> {
        if self.fs_count >= MAX_FILESYSTEMS { return Err(VfsError::NoSpace); }
        self.filesystems[self.fs_count] = Some(fs);
        self.fs_count += 1;
        Ok(())
    }

    /// Mount a filesystem at a path
    pub fn mount(&mut self, fs_index: usize, mountpoint: &[u8], device: usize) -> VfsResult<()> {
        if fs_index >= self.fs_count      { return Err(VfsError::NotFound); }
        if self.mt_count >= MAX_MOUNTS    { return Err(VfsError::NoSpace); }
        if mountpoint.len() > MAX_PATH_LEN { return Err(VfsError::InvalidPath); }

        let fs = unsafe { &mut *self.filesystems[fs_index].unwrap() };
        fs.mount(device)?;

        let mut entry = MountEntry {
            mountpoint: [0u8; MAX_PATH_LEN],
            mp_len:     mountpoint.len(),
            fs_index,
        };
        entry.mountpoint[..mountpoint.len()].copy_from_slice(mountpoint);
        self.mounts[self.mt_count] = Some(entry);
        self.mt_count += 1;
        Ok(())
    }

    /// Find the filesystem mounted at or above `path`
    fn resolve_fs(&mut self, path: &[u8]) -> Option<&mut dyn FileSystem> {
        // Find longest matching mountpoint prefix
        let mut best_len = 0;
        let mut best_fs  = MAX_FILESYSTEMS;
        for i in 0..self.mt_count {
            if let Some(ref m) = self.mounts[i] {
                let mp = &m.mountpoint[..m.mp_len];
                if path.starts_with(mp) && m.mp_len > best_len {
                    best_len = m.mp_len;
                    best_fs  = m.fs_index;
                }
            }
        }
        if best_fs < MAX_FILESYSTEMS {
            self.filesystems[best_fs].map(|p| unsafe { &mut *p })
        } else {
            None
        }
    }

    pub fn open(&mut self, path: &[u8], flags: OpenFlags) -> VfsResult<u64> {
        let fs = self.resolve_fs(path).ok_or(VfsError::NotFound)?;
        fs.open(path, flags)
    }

    pub fn read(&mut self, path: &[u8], handle: u64, buf: &mut [u8], off: u64) -> VfsResult<usize> {
        let fs = self.resolve_fs(path).ok_or(VfsError::BadFileDescriptor)?;
        fs.read(handle, buf, off)
    }

    pub fn write(&mut self, path: &[u8], handle: u64, buf: &[u8], off: u64) -> VfsResult<usize> {
        let fs = self.resolve_fs(path).ok_or(VfsError::BadFileDescriptor)?;
        fs.write(handle, buf, off)
    }

    pub fn close(&mut self, path: &[u8], handle: u64) -> VfsResult<()> {
        let fs = self.resolve_fs(path).ok_or(VfsError::BadFileDescriptor)?;
        fs.close(handle)
    }

    pub fn stat(&mut self, path: &[u8]) -> VfsResult<InodeMeta> {
        let fs = self.resolve_fs(path).ok_or(VfsError::NotFound)?;
        fs.stat(path)
    }
}

// ── Tmpfs (RAM-backed filesystem) ────────────────────────────────────────────

const TMPFS_MAX_FILES:   usize = 128;
const TMPFS_MAX_CONTENT: usize = 65536;

struct TmpfsFile {
    name:    [u8; 64],
    name_len: usize,
    data:    [u8; TMPFS_MAX_CONTENT],
    size:    usize,
    inode:   u64,
}

pub struct Tmpfs {
    files:   [Option<TmpfsFile>; TMPFS_MAX_FILES],
    count:   usize,
    next_ino: u64,
}

impl Tmpfs {
    pub const fn new() -> Self {
        Self {
            files:    [const { None }; TMPFS_MAX_FILES],
            count:    0,
            next_ino: 1,
        }
    }

    fn find_file(&self, path: &[u8]) -> Option<usize> {
        // Strip leading '/'
        let name = if path.first() == Some(&b'/') { &path[1..] } else { path };
        for i in 0..self.count {
            if let Some(ref f) = self.files[i] {
                if &f.name[..f.name_len] == name { return Some(i); }
            }
        }
        None
    }
}

impl FileSystem for Tmpfs {
    fn name(&self) -> &'static str { "tmpfs" }
    fn mount(&mut self, _device: usize) -> VfsResult<()> { Ok(()) }
    fn umount(&mut self) -> VfsResult<()> { Ok(()) }

    fn lookup(&self, path: &[u8]) -> VfsResult<InodeMeta> {
        let idx = self.find_file(path).ok_or(VfsError::NotFound)?;
        let f = self.files[idx].as_ref().unwrap();
        Ok(InodeMeta {
            kind:   InodeKind::File, size: f.size as u64,
            inode:  f.inode, nlinks: 1, uid: 0, gid: 0, mode: 0o644,
        })
    }

    fn open(&mut self, path: &[u8], flags: OpenFlags) -> VfsResult<u64> {
        let name = if path.first() == Some(&b'/') { &path[1..] } else { path };
        if flags.is_create() && self.find_file(path).is_none() {
            if self.count >= TMPFS_MAX_FILES { return Err(VfsError::NoSpace); }
            if name.len() > 64 { return Err(VfsError::InvalidPath); }
            let ino = self.next_ino;
            self.next_ino += 1;
            let mut file = TmpfsFile {
                name: [0u8; 64], name_len: name.len(),
                data: [0u8; TMPFS_MAX_CONTENT], size: 0, inode: ino,
            };
            file.name[..name.len()].copy_from_slice(name);
            self.files[self.count] = Some(file);
            self.count += 1;
            return Ok(ino);
        }
        let idx = self.find_file(path).ok_or(VfsError::NotFound)?;
        Ok(self.files[idx].as_ref().unwrap().inode)
    }

    fn close(&mut self, _handle: u64) -> VfsResult<()> { Ok(()) }

    fn read(&mut self, handle: u64, buf: &mut [u8], offset: u64) -> VfsResult<usize> {
        for i in 0..self.count {
            if let Some(ref f) = self.files[i] {
                if f.inode == handle {
                    let off = offset as usize;
                    if off >= f.size { return Ok(0); }
                    let n = buf.len().min(f.size - off);
                    buf[..n].copy_from_slice(&f.data[off..off + n]);
                    return Ok(n);
                }
            }
        }
        Err(VfsError::BadFileDescriptor)
    }

    fn write(&mut self, handle: u64, buf: &[u8], offset: u64) -> VfsResult<usize> {
        for i in 0..self.count {
            if let Some(ref mut f) = self.files[i] {
                if f.inode == handle {
                    let off = offset as usize;
                    let end = off + buf.len();
                    if end > TMPFS_MAX_CONTENT { return Err(VfsError::NoSpace); }
                    f.data[off..end].copy_from_slice(buf);
                    if end > f.size { f.size = end; }
                    return Ok(buf.len());
                }
            }
        }
        Err(VfsError::BadFileDescriptor)
    }

    fn mkdir  (&mut self, _path: &[u8]) -> VfsResult<()> { Ok(()) }
    fn unlink (&mut self, path: &[u8])  -> VfsResult<()> {
        if let Some(idx) = self.find_file(path) { self.files[idx] = None; }
        Ok(())
    }
    fn rename (&mut self, _old: &[u8], _new: &[u8]) -> VfsResult<()> { Err(VfsError::NotSupported) }
    fn readdir(&self, _path: &[u8], cb: &mut dyn FnMut(&[u8], InodeKind)) -> VfsResult<()> {
        for i in 0..self.count {
            if let Some(ref f) = self.files[i] {
                cb(&f.name[..f.name_len], InodeKind::File);
            }
        }
        Ok(())
    }
    fn stat(&self, path: &[u8]) -> VfsResult<InodeMeta> { self.lookup(path) }
}
