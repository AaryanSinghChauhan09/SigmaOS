// SPDX-License-Identifier: MIT
// SigmaOS Virtual Filesystem (VFS) — kernel/vfs/sigma_vfs.rs
// Full VFS layer: mount table, dentry cache, inode operations,
// open/read/write/seek/close syscall backends, and tmpfs built-in.

#![no_std]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── Constants ─────────────────────────────────────────────────────────────────
pub const MAX_MOUNTS:    usize = 32;
pub const MAX_INODES:    usize = 4096;
pub const MAX_DENTRIES:  usize = 4096;
pub const MAX_FDS:       usize = 1024;
pub const MAX_NAME:      usize = 256;
pub const MAX_PATH:      usize = 4096;
pub const TMPFS_SIZE:    usize = 16 * 1024 * 1024; // 16 MB tmpfs pool

// ── File Type Flags ───────────────────────────────────────────────────────────
pub const S_IFREG: u32 = 0o100000;
pub const S_IFDIR: u32 = 0o040000;
pub const S_IFLNK: u32 = 0o120000;
pub const S_IFMT:  u32 = 0o170000;

// ── Open Flags ────────────────────────────────────────────────────────────────
pub const O_RDONLY:   u32 = 0;
pub const O_WRONLY:   u32 = 1;
pub const O_RDWR:     u32 = 2;
pub const O_CREAT:    u32 = 0o100;
pub const O_TRUNC:    u32 = 0o1000;
pub const O_APPEND:   u32 = 0o2000;
pub const O_EXCL:     u32 = 0o200;
pub const O_DIRECTORY:u32 = 0o200000;

// ── Seek Whence ───────────────────────────────────────────────────────────────
pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;

// ── Error Codes ───────────────────────────────────────────────────────────────
pub const ENOENT:    i32 = -2;
pub const EEXIST:    i32 = -17;
pub const EBADF:     i32 = -9;
pub const ENOMEM:    i32 = -12;
pub const EISDIR:    i32 = -21;
pub const ENOTDIR:   i32 = -20;
pub const EINVAL:    i32 = -22;
pub const ENOSPC:    i32 = -28;
pub const EACCES:    i32 = -13;
pub const EFBIG:     i32 = -27;
pub const EIO:       i32 = -5;
pub const ENOTEMPTY: i32 = -39;

// ── Filesystem Type IDs ───────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FsType {
    Tmpfs,
    SigmaFS,
    Ext4,
    Procfs,
    Devfs,
    Unknown,
}

impl Default for FsType {
    fn default() -> Self { FsType::Unknown }
}

// ── Inode Number ─────────────────────────────────────────────────────────────
pub type Ino = u64;
static NEXT_INO: AtomicU64 = AtomicU64::new(2); // 1 = root

fn alloc_ino() -> Ino {
    NEXT_INO.fetch_add(1, Ordering::Relaxed)
}

// ── Inode ─────────────────────────────────────────────────────────────────────
#[derive(Clone, Default)]
pub struct Inode {
    pub ino:         Ino,
    pub mode:        u32,
    pub uid:         u32,
    pub gid:         u32,
    pub size:        u64,
    pub nlink:       u32,
    pub atime_sec:   u64,
    pub mtime_sec:   u64,
    pub ctime_sec:   u64,
    pub dev:         u32,
    pub data_offset: usize,
    pub data_cap:    usize,
    pub fs_type:     FsType,
    pub valid:       bool,
}

impl Inode {
    pub fn is_dir(&self) -> bool  { (self.mode & S_IFMT) == S_IFDIR }
    pub fn is_reg(&self) -> bool  { (self.mode & S_IFMT) == S_IFREG }
}

// ── Dentry ────────────────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct Dentry {
    pub name:     [u8; MAX_NAME],
    pub name_len: u16,
    pub ino:      Ino,
    pub parent:   Ino,
    pub valid:    bool,
}

impl Default for Dentry {
    fn default() -> Self {
        Self { name: [0u8; MAX_NAME], name_len: 0, ino: 0, parent: 0, valid: false }
    }
}

impl Dentry {
    pub fn name_str(&self) -> &[u8] { &self.name[..self.name_len as usize] }
    pub fn matches(&self, name: &[u8]) -> bool { self.valid && self.name_str() == name }
}

// ── Mount Entry ───────────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct Mount {
    pub mountpoint: [u8; MAX_PATH],
    pub mp_len:     u16,
    pub root_ino:   Ino,
    pub fs_type:    FsType,
    pub read_only:  bool,
    pub valid:      bool,
}

impl Default for Mount {
    fn default() -> Self {
        Self { mountpoint: [0u8; MAX_PATH], mp_len: 0, root_ino: 0,
               fs_type: FsType::Unknown, read_only: false, valid: false }
    }
}

// ── File Descriptor ───────────────────────────────────────────────────────────
#[derive(Clone, Default)]
pub struct FileDescriptor {
    pub ino:    Ino,
    pub offset: u64,
    pub flags:  u32,
    pub valid:  bool,
}

// ── Stat ─────────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Default, Debug)]
pub struct Stat {
    pub dev:       u64,
    pub ino:       u64,
    pub mode:      u32,
    pub nlink:     u32,
    pub uid:       u32,
    pub gid:       u32,
    pub rdev:      u64,
    pub size:      i64,
    pub blksize:   u32,
    _pad:          u32,
    pub blocks:    u64,
    pub atime_sec: u64,
    pub atime_ns:  u64,
    pub mtime_sec: u64,
    pub mtime_ns:  u64,
    pub ctime_sec: u64,
    pub ctime_ns:  u64,
}

// ── Tmpfs Pool ────────────────────────────────────────────────────────────────
static mut TMPFS_DATA: [u8; TMPFS_SIZE] = [0u8; TMPFS_SIZE];
static TMPFS_USED: AtomicU64 = AtomicU64::new(0);

fn tmpfs_alloc(size: usize) -> Option<usize> {
    let cur = TMPFS_USED.load(Ordering::Relaxed) as usize;
    if cur + size > TMPFS_SIZE { return None; }
    TMPFS_USED.store((cur + size) as u64, Ordering::Relaxed);
    Some(cur)
}

// ── VFS ───────────────────────────────────────────────────────────────────────
pub struct Vfs {
    pub inodes:       [Inode;           MAX_INODES],
    pub dentries:     [Dentry;          MAX_DENTRIES],
    pub mounts:       [Mount;           MAX_MOUNTS],
    pub fds:          [FileDescriptor;  MAX_FDS],
    pub inode_count:  usize,
    pub dentry_count: usize,
    pub mount_count:  usize,
}

impl Vfs {
    pub fn new_zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }

    // ── Inode helpers ─────────────────────────────────────────────────────────

    fn alloc_inode(&mut self) -> Option<&mut Inode> {
        if self.inode_count >= MAX_INODES { return None; }
        let idx = self.inode_count;
        self.inode_count += 1;
        Some(&mut self.inodes[idx])
    }

    pub fn get_inode(&self, ino: Ino) -> Option<&Inode> {
        self.inodes[..self.inode_count].iter().find(|i| i.valid && i.ino == ino)
    }

    pub fn get_inode_mut(&mut self, ino: Ino) -> Option<&mut Inode> {
        let c = self.inode_count;
        self.inodes[..c].iter_mut().find(|i| i.valid && i.ino == ino)
    }

    // ── Dentry helpers ────────────────────────────────────────────────────────

    fn alloc_dentry(&mut self) -> Option<&mut Dentry> {
        if self.dentry_count >= MAX_DENTRIES { return None; }
        let idx = self.dentry_count;
        self.dentry_count += 1;
        Some(&mut self.dentries[idx])
    }

    pub fn lookup(&self, parent: Ino, name: &[u8]) -> Option<Ino> {
        self.dentries[..self.dentry_count]
            .iter()
            .find(|d| d.parent == parent && d.matches(name))
            .map(|d| d.ino)
    }

    fn link_dentry(&mut self, parent: Ino, name: &[u8], ino: Ino) -> bool {
        let de = match self.alloc_dentry() { Some(d) => d, None => return false };
        let l = name.len().min(MAX_NAME - 1);
        de.name[..l].copy_from_slice(&name[..l]);
        de.name_len = l as u16;
        de.ino      = ino;
        de.parent   = parent;
        de.valid    = true;
        true
    }

    // ── Mounting ──────────────────────────────────────────────────────────────

    fn mount_at(&mut self, mp: &[u8], root_ino: Ino, fs: FsType, ro: bool) -> bool {
        if self.mount_count >= MAX_MOUNTS { return false; }
        let m = &mut self.mounts[self.mount_count];
        let l = mp.len().min(MAX_PATH - 1);
        m.mountpoint[..l].copy_from_slice(&mp[..l]);
        m.mp_len   = l as u16;
        m.root_ino = root_ino;
        m.fs_type  = fs;
        m.read_only= ro;
        m.valid    = true;
        self.mount_count += 1;
        true
    }

    fn make_dir_inode(&mut self, ino: Ino, fs: FsType) -> Ino {
        if let Some(node) = self.alloc_inode() {
            node.ino     = ino;
            node.mode    = S_IFDIR | 0o755;
            node.nlink   = 2;
            node.fs_type = fs;
            node.valid   = true;
        }
        ino
    }

    fn make_char_inode(&mut self, parent: Ino, name: &[u8], mode: u32, major: u32, minor: u32) {
        let ino = alloc_ino();
        if let Some(n) = self.alloc_inode() {
            n.ino    = ino;
            n.mode   = 0o020000 | mode;
            n.dev    = (major << 8) | minor;
            n.nlink  = 1;
            n.fs_type= FsType::Devfs;
            n.valid  = true;
        }
        self.link_dentry(parent, name, ino);
    }

    // ── Initialization ────────────────────────────────────────────────────────

    pub fn init(&mut self) {
        let root_ino = self.make_dir_inode(1, FsType::Tmpfs);
        self.mount_at(b"/", root_ino, FsType::Tmpfs, false);

        // /tmp
        let _ = self.mkdir_at(root_ino, b"tmp", 0o1777);
        // /home
        let _ = self.mkdir_at(root_ino, b"home", 0o755);
        // /etc
        let _ = self.mkdir_at(root_ino, b"etc", 0o755);
        // /var
        let _ = self.mkdir_at(root_ino, b"var", 0o755);
        // /proc
        let proc_ino = self.make_dir_inode(alloc_ino(), FsType::Procfs);
        self.link_dentry(root_ino, b"proc", proc_ino);
        self.mount_at(b"/proc", proc_ino, FsType::Procfs, true);
        // /dev
        let dev_ino = self.make_dir_inode(alloc_ino(), FsType::Devfs);
        self.link_dentry(root_ino, b"dev", dev_ino);
        self.mount_at(b"/dev", dev_ino, FsType::Devfs, false);
        self.make_char_inode(dev_ino, b"null", 0o0666, 1, 3);
        self.make_char_inode(dev_ino, b"zero", 0o0666, 1, 5);
        self.make_char_inode(dev_ino, b"tty",  0o0666, 5, 0);
        self.make_char_inode(dev_ino, b"urandom", 0o0444, 1, 9);
    }

    pub fn install_stdio(&mut self) {
        for fd in 0..3 {
            self.fds[fd] = FileDescriptor {
                ino:    1,
                offset: 0,
                flags:  if fd == 0 { O_RDONLY } else { O_WRONLY },
                valid:  true,
            };
        }
    }

    // ── Path Resolution ───────────────────────────────────────────────────────

    pub fn resolve_path(&self, path: &[u8]) -> Option<Ino> {
        if path.is_empty() || path[0] != b'/' { return None; }
        let mut cur = self.mounts[..self.mount_count]
            .iter()
            .find(|m| m.valid && m.mp_len == 1 && m.mountpoint[0] == b'/')
            .map(|m| m.root_ino)?;

        let mut i = 1usize;
        while i < path.len() {
            while i < path.len() && path[i] == b'/' { i += 1; }
            if i >= path.len() { break; }
            let start = i;
            while i < path.len() && path[i] != b'/' { i += 1; }
            let comp = &path[start..i];
            if comp == b"." { continue; }
            if comp == b".." {
                if let Some(d) = self.dentries[..self.dentry_count]
                    .iter().find(|d| d.valid && d.ino == cur) {
                    cur = d.parent;
                }
                continue;
            }
            cur = self.lookup(cur, comp)?;
        }
        Some(cur)
    }

    // ── Directory ops ─────────────────────────────────────────────────────────

    pub fn mkdir_at(&mut self, parent: Ino, name: &[u8], mode: u32) -> Result<Ino, i32> {
        if self.lookup(parent, name).is_some() { return Err(EEXIST); }
        let ino = alloc_ino();
        {
            let n = self.alloc_inode().ok_or(ENOMEM)?;
            n.ino    = ino;
            n.mode   = S_IFDIR | (mode & 0o7777);
            n.nlink  = 2;
            n.fs_type= FsType::Tmpfs;
            n.valid  = true;
        }
        if !self.link_dentry(parent, name, ino) { return Err(ENOMEM); }
        Ok(ino)
    }

    // ── File ops ──────────────────────────────────────────────────────────────

    pub fn create_file(&mut self, parent: Ino, name: &[u8], mode: u32) -> Result<Ino, i32> {
        if self.lookup(parent, name).is_some() { return Err(EEXIST); }
        let ino = alloc_ino();
        let off = tmpfs_alloc(0).ok_or(ENOSPC)?;
        {
            let n = self.alloc_inode().ok_or(ENOMEM)?;
            n.ino         = ino;
            n.mode        = S_IFREG | (mode & 0o7777);
            n.nlink       = 1;
            n.fs_type     = FsType::Tmpfs;
            n.data_offset = off;
            n.data_cap    = 0;
            n.size        = 0;
            n.valid       = true;
        }
        if !self.link_dentry(parent, name, ino) { return Err(ENOMEM); }
        Ok(ino)
    }

    // ── Syscall implementations ───────────────────────────────────────────────

    pub fn sys_open(&mut self, path: &[u8], flags: u32, mode: u32) -> i32 {
        let ino = match self.resolve_path(path) {
            Some(i) => i,
            None => {
                if flags & O_CREAT == 0 { return ENOENT; }
                let (pp, name) = split_parent(path);
                let parent = match self.resolve_path(pp) { Some(p) => p, None => return ENOENT };
                match self.create_file(parent, name, mode) {
                    Ok(i) => i, Err(e) => return e,
                }
            }
        };

        if flags & O_EXCL != 0 && flags & O_CREAT != 0 { return EEXIST; }
        if flags & O_TRUNC != 0 {
            if let Some(n) = self.get_inode_mut(ino) {
                if n.is_reg() { n.size = 0; n.data_cap = 0; }
            }
        }

        let fd = match self.alloc_fd() { Some(f) => f, None => return ENOMEM };
        let init_offset = if flags & O_APPEND != 0 {
            self.get_inode(ino).map(|n| n.size).unwrap_or(0)
        } else { 0 };

        self.fds[fd] = FileDescriptor { ino, offset: init_offset, flags, valid: true };
        fd as i32
    }

    pub fn sys_close(&mut self, fd: i32) -> i32 {
        if fd < 0 || fd as usize >= MAX_FDS { return EBADF; }
        let f = &mut self.fds[fd as usize];
        if !f.valid { return EBADF; }
        f.valid = false;
        0
    }

    pub fn sys_read(&mut self, fd: i32, buf: &mut [u8]) -> i32 {
        if fd < 0 || fd as usize >= MAX_FDS { return EBADF; }
        let fdesc = self.fds[fd as usize].clone();
        if !fdesc.valid { return EBADF; }
        let ino = fdesc.ino;
        let offset = fdesc.offset;
        if let Some(node) = self.get_inode(ino) {
            if node.is_dir() { return EISDIR; }
            if !node.is_reg() { return EIO; }
            let avail = node.size.saturating_sub(offset) as usize;
            let to_read = buf.len().min(avail);
            if to_read == 0 { return 0; }
            let src = node.data_offset + offset as usize;
            unsafe { buf[..to_read].copy_from_slice(&TMPFS_DATA[src..src + to_read]); }
            self.fds[fd as usize].offset += to_read as u64;
            return to_read as i32;
        }
        ENOENT
    }

    pub fn sys_write(&mut self, fd: i32, buf: &[u8]) -> i32 {
        if fd < 0 || fd as usize >= MAX_FDS { return EBADF; }
        let fdesc = self.fds[fd as usize].clone();
        if !fdesc.valid { return EBADF; }
        let ino = fdesc.ino;
        let offset = fdesc.offset;

        let (data_offset, data_cap, cur_size) = {
            match self.get_inode(ino) {
                Some(n) => (n.data_offset, n.data_cap, n.size as usize),
                None => return ENOENT,
            }
        };

        let write_end = offset as usize + buf.len();
        if write_end > TMPFS_SIZE { return EFBIG; }

        if write_end > data_offset + data_cap {
            let extra = write_end - (data_offset + data_cap);
            if tmpfs_alloc(extra).is_none() { return ENOSPC; }
            if let Some(n) = self.get_inode_mut(ino) { n.data_cap += extra; }
        }

        unsafe {
            let dst = &mut TMPFS_DATA[data_offset + offset as usize..
                                       data_offset + offset as usize + buf.len()];
            dst.copy_from_slice(buf);
        }

        let new_size = ((offset as usize + buf.len()).max(cur_size)) as u64;
        if let Some(n) = self.get_inode_mut(ino) { n.size = new_size; }
        self.fds[fd as usize].offset += buf.len() as u64;
        buf.len() as i32
    }

    pub fn sys_seek(&mut self, fd: i32, offset: i64, whence: i32) -> i64 {
        if fd < 0 || fd as usize >= MAX_FDS { return EBADF as i64; }
        let fdesc = &self.fds[fd as usize];
        if !fdesc.valid { return EBADF as i64; }
        let file_size = self.get_inode(fdesc.ino).map(|n| n.size).unwrap_or(0) as i64;
        let cur = fdesc.offset as i64;
        let new_off = match whence {
            SEEK_SET => offset,
            SEEK_CUR => cur + offset,
            SEEK_END => file_size + offset,
            _        => return EINVAL as i64,
        };
        if new_off < 0 { return EINVAL as i64; }
        self.fds[fd as usize].offset = new_off as u64;
        new_off
    }

    pub fn sys_stat(&self, path: &[u8], s: &mut Stat) -> i32 {
        let ino = match self.resolve_path(path) { Some(i) => i, None => return ENOENT };
        match self.get_inode(ino) {
            Some(n) => { fill_stat(s, n); 0 }
            None    => ENOENT,
        }
    }

    pub fn sys_fstat(&self, fd: i32, s: &mut Stat) -> i32 {
        if fd < 0 || fd as usize >= MAX_FDS { return EBADF; }
        let fdesc = &self.fds[fd as usize];
        if !fdesc.valid { return EBADF; }
        match self.get_inode(fdesc.ino) {
            Some(n) => { fill_stat(s, n); 0 }
            None    => ENOENT,
        }
    }

    pub fn sys_getdents<F: FnMut(&[u8], Ino, u32)>(&self, fd: i32, mut cb: F) -> i32 {
        if fd < 0 || fd as usize >= MAX_FDS { return EBADF; }
        let fdesc = &self.fds[fd as usize];
        if !fdesc.valid { return EBADF; }
        let ino = fdesc.ino;
        let node = match self.get_inode(ino) { Some(n) => n, None => return ENOENT };
        if !node.is_dir() { return ENOTDIR; }
        let mut count = 0i32;
        for d in self.dentries[..self.dentry_count].iter() {
            if d.valid && d.parent == ino {
                let ft = self.get_inode(d.ino).map(|n| (n.mode >> 12) & 0xF).unwrap_or(0);
                cb(d.name_str(), d.ino, ft);
                count += 1;
            }
        }
        count
    }

    fn alloc_fd(&mut self) -> Option<usize> {
        for i in 3..MAX_FDS {
            if !self.fds[i].valid { return Some(i); }
        }
        None
    }
}

fn fill_stat(s: &mut Stat, n: &Inode) {
    s.ino       = n.ino;
    s.mode      = n.mode;
    s.nlink     = n.nlink;
    s.uid       = n.uid;
    s.gid       = n.gid;
    s.size      = n.size as i64;
    s.blksize   = 4096;
    s.blocks    = (n.size + 511) / 512;
    s.atime_sec = n.atime_sec;
    s.mtime_sec = n.mtime_sec;
    s.ctime_sec = n.ctime_sec;
}

fn split_parent(path: &[u8]) -> (&[u8], &[u8]) {
    let mut i = path.len();
    while i > 1 && path[i - 1] != b'/' { i -= 1; }
    if i == 0 || path[i - 1] != b'/' { return (b"/", path); }
    let parent = &path[..i.saturating_sub(1)];
    let name   = &path[i..];
    if parent.is_empty() { (b"/", name) } else { (parent, name) }
}

// ── Global VFS ────────────────────────────────────────────────────────────────
static mut G_VFS_STORAGE: [u8; core::mem::size_of::<Vfs>()] = [0u8; core::mem::size_of::<Vfs>()];
static VFS_INITIALIZED: AtomicU32 = AtomicU32::new(0);

unsafe fn vfs() -> &'static mut Vfs {
    &mut *(G_VFS_STORAGE.as_mut_ptr() as *mut Vfs)
}

// ── C-ABI ─────────────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_init() {
    if VFS_INITIALIZED.swap(1, Ordering::SeqCst) != 0 { return; }
    let v = vfs();
    core::ptr::write(v, Vfs::new_zeroed());
    v.init();
    v.install_stdio();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_open(path: *const u8, path_len: usize, flags: u32, mode: u32) -> i32 {
    if path.is_null() { return EINVAL; }
    let p = core::slice::from_raw_parts(path, path_len);
    vfs().sys_open(p, flags, mode)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_close(fd: i32) -> i32 { vfs().sys_close(fd) }

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_read(fd: i32, buf: *mut u8, len: usize) -> i32 {
    if buf.is_null() { return EINVAL; }
    let b = core::slice::from_raw_parts_mut(buf, len);
    vfs().sys_read(fd, b)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_write(fd: i32, buf: *const u8, len: usize) -> i32 {
    if buf.is_null() { return EINVAL; }
    let b = core::slice::from_raw_parts(buf, len);
    vfs().sys_write(fd, b)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_seek(fd: i32, offset: i64, whence: i32) -> i64 {
    vfs().sys_seek(fd, offset, whence)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_mkdir(path: *const u8, path_len: usize, mode: u32) -> i32 {
    if path.is_null() { return EINVAL; }
    let p = core::slice::from_raw_parts(path, path_len);
    let (pp, name) = split_parent(p);
    let v = vfs();
    let parent = match v.resolve_path(pp) { Some(i) => i, None => return ENOENT };
    match v.mkdir_at(parent, name, mode) { Ok(_) => 0, Err(e) => e }
}
