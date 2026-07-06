// SigmaOS — VFS + Ext4-compatible Filesystem Layer
// Sovereign implementation — no external dependencies
#![no_std]
#![allow(dead_code)]

// ─── VFS Inode ───────────────────────────────────────────────────────────────
pub const MAX_INODES:   usize = 65536;
pub const MAX_DENTRIES: usize = 16384;
pub const MAX_FILES:    usize = 4096;
pub const NAME_MAX:     usize = 255;
pub const BLOCK_SIZE:   usize = 4096;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum InodeType { Unknown=0, Regular=1, Directory=2, Symlink=3, Chardev=4, Blkdev=5, Fifo=6, Socket=7 }

#[derive(Clone, Copy)]
pub struct Inode {
    pub ino:    u32,
    pub itype:  InodeType,
    pub mode:   u16,
    pub uid:    u16,
    pub gid:    u16,
    pub size:   u64,
    pub atime:  u64,
    pub mtime:  u64,
    pub ctime:  u64,
    pub nlinks: u16,
    pub blocks: [u32; 15], // direct(12) + indirect(1) + dbl(1) + trpl(1)
    pub flags:  u32,
    pub used:   bool,
}

impl Inode {
    pub const fn empty() -> Self {
        Inode {
            ino: 0, itype: InodeType::Unknown, mode: 0,
            uid: 0, gid: 0, size: 0,
            atime: 0, mtime: 0, ctime: 0,
            nlinks: 0, blocks: [0u32; 15], flags: 0, used: false,
        }
    }
    pub fn is_dir(&self) -> bool { self.itype == InodeType::Directory }
    pub fn is_file(&self) -> bool { self.itype == InodeType::Regular }
}

// ─── Directory Entry ─────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct Dentry {
    pub ino:     u32,
    pub parent:  u32,
    pub name:    [u8; NAME_MAX],
    pub namelen: u8,
    pub used:    bool,
}

impl Dentry {
    pub const fn empty() -> Self {
        Dentry { ino: 0, parent: 0, name: [0u8; NAME_MAX], namelen: 0, used: false }
    }
    pub fn name_str(&self) -> &[u8] { &self.name[..self.namelen as usize] }
}

// ─── Open File Table Entry ───────────────────────────────────────────────────
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR:   u32 = 2;
pub const O_CREAT:  u32 = 0x40;
pub const O_TRUNC:  u32 = 0x200;
pub const O_APPEND: u32 = 0x400;

#[derive(Clone, Copy)]
pub struct OpenFile {
    pub ino:    u32,
    pub flags:  u32,
    pub offset: u64,
    pub used:   bool,
    pub pid:    u32,
}

impl OpenFile {
    pub const fn empty() -> Self {
        OpenFile { ino: 0, flags: 0, offset: 0, used: false, pid: 0 }
    }
}

// ─── VFS ─────────────────────────────────────────────────────────────────────
pub struct Vfs {
    pub inodes:   [Inode; MAX_INODES],
    pub dentries: [Dentry; MAX_DENTRIES],
    pub files:    [OpenFile; MAX_FILES],
    pub next_ino: u32,
    pub inode_count: usize,
    pub dentry_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VfsError {
    NotFound, AlreadyExists, NotDir, IsDir, NoSpace,
    PermDenied, BadFd, TooBig, InvalidArg, Io,
}

impl Vfs {
    pub fn new() -> Self {
        let mut vfs = Vfs {
            inodes:   [Inode::empty(); MAX_INODES],
            dentries: [Dentry::empty(); MAX_DENTRIES],
            files:    [OpenFile::empty(); MAX_FILES],
            next_ino: 1,
            inode_count: 0,
            dentry_count: 0,
        };
        // Create root inode (ino=1)
        vfs.inodes[1].ino   = 1;
        vfs.inodes[1].itype = InodeType::Directory;
        vfs.inodes[1].mode  = 0o755;
        vfs.inodes[1].nlinks = 2;
        vfs.inodes[1].used  = true;
        vfs.next_ino = 2;
        vfs.inode_count = 1;
        // Root dentry
        vfs.dentries[0].ino    = 1;
        vfs.dentries[0].parent = 1;
        vfs.dentries[0].name[0] = b'/';
        vfs.dentries[0].namelen = 1;
        vfs.dentries[0].used   = true;
        vfs.dentry_count = 1;
        vfs
    }

    /// Allocate new inode.
    fn alloc_inode(&mut self) -> Option<u32> {
        for i in self.next_ino as usize..MAX_INODES {
            if !self.inodes[i].used {
                self.inodes[i].used = true;
                self.inodes[i].ino  = i as u32;
                self.next_ino = (i + 1) as u32;
                self.inode_count += 1;
                return Some(i as u32);
            }
        }
        None
    }

    /// Look up a filename in a directory inode.
    pub fn lookup(&self, dir_ino: u32, name: &[u8]) -> Option<u32> {
        for d in &self.dentries[..self.dentry_count] {
            if d.used && d.parent == dir_ino && d.name_str() == name {
                return Some(d.ino);
            }
        }
        None
    }

    /// Create a regular file.
    pub fn create(&mut self, dir_ino: u32, name: &[u8], mode: u16, uid: u16, pid: u32) -> Result<u32, VfsError> {
        if name.len() > NAME_MAX { return Err(VfsError::InvalidArg); }
        if !self.inodes[dir_ino as usize].is_dir() { return Err(VfsError::NotDir); }
        if self.lookup(dir_ino, name).is_some() { return Err(VfsError::AlreadyExists); }
        let ino = self.alloc_inode().ok_or(VfsError::NoSpace)?;
        let i = &mut self.inodes[ino as usize];
        i.itype = InodeType::Regular;
        i.mode  = mode;
        i.uid   = uid;
        i.nlinks = 1;
        // Add dentry
        if self.dentry_count >= MAX_DENTRIES { return Err(VfsError::NoSpace); }
        let d = &mut self.dentries[self.dentry_count];
        d.ino    = ino;
        d.parent = dir_ino;
        let nlen = name.len().min(NAME_MAX);
        d.name[..nlen].copy_from_slice(&name[..nlen]);
        d.namelen = nlen as u8;
        d.used    = true;
        self.dentry_count += 1;
        Ok(ino)
    }

    /// Create directory.
    pub fn mkdir(&mut self, dir_ino: u32, name: &[u8], mode: u16, uid: u16) -> Result<u32, VfsError> {
        let ino = self.create(dir_ino, name, mode, uid, 0)?;
        self.inodes[ino as usize].itype  = InodeType::Directory;
        self.inodes[ino as usize].nlinks = 2;
        Ok(ino)
    }

    /// Open a file, returns fd.
    pub fn open(&mut self, path_ino: u32, flags: u32, pid: u32) -> Result<usize, VfsError> {
        if !self.inodes[path_ino as usize].used { return Err(VfsError::NotFound); }
        if flags & O_WRONLY != 0 && flags & O_TRUNC != 0 {
            self.inodes[path_ino as usize].size = 0;
        }
        for (fd, f) in self.files.iter_mut().enumerate() {
            if !f.used {
                *f = OpenFile { ino: path_ino, flags, offset: 0, used: true, pid };
                return Ok(fd);
            }
        }
        Err(VfsError::NoSpace)
    }

    /// Close fd.
    pub fn close(&mut self, fd: usize) -> Result<(), VfsError> {
        if fd >= MAX_FILES || !self.files[fd].used { return Err(VfsError::BadFd); }
        self.files[fd].used = false;
        Ok(())
    }

    /// Stat by inode.
    pub fn stat(&self, ino: u32) -> Option<&Inode> {
        let i = &self.inodes[ino as usize];
        if i.used { Some(i) } else { None }
    }

    /// Unlink (remove dentry + decrement nlinks).
    pub fn unlink(&mut self, dir_ino: u32, name: &[u8]) -> Result<(), VfsError> {
        let ino = self.lookup(dir_ino, name).ok_or(VfsError::NotFound)?;
        // Remove dentry
        for d in &mut self.dentries[..self.dentry_count] {
            if d.used && d.parent == dir_ino && d.name_str() == name {
                d.used = false;
                break;
            }
        }
        let nlinks = &mut self.inodes[ino as usize].nlinks;
        if *nlinks > 0 { *nlinks -= 1; }
        if *nlinks == 0 { self.inodes[ino as usize].used = false; self.inode_count -= 1; }
        Ok(())
    }

    /// Rename.
    pub fn rename(&mut self, old_dir: u32, old_name: &[u8], new_dir: u32, new_name: &[u8]) -> Result<(), VfsError> {
        let ino = self.lookup(old_dir, old_name).ok_or(VfsError::NotFound)?;
        // Update dentry
        for d in &mut self.dentries[..self.dentry_count] {
            if d.used && d.parent == old_dir && d.name_str() == old_name {
                d.parent = new_dir;
                let nlen = new_name.len().min(NAME_MAX);
                d.name[..nlen].copy_from_slice(&new_name[..nlen]);
                d.namelen = nlen as u8;
                return Ok(());
            }
        }
        Err(VfsError::NotFound)
    }

    /// List directory entries.
    pub fn readdir(&self, dir_ino: u32, out: &mut [(u32, [u8; NAME_MAX], u8)]) -> usize {
        let mut n = 0;
        for d in &self.dentries[..self.dentry_count] {
            if n >= out.len() { break; }
            if d.used && d.parent == dir_ino {
                out[n].0 = d.ino;
                out[n].1[..d.namelen as usize].copy_from_slice(&d.name[..d.namelen as usize]);
                out[n].2 = d.namelen;
                n += 1;
            }
        }
        n
    }

    /// Resolve absolute path to inode number.
    pub fn path_resolve(&self, path: &[u8]) -> Option<u32> {
        if path.is_empty() || path[0] != b'/' { return None; }
        let mut cur_ino = 1u32; // root
        let mut start = 1usize;
        while start < path.len() {
            let end = path[start..].iter().position(|&b| b == b'/').map(|p| start + p).unwrap_or(path.len());
            let component = &path[start..end];
            if component.is_empty() { start = end + 1; continue; }
            if component == b"." { start = end + 1; continue; }
            if component == b".." {
                // Find parent
                for d in &self.dentries[..self.dentry_count] {
                    if d.used && d.ino == cur_ino { cur_ino = d.parent; break; }
                }
                start = end + 1; continue;
            }
            cur_ino = self.lookup(cur_ino, component)?;
            start = end + 1;
        }
        Some(cur_ino)
    }
}
