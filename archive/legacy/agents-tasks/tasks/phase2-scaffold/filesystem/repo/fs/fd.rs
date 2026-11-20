// File Descriptor and File Handle Representation
// Part of SigmaOS File Management Subsystem

extern crate alloc;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};

pub const FD_CLOEXEC: u32 = 0x01;

pub const O_RDONLY: u32 = 0x0000;
pub const O_WRONLY: u32 = 0x0001;
pub const O_RDWR: u32 = 0x0002;
pub const O_ACCMODE: u32 = 0x0003;
pub const O_CREAT: u32 = 0x0040;
pub const O_EXCL: u32 = 0x0080;
pub const O_TRUNC: u32 = 0x0200;
pub const O_APPEND: u32 = 0x0400;
pub const O_NONBLOCK: u32 = 0x0800;
pub const O_DIRECTORY: u32 = 0x10000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    InvalidFileDescriptor,
    NoFileDescriptorsAvailable,
    FileAlreadyExists,
    IsADirectory,
    NotADirectory,
    DirectoryNotEmpty,
    ResourceBusy,
    StorageFull,
    OutOfMemory,
    IoError,
    BadOffset,
    InvalidOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekFrom {
    Start(u64),
    Current(i64),
    End(i64),
}

#[derive(Debug, Clone)]
pub struct FileHandle {
    pub handle_id: u64,
    pub inode_id: u64,
    pub open_flags: u32,
    pub offset: Arc<AtomicU64>,
}

impl FileHandle {
    pub fn new(handle_id: u64, inode_id: u64, open_flags: u32, initial_offset: u64) -> Self {
        Self {
            handle_id,
            inode_id,
            open_flags,
            offset: Arc::new(AtomicU64::new(initial_offset)),
        }
    }

    pub fn get_offset(&self) -> u64 {
        self.offset.load(Ordering::Relaxed)
    }

    pub fn set_offset(&self, val: u64) {
        self.offset.store(val, Ordering::Relaxed);
    }

    pub fn seek(&self, pos: SeekFrom, file_size: u64) -> Result<u64, FsError> {
        let current = self.get_offset();
        let new_offset = match pos {
            SeekFrom::Start(off) => off,
            SeekFrom::Current(off) => {
                if off >= 0 {
                    current.checked_add(off as u64).ok_or(FsError::BadOffset)?
                } else {
                    let neg = (-off) as u64;
                    current.checked_sub(neg).ok_or(FsError::BadOffset)?
                }
            }
            SeekFrom::End(off) => {
                if off >= 0 {
                    file_size.checked_add(off as u64).ok_or(FsError::BadOffset)?
                } else {
                    let neg = (-off) as u64;
                    file_size.checked_sub(neg).ok_or(FsError::BadOffset)?
                }
            }
        };
        self.set_offset(new_offset);
        Ok(new_offset)
    }
}

#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub fd: i32,
    pub flags: u32,
    pub handle: FileHandle,
}

impl FileDescriptor {
    pub fn new(fd: i32, flags: u32, handle: FileHandle) -> Self {
        Self { fd, flags, handle }
    }

    pub fn is_cloexec(&self) -> bool {
        (self.flags & FD_CLOEXEC) != 0
    }

    pub fn set_cloexec(&mut self, cloexec: bool) {
        if cloexec {
            self.flags |= FD_CLOEXEC;
        } else {
            self.flags &= !FD_CLOEXEC;
        }
    }
}
