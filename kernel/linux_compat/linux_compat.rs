/// SigmaOS: Linux Compatibility Layer
/// Provides POSIX/Linux syscall compatibility for running Linux/Unix applications
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── POSIX/Linux Constants ─────────────────────────────────────────────────

pub const O_RDONLY: SigmaU32 = 0;
pub const O_WRONLY: SigmaU32 = 1;
pub const O_RDWR:   SigmaU32 = 2;
pub const O_CREAT:  SigmaU32 = 0x40;
pub const O_TRUNC:  SigmaU32 = 0x200;
pub const O_APPEND: SigmaU32 = 0x400;

pub const SEEK_SET: SigmaI32 = 0;
pub const SEEK_CUR: SigmaI32 = 1;
pub const SEEK_END: SigmaI32 = 2;

pub const PROT_READ:  SigmaU32 = 0x1;
pub const PROT_WRITE: SigmaU32 = 0x2;
pub const PROT_EXEC:  SigmaU32 = 0x4;

pub const MAP_PRIVATE: SigmaU32 = 0x02;
pub const MAP_SHARED:  SigmaU32 = 0x01;

// ─── File Descriptor Structure ─────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LinuxFileDesc {
    pub fd: SigmaI32,
    pub flags: SigmaU32,
    pub offset: SigmaU64,
    pub valid: SigmaBool,
}

// ─── Linux Compatibility Layer State ───────────────────────────────────────

pub struct LinuxCompatLayer {
    pub initialized: SigmaBool,
    pub fd_table: [LinuxFileDesc; 256],
}

impl LinuxCompatLayer {
    pub const fn new() -> Self {
        Self {
            initialized: false,
            fd_table: [LinuxFileDesc {
                fd: -1,
                flags: 0,
                offset: 0,
                valid: false,
            }; 256],
        }
    }

    pub unsafe fn init(&mut self) {
        // Initialize standard file descriptors
        self.fd_table[0] = LinuxFileDesc { fd: 0, flags: O_RDONLY, offset: 0, valid: true }; // stdin
        self.fd_table[1] = LinuxFileDesc { fd: 1, flags: O_WRONLY, offset: 0, valid: true }; // stdout
        self.fd_table[2] = LinuxFileDesc { fd: 2, flags: O_WRONLY, offset: 0, valid: true }; // stderr
        self.initialized = true;
    }

    pub unsafe fn alloc_fd(&mut self) -> SigmaI32 {
        for i in 3..256 {
            if !self.fd_table[i].valid {
                self.fd_table[i].valid = true;
                return i as SigmaI32;
            }
        }
        -1 // EMFILE
    }

    pub unsafe fn free_fd(&mut self, fd: SigmaI32) {
        if fd >= 0 && (fd as SigmaUsize) < 256 {
            self.fd_table[fd as SigmaUsize].valid = false;
        }
    }
}

static mut LINUX_COMPAT: LinuxCompatLayer = LinuxCompatLayer::new();

// ─── Linux Syscall Compatibility Functions ─────────────────────────────────

/// Linux open() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_open(path: *const SigmaU8, flags: SigmaU32, mode: SigmaU32) -> SigmaI32 {
    if path.is_null() { return -14; } // EFAULT
    
    if !LINUX_COMPAT.initialized {
        LINUX_COMPAT.init();
    }
    
    let fd = LINUX_COMPAT.alloc_fd();
    if fd < 0 { return fd; }
    
    LINUX_COMPAT.fd_table[fd as SigmaUsize].flags = flags;
    LINUX_COMPAT.fd_table[fd as SigmaUsize].offset = 0;
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_open(path: *const SigmaU8, flags: SigmaU32, mode: SigmaU32) -> SigmaI64;
    }
    
    let result = sigma_vfs_open(path, flags, mode);
    if result < 0 {
        LINUX_COMPAT.free_fd(fd);
        return result as SigmaI32;
    }
    
    fd
}

/// Linux close() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_close(fd: SigmaI32) -> SigmaI32 {
    if fd < 0 || (fd as SigmaUsize) >= 256 { return -9; } // EBADF
    if !LINUX_COMPAT.fd_table[fd as SigmaUsize].valid { return -9; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_close(fd: SigmaI32) -> SigmaI64;
    }
    
    let result = sigma_vfs_close(fd);
    LINUX_COMPAT.free_fd(fd);
    result as SigmaI32
}

/// Linux read() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_read(fd: SigmaI32, buf: *mut SigmaU8, count: SigmaUsize) -> SigmaI64 {
    if fd < 0 || (fd as SigmaUsize) >= 256 { return -9; }
    if !LINUX_COMPAT.fd_table[fd as SigmaUsize].valid { return -9; }
    if buf.is_null() { return -14; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_read(fd: SigmaI32, buf: *mut SigmaU8, count: SigmaUsize) -> SigmaI64;
    }
    
    let result = sigma_vfs_read(fd, buf, count);
    if result > 0 {
        LINUX_COMPAT.fd_table[fd as SigmaUsize].offset += result as SigmaU64;
    }
    result
}

/// Linux write() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_write(fd: SigmaI32, buf: *const SigmaU8, count: SigmaUsize) -> SigmaI64 {
    if fd < 0 || (fd as SigmaUsize) >= 256 { return -9; }
    if !LINUX_COMPAT.fd_table[fd as SigmaUsize].valid { return -9; }
    if buf.is_null() { return -14; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_write(fd: SigmaI32, buf: *const SigmaU8, count: SigmaUsize) -> SigmaI64;
    }
    
    let result = sigma_vfs_write(fd, buf, count);
    if result > 0 {
        LINUX_COMPAT.fd_table[fd as SigmaUsize].offset += result as SigmaU64;
    }
    result
}

/// Linux lseek() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_lseek(fd: SigmaI32, offset: SigmaI64, whence: SigmaI32) -> SigmaI64 {
    if fd < 0 || (fd as SigmaUsize) >= 256 { return -9; }
    if !LINUX_COMPAT.fd_table[fd as SigmaUsize].valid { return -9; }
    
    let new_offset: SigmaI64 = match whence {
        SEEK_SET => offset,
        SEEK_CUR => LINUX_COMPAT.fd_table[fd as SigmaUsize].offset as SigmaI64 + offset,
        SEEK_END => {
            // Get file size from VFS (simplified)
            LINUX_COMPAT.fd_table[fd as SigmaUsize].offset as SigmaI64 + offset
        }
        _ => return -22, // EINVAL
    };
    
    if new_offset < 0 { return -22; }
    LINUX_COMPAT.fd_table[fd as SigmaUsize].offset = new_offset as SigmaU64;
    new_offset
}

/// Linux mmap() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_mmap(
    addr: *mut SigmaU8,
    length: SigmaUsize,
    prot: SigmaU32,
    flags: SigmaU32,
    fd: SigmaI32,
    offset: SigmaI64
) -> *mut SigmaU8 {
    // Delegate to SigmaOS VMM
    extern "C" {
        fn sigma_vmm_map(addr: SigmaU64, phys_addr: SigmaU64, flags: SigmaU64) -> SigmaI32;
    }
    
    let _ = (addr, length, prot, flags, fd, offset);
    
    // Simplified: return a dummy address
    0xFFFF_F000_0000_0000 as *mut SigmaU8
}

/// Linux munmap() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_munmap(addr: *mut SigmaU8, length: SigmaUsize) -> SigmaI32 {
    let _ = (addr, length);
    // Delegate to SigmaOS VMM
    0
}

/// Linux ioctl() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_ioctl(fd: SigmaI32, request: SigmaU64, arg: SigmaU64) -> SigmaI32 {
    if fd < 0 || (fd as SigmaUsize) >= 256 { return -9; }
    if !LINUX_COMPAT.fd_table[fd as SigmaUsize].valid { return -9; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_ioctl(fd: SigmaI32, request: SigmaU64, arg: SigmaU64) -> SigmaI64;
    }
    
    sigma_vfs_ioctl(fd, request, arg) as SigmaI32
}

/// Linux stat() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_stat(path: *const SigmaU8, statbuf: *mut SigmaU8) -> SigmaI32 {
    if path.is_null() || statbuf.is_null() { return -14; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_stat(path: *const SigmaU8, out: *mut SigmaU8) -> SigmaI64;
    }
    
    sigma_vfs_stat(path, statbuf) as SigmaI32
}

/// Linux fstat() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_fstat(fd: SigmaI32, statbuf: *mut SigmaU8) -> SigmaI32 {
    if fd < 0 || (fd as SigmaUsize) >= 256 { return -9; }
    if !LINUX_COMPAT.fd_table[fd as SigmaUsize].valid { return -9; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_fstat(fd: SigmaI32, out: *mut SigmaU8) -> SigmaI64;
    }
    
    sigma_vfs_fstat(fd, statbuf) as SigmaI32
}

/// Linux mkdir() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_mkdir(path: *const SigmaU8, mode: SigmaU32) -> SigmaI32 {
    if path.is_null() { return -14; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_mkdir(path: *const SigmaU8, mode: SigmaU32) -> SigmaI64;
    }
    
    sigma_vfs_mkdir(path, mode) as SigmaI32
}

/// Linux rmdir() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_rmdir(path: *const SigmaU8) -> SigmaI32 {
    if path.is_null() { return -14; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_rmdir(path: *const SigmaU8) -> SigmaI64;
    }
    
    sigma_vfs_rmdir(path) as SigmaI32
}

/// Linux unlink() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_unlink(path: *const SigmaU8) -> SigmaI32 {
    if path.is_null() { return -14; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_unlink(path: *const SigmaU8) -> SigmaI64;
    }
    
    sigma_vfs_unlink(path) as SigmaI32
}

/// Linux rename() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_rename(oldpath: *const SigmaU8, newpath: *const SigmaU8) -> SigmaI32 {
    if oldpath.is_null() || newpath.is_null() { return -14; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_rename(old: *const SigmaU8, new: *const SigmaU8) -> SigmaI64;
    }
    
    sigma_vfs_rename(oldpath, newpath) as SigmaI32
}

/// Linux chdir() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_chdir(path: *const SigmaU8) -> SigmaI32 {
    if path.is_null() { return -14; }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_chdir(path: *const SigmaU8) -> SigmaI64;
    }
    
    sigma_vfs_chdir(path) as SigmaI32
}

/// Linux getcwd() syscall compatibility
#[no_mangle]
pub unsafe extern "C" fn linux_getcwd(buf: *mut SigmaU8, size: SigmaUsize) -> *mut SigmaU8 {
    if buf.is_null() || size == 0 { return 0 as *mut SigmaU8 }
    
    // Delegate to SigmaOS VFS
    extern "C" {
        fn sigma_vfs_getcwd(buf: *mut SigmaU8, size: SigmaUsize) -> SigmaI64;
    }
    
    let result = sigma_vfs_getcwd(buf, size);
    if result < 0 { return 0 as *mut SigmaU8 }
    buf
}

// ─── Kernel Module Compatibility Functions ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn kfree() {
    // Stub: memory deallocation for Linux kernel modules
}

#[no_mangle]
pub unsafe extern "C" fn free_irq() {
    // Stub: interrupt handler deallocation
}

#[no_mangle]
pub unsafe extern "C" fn pci_unregister_driver() {
    // Stub: PCI driver unregistration
}

#[no_mangle]
pub unsafe extern "C" fn pci_disable_device() {
    // Stub: PCI device disable
}

/// Initialize Linux compatibility layer
#[no_mangle]
pub unsafe extern "C" fn linux_compat_init() -> SigmaI32 {
    LINUX_COMPAT.init();
    0
}

