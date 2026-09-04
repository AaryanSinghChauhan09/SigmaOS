//! Custom file system implementation for SigmaOS
//! This module provides no_std alternatives to std::fs

use core::ffi::c_char;
use core::fmt;
use std::string::String;
use std::vec::Vec;

/// Raw file descriptor type
pub type RawFd = i32;

/// Error types for file system operations
#[derive(Debug)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    InvalidPath,
    IoError,
    IsDirectory,
    NotADirectory,
    InvalidInput,
}

impl fmt::Display for FsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsError::NotFound => write!(f, "File not found"),
            FsError::PermissionDenied => write!(f, "Permission denied"),
            FsError::AlreadyExists => write!(f, "File already exists"),
            FsError::InvalidPath => write!(f, "Invalid path"),
            FsError::IoError => write!(f, "I/O error"),
            FsError::IsDirectory => write!(f, "Is a directory"),
            FsError::NotADirectory => write!(f, "Not a directory"),
            FsError::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

/// File open modes
pub enum OpenMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    Append,
    Create,
    CreateNew,
    Truncate,
}

/// Seek origin
pub enum SeekFrom {
    Start(u64),
    Current(i64),
    End(i64),
}

/// File metadata
pub struct Metadata {
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub modified: u64,
    pub accessed: u64,
    pub created: u64,
}

/// Custom file implementation
pub struct SigmaFile {
    fd: RawFd,
    path: crate::klib::string::SigmaString,
}

impl SigmaFile {
    /// Open a file with the specified mode
    pub fn open(path: &str, mode: OpenMode) -> Result<Self, FsError> {
        let path_cstr = Self::path_to_cstring(path)?;
        let flags = Self::mode_to_flags(mode);

        let fd = unsafe { syscall_open(path_cstr.as_ptr(), flags, 0o644) };

        if fd < 0 {
            return Err(FsError::IoError);
        }

        Ok(Self {
            fd,
            path: crate::klib::string::SigmaString::from_str(path),
        })
    }

    /// Read data from the file
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, FsError> {
        let result = unsafe { syscall_read(self.fd, buffer.as_mut_ptr(), buffer.len()) };

        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(result as usize)
        }
    }

    /// Write data to the file
    pub fn write(&mut self, data: &[u8]) -> Result<usize, FsError> {
        let result = unsafe { syscall_write(self.fd, data.as_ptr(), data.len()) };

        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(result as usize)
        }
    }

    /// Seek within the file
    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64, FsError> {
        let (whence, offset) = match pos {
            SeekFrom::Start(offset) => (0, offset as i64),
            SeekFrom::Current(offset) => (1, offset),
            SeekFrom::End(offset) => (2, offset),
        };

        let result = unsafe { syscall_lseek(self.fd, offset, whence) };

        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(result as u64)
        }
    }

    /// Flush the file buffer
    pub fn flush(&mut self) -> Result<(), FsError> {
        let result = unsafe { syscall_fsync(self.fd) };

        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(())
        }
    }

    /// Get file metadata
    pub fn metadata(&self) -> Result<Metadata, FsError> {
        let mut stat = Stat::default();
        let result = unsafe { syscall_fstat(self.fd, &mut stat) };

        if result < 0 {
            return Err(FsError::IoError);
        }

        Ok(Metadata {
            size: stat.st_size,
            is_file: stat.is_file(),
            is_directory: stat.is_directory(),
            modified: stat.modified,
            accessed: stat.accessed,
            created: stat.created,
        })
    }

    /// Close the file
    pub fn close(self) -> Result<(), FsError> {
        let result = unsafe { syscall_close(self.fd) };

        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(())
        }
    }

    /// Convert open mode to flags
    fn mode_to_flags(mode: OpenMode) -> u32 {
        match mode {
            OpenMode::ReadOnly => O_RDONLY,
            OpenMode::WriteOnly => O_WRONLY,
            OpenMode::ReadWrite => O_RDWR,
            OpenMode::Append => O_WRONLY | O_APPEND,
            OpenMode::Create => O_CREAT | O_WRONLY | O_TRUNC,
            OpenMode::CreateNew => O_CREAT | O_WRONLY | O_EXCL,
            OpenMode::Truncate => O_WRONLY | O_TRUNC,
        }
    }

    /// Convert path to C string
    fn path_to_cstring(path: &str) -> Result<[u8; 256], FsError> {
        let mut cstr = [0u8; 256];
        let bytes = path.as_bytes();

        if bytes.len() >= 256 {
            return Err(FsError::InvalidPath);
        }

        for (i, &byte) in bytes.iter().enumerate() {
            cstr[i] = byte;
        }

        Ok(cstr)
    }
}

impl Drop for SigmaFile {
    fn drop(&mut self) {
        let _ = unsafe { syscall_close(self.fd) };
    }
}

/// File statistics structure
#[repr(C)]
#[derive(Default)]
struct Stat {
    st_dev: u64,
    st_ino: u64,
    st_mode: u32,
    st_nlink: u32,
    st_uid: u32,
    st_gid: u32,
    st_rdev: u64,
    st_size: u64,
    st_blksize: u64,
    st_blocks: u64,
    accessed: u64,
    modified: u64,
    created: u64,
}

impl Stat {
    fn is_file(&self) -> bool {
        (self.st_mode & 0o170000) == 0o100000
    }

    fn is_directory(&self) -> bool {
        (self.st_mode & 0o170000) == 0o040000
    }
}

/// Directory entry
pub struct DirEntry {
    pub name: String,
    pub is_file: bool,
    pub is_directory: bool,
}

/// Directory reader
pub struct SigmaDir {
    fd: RawFd,
    path: crate::klib::string::SigmaString,
}

impl SigmaDir {
    /// Open a directory
    pub fn open(path: &str) -> Result<Self, FsError> {
        let path_cstr = SigmaFile::path_to_cstring(path)?;
        let fd = unsafe { syscall_opendir(path_cstr.as_ptr()) };

        if fd < 0 {
            return Err(FsError::IoError);
        }

        Ok(Self {
            fd,
            path: crate::klib::string::SigmaString::from_str(path),
        })
    }

    /// Read directory entries
    pub fn read(&mut self) -> Result<Vec<DirEntry>, FsError> {
        let mut entries = Vec::new();
        let mut entry = DirEntryRaw::default();

        loop {
            let result = unsafe { syscall_readdir(self.fd, &mut entry) };

            if result < 0 {
                break;
            }

            let name = unsafe { Self::cstr_to_string(entry.d_name.as_ptr()) };

            let is_file = entry.d_type == 8; // DT_REG
            let is_directory = entry.d_type == 4; // DT_DIR

            entries.push(DirEntry {
                name,
                is_file,
                is_directory,
            });
        }

        Ok(entries)
    }

    /// Close the directory
    pub fn close(self) -> Result<(), FsError> {
        let result = unsafe { syscall_closedir(self.fd) };

        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(())
        }
    }

    /// Convert C string to Rust string
    unsafe fn cstr_to_string(ptr: *const c_char) -> String {
        let mut len = 0;
        loop {
            if *ptr.add(len) == 0 {
                let bytes = core::slice::from_raw_parts(ptr as *const u8, len);
                return String::from_utf8_unchecked(bytes.iter().map(|&b| b).collect());
            }
            len += 1;
        }
    }
}

impl Drop for SigmaDir {
    fn drop(&mut self) {
        let _ = unsafe { syscall_closedir(self.fd) };
    }
}

/// Sovereign C-FFI Bridge providing POSIX/C string and buffer conversion helpers
pub struct SovereignCffiBridge;

impl SovereignCffiBridge {
    pub unsafe fn cstr_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
        if ptr.is_null() {
            return None;
        }
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        let bytes = core::slice::from_raw_parts(ptr as *const u8, len);
        core::str::from_utf8(bytes).ok()
    }

    pub fn str_to_c_buffer(s: &str, buf: &mut [c_char]) -> bool {
        let bytes = s.as_bytes();
        if bytes.len() >= buf.len() {
            return false;
        }
        for (i, &b) in bytes.iter().enumerate() {
            buf[i] = b as c_char;
        }
        buf[bytes.len()] = 0;
        true
    }
}

/// Raw directory entry
#[repr(C)]
struct DirEntryRaw {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

impl Default for DirEntryRaw {
    fn default() -> Self {
        Self {
            d_ino: 0,
            d_off: 0,
            d_reclen: 0,
            d_type: 0,
            d_name: [0; 256],
        }
    }
}

// Syscall flags
const O_RDONLY: u32 = 0o000000;
const O_WRONLY: u32 = 0o000001;
const O_RDWR: u32 = 0o000002;
const O_CREAT: u32 = 0o000100;
const O_EXCL: u32 = 0o000200;
const O_TRUNC: u32 = 0o001000;
const O_APPEND: u32 = 0o002000;

// Syscall functions (platform-specific)
#[inline(always)]
unsafe fn syscall_open(path: *const u8, flags: u32, mode: u32) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: i32;
        core::arch::asm!(
            "syscall",
            inout("eax") 2i32 => ret,
            in("rdi") path,
            in("rsi") flags,
            in("rdx") mode,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (path, flags, mode);
        -1
    }
}

#[inline(always)]
unsafe fn syscall_read(fd: RawFd, buf: *mut u8, count: usize) -> isize {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: isize;
        core::arch::asm!(
            "syscall",
            inout("rax") 0isize => ret,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (fd, buf, count);
        -1
    }
}

#[inline(always)]
unsafe fn syscall_write(fd: RawFd, buf: *const u8, count: usize) -> isize {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: isize;
        core::arch::asm!(
            "syscall",
            inout("rax") 1isize => ret,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (fd, buf, count);
        -1
    }
}

#[inline(always)]
unsafe fn syscall_lseek(fd: RawFd, offset: i64, whence: i32) -> i64 {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: i64;
        core::arch::asm!(
            "syscall",
            inout("rax") 8isize => ret,
            in("rdi") fd,
            in("rsi") offset,
            in("rdx") whence,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (fd, offset, whence);
        -1
    }
}

#[inline(always)]
unsafe fn syscall_fsync(fd: RawFd) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: i32;
        core::arch::asm!(
            "syscall",
            inout("eax") 74i32 => ret,
            in("rdi") fd,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = fd;
        -1
    }
}

#[inline(always)]
unsafe fn syscall_close(fd: RawFd) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: i32;
        core::arch::asm!(
            "syscall",
            inout("eax") 3i32 => ret,
            in("rdi") fd,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = fd;
        -1
    }
}

#[inline(always)]
unsafe fn syscall_fstat(fd: RawFd, stat: *mut Stat) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: i32;
        core::arch::asm!(
            "syscall",
            inout("eax") 5i32 => ret,
            in("rdi") fd,
            in("rsi") stat,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (fd, stat);
        -1
    }
}

#[inline(always)]
unsafe fn syscall_opendir(path: *const u8) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: i32;
        core::arch::asm!(
            "syscall",
            inout("eax") 78i32 => ret,
            in("rdi") path,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = path;
        -1
    }
}

#[inline(always)]
unsafe fn syscall_readdir(fd: RawFd, entry: *mut DirEntryRaw) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: i32;
        core::arch::asm!(
            "syscall",
            inout("eax") 79i32 => ret,
            in("rdi") fd,
            in("rsi") entry,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (fd, entry);
        -1
    }
}

#[inline(always)]
unsafe fn syscall_closedir(fd: RawFd) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: i32;
        core::arch::asm!(
            "syscall",
            inout("eax") 80i32 => ret,
            in("rdi") fd,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = fd;
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_cstring() {
        let result = SigmaFile::path_to_cstring("test.txt");
        assert!(result.is_ok());

        let cstr = result.unwrap();
        assert_eq!(cstr[0], b't');
        assert_eq!(cstr[1], b'e');
        assert_eq!(cstr[2], b's');
        assert_eq!(cstr[3], b't');
        assert_eq!(cstr[4], b'.');
        assert_eq!(cstr[5], b't');
        assert_eq!(cstr[6], b'x');
        assert_eq!(cstr[7], b't');
        assert_eq!(cstr[8], 0);
    }

    #[test]
    fn test_mode_to_flags() {
        assert_eq!(SigmaFile::mode_to_flags(OpenMode::ReadOnly), O_RDONLY);
        assert_eq!(SigmaFile::mode_to_flags(OpenMode::WriteOnly), O_WRONLY);
        assert_eq!(SigmaFile::mode_to_flags(OpenMode::ReadWrite), O_RDWR);
        assert_eq!(
            SigmaFile::mode_to_flags(OpenMode::Append),
            O_WRONLY | O_APPEND
        );
        assert_eq!(
            SigmaFile::mode_to_flags(OpenMode::Create),
            O_CREAT | O_WRONLY | O_TRUNC
        );
    }

    #[test]
    fn test_stat_types() {
        let mut stat = Stat::default();
        stat.st_mode = 0o100644; // Regular file
        assert!(stat.is_file());
        assert!(!stat.is_directory());

        stat.st_mode = 0o040755; // Directory
        assert!(!stat.is_file());
        assert!(stat.is_directory());
    }

    #[test]
    fn test_sovereign_cffi_bridge() {
        let mut buf = [0 as c_char; 16];
        assert!(SovereignCffiBridge::str_to_c_buffer("hello", &mut buf));
        let converted = unsafe { SovereignCffiBridge::cstr_to_str(buf.as_ptr()) };
        assert_eq!(converted, Some("hello"));
    }
}
