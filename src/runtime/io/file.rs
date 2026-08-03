#![no_std]
#![no_main]

/// Custom File I/O for SigmaOS
/// Implements file operations without relying on std::fs
/// Uses capability-based access control

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};

/// File descriptor structure
#[repr(C)]
pub struct FileDescriptor {
    pub fd: AtomicUsize,
    pub flags: FileFlags,
    pub offset: AtomicUsize,
    pub capability: Capability,
}

/// File flags
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FileFlags {
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub create: bool,
    pub truncate: bool,
}

impl FileFlags {
    pub fn new() -> Self {
        FileFlags {
            read: false,
            write: false,
            append: false,
            create: false,
            truncate: false,
        }
    }

    pub fn read_only() -> Self {
        let mut flags = FileFlags::new();
        flags.read = true;
        flags
    }

    pub fn write_only() -> Self {
        let mut flags = FileFlags::new();
        flags.write = true;
        flags
    }

    pub fn read_write() -> Self {
        let mut flags = FileFlags::new();
        flags.read = true;
        flags.write = true;
        flags
    }
}

/// Capability for file access
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Capability {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Capability {
    pub fn new() -> Self {
        Capability {
            read: false,
            write: false,
            execute: false,
        }
    }

    pub fn full() -> Self {
        Capability {
            read: true,
            write: true,
            execute: true,
        }
    }
}

/// File open modes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OpenMode {
    ReadOnly = 0,
    WriteOnly = 1,
    ReadWrite = 2,
    Append = 3,
    Create = 4,
    Truncate = 5,
}

/// File seek origin
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SeekOrigin {
    Set = 0,
    Current = 1,
    End = 2,
}

/// File I/O manager
pub struct FileManager {
    pub next_fd: AtomicUsize,
    pub max_fds: usize,
    pub open_files: [Option<NonNull<FileDescriptor>>; 1024],
}

impl FileManager {
    pub fn new() -> Self {
        let mut open_files: [Option<NonNull<FileDescriptor>>; 1024] = [None; 1024];

        // Standard Input (FD 0)
        let stdin_desc = FileDescriptor {
            fd: AtomicUsize::new(0),
            flags: FileFlags { read: true, write: false, append: false, create: false, truncate: false },
            offset: AtomicUsize::new(0),
            capability: Capability::full(),
        };

        // Standard Output (FD 1)
        let stdout_desc = FileDescriptor {
            fd: AtomicUsize::new(1),
            flags: FileFlags { read: false, write: true, append: false, create: false, truncate: false },
            offset: AtomicUsize::new(0),
            capability: Capability::full(),
        };

        // Standard Error (FD 2)
        let stderr_desc = FileDescriptor {
            fd: AtomicUsize::new(2),
            flags: FileFlags { read: false, write: true, append: false, create: false, truncate: false },
            offset: AtomicUsize::new(0),
            capability: Capability::full(),
        };

        unsafe {
            let stdin_ptr = alloc(core::mem::size_of::<FileDescriptor>()) as *mut FileDescriptor;
            if !stdin_ptr.is_null() {
                ptr::write(stdin_ptr, stdin_desc);
                open_files[0] = Some(NonNull::new_unchecked(stdin_ptr));
            }
            let stdout_ptr = alloc(core::mem::size_of::<FileDescriptor>()) as *mut FileDescriptor;
            if !stdout_ptr.is_null() {
                ptr::write(stdout_ptr, stdout_desc);
                open_files[1] = Some(NonNull::new_unchecked(stdout_ptr));
            }
            let stderr_ptr = alloc(core::mem::size_of::<FileDescriptor>()) as *mut FileDescriptor;
            if !stderr_ptr.is_null() {
                ptr::write(stderr_ptr, stderr_desc);
                open_files[2] = Some(NonNull::new_unchecked(stderr_ptr));
            }
        }

        FileManager {
            next_fd: AtomicUsize::new(3), // Start after stdin, stdout, stderr
            max_fds: 1024,
            open_files,
        }
    }

    /// Dynamically redirects standard streams to other open file descriptors (dup2 equivalent in Linux)
    pub unsafe fn redirect_stream(&mut self, src_fd: usize, dst_fd: usize) -> Result<(), ()> {
        if src_fd >= self.max_fds || dst_fd >= self.max_fds {
            return Err(());
        }

        if src_fd == dst_fd {
            return Ok(());
        }

        // Retrieve source file descriptor pointer
        let src_ptr_opt = self.open_files[src_fd];
        if src_ptr_opt.is_none() {
            return Err(());
        }
        let src_ptr = src_ptr_opt.unwrap();

        // Close dst_fd if currently open
        if let Some(dst_ptr) = self.open_files[dst_fd] {
            ptr::drop_in_place(dst_ptr.as_ptr());
            free(dst_ptr.as_ptr() as *mut u8);
            self.open_files[dst_fd] = None;
        }

        // Allocate a new copy of FileDescriptor for dst_fd
        let new_desc = FileDescriptor {
            fd: AtomicUsize::new(dst_fd),
            flags: (*src_ptr.as_ptr()).flags,
            offset: AtomicUsize::new((*src_ptr.as_ptr()).offset.load(Ordering::SeqCst)),
            capability: (*src_ptr.as_ptr()).capability,
        };

        let dst_ptr_new = alloc(core::mem::size_of::<FileDescriptor>()) as *mut FileDescriptor;
        if dst_ptr_new.is_null() {
            return Err(());
        }

        ptr::write(dst_ptr_new, new_desc);
        self.open_files[dst_fd] = Some(NonNull::new_unchecked(dst_ptr_new));

        Ok(())
    }

    /// Open a file
    pub unsafe fn open(&mut self, path: &[u8], mode: OpenMode) -> Option<FileDescriptor> {
        let flags = match mode {
            OpenMode::ReadOnly => FileFlags::read_only(),
            OpenMode::WriteOnly => FileFlags::write_only(),
            OpenMode::ReadWrite => FileFlags::read_write(),
            OpenMode::Append => {
                let mut flags = FileFlags::write_only();
                flags.append = true;
                flags
            }
            OpenMode::Create => {
                let mut flags = FileFlags::read_write();
                flags.create = true;
                flags
            }
            OpenMode::Truncate => {
                let mut flags = FileFlags::read_write();
                flags.truncate = true;
                flags
            }
        };

        // Check capability
        let capability = self.check_capability(path);
        if (flags.read && !capability.read) || (flags.write && !capability.write) {
            return None;
        }

        // Allocate file descriptor
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
        if fd >= self.max_fds {
            return None;
        }

        let file_descriptor = FileDescriptor {
            fd: AtomicUsize::new(fd),
            flags,
            offset: AtomicUsize::new(0),
            capability,
        };

        // Store file descriptor
        let fd_ptr = alloc(core::mem::size_of::<FileDescriptor>()) as *mut FileDescriptor;
        if fd_ptr.is_null() {
            return None;
        }

        ptr::write(fd_ptr, file_descriptor);
        self.open_files[fd] = Some(NonNull::new_unchecked(fd_ptr));

        Some(file_descriptor)
    }

    /// Close a file
    pub unsafe fn close(&mut self, fd: FileDescriptor) -> bool {
        let fd_num = fd.fd.load(Ordering::SeqCst);
        if fd_num >= self.max_fds {
            return false;
        }

        if let Some(fd_ptr) = self.open_files[fd_num] {
            ptr::drop_in_place(fd_ptr.as_ptr());
            free(fd_ptr.as_ptr() as *mut u8);
            self.open_files[fd_num] = None;
            true
        } else {
            false
        }
    }

    /// Read from file
    pub unsafe fn read(&self, fd: &FileDescriptor, buffer: *mut u8, size: usize) -> isize {
        if !fd.flags.read {
            return -1;
        }

        let offset = fd.offset.load(Ordering::SeqCst);
        let bytes_read = self.sys_read(fd.fd.load(Ordering::SeqCst), buffer, size, offset);
        
        if bytes_read > 0 {
            fd.offset.fetch_add(bytes_read as usize, Ordering::SeqCst);
        }

        bytes_read
    }

    /// Write to file
    pub unsafe fn write(&self, fd: &FileDescriptor, buffer: *const u8, size: usize) -> isize {
        if !fd.flags.write {
            return -1;
        }

        let offset = if fd.flags.append {
            self.sys_get_size(fd.fd.load(Ordering::SeqCst))
        } else {
            fd.offset.load(Ordering::SeqCst)
        };

        let bytes_written = self.sys_write(fd.fd.load(Ordering::SeqCst), buffer, size, offset);
        
        if bytes_written > 0 {
            fd.offset.fetch_add(bytes_written as usize, Ordering::SeqCst);
        }

        bytes_written
    }

    /// Seek in file
    pub unsafe fn seek(&self, fd: &FileDescriptor, offset: isize, origin: SeekOrigin) -> isize {
        let current_offset = fd.offset.load(Ordering::SeqCst) as isize;
        let file_size = self.sys_get_size(fd.fd.load(Ordering::SeqCst)) as isize;

        let new_offset = match origin {
            SeekOrigin::Set => offset,
            SeekOrigin::Current => current_offset + offset,
            SeekOrigin::End => file_size + offset,
        };

        if new_offset < 0 || new_offset > file_size {
            return -1;
        }

        fd.offset.store(new_offset as usize, Ordering::SeqCst);
        new_offset
    }

    /// Get file size
    pub unsafe fn get_size(&self, fd: &FileDescriptor) -> isize {
        self.sys_get_size(fd.fd.load(Ordering::SeqCst))
    }

    /// Check capability for file access
    fn check_capability(&self, _path: &[u8]) -> Capability {
        // In a real implementation, this would check against the capability system
        // For now, return full capability
        Capability::full()
    }

    /// System call for reading (placeholder)
    unsafe fn sys_read(&self, fd: usize, buffer: *mut u8, size: usize, _offset: usize) -> isize {
        if fd == 0 {
            // Mock stdin read: fill buffer with space and return 1
            if !buffer.is_null() && size > 0 {
                *buffer = b' ';
                return 1;
            }
            return 0;
        }
        0
    }

    /// System call for writing (placeholder)
    unsafe fn sys_write(&self, fd: usize, _buffer: *const u8, size: usize, _offset: usize) -> isize {
        if fd == 1 || fd == 2 {
            // Mock stdout / stderr write: simulate console print by returning written size
            return size as isize;
        }
        0
    }

    /// System call for getting file size (placeholder)
    unsafe fn sys_get_size(&self, _fd: usize) -> isize {
        // In a real implementation, this would make a syscall to get file size
        // For now, return 0
        0
    }
}

/// Global file manager
static mut GLOBAL_FILE_MANAGER: Option<FileManager> = None;

/// Initialize file manager
pub unsafe fn init_file_manager() {
    GLOBAL_FILE_MANAGER = Some(FileManager::new());
}

/// Open a file
pub unsafe fn open(path: &[u8], mode: OpenMode) -> Option<FileDescriptor> {
    if let Some(ref mut manager) = GLOBAL_FILE_MANAGER {
        manager.open(path, mode)
    } else {
        None
    }
}

/// Close a file
pub unsafe fn close(fd: FileDescriptor) -> bool {
    if let Some(ref mut manager) = GLOBAL_FILE_MANAGER {
        manager.close(fd)
    } else {
        false
    }
}

/// Read from file
pub unsafe fn read(fd: &FileDescriptor, buffer: *mut u8, size: usize) -> isize {
    if let Some(ref manager) = GLOBAL_FILE_MANAGER {
        manager.read(fd, buffer, size)
    } else {
        -1
    }
}

/// Write to file
pub unsafe fn write(fd: &FileDescriptor, buffer: *const u8, size: usize) -> isize {
    if let Some(ref manager) = GLOBAL_FILE_MANAGER {
        manager.write(fd, buffer, size)
    } else {
        -1
    }
}

/// Seek in file
pub unsafe fn seek(fd: &FileDescriptor, offset: isize, origin: SeekOrigin) -> isize {
    if let Some(ref manager) = GLOBAL_FILE_MANAGER {
        manager.seek(fd, offset, origin)
    } else {
        -1
    }
}

/// Get file size
pub unsafe fn get_size(fd: &FileDescriptor) -> isize {
    if let Some(ref manager) = GLOBAL_FILE_MANAGER {
        manager.get_size(fd)
    } else {
        -1
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_streams_initialization() {
        let manager = FileManager::new();

        // Assert FDs 0, 1, and 2 are pre-allocated standard streams
        assert!(manager.open_files[0].is_some());
        assert!(manager.open_files[1].is_some());
        assert!(manager.open_files[2].is_some());

        unsafe {
            let stdin_fd = manager.open_files[0].unwrap();
            let stdout_fd = manager.open_files[1].unwrap();
            let stderr_fd = manager.open_files[2].unwrap();

            assert_eq!((*stdin_fd.as_ptr()).fd.load(Ordering::SeqCst), 0);
            assert_eq!((*stdout_fd.as_ptr()).fd.load(Ordering::SeqCst), 1);
            assert_eq!((*stderr_fd.as_ptr()).fd.load(Ordering::SeqCst), 2);

            assert!((*stdin_fd.as_ptr()).flags.read);
            assert!(!(*stdin_fd.as_ptr()).flags.write);

            assert!(!(*stdout_fd.as_ptr()).flags.read);
            assert!((*stdout_fd.as_ptr()).flags.write);

            assert!(!(*stderr_fd.as_ptr()).flags.read);
            assert!((*stderr_fd.as_ptr()).flags.write);
        }
    }

    #[test]
    fn test_stdio_read_write() {
        let manager = FileManager::new();
        unsafe {
            let stdout_fd = manager.open_files[1].unwrap().as_ref();
            let stdin_fd = manager.open_files[0].unwrap().as_ref();

            let payload = b"Hello standard streams!";
            let written = manager.write(stdout_fd, payload.as_ptr(), payload.len());
            assert_eq!(written, payload.len() as isize);

            let mut buf = [0u8; 10];
            let read_bytes = manager.read(stdin_fd, buf.as_mut_ptr(), buf.len());
            assert_eq!(read_bytes, 1);
            assert_eq!(buf[0], b' ');
        }
    }

    #[test]
    fn test_stream_redirection() {
        let mut manager = FileManager::new();

        // Stdin initially open
        assert!(manager.open_files[0].is_some());

        unsafe {
            // Redirect stdout (FD 1) to stdin (FD 0)
            assert!(manager.redirect_stream(1, 0).is_ok());

            let redirected_stdin = manager.open_files[0].unwrap().as_ref();
            // Stdin (FD 0) should now have inherited stdout's flags (Write only)
            assert!(!redirected_stdin.flags.read);
            assert!(redirected_stdin.flags.write);
        }
    }
}
