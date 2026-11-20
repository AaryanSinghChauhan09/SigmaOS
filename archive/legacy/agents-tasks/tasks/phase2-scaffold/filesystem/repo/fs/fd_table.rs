// Process-Local File Descriptor Table Management
// Part of SigmaOS File Management Subsystem

extern crate alloc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicI32, Ordering};

use super::fd::{FileDescriptor, FileHandle, FsError, FD_CLOEXEC};

pub const DEFAULT_MAX_FDS: usize = 1024;
pub const STDIN_FILENO: i32 = 0;
pub const STDOUT_FILENO: i32 = 1;
pub const STDERR_FILENO: i32 = 2;

#[derive(Debug)]
pub struct ProcessFdTable {
    entries: Vec<Option<FileDescriptor>>,
    max_fds: usize,
    next_handle_id: AtomicI32,
}

impl ProcessFdTable {
    pub fn new(max_fds: usize) -> Self {
        let max = if max_fds == 0 { DEFAULT_MAX_FDS } else { max_fds };
        let mut entries = Vec::with_capacity(max);
        for _ in 0..max {
            entries.push(None);
        }
        Self {
            entries,
            max_fds: max,
            next_handle_id: AtomicI32::new(100),
        }
    }

    pub fn allocate_fd(&mut self, handle: FileHandle, flags: u32) -> Result<i32, FsError> {
        self.allocate_fd_above(0, handle, flags)
    }

    pub fn allocate_fd_above(&mut self, min_fd: i32, handle: FileHandle, flags: u32) -> Result<i32, FsError> {
        if min_fd < 0 || (min_fd as usize) >= self.max_fds {
            return Err(FsError::InvalidFileDescriptor);
        }
        let min_idx = min_fd as usize;
        for i in min_idx..self.max_fds {
            if self.entries[i].is_none() {
                let descriptor = FileDescriptor::new(i as i32, flags, handle);
                self.entries[i] = Some(descriptor);
                return Ok(i as i32);
            }
        }
        Err(FsError::NoFileDescriptorsAvailable)
    }

    pub fn get_fd(&self, fd: i32) -> Result<&FileDescriptor, FsError> {
        if fd < 0 || (fd as usize) >= self.max_fds {
            return Err(FsError::InvalidFileDescriptor);
        }
        self.entries[fd as usize]
            .as_ref()
            .ok_or(FsError::InvalidFileDescriptor)
    }

    pub fn get_fd_mut(&mut self, fd: i32) -> Result<&mut FileDescriptor, FsError> {
        if fd < 0 || (fd as usize) >= self.max_fds {
            return Err(FsError::InvalidFileDescriptor);
        }
        self.entries[fd as usize]
            .as_mut()
            .ok_or(FsError::InvalidFileDescriptor)
    }

    pub fn close_fd(&mut self, fd: i32) -> Result<FileDescriptor, FsError> {
        if fd < 0 || (fd as usize) >= self.max_fds {
            return Err(FsError::InvalidFileDescriptor);
        }
        self.entries[fd as usize]
            .take()
            .ok_or(FsError::InvalidFileDescriptor)
    }

    pub fn dup_fd(&mut self, old_fd: i32, min_fd: i32, flags: u32) -> Result<i32, FsError> {
        let old_desc = self.get_fd(old_fd)?.clone();
        let new_handle = old_desc.handle.clone();
        self.allocate_fd_above(min_fd, new_handle, flags)
    }

    pub fn dup2_fd(&mut self, old_fd: i32, new_fd: i32) -> Result<i32, FsError> {
        if old_fd < 0 || (old_fd as usize) >= self.max_fds || new_fd < 0 || (new_fd as usize) >= self.max_fds {
            return Err(FsError::InvalidFileDescriptor);
        }
        if old_fd == new_fd {
            let _ = self.get_fd(old_fd)?;
            return Ok(new_fd);
        }
        let old_desc = self.get_fd(old_fd)?.clone();
        let _ = self.close_fd(new_fd);
        let new_desc = FileDescriptor::new(new_fd, 0, old_desc.handle);
        self.entries[new_fd as usize] = Some(new_desc);
        Ok(new_fd)
    }

    pub fn handle_exec(&mut self) {
        for i in 0..self.max_fds {
            if let Some(ref desc) = self.entries[i] {
                if desc.is_cloexec() {
                    self.entries[i] = None;
                }
            }
        }
    }

    pub fn active_fd_count(&self) -> usize {
        self.entries.iter().filter(|e| e.is_some()).count()
    }

    pub fn create_handle(&self, inode_id: u64, flags: u32, initial_offset: u64) -> FileHandle {
        let id = self.next_handle_id.fetch_add(1, Ordering::Relaxed) as u64;
        FileHandle::new(id, inode_id, flags, initial_offset)
    }
}
