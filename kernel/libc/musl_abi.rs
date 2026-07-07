//! SigmaOS Musl libc ABI Compatibility Layer
//! Phase 11.3: Lightweight C library support inspired by Void Linux musl
//! Provides minimal, fast, and secure libc implementation

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;
type SigmaIsize = isize;

/// Musl libc version
pub const MUSL_VERSION: &str = "1.2.5-sigma";

/// Standard file descriptors
pub const STDIN_FILENO: SigmaI32 = 0;
pub const STDOUT_FILENO: SigmaI32 = 1;
pub const STDERR_FILENO: SigmaI32 = 2;

/// Open flags
pub const O_RDONLY: SigmaI32 = 0;
pub const O_WRONLY: SigmaI32 = 1;
pub const O_RDWR: SigmaI32 = 2;
pub const O_CREAT: SigmaI32 = 64;
pub const O_TRUNC: SigmaI32 = 512;
pub const O_APPEND: SigmaI32 = 1024;
pub const O_NONBLOCK: SigmaI32 = 2048;

/// Seek constants
pub const SEEK_SET: SigmaI32 = 0;
pub const SEEK_CUR: SigmaI32 = 1;
pub const SEEK_END: SigmaI32 = 2;

/// Memory protection flags
pub const PROT_READ: SigmaI32 = 1;
pub const PROT_WRITE: SigmaI32 = 2;
pub const PROT_EXEC: SigmaI32 = 4;
pub const PROT_NONE: SigmaI32 = 0;

/// Memory mapping flags
pub const MAP_SHARED: SigmaI32 = 1;
pub const MAP_PRIVATE: SigmaI32 = 2;
pub const MAP_FIXED: SigmaI32 = 16;
pub const MAP_ANONYMOUS: SigmaI32 = 32;

/// File descriptor structure (minimal)
#[repr(C)]
pub struct MuslFile {
    pub fd: SigmaI32,
    pub flags: SigmaI32,
    pub offset: SigmaI64,
    pub buffer: *mut SigmaU8,
    pub buf_size: SigmaUsize,
    pub buf_pos: SigmaUsize,
}

/// Directory entry structure
#[repr(C)]
pub struct MuslDirent {
    pub d_ino: SigmaU64,
    pub d_off: SigmaI64,
    pub d_reclen: SigmaU16,
    pub d_type: SigmaU8,
    pub d_name: [SigmaU8; 256],
}

/// Stat structure (minimal)
#[repr(C)]
pub struct MuslStat {
    pub st_dev: SigmaU64,
    pub st_ino: SigmaU64,
    pub st_mode: SigmaU32,
    pub st_nlink: SigmaU32,
    pub st_uid: SigmaU32,
    pub st_gid: SigmaU32,
    pub st_rdev: SigmaU64,
    pub st_size: SigmaI64,
    pub st_blksize: SigmaI64,
    pub st_blocks: SigmaI64,
    pub st_atime: SigmaI64,
    pub st_mtime: SigmaI64,
    pub st_ctime: SigmaI64,
}

/// Timeval structure
#[repr(C)]
pub struct MuslTimeval {
    pub tv_sec: SigmaI64,
    pub tv_usec: SigmaI64,
}

/// Timespec structure
#[repr(C)]
pub struct MuslTimespec {
    pub tv_sec: SigmaI64,
    pub tv_nsec: SigmaI64,
}

/// Process ID structure
#[repr(C)]
pub struct MuslPid {
    pub pid: SigmaI32,
    pub ppid: SigmaI32,
    pub pgid: SigmaI32,
    pub sid: SigmaI32,
}

/// Musl libc ABI functions (minimal subset for kernel-space compatibility)

/// Write to file descriptor (wired to kernel syscall)
#[no_mangle]
pub unsafe extern "C" fn musl_write(fd: SigmaI32, buf: *const SigmaU8, count: SigmaUsize) -> SigmaIsize {
    extern "C" {
        fn sigma_sys_write(fd: SigmaI32, buf: *const SigmaU8, count: SigmaUsize) -> SigmaIsize;
    }
    sigma_sys_write(fd, buf, count)
}

/// Read from file descriptor (wired to kernel syscall)
#[no_mangle]
pub unsafe extern "C" fn musl_read(fd: SigmaI32, buf: *mut SigmaU8, count: SigmaUsize) -> SigmaIsize {
    extern "C" {
        fn sigma_sys_read(fd: SigmaI32, buf: *mut SigmaU8, count: SigmaUsize) -> SigmaIsize;
    }
    sigma_sys_read(fd, buf, count)
}

/// Open file (wired to kernel syscall)
#[no_mangle]
pub unsafe extern "C" fn musl_open(pathname: *const SigmaU8, flags: SigmaI32, mode: SigmaI32) -> SigmaI32 {
    extern "C" {
        fn sigma_sys_open(pathname: *const SigmaU8, flags: SigmaI32, mode: SigmaI32) -> SigmaI32;
    }
    sigma_sys_open(pathname, flags, mode)
}

/// Close file descriptor (wired to kernel syscall)
#[no_mangle]
pub unsafe extern "C" fn musl_close(fd: SigmaI32) -> SigmaI32 {
    extern "C" {
        fn sigma_sys_close(fd: SigmaI32) -> SigmaI32;
    }
    sigma_sys_close(fd)
}

/// Seek in file (wired to kernel syscall)
#[no_mangle]
pub unsafe extern "C" fn musl_lseek(fd: SigmaI32, offset: SigmaI64, whence: SigmaI32) -> SigmaI64 {
    extern "C" {
        fn sigma_sys_lseek(fd: SigmaI32, offset: SigmaI64, whence: SigmaI32) -> SigmaI64;
    }
    sigma_sys_lseek(fd, offset, whence)
}

/// Get file status (wired to kernel syscall)
#[no_mangle]
pub unsafe extern "C" fn musl_stat(pathname: *const SigmaU8, statbuf: *mut MuslStat) -> SigmaI32 {
    extern "C" {
        fn sigma_sys_stat(pathname: *const SigmaU8, statbuf: *mut MuslStat) -> SigmaI32;
    }
    sigma_sys_stat(pathname, statbuf)
}

/// Memory map (wired to VMM)
#[no_mangle]
pub unsafe extern "C" fn musl_mmap(
    addr: *mut SigmaU8,
    length: SigmaUsize,
    prot: SigmaI32,
    flags: SigmaI32,
    fd: SigmaI32,
    offset: SigmaI64,
) -> *mut SigmaU8 {
    extern "C" {
        fn sigma_vmm_map(
            virt: usize,
            phys: usize,
            pages: usize,
            flags: SigmaU32,
        ) -> SigmaI32;
        fn sigma_buddy_alloc_for_vmm(order: usize) -> usize;
    }
    
    // Calculate required pages
    let pages = (length + 4095) / 4096;
    let order = if pages == 1 { 0 } else { (pages - 1).trailing_zeros() as usize };
    
    // Allocate physical pages
    let phys = sigma_buddy_alloc_for_vmm(order);
    if phys == 0 {
        return core::ptr::null_mut();
    }
    
    // Map virtual address
    let virt = if addr.is_null() {
        // Let kernel choose address
        0x10000000 // Placeholder - should use proper VMM allocation
    } else {
        addr as usize
    };
    
    let vmm_flags = match prot {
        p if p & PROT_READ != 0 && p & PROT_WRITE != 0 && p & PROT_EXEC != 0 => 0x7,
        p if p & PROT_READ != 0 && p & PROT_WRITE != 0 => 0x3,
        p if p & PROT_READ != 0 && p & PROT_EXEC != 0 => 0x5,
        p if p & PROT_READ != 0 => 0x1,
        p if p & PROT_WRITE != 0 => 0x2,
        p if p & PROT_EXEC != 0 => 0x4,
        _ => 0x1,
    } as SigmaU32;
    
    if sigma_vmm_map(virt, phys, pages, vmm_flags) == 0 {
        virt as *mut SigmaU8
    } else {
        core::ptr::null_mut()
    }
}

/// Memory unmap (wired to VMM)
#[no_mangle]
pub unsafe extern "C" fn musl_munmap(addr: *mut SigmaU8, length: SigmaUsize) -> SigmaI32 {
    extern "C" {
        fn sigma_vmm_unmap(virt: usize, pages: usize) -> SigmaI32;
        fn sigma_buddy_free_for_vmm(phys: usize, order: usize);
    }
    
    if addr.is_null() {
        return 0;
    }
    
    let virt = addr as usize;
    let pages = (length + 4095) / 4096;
    let order = if pages == 1 { 0 } else { (pages - 1).trailing_zeros() as usize };
    
    // Unmap and free
    let result = sigma_vmm_unmap(virt, pages);
    sigma_buddy_free_for_vmm(virt, order);
    
    result
}

/// Allocate memory using slab allocator for small allocations
#[no_mangle]
pub unsafe extern "C" fn musl_malloc(size: SigmaUsize) -> *mut SigmaU8 {
    extern "C" {
        fn sigma_slab_alloc(size: SigmaUsize) -> *mut SigmaU8;
        fn sigma_buddy_alloc_for_vmm(order: usize) -> usize;
    }
    
    if size == 0 {
        return core::ptr::null_mut();
    }
    
    // Use slab allocator for small allocations (< 4096 bytes)
    if size < 4096 {
        return sigma_slab_alloc(size);
    }
    
    // Use buddy allocator for large allocations
    let pages = (size + 4095) / 4096;
    let order = if pages == 1 { 0 } else { (pages - 1).trailing_zeros() as usize };
    
    let addr = sigma_buddy_alloc_for_vmm(order);
    if addr == 0 {
        core::ptr::null_mut()
    } else {
        addr as *mut SigmaU8
    }
}

/// Free memory
#[no_mangle]
pub unsafe extern "C" fn musl_free(ptr: *mut SigmaU8) {
    extern "C" {
        fn sigma_slab_free(ptr: *mut SigmaU8);
        fn sigma_buddy_free_for_vmm(phys: usize, order: usize);
    }
    
    if ptr.is_null() {
        return;
    }
    
    // Try slab free first (will fail if not from slab)
    sigma_slab_free(ptr);
    
    // For buddy allocator, we need size tracking
    // This is simplified - proper implementation needs allocation metadata
    let addr = ptr as usize;
    sigma_buddy_free_for_vmm(addr, 0);
}

/// Reallocate memory
#[no_mangle]
pub unsafe extern "C" fn musl_realloc(ptr: *mut SigmaU8, size: SigmaUsize) -> *mut SigmaU8 {
    if ptr.is_null() {
        return musl_malloc(size);
    }
    
    if size == 0 {
        musl_free(ptr);
        return core::ptr::null_mut();
    }
    
    // Simple implementation: allocate new, copy, free old
    let new_ptr = musl_malloc(size);
    if !new_ptr.is_null() {
        // Copy old data (assume original size was similar)
        // Proper implementation needs size tracking
        musl_memcpy(new_ptr, ptr, size);
        musl_free(ptr);
    }
    
    new_ptr
}

/// Exit process (wired to scheduler)
#[no_mangle]
pub unsafe extern "C" fn musl_exit(status: SigmaI32) -> ! {
    extern "C" {
        fn sigma_sys_exit(status: SigmaI32) -> !;
    }
    sigma_sys_exit(status)
}

/// Get process ID (wired to scheduler)
#[no_mangle]
pub unsafe extern "C" fn musl_getpid() -> SigmaI32 {
    extern "C" {
        fn sigma_sys_getpid() -> SigmaI32;
    }
    sigma_sys_getpid()
}

/// Get parent process ID (wired to scheduler)
#[no_mangle]
pub unsafe extern "C" fn musl_getppid() -> SigmaI32 {
    extern "C" {
        fn sigma_sys_getppid() -> SigmaI32;
    }
    sigma_sys_getppid()
}

/// Fork process (wired to scheduler)
#[no_mangle]
pub unsafe extern "C" fn musl_fork() -> SigmaI32 {
    extern "C" {
        fn sigma_sys_fork() -> SigmaI32;
    }
    sigma_sys_fork()
}

/// Execute program (wired to scheduler)
#[no_mangle]
pub unsafe extern "C" fn musl_execve(
    pathname: *const SigmaU8,
    argv: *const *const SigmaU8,
    envp: *const *const SigmaU8,
) -> SigmaI32 {
    extern "C" {
        fn sigma_sys_execve(
            pathname: *const SigmaU8,
            argv: *const *const SigmaU8,
            envp: *const *const SigmaU8,
        ) -> SigmaI32;
    }
    sigma_sys_execve(pathname, argv, envp)
}

/// Wait for process (wired to scheduler)
#[no_mangle]
pub unsafe extern "C" fn musl_waitpid(pid: SigmaI32, status: *mut SigmaI32, options: SigmaI32) -> SigmaI32 {
    extern "C" {
        fn sigma_sys_waitpid(pid: SigmaI32, status: *mut SigmaI32, options: SigmaI32) -> SigmaI32;
    }
    sigma_sys_waitpid(pid, status, options)
}

/// Get current time (wired to timer)
#[no_mangle]
pub unsafe extern "C" fn musl_gettimeofday(tv: *mut MuslTimeval, tz: *mut SigmaU8) -> SigmaI32 {
    extern "C" {
        fn sigma_sys_gettimeofday(tv: *mut MuslTimeval, tz: *mut SigmaU8) -> SigmaI32;
    }
    sigma_sys_gettimeofday(tv, tz)
}

/// Sleep for nanoseconds (wired to timer)
#[no_mangle]
pub unsafe extern "C" fn musl_nanosleep(req: *const MuslTimespec, rem: *mut MuslTimespec) -> SigmaI32 {
    extern "C" {
        fn sigma_sys_nanosleep(req: *const MuslTimespec, rem: *mut MuslTimespec) -> SigmaI32;
    }
    sigma_sys_nanosleep(req, rem)
}

/// String length (minimal string.h implementation)
#[no_mangle]
pub unsafe extern "C" fn musl_strlen(s: *const SigmaU8) -> SigmaUsize {
    if s.is_null() {
        return 0;
    }
    
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

/// String compare
#[no_mangle]
pub unsafe extern "C" fn musl_strcmp(s1: *const SigmaU8, s2: *const SigmaU8) -> SigmaI32 {
    if s1.is_null() || s2.is_null() {
        return if s1.is_null() && s2.is_null() { 0 } else { -1 };
    }
    
    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        
        if c1 == 0 && c2 == 0 {
            return 0;
        }
        
        if c1 != c2 {
            return (c1 as SigmaI32) - (c2 as SigmaI32);
        }
        
        i += 1;
    }
}

/// String copy
#[no_mangle]
pub unsafe extern "C" fn musl_strcpy(dest: *mut SigmaU8, src: *const SigmaU8) -> *mut SigmaU8 {
    if dest.is_null() || src.is_null() {
        return dest;
    }
    
    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
    
    dest
}

/// Memory set
#[no_mangle]
pub unsafe extern "C" fn musl_memset(s: *mut SigmaU8, c: SigmaI32, n: SigmaUsize) -> *mut SigmaU8 {
    if s.is_null() || n == 0 {
        return s;
    }
    
    let byte = c as SigmaU8;
    for i in 0..n {
        *s.add(i) = byte;
    }
    
    s
}

/// Memory copy
#[no_mangle]
pub unsafe extern "C" fn musl_memcpy(dest: *mut SigmaU8, src: *const SigmaU8, n: SigmaUsize) -> *mut SigmaU8 {
    if dest.is_null() || src.is_null() || n == 0 {
        return dest;
    }
    
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
    
    dest
}

/// Memory compare
#[no_mangle]
pub unsafe extern "C" fn musl_memcmp(s1: *const SigmaU8, s2: *const SigmaU8, n: SigmaUsize) -> SigmaI32 {
    if s1.is_null() || s2.is_null() {
        return 0;
    }
    
    for i in 0..n {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 {
            return (c1 as SigmaI32) - (c2 as SigmaI32);
        }
    }
    
    0
}

/// Get musl version string
#[no_mangle]
pub unsafe extern "C" fn musl_version() -> *const SigmaU8 {
    MUSL_VERSION.as_ptr()
}
