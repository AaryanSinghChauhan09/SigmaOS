//! SigmaOS Zero-Allocation Optimizations
//! Phase 12.1: Eliminate dynamic allocations in critical paths
//! Inspired by musl libc design principles

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaUsize = usize;

/// Stack-based allocation helper (Task 12.1.2)
/// Replaces heap allocations with stack allocations where possible
#[inline(always)]
pub unsafe fn stack_alloc<T, const N: usize>() -> [T; N] {
    core::mem::MaybeUninit::uninit().assume_init()
}

/// Stack-based byte buffer
#[repr(C)]
pub struct StackBuffer<const N: usize> {
    pub data: [SigmaU8; N],
    pub len: SigmaUsize,
}

impl<const N: usize> StackBuffer<N> {
    pub const fn new() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }

    pub unsafe fn push(&mut self, byte: SigmaU8) -> bool {
        if self.len < N {
            self.data[self.len] = byte;
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub unsafe fn pop(&mut self) -> Option<SigmaU8> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.data[self.len])
        } else {
            None
        }
    }

    pub unsafe fn as_slice(&self) -> &[SigmaU8] {
        &self.data[..self.len]
    }
}

/// Fixed-size stack string (avoids heap allocation)
#[repr(C)]
pub struct StackString<const N: usize> {
    pub data: [SigmaU8; N],
    pub len: SigmaUsize,
}

impl<const N: usize> StackString<N> {
    pub const fn new() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }

    pub unsafe fn from_str(s: &str) -> Self {
        let mut result = Self::new();
        let bytes = s.as_bytes();
        for i in 0..bytes.len().min(N) {
            result.data[i] = bytes[i];
        }
        result.len = bytes.len().min(N);
        result
    }

    pub unsafe fn as_str(&self) -> &str {
        core::str::from_utf8_unchecked(&self.data[..self.len])
    }

    pub unsafe fn push(&mut self, byte: SigmaU8) -> bool {
        if self.len < N {
            self.data[self.len] = byte;
            self.len += 1;
            true
        } else {
            false
        }
    }
}

/// Stack-based vector (fixed capacity, no heap allocation)
#[repr(C)]
pub struct StackVec<T, const N: usize> {
    pub data: [T; N],
    pub len: SigmaUsize,
}

impl<T: Copy, const N: usize> StackVec<T, N> {
    pub const fn new() -> Self {
        Self {
            data: [unsafe { core::mem::zeroed() }; N],
            len: 0,
        }
    }

    pub unsafe fn push(&mut self, item: T) -> bool {
        if self.len < N {
            self.data[self.len] = item;
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub unsafe fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.data[self.len])
        } else {
            None
        }
    }

    pub unsafe fn as_slice(&self) -> &[T] {
        &self.data[..self.len]
    }

    pub unsafe fn len(&self) -> SigmaUsize {
        self.len
    }

    pub unsafe fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub unsafe fn clear(&mut self) {
        self.len = 0;
    }
}

/// Zero-allocation string builder
#[repr(C)]
pub struct StringBuilder<const N: usize> {
    pub buffer: StackString<N>,
}

impl<const N: usize> StringBuilder<N> {
    pub const fn new() -> Self {
        Self {
            buffer: StackString::new(),
        }
    }

    pub unsafe fn append(&mut self, s: &str) -> bool {
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            if !self.buffer.push(bytes[i]) {
                return false;
            }
        }
        true
    }

    pub unsafe fn append_byte(&mut self, byte: SigmaU8) -> bool {
        self.buffer.push(byte)
    }

    pub unsafe fn as_str(&self) -> &str {
        self.buffer.as_str()
    }

    pub unsafe fn len(&self) -> SigmaUsize {
        self.buffer.len
    }
}

/// Critical path marker (Task 12.1.1)
/// Used to identify and analyze critical paths
#[repr(C)]
pub struct CriticalPathMarker {
    pub name: [SigmaU8; 64],
    pub start_time: SigmaU64,
    pub end_time: SigmaU64,
    pub allocation_count: SigmaU32,
}

impl CriticalPathMarker {
    pub const fn new(name: &str) -> Self {
        let mut name_bytes = [0u8; 64];
        let name_str = name.as_bytes();
        let mut i = 0;
        while i < name_str.len().min(64) {
            name_bytes[i] = name_str[i];
            i += 1;
        }
        Self {
            name: name_bytes,
            start_time: 0,
            end_time: 0,
            allocation_count: 0,
        }
    }

    pub unsafe fn start(&mut self) {
        // TODO: Get actual timestamp
        self.start_time = 0;
        self.allocation_count = 0;
    }

    pub unsafe fn end(&mut self) {
        // TODO: Get actual timestamp
        self.end_time = 0;
    }

    pub unsafe fn record_allocation(&mut self) {
        self.allocation_count += 1;
    }

    pub unsafe fn duration(&self) -> SigmaU64 {
        self.end_time.saturating_sub(self.start_time)
    }
}

/// Global critical path tracking
static mut CRITICAL_PATHS: [Option<CriticalPathMarker>; 32] = [None; 32];
static mut CRITICAL_PATH_COUNT: SigmaUsize = 0;

#[no_mangle]
pub unsafe extern "C" fn critical_path_register(name: *const SigmaU8) -> SigmaU32 {
    if CRITICAL_PATH_COUNT >= 32 {
        return 0;
    }

    let mut name_bytes = [0u8; 64];
    let mut i = 0;
    while i < 64 {
        let byte = *name.add(i);
        if byte == 0 {
            break;
        }
        name_bytes[i] = byte;
        i += 1;
    }

    let name_str = core::str::from_utf8_unchecked(&name_bytes[..i]);
    CRITICAL_PATHS[CRITICAL_PATH_COUNT] = Some(CriticalPathMarker::new(name_str));
    
    let id = CRITICAL_PATH_COUNT as SigmaU32 + 1;
    CRITICAL_PATH_COUNT += 1;
    id
}

#[no_mangle]
pub unsafe extern "C" fn critical_path_start(id: SigmaU32) {
    let idx = (id - 1) as usize;
    if idx < CRITICAL_PATH_COUNT {
        if let Some(ref mut marker) = CRITICAL_PATHS[idx] {
            let marker_ptr = marker as *const CriticalPathMarker as *mut CriticalPathMarker;
            (*marker_ptr).start();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn critical_path_end(id: SigmaU32) {
    let idx = (id - 1) as usize;
    if idx < CRITICAL_PATH_COUNT {
        if let Some(ref mut marker) = CRITICAL_PATHS[idx] {
            let marker_ptr = marker as *const CriticalPathMarker as *mut CriticalPathMarker;
            (*marker_ptr).end();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn critical_path_record_alloc(id: SigmaU32) {
    let idx = (id - 1) as usize;
    if idx < CRITICAL_PATH_COUNT {
        if let Some(ref mut marker) = CRITICAL_PATHS[idx] {
            let marker_ptr = marker as *const CriticalPathMarker as *mut CriticalPathMarker;
            (*marker_ptr).record_allocation();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn critical_path_get_alloc_count(id: SigmaU32) -> SigmaU32 {
    let idx = (id - 1) as usize;
    if idx < CRITICAL_PATH_COUNT {
        if let Some(ref marker) = CRITICAL_PATHS[idx] {
            marker.allocation_count
        } else {
            0
        }
    } else {
        0
    }
}
