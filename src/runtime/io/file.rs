#![no_std]
#![no_main]

/// Custom File I/O for SigmaOS
/// Implements file operations without relying on std::fs
/// Uses capability-based access control

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};

/// File descriptor structure
#[repr(C)]
#[derive(Debug)]
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
    pub is_line_buffered: bool,
    pub non_blocking: bool,
}

impl FileFlags {
    pub fn new() -> Self {
        FileFlags {
            read: false,
            write: false,
            append: false,
            create: false,
            truncate: false,
            is_line_buffered: false,
            non_blocking: false,
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
    next_fd: AtomicUsize,
    max_fds: usize,
    pub open_files: [Option<NonNull<FileDescriptor>>; 1024],
}

impl FileManager {
    pub fn new() -> Self {
        let mut manager = FileManager {
            next_fd: AtomicUsize::new(3), // Start after stdin, stdout, stderr
            max_fds: 1024,
            open_files: [None; 1024],
        };

        // Pre-allocate standard stream file descriptors 0 (stdin), 1 (stdout), and 2 (stderr)
        unsafe {
            manager.allocate_standard_streams();
        }

        manager
    }

    /// Pre-allocates file descriptors 0, 1, 2 for stdin, stdout, stderr (mimics Linux standard streams)
    unsafe fn allocate_standard_streams(&mut self) {
        for fd in 0..3 {
            let mut flags = if fd == 0 { FileFlags::read_only() } else { FileFlags::write_only() };
            // Set stdin/stdout as line buffered by default (Linux/BSD stdio paradigm)
            if fd == 1 || fd == 2 {
                flags.is_line_buffered = true;
            }
            let descriptor = FileDescriptor {
                fd: AtomicUsize::new(fd),
                flags,
                offset: AtomicUsize::new(0),
                capability: Capability::full(),
            };
            let fd_ptr = alloc(core::mem::size_of::<FileDescriptor>()) as *mut FileDescriptor;
            if !fd_ptr.is_null() {
                ptr::write(fd_ptr, descriptor);
                self.open_files[fd] = Some(NonNull::new_unchecked(fd_ptr));
            }
        }
    }

    /// Duplicates an open file descriptor onto another (mimics Linux dup2 system call)
    pub unsafe fn redirect_stream(&mut self, source_fd: usize, target_fd: usize) -> bool {
        if source_fd >= self.max_fds || target_fd >= self.max_fds {
            return false;
        }

        // Check if source exists
        let source_ptr_opt = self.open_files[source_fd];
        let source_ptr = match source_ptr_opt {
            Some(p) => p,
            None => return false,
        };

        // If target descriptor is open, close it first (standard dup2 behavior)
        if let Some(target_ptr) = self.open_files[target_fd] {
            ptr::drop_in_place(target_ptr.as_ptr());
            free(target_ptr.as_ptr() as *mut u8);
            self.open_files[target_fd] = None;
        }

        // Duplicate the descriptor structure
        let source_ref = source_ptr.as_ref();
        let duplicated = FileDescriptor {
            fd: AtomicUsize::new(target_fd),
            flags: source_ref.flags,
            offset: AtomicUsize::new(source_ref.offset.load(Ordering::SeqCst)),
            capability: source_ref.capability,
        };

        let fd_ptr = alloc(core::mem::size_of::<FileDescriptor>()) as *mut FileDescriptor;
        if fd_ptr.is_null() {
            return false;
        }

        ptr::write(fd_ptr, duplicated);
        self.open_files[target_fd] = Some(NonNull::new_unchecked(fd_ptr));
        true
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
    fn check_capability(&self, path: &[u8]) -> Capability {
        // In a real implementation, this would check against the capability system
        // For now, return full capability
        Capability::full()
    }

    /// System call for reading (placeholder)
    unsafe fn sys_read(&self, fd: usize, buffer: *mut u8, size: usize, offset: usize) -> isize {
        // In a real implementation, this would make a syscall to read from file
        // For now, return 0
        0
    }

    /// System call for writing (placeholder)
    unsafe fn sys_write(&self, fd: usize, buffer: *const u8, size: usize, offset: usize) -> isize {
        // In a real implementation, this would make a syscall to write to file
        // For now, return 0
        0
    }

    /// System call for getting file size (placeholder)
    unsafe fn sys_get_size(&self, fd: usize) -> isize {
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

/// Redirect standard stream (dup2 equivalent)
pub unsafe fn redirect_stream(source_fd: usize, target_fd: usize) -> bool {
    if let Some(ref mut manager) = GLOBAL_FILE_MANAGER {
        manager.redirect_stream(source_fd, target_fd)
    } else {
        false
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
    fn test_standard_streams_preallocation() {
        unsafe {
            init_file_manager();
            if let Some(ref manager) = GLOBAL_FILE_MANAGER {
                // Assert 0, 1, 2 standard stream FDs are pre-allocated
                assert!(manager.open_files[0].is_some());
                assert!(manager.open_files[1].is_some());
                assert!(manager.open_files[2].is_some());

                let stdin_ref = manager.open_files[0].unwrap().as_ref();
                let stdout_ref = manager.open_files[1].unwrap().as_ref();

                assert_eq!(stdin_ref.fd.load(Ordering::SeqCst), 0);
                assert_eq!(stdout_ref.fd.load(Ordering::SeqCst), 1);

                assert!(stdin_ref.flags.read);
                assert!(!stdin_ref.flags.write);
                assert!(stdout_ref.flags.write);
                assert!(!stdout_ref.flags.read);
            }
        }
    }

    #[test]
    fn test_standard_stream_redirection() {
        unsafe {
            init_file_manager();
            let mut path = [b't', b'e', b's', b't'];
            let file_fd = open(&path, OpenMode::ReadWrite).unwrap();
            let source_fd_num = file_fd.fd.load(Ordering::SeqCst);

            // Redirect stdout (FD 1) to point to the newly opened file
            assert!(redirect_stream(source_fd_num, 1));

            if let Some(ref manager) = GLOBAL_FILE_MANAGER {
                let redirected_stdout = manager.open_files[1].unwrap().as_ref();
                // Redirected stdout now has file mode capability and flags
                assert!(redirected_stdout.flags.read);
                assert!(redirected_stdout.flags.write);
            }
        }
    }
}
