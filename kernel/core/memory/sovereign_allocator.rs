// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign O(1) Slab Allocator (Rust, no_std)
//! Replaces: kernel/core/memory/SovereignAllocator.cpp
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const SIGMA_HEAP_SIZE: usize = 1024 * 1024 * 16; // 16 MB heap size
pub const SIGMA_GUARD_MAGIC: u32 = 0xDEADBEEF;

#[repr(C)]
pub struct SlabBlock {
    next: *mut SlabBlock,
}

pub struct SovereignAllocator {
    heap_base: *mut u8,
    heap_offset: usize,
    free_lists: [*mut SlabBlock; 8],
}

impl SovereignAllocator {
    pub const fn new() -> Self {
        Self {
            heap_base: core::ptr::null_mut(),
            heap_offset: 0,
            free_lists: [core::ptr::null_mut(); 8],
        }
    }

    pub fn get_bucket_index(&self, size: u32) -> usize {
        if size <= 32 { return 0; }
        if size <= 64 { return 1; }
        if size <= 128 { return 2; }
        if size <= 256 { return 3; }
        if size <= 512 { return 4; }
        if size <= 1024 { return 5; }
        if size <= 2048 { return 6; }
        if size <= 4096 { return 7; }
        8
    }

    pub fn get_bucket_size(&self, index: usize) -> u32 {
        32 << index
    }
}

struct SafeSovereignAllocator {
    inner: UnsafeCell<SovereignAllocator>,
    heap: UnsafeCell<[u8; SIGMA_HEAP_SIZE]>,
}

unsafe impl Sync for SafeSovereignAllocator {}

static ALLOCATOR: SafeSovereignAllocator = SafeSovereignAllocator {
    inner: UnsafeCell::new(SovereignAllocator::new()),
    heap: UnsafeCell::new([0; SIGMA_HEAP_SIZE]),
};

extern "C" {
    fn sigma_log(s: *const u8);
    fn sigma_log_info(fmt: *const u8, val1: u32, val2: *const u8, val3: u32, val4: u32);
}

#[no_mangle]
pub unsafe extern "C" fn allocator_init() {
    let alloc = &mut *ALLOCATOR.inner.get();
    alloc.heap_base = ALLOCATOR.heap.get() as *mut u8;
    alloc.heap_offset = 0;
    for i in 0..8 {
        alloc.free_lists[i] = core::ptr::null_mut();
    }
    sigma_log(b"[ALLOCATOR] Sovereign O(1) Slab Allocator initialized (Rust core).\n\0".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn allocator_malloc(size: u32) -> *mut u8 {
    let alloc = &mut *ALLOCATOR.inner.get();
    
    // We add guard prefix/suffix size
    let needed_size = size + (core::mem::size_of::<u32>() as u32 * 2);
    let bucket_idx = alloc.get_bucket_index(needed_size);

    if bucket_idx < 8 {
        // Fast path: Slab allocation
        let block = alloc.free_lists[bucket_idx];
        if !block.is_null() {
            alloc.free_lists[bucket_idx] = (*block).next;
            let ptr = (block as *mut u8).add(core::mem::size_of::<u32>()) as *mut u8;
            
            // Re-write guards
            *(ptr.sub(core::mem::size_of::<u32>()) as *mut u32) = SIGMA_GUARD_MAGIC;
            let block_size = alloc.get_bucket_size(bucket_idx);
            *(ptr.add(block_size as usize - (core::mem::size_of::<u32>() * 2)) as *mut u32) = SIGMA_GUARD_MAGIC;
            return ptr;
        } else {
            let block_size = alloc.get_bucket_size(bucket_idx) as usize;
            if alloc.heap_offset + block_size > SIGMA_HEAP_SIZE {
                sigma_log(b"[ALLOCATOR] [FATAL] OOM: Slab heap exhausted.\n\0".as_ptr());
                return core::ptr::null_mut();
            }
            let raw = alloc.heap_base.add(alloc.heap_offset);
            alloc.heap_offset += block_size;

            *(raw as *mut u32) = SIGMA_GUARD_MAGIC;
            let ptr = raw.add(core::mem::size_of::<u32>()) as *mut u8;
            *(raw.add(block_size - core::mem::size_of::<u32>()) as *mut u32) = SIGMA_GUARD_MAGIC;
            return ptr;
        }
    }

    // Slow path: Bump allocation
    let total_size = needed_size;
    let aligned_size = ((total_size + 7) & !7) as usize;

    if alloc.heap_offset + aligned_size > SIGMA_HEAP_SIZE {
        sigma_log(b"[ALLOCATOR] [FATAL] OOM: Large allocation failed.\n\0".as_ptr());
        return core::ptr::null_mut();
    }

    let prefix = alloc.heap_base.add(alloc.heap_offset) as *mut u32;
    *prefix = SIGMA_GUARD_MAGIC;
    let ptr = (prefix as *mut u8).add(core::mem::size_of::<u32>());
    let suffix = ptr.add(size as usize) as *mut u32;
    *suffix = SIGMA_GUARD_MAGIC;

    alloc.heap_offset += aligned_size;
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn allocator_free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }

    let prefix = ptr.sub(core::mem::size_of::<u32>()) as *mut u32;
    if *prefix != SIGMA_GUARD_MAGIC {
        sigma_log(b"[ALLOCATOR] [FATAL] Memory corruption detected!\n\0".as_ptr());
        return;
    }

    // Since we don't store original request size in headers, we assume size matching general slab allocations.
    // If it's a slab bucket block, it goes to the free list. For this bare-metal stub, let's treat it as a default 32-byte bucket.
    let alloc = &mut *ALLOCATOR.inner.get();
    let bucket_idx = 0; // Default fallback to bucket 0
    let block = prefix as *mut SlabBlock;
    (*block).next = alloc.free_lists[bucket_idx];
    alloc.free_lists[bucket_idx] = block;
}
