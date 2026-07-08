// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// posix/posix_file.rs — POSIX File I/O Primitives
//
// Implements POSIX file I/O primitives: open, read, write, close, lseek, stat, fstat, mkdir, rmdir, unlink
// Maps to SigmaOS's object-oriented storage model.
//
// Language: Rust (no_std for kernel compatibility)

#![no_std]

use super::posix_base::{
    PosixFileDescriptor, FileDescriptorTable, FD_TABLE,
    set_errno_and_return, clear_errno, EPERM, ENOENT, EACCES, EISDIR, ENOTDIR, ENOEXEC,
    EBADF, EINVAL, ENOMEM, EEXIST, ENOSPC, EROFS, EIO,
    O_RDONLY, O_WRONLY, O_RDWR, O_CREAT, O_EXCL, O_TRUNC, O_APPEND, O_SYNC,
};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type I64 = i64;
type Isize = isize;
type Usize = usize;

// ─── File Mode Bits ─────────────────────────────────────

pub const S_IRUSR: U32 = 0o400;
pub const S_IWUSR: U32 = 0o200;
pub const S_IXUSR: U32 = 0o100;
pub const S_IRGRP: U32 = 0o040;
pub const S_IWGRP: U32 = 0o020;
pub const S_IXGRP: U32 = 0o010;
pub const S_IROTH: U32 = 0o004;
pub const S_IWOTH: U32 = 0o002;
pub const S_IXOTH: U32 = 0o001;
pub const S_IFREG: U32 = 0o100000;
pub const S_IFDIR: U32 = 0o040000;
pub const S_IFCHR: U32 = 0o020000;
pub const S_IFBLK: U32 = 0o060000;
pub const S_IFIFO: U32 = 0o010000;
pub const S_IFLNK: U32 = 0o120000;
pub const S_IFSOCK: U32 = 0o140000;

// ─── File Stat Structure ─────────────────────────────────

#[repr(C)]
pub struct Stat {
    pub st_dev: U64,
    pub st_ino: U64,
    pub st_mode: U32,
    pub st_nlink: U32,
    pub st_uid: U32,
    pub st_gid: U32,
    pub st_rdev: U64,
    pub st_size: I64,
    pub st_blksize: I32,
    pub st_blocks: I64,
    pub st_atime: I64,
    pub st_mtime: I64,
    pub st_ctime: I64,
}

impl Stat {
    pub const fn new() -> Self {
        Stat {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 4096,
            st_blocks: 0,
            st_atime: 0,
            st_mtime: 0,
            st_ctime: 0,
        }
    }
}

// ─── Seek Origin ─────────────────────────────────────────

pub const SEEK_SET: I32 = 0;
pub const SEEK_CUR: I32 = 1;
pub const SEEK_END: I32 = 2;

// ─── SigmaOS File Operations (stubs) ───────────────────

// These would call into SigmaOS's storage driver
// For now, we provide stub implementations

unsafe fn sigma_file_open(path: *const U8, flags: U32, mode: U32) -> Result<U64, I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(1) // Return a handle
}

unsafe fn sigma_file_close(handle: U64) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(())
}

unsafe fn sigma_file_read(handle: U64, buffer: *mut U8, count: Usize) -> Result<Isize, I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(0)
}

unsafe fn sigma_file_write(handle: U64, buffer: *const U8, count: Usize) -> Result<Isize, I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(count as Isize)
}

unsafe fn sigma_file_seek(handle: U64, offset: I64, whence: I32) -> Result<I64, I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(0)
}

unsafe fn sigma_file_stat(path: *const U8, stat: *mut Stat) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(())
}

unsafe fn sigma_file_fstat(handle: U64, stat: *mut Stat) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(())
}

unsafe fn sigma_mkdir(path: *const U8, mode: U32) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(())
}

unsafe fn sigma_rmdir(path: *const U8) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(())
}

unsafe fn sigma_unlink(path: *const U8) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS storage driver
    Ok(())
}

// ─── POSIX open() ───────────────────────────────────────

/// Open a file
#[no_mangle]
pub unsafe extern "C" fn posix_open(path: *const U8, flags: I32, mode: U32) -> I32 {
    clear_errno();

    if path.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Convert flags and mode
    let sigma_flags = flags as U32;
    let sigma_mode = mode;

    // Call SigmaOS file open
    match sigma_file_open(path, sigma_flags, sigma_mode) {
        Ok(handle) => {
            // Allocate file descriptor
            let fd_table = &mut FD_TABLE;
            let fd = fd_table.allocate_fd();
            
            if fd < 0 {
                sigma_file_close(handle).ok();
                return fd;
            }

            // Set up file descriptor
            if let Some(pfd) = fd_table.get_fd(fd) {
                pfd.fd = fd;
                pfd.sigma_handle.handle = handle;
                pfd.flags = sigma_flags;
                pfd.mode = sigma_mode;
                pfd.is_open = true;
            }

            fd
        }
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX close() ──────────────────────────────────────

/// Close a file descriptor
#[no_mangle]
pub unsafe extern "C" fn posix_close(fd: I32) -> I32 {
    clear_errno();

    let fd_table = &mut FD_TABLE;
    
    if let Some(pfd) = fd_table.get_fd(fd) {
        if !pfd.is_open {
            return set_errno_and_return(EBADF);
        }

        // Close SigmaOS handle
        sigma_file_close(pfd.sigma_handle.handle).ok();
        
        // Free file descriptor
        fd_table.free_fd(fd);
        
        0
    } else {
        set_errno_and_return(EBADF)
    }
}

// ─── POSIX read() ──────────────────────────────────────

/// Read from a file descriptor
#[no_mangle]
pub unsafe extern "C" fn posix_read(fd: I32, buffer: *mut U8, count: Usize) -> Isize {
    clear_errno();

    if buffer.is_null() || count == 0 {
        return set_errno_and_return(EFAULT) as Isize;
    }

    let fd_table = &mut FD_TABLE;
    
    if let Some(pfd) = fd_table.get_fd(fd) {
        if !pfd.is_open {
            return set_errno_and_return(EBADF) as Isize;
        }

        // Check if file is opened for reading
        if pfd.flags & (O_RDONLY as U32 | O_RDWR as U32) == 0 {
            return set_errno_and_return(EACCES) as Isize;
        }

        // Call SigmaOS file read
        match sigma_file_read(pfd.sigma_handle.handle, buffer, count) {
            Ok(bytes_read) => bytes_read,
            Err(e) => set_errno_and_return(e) as Isize,
        }
    } else {
        set_errno_and_return(EBADF) as Isize
    }
}

// ─── POSIX write() ─────────────────────────────────────

/// Write to a file descriptor
#[no_mangle]
pub unsafe extern "C" fn posix_write(fd: I32, buffer: *const U8, count: Usize) -> Isize {
    clear_errno();

    if buffer.is_null() || count == 0 {
        return set_errno_and_return(EFAULT) as Isize;
    }

    let fd_table = &mut FD_TABLE;
    
    if let Some(pfd) = fd_table.get_fd(fd) {
        if !pfd.is_open {
            return set_errno_and_return(EBADF) as Isize;
        }

        // Check if file is opened for writing
        if pfd.flags & (O_WRONLY as U32 | O_RDWR as U32) == 0 {
            return set_errno_and_return(EACCES) as Isize;
        }

        // Call SigmaOS file write
        match sigma_file_write(pfd.sigma_handle.handle, buffer, count) {
            Ok(bytes_written) => bytes_written,
            Err(e) => set_errno_and_return(e) as Isize,
        }
    } else {
        set_errno_and_return(EBADF) as Isize
    }
}

// ─── POSIX lseek() ─────────────────────────────────────

/// Seek within a file
#[no_mangle]
pub unsafe extern "C" fn posix_lseek(fd: I32, offset: I64, whence: I32) -> I64 {
    clear_errno();

    let fd_table = &mut FD_TABLE;
    
    if let Some(pfd) = fd_table.get_fd(fd) {
        if !pfd.is_open {
            return set_errno_and_return(EBADF) as I64;
        }

        // Validate whence
        if whence != SEEK_SET && whence != SEEK_CUR && whence != SEEK_END {
            return set_errno_and_return(EINVAL) as I64;
        }

        // Call SigmaOS file seek
        match sigma_file_seek(pfd.sigma_handle.handle, offset, whence) {
            Ok(new_offset) => new_offset,
            Err(e) => set_errno_and_return(e) as I64,
        }
    } else {
        set_errno_and_return(EBADF) as I64
    }
}

// ─── POSIX stat() ───────────────────────────────────────

/// Get file status
#[no_mangle]
pub unsafe extern "C" fn posix_stat(path: *const U8, stat: *mut Stat) -> I32 {
    clear_errno();

    if path.is_null() || stat.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Call SigmaOS file stat
    match sigma_file_stat(path, stat) {
        Ok(()) => 0,
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX fstat() ──────────────────────────────────────

/// Get file status from file descriptor
#[no_mangle]
pub unsafe extern "C" fn posix_fstat(fd: I32, stat: *mut Stat) -> I32 {
    clear_errno();

    if stat.is_null() {
        return set_errno_and_return(EFAULT);
    }

    let fd_table = &mut FD_TABLE;
    
    if let Some(pfd) = fd_table.get_fd(fd) {
        if !pfd.is_open {
            return set_errno_and_return(EBADF);
        }

        // Call SigmaOS file fstat
        match sigma_file_fstat(pfd.sigma_handle.handle, stat) {
            Ok(()) => 0,
            Err(e) => set_errno_and_return(e),
        }
    } else {
        set_errno_and_return(EBADF)
    }
}

// ─── POSIX mkdir() ──────────────────────────────────────

/// Create a directory
#[no_mangle]
pub unsafe extern "C" fn posix_mkdir(path: *const U8, mode: U32) -> I32 {
    clear_errno();

    if path.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Call SigmaOS mkdir
    match sigma_mkdir(path, mode) {
        Ok(()) => 0,
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX rmdir() ──────────────────────────────────────

/// Remove a directory
#[no_mangle]
pub unsafe extern "C" fn posix_rmdir(path: *const U8) -> I32 {
    clear_errno();

    if path.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Call SigmaOS rmdir
    match sigma_rmdir(path) {
        Ok(()) => 0,
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX unlink() ──────────────────────────────────────

/// Remove a file
#[no_mangle]
pub unsafe extern "C" fn posix_unlink(path: *const U8) -> I32 {
    clear_errno();

    if path.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Call SigmaOS unlink
    match sigma_unlink(path) {
        Ok(()) => 0,
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX access() ─────────────────────────────────────

/// Check file accessibility
#[no_mangle]
pub unsafe extern "C" fn posix_access(path: *const U8, mode: I32) -> I32 {
    clear_errno();

    if path.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Stub: In real implementation, check file accessibility
    0
}
