//! SigmaOS — Slab Allocator
//! Fixed-size object caching allocator built on top of the buddy allocator.
//! No std, no heap — all structures statically allocated.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── Constants ───────────────────────────────────────────────────────────────
const PAGE_SIZE:          Usize = 4096;
const MAX_CACHES:         usize = 32;
const MAX_SLABS_PER_CACHE: usize = 64;
const MAX_OBJECTS_PER_SLAB: usize = 512;
const SLAB_MAGIC:         U32 = 0x534C4142; // "SLAB"

// ── Slab States ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum SlabState {
    Free    = 0,
    Partial = 1,
    Full    = 2,
}

// ── Slab Descriptor ─────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct Slab {
    pub magic:       U32,
    pub state:       SlabState,
    pub base_addr:   U64,           // Base address of slab memory
    pub obj_size:    U32,           // Size of each object
    pub total_objs:  U16,          // Total objects in this slab
    pub free_objs:   U16,          // Free objects count
    pub free_bitmap: [U64; 8],     // Bitmap: 512 objects max (8 * 64)
    pub color_offset: U16,         // Cache coloring offset
}

impl Slab {
    pub const fn empty() -> Self {
        Slab {
            magic: 0,
            state: SlabState::Free,
            base_addr: 0,
            obj_size: 0,
            total_objs: 0,
            free_objs: 0,
            free_bitmap: [0u64; 8],
            color_offset: 0,
        }
    }

    /// Find and allocate a free object. Returns offset within slab or -1.
    pub fn alloc_object(&mut self) -> i32 {
        if self.free_objs == 0 { return -1; }
        // Scan bitmap for a free bit (1 = free)
        for word_idx in 0..8 {
            if self.free_bitmap[word_idx] != 0 {
                // Find first set bit
                let mut bit = 0u32;
                let mut mask = self.free_bitmap[word_idx];
                while mask & 1 == 0 {
                    mask >>= 1;
                    bit += 1;
                }
                let obj_idx = word_idx * 64 + bit as usize;
                if obj_idx >= self.total_objs as usize { continue; }
                // Mark as allocated (clear bit)
                self.free_bitmap[word_idx] &= !(1u64 << bit);
                self.free_objs -= 1;
                if self.free_objs == 0 {
                    self.state = SlabState::Full;
                } else {
                    self.state = SlabState::Partial;
                }
                return obj_idx as i32;
            }
        }
        -1
    }

    /// Free an object at the given index.
    pub fn free_object(&mut self, obj_idx: usize) {
        let word_idx = obj_idx / 64;
        let bit = obj_idx % 64;
        if word_idx < 8 {
            self.free_bitmap[word_idx] |= 1u64 << bit;
            self.free_objs += 1;
            if self.free_objs == self.total_objs {
                self.state = SlabState::Free;
            } else {
                self.state = SlabState::Partial;
            }
        }
    }

    /// Initialize a slab for objects of given size.
    pub fn init(&mut self, base_addr: U64, obj_size: U32, color: U16) {
        self.magic = SLAB_MAGIC;
        self.base_addr = base_addr + color as U64;
        self.obj_size = obj_size;
        self.color_offset = color;

        let usable = PAGE_SIZE - color as usize;
        let total = usable / obj_size as usize;
        let total = if total > MAX_OBJECTS_PER_SLAB { MAX_OBJECTS_PER_SLAB } else { total };
        self.total_objs = total as U16;
        self.free_objs = total as U16;
        self.state = SlabState::Free;

        // Set all bits as free
        for word_idx in 0..8 {
            self.free_bitmap[word_idx] = 0;
        }
        for i in 0..total {
            let word_idx = i / 64;
            let bit = i % 64;
            self.free_bitmap[word_idx] |= 1u64 << bit;
        }
    }
}

// ── Slab Cache ──────────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct SlabCache {
    pub name:        [U8; 32],
    pub name_len:    usize,
    pub obj_size:    U32,
    pub alignment:   U32,
    pub slabs:       [Slab; MAX_SLABS_PER_CACHE],
    pub slab_count:  usize,
    pub total_alloc: U64,
    pub total_free:  U64,
    pub active:      bool,
    pub color_next:  U16,   // Next cache coloring offset
    pub color_range: U16,   // Max color offset
}

impl SlabCache {
    pub const fn empty() -> Self {
        SlabCache {
            name: [0u8; 32],
            name_len: 0,
            obj_size: 0,
            alignment: 8,
            slabs: [Slab::empty(); MAX_SLABS_PER_CACHE],
            slab_count: 0,
            total_alloc: 0,
            total_free: 0,
            active: false,
            color_next: 0,
            color_range: 64,
        }
    }
}

// ── Global State ────────────────────────────────────────────────────────────
pub struct SlabAllocator {
    pub caches: [SlabCache; MAX_CACHES],
    pub cache_count: usize,
}

static mut SLAB: SlabAllocator = SlabAllocator {
    caches: [SlabCache::empty(); MAX_CACHES],
    cache_count: 0,
};

// ── Public API ──────────────────────────────────────────────────────────────

/// Create a new slab cache for objects of the given size.
/// Returns cache index or negative error.
#[no_mangle]
pub unsafe extern "C" fn sigma_slab_create_cache(
    name: *const U8,
    name_len: usize,
    obj_size: U32,
    alignment: U32,
) -> i32 {
    if SLAB.cache_count >= MAX_CACHES { return -1; }
    if obj_size == 0 || obj_size > PAGE_SIZE as U32 { return -2; }

    let idx = SLAB.cache_count;
    let cache = &mut SLAB.caches[idx];

    // Align object size
    let align = if alignment < 8 { 8 } else { alignment };
    let aligned_size = (obj_size + align - 1) & !(align - 1);

    cache.obj_size = aligned_size;
    cache.alignment = align;
    cache.active = true;
    cache.slab_count = 0;
    cache.total_alloc = 0;
    cache.total_free = 0;
    cache.color_next = 0;
    cache.color_range = (align.min(64)) as U16;

    // Copy name
    if !name.is_null() {
        let copy_len = if name_len < 32 { name_len } else { 31 };
        for i in 0..copy_len {
            cache.name[i] = *name.add(i);
        }
        cache.name_len = copy_len;
    }

    SLAB.cache_count += 1;
    idx as i32
}

/// Allocate an object from a slab cache. Returns virtual address or 0 on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_slab_alloc(cache_idx: U32) -> U64 {
    let idx = cache_idx as usize;
    if idx >= SLAB.cache_count { return 0; }

    let cache = &mut SLAB.caches[idx];

    // Try partial slabs first
    for s in 0..cache.slab_count {
        if cache.slabs[s].state == SlabState::Partial || cache.slabs[s].state == SlabState::Free {
            let obj_idx = cache.slabs[s].alloc_object();
            if obj_idx >= 0 {
                cache.total_alloc += 1;
                return cache.slabs[s].base_addr + (obj_idx as U64 * cache.obj_size as U64);
            }
        }
    }

    // Need a new slab — allocate a page from buddy allocator
    if cache.slab_count >= MAX_SLABS_PER_CACHE { return 0; }

    // In a real implementation, we'd call sigma_buddy_alloc(0) here
    // For now, use a placeholder address based on slab index
    let slab_idx = cache.slab_count;
    let fake_page_addr: U64 = 0x1000_0000 + (idx as U64 * 0x10_0000) + (slab_idx as U64 * PAGE_SIZE as U64);

    let color = cache.color_next;
    cache.color_next = (cache.color_next + cache.alignment as U16) % cache.color_range;

    cache.slabs[slab_idx].init(fake_page_addr, cache.obj_size, color);
    cache.slab_count += 1;

    let obj_idx = cache.slabs[slab_idx].alloc_object();
    if obj_idx >= 0 {
        cache.total_alloc += 1;
        cache.slabs[slab_idx].base_addr + (obj_idx as U64 * cache.obj_size as U64)
    } else {
        0
    }
}

/// Free an object back to its slab cache.
#[no_mangle]
pub unsafe extern "C" fn sigma_slab_free(cache_idx: U32, addr: U64) -> i32 {
    let idx = cache_idx as usize;
    if idx >= SLAB.cache_count { return -1; }

    let cache = &mut SLAB.caches[idx];

    // Find which slab owns this address
    for s in 0..cache.slab_count {
        let slab = &mut cache.slabs[s];
        let slab_end = slab.base_addr + (slab.total_objs as U64 * cache.obj_size as U64);
        if addr >= slab.base_addr && addr < slab_end {
            let obj_idx = ((addr - slab.base_addr) / cache.obj_size as U64) as usize;
            slab.free_object(obj_idx);
            cache.total_free += 1;
            return 0;
        }
    }
    -2 // Address not found in any slab
}

/// Destroy a slab cache and release all memory.
#[no_mangle]
pub unsafe extern "C" fn sigma_slab_destroy_cache(cache_idx: U32) -> i32 {
    let idx = cache_idx as usize;
    if idx >= SLAB.cache_count { return -1; }
    SLAB.caches[idx].active = false;
    SLAB.caches[idx].slab_count = 0;
    0
}

/// Get stats for a cache.
#[no_mangle]
pub unsafe extern "C" fn sigma_slab_cache_alloc_count(cache_idx: U32) -> U64 {
    let idx = cache_idx as usize;
    if idx >= SLAB.cache_count { return 0; }
    SLAB.caches[idx].total_alloc
}

/// Get total number of caches.
#[no_mangle]
pub unsafe extern "C" fn sigma_slab_cache_count() -> U32 {
    SLAB.cache_count as U32
}
