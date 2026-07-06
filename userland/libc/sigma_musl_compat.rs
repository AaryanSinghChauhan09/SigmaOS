//! SigmaOS Musl Compatibility Layer
//! Inspired by Void Linux musl integration
//! Provides lightweight, fast libc compatibility with minimal dependencies

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
type SigmaSsize = isize;

/// Musl-compatible global data size limit
pub const MUSL_GLOBAL_DATA_SIZE: usize = 8192; // < 8k

/// Musl-compatible stack size for small stacks
pub const MUSL_MIN_STACK_SIZE: usize = 4096; // 4k minimum

/// Musl error codes
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MuslError {
    Success = 0,
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    ENOTBLK = 15,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
}

/// Musl file descriptor
#[repr(C)]
pub struct MuslFile {
    pub fd: SigmaI32,
    pub flags: SigmaI32,
    pub offset: SigmaI64,
    pub buf: [SigmaU8; 4096],
    pub buf_pos: SigmaUsize,
    pub buf_size: SigmaUsize,
}

/// Musl memory allocation state
#[repr(C)]
pub struct MuslAllocator {
    pub heap_start: SigmaU64,
    pub heap_end: SigmaU64,
    pub brk: SigmaU64,
    pub mmap_base: SigmaU64,
    pub total_allocated: SigmaUsize,
}

static mut MUSL_ALLOCATOR: MuslAllocator = MuslAllocator {
    heap_start: 0,
    heap_end: 0,
    brk: 0,
    mmap_base: 0,
    total_allocated: 0,
};

/// Initialize musl compatibility layer
#[no_mangle]
pub unsafe extern "C" fn musl_init(heap_start: SigmaU64, heap_size: SigmaUsize) -> SigmaI32 {
    MUSL_ALLOCATOR.heap_start = heap_start;
    MUSL_ALLOCATOR.heap_end = heap_start + heap_size as SigmaU64;
    MUSL_ALLOCATOR.brk = heap_start;
    MUSL_ALLOCATOR.mmap_base = heap_start + heap_size as SigmaU64;
    MUSL_ALLOCATOR.total_allocated = 0;
    
    0
}

/// Musl-compatible malloc (no_std, minimal implementation)
#[no_mangle]
pub unsafe extern "C" fn musl_malloc(size: SigmaUsize) -> *mut SigmaU8 {
    if size == 0 {
        return core::ptr::null_mut();
    }

    // Simple bump allocator for musl compatibility
    // In a real implementation, this would use a proper allocator
    let aligned_size = (size + 15) & !15; // 16-byte alignment
    
    if MUSL_ALLOCATOR.brk + aligned_size as SigmaU64 > MUSL_ALLOCATOR.heap_end {
        // Out of memory
        return core::ptr::null_mut();
    }

    let ptr = MUSL_ALLOCATOR.brk as *mut SigmaU8;
    MUSL_ALLOCATOR.brk += aligned_size as SigmaU64;
    MUSL_ALLOCATOR.total_allocated += aligned_size;
    
    ptr
}

/// Musl-compatible free
#[no_mangle]
pub unsafe extern "C" fn musl_free(ptr: *mut SigmaU8) {
    // With bump allocator, we can't free individual allocations
    // In a real implementation, this would use a proper free list
}

/// Musl-compatible realloc
#[no_mangle]
pub unsafe extern "C" fn musl_realloc(ptr: *mut SigmaU8, size: SigmaUsize) -> *mut SigmaU8 {
    if ptr.is_null() {
        return musl_malloc(size);
    }
    
    if size == 0 {
        musl_free(ptr);
        return core::ptr::null_mut();
    }

    // Simple implementation: allocate new and copy
    let new_ptr = musl_malloc(size);
    if !new_ptr.is_null() {
        // Copy old data (we don't know old size, so this is simplified)
        // In a real implementation, we'd track allocation sizes
    }
    
    new_ptr
}

/// Musl-compatible calloc
#[no_mangle]
pub unsafe extern "C" fn musl_calloc(nmemb: SigmaUsize, size: SigmaUsize) -> *mut SigmaU8 {
    let total_size = nmemb * size;
    let ptr = musl_malloc(total_size);
    
    if !ptr.is_null() {
        // Zero the memory
        core::ptr::write_bytes(ptr, 0, total_size);
    }
    
    ptr
}

/// Musl-compatible brk (program break)
#[no_mangle]
pub unsafe extern "C" fn musl_brk(addr: SigmaU64) -> SigmaU64 {
    if addr == 0 {
        return MUSL_ALLOCATOR.brk;
    }
    
    if addr >= MUSL_ALLOCATOR.heap_start && addr <= MUSL_ALLOCATOR.heap_end {
        MUSL_ALLOCATOR.brk = addr;
        return addr;
    }
    
    MUSL_ALLOCATOR.brk
}

/// Musl-compatible sbrk (increment program break)
#[no_mangle]
pub unsafe extern "C" fn musl_sbrk(increment: SigmaI64) -> SigmaU64 {
    let old_brk = MUSL_ALLOCATOR.brk;
    
    if increment > 0 {
        let new_brk = old_brk + increment as SigmaU64;
        if new_brk <= MUSL_ALLOCATOR.heap_end {
            MUSL_ALLOCATOR.brk = new_brk;
        } else {
            return 0xFFFFFFFFFFFFFFFF; // Error
        }
    } else if increment < 0 {
        let new_brk = old_brk - ((-increment) as SigmaU64);
        if new_brk >= MUSL_ALLOCATOR.heap_start {
            MUSL_ALLOCATOR.brk = new_brk;
        } else {
            return 0xFFFFFFFFFFFFFFFF; // Error
        }
    }
    
    old_brk
}

/// Musl-compatible strerror (error string)
#[no_mangle]
pub unsafe extern "C" fn musl_strerror(errnum: SigmaI32) -> *const SigmaU8 {
    static ERROR_STRINGS: [&str; 35] = [
        "Success",
        "Operation not permitted",
        "No such file or directory",
        "No such process",
        "Interrupted system call",
        "I/O error",
        "No such device or address",
        "Argument list too long",
        "Exec format error",
        "Bad file number",
        "No child processes",
        "Try again",
        "Out of memory",
        "Permission denied",
        "Bad address",
        "Block device required",
        "Device or resource busy",
        "File exists",
        "Cross-device link",
        "No such device",
        "Not a directory",
        "Is a directory",
        "Invalid argument",
        "File table overflow",
        "Too many open files",
        "Not a typewriter",
        "Text file busy",
        "File too large",
        "No space left on device",
        "Illegal seek",
        "Read-only file system",
        "Too many links",
        "Broken pipe",
        "Math argument out of domain of func",
        "Math result not representable",
    ];

    let idx = if errnum >= 0 && (errnum as usize) < ERROR_STRINGS.len() {
        errnum as usize
    } else {
        0
    };

    ERROR_STRINGS[idx].as_ptr()
}

/// Musl-compatible strlen
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

/// Musl-compatible strcpy
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

/// Musl-compatible strcmp
#[no_mangle]
pub unsafe extern "C" fn musl_strcmp(s1: *const SigmaU8, s2: *const SigmaU8) -> SigmaI32 {
    if s1.is_null() || s2.is_null() {
        return if s1.is_null() && s2.is_null() { 0 } else { -1 };
    }

    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        
        if c1 != c2 {
            return (c1 as SigmaI32) - (c2 as SigmaI32);
        }
        
        if c1 == 0 {
            return 0;
        }
        
        i += 1;
    }
}

/// Musl-compatible memcpy
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

/// Musl-compatible memset
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

/// Musl-compatible memcmp
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

/// Get musl allocator statistics
#[no_mangle]
pub unsafe extern "C" fn musl_get_stats(
    total_allocated: *mut SigmaUsize,
    heap_used: *mut SigmaUsize,
    heap_free: *mut SigmaUsize,
) -> SigmaI32 {
    if total_allocated.is_null() || heap_used.is_null() || heap_free.is_null() {
        return -1;
    }

    *total_allocated = MUSL_ALLOCATOR.total_allocated;
    *heap_used = (MUSL_ALLOCATOR.brk - MUSL_ALLOCATOR.heap_start) as SigmaUsize;
    *heap_free = (MUSL_ALLOCATOR.heap_end - MUSL_ALLOCATOR.brk) as SigmaUsize;

    0
}

/// Check if musl compatibility is initialized
#[no_mangle]
pub unsafe extern "C" fn musl_initialized() -> SigmaBool {
    MUSL_ALLOCATOR.heap_start != 0
}
