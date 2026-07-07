//! SigmaOS Zero-Allocation Optimizations
//! Inspired by musl libc - provides string manipulation and memory operations
//! without heap allocation for performance and security

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaUsize = usize;
type SigmaIsize = isize;

/// Zero-allocation string operations
pub struct ZeroAllocString {
    pub data: [SigmaU8; 256],
    pub len: SigmaUsize,
}

impl ZeroAllocString {
    pub const fn new() -> Self {
        Self {
            data: [0; 256],
            len: 0,
        }
    }

    pub const fn from_bytes(bytes: &[SigmaU8]) -> Self {
        let mut s = Self::new();
        let mut i = 0;
        while i < bytes.len().min(256) {
            s.data[i] = bytes[i];
            i += 1;
        }
        s.len = bytes.len().min(256);
        s
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.data[..self.len])
        }
    }

    pub fn append(&mut self, bytes: &[SigmaU8]) -> bool {
        if self.len + bytes.len() > 256 {
            return false;
        }
        let mut i = 0;
        while i < bytes.len() {
            self.data[self.len + i] = bytes[i];
            i += 1;
        }
        self.len += bytes.len();
        true
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

/// Zero-allocation string comparison
#[no_mangle]
pub unsafe extern "C" fn sigma_strcmp(s1: *const SigmaU8, s2: *const SigmaU8) -> SigmaI32 {
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
        if c1 == 0 || c2 == 0 {
            return (c1 as SigmaI32) - (c2 as SigmaI32);
        }
        i += 1;
    }
}

/// Zero-allocation string length
#[no_mangle]
pub unsafe extern "C" fn sigma_strlen(s: *const SigmaU8) -> SigmaUsize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

/// Zero-allocation string copy
#[no_mangle]
pub unsafe extern "C" fn sigma_strcpy(dst: *mut SigmaU8, src: *const SigmaU8) -> *mut SigmaU8 {
    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dst.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
    dst
}

/// Zero-allocation string concatenation (with bounds checking)
#[no_mangle]
pub unsafe extern "C" fn sigma_strcat(dst: *mut SigmaU8, src: *const SigmaU8, max_len: SigmaUsize) -> *mut SigmaU8 {
    let dst_len = sigma_strlen(dst);
    let mut i = 0;
    while dst_len + i < max_len {
        let c = *src.add(i);
        if c == 0 {
            break;
        }
        *dst.add(dst_len + i) = c;
        i += 1;
    }
    *dst.add(dst_len + i) = 0;
    dst
}

/// Zero-allocation memory copy
#[no_mangle]
pub unsafe extern "C" fn sigma_memcpy(dst: *mut SigmaU8, src: *const SigmaU8, n: SigmaUsize) -> *mut SigmaU8 {
    let mut i = 0;
    while i < n {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
    dst
}

/// Zero-allocation memory set
#[no_mangle]
pub unsafe extern "C" fn sigma_memset(s: *mut SigmaU8, c: SigmaI32, n: SigmaUsize) -> *mut SigmaU8 {
    let byte = c as SigmaU8;
    let mut i = 0;
    while i < n {
        *s.add(i) = byte;
        i += 1;
    }
    s
}

/// Zero-allocation memory compare
#[no_mangle]
pub unsafe extern "C" fn sigma_memcmp(s1: *const SigmaU8, s2: *const SigmaU8, n: SigmaUsize) -> SigmaI32 {
    let mut i = 0;
    while i < n {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 {
            return (c1 as SigmaI32) - (c2 as SigmaI32);
        }
        i += 1;
    }
    0
}

/// Zero-allocation string find
#[no_mangle]
pub unsafe extern "C" fn sigma_strchr(s: *const SigmaU8, c: SigmaI32) -> *const SigmaU8 {
    let byte = c as SigmaU8;
    let mut i = 0;
    loop {
        let cur = *s.add(i);
        if cur == byte {
            return s.add(i);
        }
        if cur == 0 {
            return core::ptr::null();
        }
        i += 1;
    }
}

/// Zero-allocation string reverse find
#[no_mangle]
pub unsafe extern "C" fn sigma_strrchr(s: *const SigmaU8, c: SigmaI32) -> *const SigmaU8 {
    let byte = c as SigmaU8;
    let mut i = sigma_strlen(s);
    let mut last = core::ptr::null();
    while i > 0 {
        i -= 1;
        if *s.add(i) == byte {
            last = s.add(i);
        }
    }
    last
}

/// Zero-allocation string to integer conversion
#[no_mangle]
pub unsafe extern "C" fn sigma_atoi(s: *const SigmaU8) -> SigmaI32 {
    let mut result: SigmaI32 = 0;
    let mut i = 0;
    let mut negative = false;

    // Skip whitespace
    while *s.add(i) == b' ' || *s.add(i) == b'\t' || *s.add(i) == b'\n' {
        i += 1;
    }

    // Check for sign
    if *s.add(i) == b'-' {
        negative = true;
        i += 1;
    } else if *s.add(i) == b'+' {
        i += 1;
    }

    // Parse digits
    while i < 256 {
        let c = *s.add(i);
        if c < b'0' || c > b'9' {
            break;
        }
        result = result * 10 + (c - b'0') as SigmaI32;
        i += 1;
    }

    if negative {
        -result
    } else {
        result
    }
}

/// Zero-allocation integer to string conversion
#[no_mangle]
pub unsafe extern "C" fn sigma_itoa(value: SigmaI32, buf: *mut SigmaU8, buf_len: SigmaUsize) -> SigmaI32 {
    if buf_len == 0 {
        return -1;
    }

    let mut v = value;
    let mut i = 0;
    let mut negative = false;

    if v < 0 {
        negative = true;
        v = -v;
    }

    // Handle zero
    if v == 0 {
        if buf_len < 2 {
            return -1;
        }
        *buf.add(0) = b'0';
        *buf.add(1) = 0;
        return 1;
    }

    // Convert digits (reverse order)
    let mut digits: [SigmaU8; 12] = [0; 12];
    let mut digit_count = 0;
    while v > 0 && digit_count < 12 {
        digits[digit_count] = (v % 10) as SigmaU8 + b'0';
        v /= 10;
        digit_count += 1;
    }

    // Check buffer size
    let total_len = if negative { digit_count + 1 } else { digit_count };
    if total_len + 1 > buf_len {
        return -1;
    }

    // Copy to buffer (reverse order)
    let mut pos = 0;
    if negative {
        *buf.add(0) = b'-';
        pos = 1;
    }
    while digit_count > 0 {
        digit_count -= 1;
        *buf.add(pos) = digits[digit_count];
        pos += 1;
    }
    *buf.add(pos) = 0;

    pos as SigmaI32
}

/// Zero-allocation string copy with length limit
#[no_mangle]
pub unsafe extern "C" fn sigma_strncpy(dst: *mut SigmaU8, src: *const SigmaU8, n: SigmaUsize) -> *mut SigmaU8 {
    let mut i = 0;
    while i < n {
        let c = *src.add(i);
        *dst.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
    // Pad with zeros if source is shorter
    while i < n {
        *dst.add(i) = 0;
        i += 1;
    }
    dst
}

/// Zero-allocation string compare with length limit
#[no_mangle]
pub unsafe extern "C" fn sigma_strncmp(s1: *const SigmaU8, s2: *const SigmaU8, n: SigmaUsize) -> SigmaI32 {
    let mut i = 0;
    while i < n {
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
    0
}

/// Zero-allocation buffer pool for temporary allocations
const POOL_SIZE: usize = 16;
const POOL_BUFFER_SIZE: usize = 4096;

pub struct ZeroAllocPool {
    buffers: [*mut SigmaU8; POOL_SIZE],
    in_use: [bool; POOL_SIZE],
}

static mut ZERO_POOL: ZeroAllocPool = ZeroAllocPool {
    buffers: [core::ptr::null_mut(); POOL_SIZE],
    in_use: [false; POOL_SIZE],
};

/// Initialize zero-allocation pool
#[no_mangle]
pub unsafe extern "C" fn sigma_zero_pool_init() -> SigmaI32 {
    // In a real implementation, this would allocate from a pre-reserved memory region
    // For now, we'll use static buffers
    static mut STATIC_BUFFERS: [[SigmaU8; POOL_BUFFER_SIZE]; POOL_SIZE] = [[0; POOL_BUFFER_SIZE]; POOL_SIZE];
    
    for i in 0..POOL_SIZE {
        ZERO_POOL.buffers[i] = STATIC_BUFFERS[i].as_mut_ptr();
        ZERO_POOL.in_use[i] = false;
    }
    
    0
}

/// Allocate from zero-allocation pool
#[no_mangle]
pub unsafe extern "C" fn sigma_zero_alloc(size: SigmaUsize) -> *mut SigmaU8 {
    if size > POOL_BUFFER_SIZE {
        return core::ptr::null_mut();
    }
    
    for i in 0..POOL_SIZE {
        if !ZERO_POOL.in_use[i] {
            ZERO_POOL.in_use[i] = true;
            // Clear buffer
            sigma_memset(ZERO_POOL.buffers[i], 0, size);
            return ZERO_POOL.buffers[i];
        }
    }
    
    core::ptr::null_mut()
}

/// Free to zero-allocation pool
#[no_mangle]
pub unsafe extern "C" fn sigma_zero_free(ptr: *mut SigmaU8) {
    for i in 0..POOL_SIZE {
        if ZERO_POOL.buffers[i] == ptr {
            ZERO_POOL.in_use[i] = false;
            return;
        }
    }
}

/// Zero-allocation string format (simplified - only supports %d and %s)
#[no_mangle]
pub unsafe extern "C" fn sigma_sprintf(
    buf: *mut SigmaU8,
    buf_len: SigmaUsize,
    fmt: *const SigmaU8,
    args: *const SigmaU64,
    arg_count: SigmaUsize,
) -> SigmaI32 {
    let mut pos = 0;
    let mut fmt_idx = 0;
    let mut arg_idx = 0;

    while pos < buf_len - 1 {
        let c = *fmt.add(fmt_idx);
        if c == 0 {
            break;
        }
        
        if c == b'%' {
            fmt_idx += 1;
            let spec = *fmt.add(fmt_idx);
            
            if spec == b'd' && arg_idx < arg_count {
                let val = *args.add(arg_idx) as SigmaI32;
                let tmp_buf: [SigmaU8; 32] = [0; 32];
                let len = sigma_itoa(val, tmp_buf.as_ptr() as *mut SigmaU8, 32);
                if len > 0 && (pos + len as usize) < buf_len {
                    sigma_memcpy(buf.add(pos), tmp_buf.as_ptr(), len as usize);
                    pos += len as usize;
                }
                arg_idx += 1;
            } else if spec == b's' && arg_idx < arg_count {
                let str_ptr = *args.add(arg_idx) as *const SigmaU8;
                let str_len = sigma_strlen(str_ptr);
                if pos + str_len < buf_len {
                    sigma_memcpy(buf.add(pos), str_ptr, str_len);
                    pos += str_len;
                }
                arg_idx += 1;
            } else if spec == b'%' {
                *buf.add(pos) = b'%';
                pos += 1;
            }
        } else {
            *buf.add(pos) = c;
            pos += 1;
        }
        
        fmt_idx += 1;
    }
    
    *buf.add(pos) = 0;
    pos as SigmaI32
}
