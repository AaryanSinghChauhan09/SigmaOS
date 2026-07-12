/// SigmaOS: Slab Allocator (kmalloc)
/// Phase G Blocker #3: Slab allocator (kmalloc)
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.


#[allow(dead_code)]

use crate::mm::sigma_buddy_alloc;

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Constants ─────────────────────────────────────────────────────────────

pub const SLAB_MIN_SIZE: usize = 8;
pub const SLAB_MAX_SIZE: usize = 4096;
pub const SLAB_OBJ_PER_SLAB: usize = 64;
pub const SLAB_CACHE_COUNT: usize = 16;
pub const MAX_SLABS: usize = 256; // Maximum slabs per cache

// ─── Slab Object ───────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SlabObject {
    pub in_use: SigmaBool,
    pub next: Option<SigmaU64>,
}

// ─── Slab ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Slab {
    pub objects: [SlabObject; SLAB_OBJ_PER_SLAB],
    pub free_count: SigmaUsize,
    pub total_count: SigmaUsize,
    pub base_addr: SigmaU64,
    pub next: Option<SigmaU64>,
}

// ─── Slab Cache ───────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SlabCache {
    pub object_size: SigmaUsize,
    pub slab_count: SigmaUsize,
    pub free_slabs: Option<SigmaU64>,
    pub used_slabs: Option<SigmaU64>,
    pub partial_slabs: Option<SigmaU64>,
    pub total_objects: SigmaUsize,
    pub free_objects: SigmaU64,
}

// ─── Slab Allocator ─────────────────────────────────────────────────────

pub struct SlabAllocator {
    initialized: SigmaBool,
    caches: [SlabCache; SLAB_CACHE_COUNT],
    slabs: [[Option<Slab>; MAX_SLABS]; SLAB_CACHE_COUNT], // BUG-001 Fix: Static slab storage
    total_memory: SigmaU64,
    used_memory: SigmaU64,
}

impl SlabAllocator {
    pub const fn new() -> Self {
        Self {
            initialized: false,
            caches: [SlabCache {
                object_size: 0,
                slab_count: 0,
                free_slabs: None,
                used_slabs: None,
                partial_slabs: None,
                total_objects: 0,
                free_objects: 0,
            }; SLAB_CACHE_COUNT],
            slabs: [[const { None }; MAX_SLABS]; SLAB_CACHE_COUNT], // BUG-001 Fix
            total_memory: 0,
            used_memory: 0,
        }
    }

    /// Initialize slab allocator
    pub unsafe fn init(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Err("Slab allocator already initialized");
        }

        // Initialize caches for different sizes
        let mut size = SLAB_MIN_SIZE;
        for i in 0..SLAB_CACHE_COUNT {
            self.caches[i].object_size = size;
            self.caches[i].slab_count = 0;
            self.caches[i].free_slabs = None;
            self.caches[i].used_slabs = None;
            self.caches[i].partial_slabs = None;
            self.caches[i].total_objects = 0;
            self.caches[i].free_objects = 0;

            size *= 2;
            if size > SLAB_MAX_SIZE {
                break;
            }
        }

        self.total_memory = 0;
        self.used_memory = 0;
        self.initialized = true;

        Ok(())
    }

    /// Allocate memory (kmalloc)
    pub unsafe fn kmalloc(&mut self, size: SigmaUsize) -> Option<SigmaU64> {
        if !self.initialized {
            return None;
        }

        if size == 0 {
            return None;
        }

        // Find appropriate cache
        let cache_idx = self.find_cache(size);
        if cache_idx >= SLAB_CACHE_COUNT {
            return None;
        }

        let (object_size, partial_slabs, free_slabs) = {
            let cache = &self.caches[cache_idx];
            (cache.object_size, cache.partial_slabs, cache.free_slabs)
        };

        // Try partial slabs first
        if let Some(slab_addr) = partial_slabs {
            if let Some(obj_addr) = self.alloc_from_slab(slab_addr, object_size) {
                self.caches[cache_idx].free_objects -= 1;
                return Some(obj_addr);
            }
        }

        // Try free slabs
        if let Some(slab_addr) = free_slabs {
            if let Some(obj_addr) = self.alloc_from_slab(slab_addr, object_size) {
                self.caches[cache_idx].free_objects -= 1;
                return Some(obj_addr);
            }
        }

        // Allocate new slab
        if let Some(slab_addr) = self.alloc_new_slab(object_size) {
            if let Some(obj_addr) = self.alloc_from_slab(slab_addr, object_size) {
                let cache = &mut self.caches[cache_idx];
                cache.slab_count += 1;
                cache.total_objects += SLAB_OBJ_PER_SLAB;
                cache.free_objects = SLAB_OBJ_PER_SLAB as SigmaU64 - 1;
                return Some(obj_addr);
            }
        }

        None
    }

    /// Free memory (kfree)
    pub unsafe fn kfree(&mut self, ptr: SigmaU64, size: SigmaUsize) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Slab allocator not initialized");
        }

        if ptr == 0 {
            return Err("Null pointer");
        }

        // Find appropriate cache
        let cache_idx = self.find_cache(size);
        if cache_idx >= SLAB_CACHE_COUNT {
            return Err("Invalid size");
        }

        let object_size = self.caches[cache_idx].object_size;

        // Find slab containing this pointer
        let slab_addr = self.find_slab_for_ptr(ptr, object_size);
        if slab_addr == 0 {
            return Err("Invalid pointer");
        }

        // Free object in slab
        self.free_in_slab(slab_addr, ptr, object_size);
        self.caches[cache_idx].free_objects += 1;

        Ok(())
    }

    /// Find cache for given size
    fn find_cache(&self, size: SigmaUsize) -> usize {
        let mut cache_idx = 0;
        let mut cache_size = SLAB_MIN_SIZE;

        while cache_idx < SLAB_CACHE_COUNT && cache_size < size {
            cache_size *= 2;
            cache_idx += 1;
        }

        cache_idx
    }

    /// Allocate from existing slab (BUG-001 Fix)
    unsafe fn alloc_from_slab(&mut self, slab_addr: SigmaU64, obj_size: SigmaUsize) -> Option<SigmaU64> {
        let cache_idx = self.find_cache(obj_size);
        if cache_idx >= SLAB_CACHE_COUNT {
            return None;
        }

        // Find slab in storage
        let slab_idx = self.find_slab_index(slab_addr, cache_idx);
        if slab_idx >= MAX_SLABS {
            return None;
        }

        if let Some(ref mut slab) = self.slabs[cache_idx][slab_idx] {
            // Find first free object
            for i in 0..SLAB_OBJ_PER_SLAB {
                if !slab.objects[i].in_use {
                    slab.objects[i].in_use = true;
                    slab.free_count -= 1;
                    
                    // Calculate object address
                    let obj_offset = i as SigmaU64 * obj_size as SigmaU64;
                    let obj_addr = slab.base_addr + obj_offset;
                    
                    return Some(obj_addr);
                }
            }
        }

        None
    }

    /// Free object in slab (BUG-001 Fix)
    unsafe fn free_in_slab(&mut self, slab_addr: SigmaU64, ptr: SigmaU64, obj_size: SigmaUsize) {
        let cache_idx = self.find_cache(obj_size);
        if cache_idx >= SLAB_CACHE_COUNT {
            return;
        }

        let slab_idx = self.find_slab_index(slab_addr, cache_idx);
        if slab_idx >= MAX_SLABS {
            return;
        }

        if let Some(ref mut slab) = self.slabs[cache_idx][slab_idx] {
            // Calculate object index
            let offset = ptr - slab.base_addr;
            let obj_idx = (offset / obj_size as SigmaU64) as usize;

            if obj_idx < SLAB_OBJ_PER_SLAB {
                slab.objects[obj_idx].in_use = false;
                slab.free_count += 1;
            }
        }
    }

    /// Allocate new slab (BUG-001 Fix)
    unsafe fn alloc_new_slab(&mut self, obj_size: SigmaUsize) -> Option<SigmaU64> {
        let cache_idx = self.find_cache(obj_size);
        if cache_idx >= SLAB_CACHE_COUNT {
            return None;
        }

        // Calculate slab size
        let slab_size = obj_size * SLAB_OBJ_PER_SLAB;
        
        // Calculate required order for buddy allocator
        let order = self.calculate_order(slab_size);
        
        // Allocate from buddy allocator (external function)
        let slab_addr = sigma_buddy_alloc(order as SigmaU8);
        if slab_addr == 0 {
            return None;
        }

        // Find free slab slot
        let slab_idx = self.find_free_slab_slot(cache_idx);
        if slab_idx >= MAX_SLABS {
            return None;
        }

        // Initialize slab
        let mut new_slab = Slab {
            objects: [SlabObject {
                in_use: false,
                next: None,
            }; SLAB_OBJ_PER_SLAB],
            free_count: SLAB_OBJ_PER_SLAB,
            total_count: SLAB_OBJ_PER_SLAB,
            base_addr: slab_addr,
            next: None,
        };

        // Initialize free list
        for i in 0..SLAB_OBJ_PER_SLAB {
            new_slab.objects[i].next = if i < SLAB_OBJ_PER_SLAB - 1 {
                Some(slab_addr + ((i + 1) * obj_size) as SigmaU64)
            } else {
                None
            };
        }

        self.slabs[cache_idx][slab_idx] = Some(new_slab);
        self.total_memory += slab_size as SigmaU64;
        self.used_memory += slab_size as SigmaU64;

        Some(slab_addr)
    }

    /// Find slab containing pointer (BUG-001 Fix)
    unsafe fn find_slab_for_ptr(&self, ptr: SigmaU64, obj_size: SigmaUsize) -> SigmaU64 {
        let cache_idx = self.find_cache(obj_size);
        if cache_idx >= SLAB_CACHE_COUNT {
            return 0;
        }

        // Search through all slabs in this cache
        for slab_idx in 0..MAX_SLABS {
            if let Some(ref slab) = self.slabs[cache_idx][slab_idx] {
                let slab_size = obj_size * SLAB_OBJ_PER_SLAB;
                if ptr >= slab.base_addr && ptr < slab.base_addr + slab_size as SigmaU64 {
                    return slab.base_addr;
                }
            }
        }

        0
    }
    
    /// Find slab index by address (BUG-001 Fix)
    unsafe fn find_slab_index(&self, slab_addr: SigmaU64, cache_idx: usize) -> usize {
        for i in 0..MAX_SLABS {
            if let Some(ref slab) = self.slabs[cache_idx][i] {
                if slab.base_addr == slab_addr {
                    return i;
                }
            }
        }
        MAX_SLABS
    }
    
    /// Find free slab slot (BUG-001 Fix)
    unsafe fn find_free_slab_slot(&self, cache_idx: usize) -> usize {
        for i in 0..MAX_SLABS {
            if self.slabs[cache_idx][i].is_none() {
                return i;
            }
        }
        MAX_SLABS
    }
    
    /// Calculate buddy order for size (BUG-001 Fix)
    fn calculate_order(&self, size: usize) -> usize {
        let mut order = 0;
        let mut block_size = 4096; // PAGE_SIZE
        while block_size < size {
            block_size *= 2;
            order += 1;
        }
        order
    }

    /// Get total memory used
    pub unsafe fn get_used_memory(&mut self) -> SigmaU64 {
        self.used_memory
    }

    /// Get total memory allocated
    pub unsafe fn get_total_memory(&mut self) -> SigmaU64 {
        self.total_memory
    }

    /// Get cache statistics
    pub unsafe fn get_cache_stats(&mut self, cache_idx: usize) -> Option<(SigmaUsize, SigmaU64)> {
        if cache_idx >= SLAB_CACHE_COUNT {
            return None;
        }

        let cache = &self.caches[cache_idx];
        Some((cache.slab_count, cache.free_objects))
    }

    /// Print allocator statistics
    pub unsafe fn print_stats(&mut self) {
        // TODO: Implement proper printing
        let _ = (self.total_memory, self.used_memory);
    }
}

// ─── Global Slab Allocator Instance ───────────────────────────────────────

static mut SLAB_ALLOCATOR: SlabAllocator = SlabAllocator::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_slab_init() -> SigmaI32 {
    match SLAB_ALLOCATOR.init() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_kmalloc(size: SigmaUsize) -> SigmaU64 {
    match SLAB_ALLOCATOR.kmalloc(size) {
        Some(ptr) => ptr,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_kfree(ptr: SigmaU64, size: SigmaUsize) -> SigmaI32 {
    match SLAB_ALLOCATOR.kfree(ptr, size) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_slab_get_used() -> SigmaU64 {
    SLAB_ALLOCATOR.get_used_memory()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_slab_get_total() -> SigmaU64 {
    SLAB_ALLOCATOR.get_total_memory()
}
