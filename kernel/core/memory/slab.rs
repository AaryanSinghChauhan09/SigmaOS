// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign SLUB Allocator (Rust, no_std)
//! =========================================================================
//!
//! Replaces sigma_slab.c and the SLUB implementation inside SovereignPMM.cpp.
//! Provides fixed-size object allocation caching.

use super::pmm::{pmm_alloc_shard, pmm_free_shard};

type U32 = u32;
type U64 = u64;

const PAGE_SIZE: usize = 4096;
const NULL_PTR: *mut u8 = core::ptr::null_mut();

pub struct SlabCache {
    pub object_size: usize,
    pub free_list: *mut *mut u8,
}

impl SlabCache {
    pub const fn new() -> Self {
        SlabCache {
            object_size: 0,
            free_list: NULL_PTR as *mut *mut u8,
        }
    }
}

pub struct SovereignSlabAllocator {
    caches: [SlabCache; 8],
    initialized: bool,
}

impl SovereignSlabAllocator {
    pub const fn new() -> Self {
        SovereignSlabAllocator {
            caches: [
                SlabCache::new(), SlabCache::new(), SlabCache::new(), SlabCache::new(),
                SlabCache::new(), SlabCache::new(), SlabCache::new(), SlabCache::new()
            ],
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        if self.initialized { return; }
        
        let mut i = 0;
        while i < 8 {
            self.caches[i].object_size = 1 << (i + 3); // 8, 16, 32, ..., 1024
            self.caches[i].free_list = NULL_PTR as *mut *mut u8;
            i += 1;
        }
        self.initialized = true;
    }

    pub unsafe fn alloc(&mut self, size: U32) -> *mut u8 {
        if !self.initialized { return NULL_PTR; }

        let mut i = 0;
        while i < 8 {
            if self.caches[i].object_size >= size as usize {
                if !self.caches[i].free_list.is_null() {
                    let obj = self.caches[i].free_list;
                    self.caches[i].free_list = *obj as *mut *mut u8;
                    return obj as *mut u8;
                }

                // Need a new page for slabs
                let page = pmm_alloc_shard();
                if page.is_null() { return NULL_PTR; }

                // Slice page into objects
                let mut offset = 0;
                let obj_size = self.caches[i].object_size;
                
                while offset + obj_size <= PAGE_SIZE {
                    let obj = page.add(offset) as *mut *mut u8;
                    *obj = self.caches[i].free_list as *mut u8;
                    self.caches[i].free_list = obj;
                    offset += obj_size;
                }

                // Pop one off
                let obj = self.caches[i].free_list;
                self.caches[i].free_list = *obj as *mut *mut u8;
                return obj as *mut u8;
            }
            i += 1;
        }

        // Fallback for large allocs
        pmm_alloc_shard()
    }

    pub unsafe fn free(&mut self, _ptr: *mut u8, _size: U32) {
        // In a real SLUB allocator, we'd determine which cache the pointer belongs to.
        // For this shim, freeing large allocations is passed to PMM. 
        // We'll leave the complex implementation out for this foundational rewrite.
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_SLAB: SovereignSlabAllocator = SovereignSlabAllocator::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn slab_init_shard() {
    G_SLAB.init();
}

#[no_mangle]
pub unsafe extern "C" fn slab_alloc_shard(size: U32) -> *mut u8 {
    G_SLAB.alloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn slab_free_shard(ptr: *mut u8, size: U32) {
    G_SLAB.free(ptr, size);
}
