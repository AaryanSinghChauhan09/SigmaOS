#![no_std]
#![allow(dead_code)]

/// SigmaOS Virtual File System (VFS)
/// A zero-allocation POSIX-like abstraction for multiple file systems.

use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_FDS: usize = 1024;
const MAX_MOUNTS: usize = 8;
const MAX_PATH_LEN: usize = 256;

#[derive(Copy, Clone, PartialEq)]
pub enum FsType {
    Ext4,
    APFS,
    Tmpfs,
    Unknown,
}

#[derive(Copy, Clone)]
pub struct MountPoint {
    path: [u8; MAX_PATH_LEN],
    path_len: usize,
    fs_type: FsType,
    device_id: u32,
    active: bool,
}

#[derive(Copy, Clone)]
pub struct FileDescriptor {
    pub mount_idx: usize,
    pub inode: u64,
    pub offset: u64,
    pub active: bool,
    pub flags: u32, // O_RDONLY, O_WRONLY, etc.
}

pub struct VirtualFileSystem {
    mounts: [MountPoint; MAX_MOUNTS],
    fds: [FileDescriptor; MAX_FDS],
    next_fd: AtomicUsize,
}

impl VirtualFileSystem {
    pub const fn new() -> Self {
        let empty_mount = MountPoint {
            path: [0; MAX_PATH_LEN],
            path_len: 0,
            fs_type: FsType::Unknown,
            device_id: 0,
            active: false,
        };
        let empty_fd = FileDescriptor {
            mount_idx: 0,
            inode: 0,
            offset: 0,
            active: false,
            flags: 0,
        };
        Self {
            mounts: [empty_mount; MAX_MOUNTS],
            fds: [empty_fd; MAX_FDS],
            next_fd: AtomicUsize::new(0),
        }
    }

    pub fn mount(&mut self, path: &[u8], fs_type: FsType, device_id: u32) -> Result<(), &'static str> {
        if path.len() > MAX_PATH_LEN {
            return Err("Path too long");
        }
        
        for m in self.mounts.iter_mut() {
            if !m.active {
                m.path[..path.len()].copy_from_slice(path);
                m.path_len = path.len();
                m.fs_type = fs_type;
                m.device_id = device_id;
                m.active = true;
                return Ok(());
            }
        }
        Err("Too many mounts")
    }

    /// Very basic path matching stub
    pub fn open(&mut self, path: &[u8], flags: u32) -> Result<usize, &'static str> {
        // Find longest mount match (simulated)
        let mut best_mount = None;
        let mut best_len = 0;
        
        for (i, m) in self.mounts.iter().enumerate() {
            if m.active && path.starts_with(&m.path[..m.path_len]) {
                if m.path_len > best_len {
                    best_len = m.path_len;
                    best_mount = Some(i);
                }
            }
        }
        
        let mount_idx = best_mount.ok_or("No mount point found")?;
        
        // Find free FD
        let fd = self.next_fd.fetch_add(1, Ordering::Relaxed) % MAX_FDS;
        if self.fds[fd].active {
            return Err("Out of file descriptors"); // Simplified wrap handling
        }
        
        self.fds[fd] = FileDescriptor {
            mount_idx,
            inode: 1, // Simulated root inode lookup
            offset: 0,
            active: true,
            flags,
        };
        
        Ok(fd)
    }

    pub fn read(&mut self, fd: usize, buf: &mut [u8]) -> Result<usize, &'static str> {
        if fd >= MAX_FDS || !self.fds[fd].active {
            return Err("Invalid file descriptor");
        }
        
        let file = &mut self.fds[fd];
        let mount = &self.mounts[file.mount_idx];
        
        // Dispatch to underlying FS
        match mount.fs_type {
            FsType::Ext4 => self.ext4_read(file, buf),
            FsType::APFS => self.apfs_read(file, buf),
            FsType::Tmpfs => self.tmpfs_read(file, buf),
            FsType::Unknown => Err("Unknown filesystem"),
        }
    }
    
    // --- File System Driver Stubs ---
    
    fn ext4_read(&mut self, file: &mut FileDescriptor, buf: &mut [u8]) -> Result<usize, &'static str> {
        // Simulated ext4 extent parsing
        if buf.len() > 0 {
            buf[0] = b'E';
        }
        file.offset += buf.len() as u64;
        Ok(buf.len())
    }

    fn apfs_read(&mut self, file: &mut FileDescriptor, buf: &mut [u8]) -> Result<usize, &'static str> {
        // Simulated APFS spaceman reading
        if buf.len() > 0 {
            buf[0] = b'A';
        }
        file.offset += buf.len() as u64;
        Ok(buf.len())
    }
    
    fn tmpfs_read(&mut self, file: &mut FileDescriptor, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() > 0 {
            buf[0] = b'T';
        }
        file.offset += buf.len() as u64;
        Ok(buf.len())
    }
}

static mut G_VFS: VirtualFileSystem = VirtualFileSystem::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_vfs_init() {
    // Mount root ext4
    let _ = G_VFS.mount(b"/", FsType::Ext4, 1);
    // Mount mac drive
    let _ = G_VFS.mount(b"/mac", FsType::APFS, 2);
    // Mount tmp
    let _ = G_VFS.mount(b"/tmp", FsType::Tmpfs, 0);
}
