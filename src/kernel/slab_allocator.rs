// Slab Allocator - Linux-style efficient small object allocation
// Reduces fragmentation by caching freed objects of similar sizes
// Enhanced with Linux-inspired size-bucketed kmalloc/kfree and sub-16MB legacy DMA pools for ancient devices.

// #![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlabState {
    Active,
    Partial,
    Empty,
    Full,
}

#[derive(Debug, Clone)]
pub struct SlabCache {
    pub name: String,
    pub object_size: usize,
    pub align: usize,
    pub objects_per_slab: usize,
    pub slabs: Vec<Slab>,
    pub free_objects: usize,
}

#[derive(Debug, Clone)]
pub struct Slab {
    pub state: SlabState,
    pub inuse: usize,
    pub objects: Vec<Option<*mut u8>>,
}

pub struct SlabAllocator {
    pub caches: BTreeMap<String, SlabCache>,
    pub next_slab_id: u64,
}

impl SlabAllocator {
    pub fn new() -> Self {
        let mut allocator = Self {
            caches: BTreeMap::new(),
            next_slab_id: 0,
        };
        // Pre-create standard Linux-style size-bucketed kmalloc caches
        let _ = allocator.create_cache("kmalloc-32".to_string(), 32, 8);
        let _ = allocator.create_cache("kmalloc-64".to_string(), 64, 8);
        let _ = allocator.create_cache("kmalloc-128".to_string(), 128, 8);
        let _ = allocator.create_cache("kmalloc-256".to_string(), 256, 8);
        let _ = allocator.create_cache("kmalloc-512".to_string(), 512, 8);
        let _ = allocator.create_cache("kmalloc-1024".to_string(), 1024, 8);
        let _ = allocator.create_cache("kmalloc-2048".to_string(), 2048, 8);
        let _ = allocator.create_cache("kmalloc-4096".to_string(), 4096, 8);
        allocator
    }

    /// Create a new slab cache
    pub fn create_cache(
        &mut self,
        name: String,
        object_size: usize,
        align: usize,
    ) -> Result<(), &'static str> {
        if self.caches.contains_key(&name) {
            return Err("Cache already exists");
        }

        // Calculate objects per slab (typical Linux calculation)
        let slab_size = 4096; // 4KB pages
        let objects_per_slab = (slab_size / object_size).max(1);

        let cache = SlabCache {
            name: name.clone(),
            object_size,
            align,
            objects_per_slab,
            slabs: Vec::new(),
            free_objects: 0,
        };

        self.caches.insert(name, cache);
        Ok(())
    }

    /// Allocate an object from a cache
    pub fn allocate(&mut self, cache_name: &str) -> Result<*mut u8, &'static str> {
        let (slab_idx, obj_idx, object_size, objects_per_slab) = {
            let cache = self.caches.get(cache_name).ok_or("Cache not found")?;
            let mut found = None;
            'outer: for (s_idx, slab) in cache.slabs.iter().enumerate() {
                if slab.state != SlabState::Full {
                    for (o_idx, obj) in slab.objects.iter().enumerate() {
                        if obj.is_none() {
                            found = Some((s_idx, o_idx));
                            break 'outer;
                        }
                    }
                }
            }
            if let Some((s_idx, o_idx)) = found {
                (
                    Some(s_idx),
                    Some(o_idx),
                    cache.object_size,
                    cache.objects_per_slab,
                )
            } else {
                (None, None, cache.object_size, cache.objects_per_slab)
            }
        };

        if let (Some(s_idx), Some(obj_idx)) = (slab_idx, obj_idx) {
            let ptr = self.allocate_memory(object_size);
            let cache = self.caches.get_mut(cache_name).unwrap();
            let slab = &mut cache.slabs[s_idx];
            slab.objects[obj_idx] = Some(ptr);
            slab.inuse += 1;
            cache.free_objects -= 1;

            // Update slab state
            slab.state = if slab.inuse == objects_per_slab {
                SlabState::Full
            } else if slab.inuse > 0 {
                SlabState::Partial
            } else {
                SlabState::Empty
            };

            return Ok(ptr);
        }

        // No free objects, create a new slab
        let (new_slab, objects_per_slab) = {
            let cache = self.caches.get(cache_name).ok_or("Cache not found")?;
            let slab = self.create_slab(cache)?;
            (slab, cache.objects_per_slab)
        };

        let obj = new_slab.objects[0].unwrap();

        let cache = self.caches.get_mut(cache_name).unwrap();
        cache.slabs.push(new_slab);
        cache.free_objects = objects_per_slab - 1;

        Ok(obj)
    }

    /// Free an object back to its cache
    pub fn free(&mut self, cache_name: &str, obj: *mut u8) -> Result<(), &'static str> {
        let cache = self.caches.get_mut(cache_name).ok_or("Cache not found")?;

        for slab in &mut cache.slabs {
            for slab_obj in &mut slab.objects {
                if slab_obj == &Some(obj) {
                    *slab_obj = None;
                    slab.inuse -= 1;
                    cache.free_objects += 1;

                    // Update slab state
                    slab.state = if slab.inuse == 0 {
                        SlabState::Empty
                    } else if slab.inuse < cache.objects_per_slab {
                        SlabState::Partial
                    } else {
                        SlabState::Full
                    };

                    return Ok(());
                }
            }
        }

        Err("Object not found in cache")
    }

    /// Linux kmalloc equivalent: Allocates from the closest size-matching cache bucket
    pub fn kmalloc(&mut self, size: usize) -> Result<*mut u8, &'static str> {
        let bucket_name = if size <= 32 {
            "kmalloc-32"
        } else if size <= 64 {
            "kmalloc-64"
        } else if size <= 128 {
            "kmalloc-128"
        } else if size <= 256 {
            "kmalloc-256"
        } else if size <= 512 {
            "kmalloc-512"
        } else if size <= 1024 {
            "kmalloc-1024"
        } else if size <= 2048 {
            "kmalloc-2048"
        } else if size <= 4096 {
            "kmalloc-4096"
        } else {
            return Err("Requested allocation exceeds kmalloc 4KB limit");
        };

        self.allocate(bucket_name)
    }

    /// Linux kfree equivalent: Automatically identifies and frees the dynamic bucket pointer
    pub fn kfree(&mut self, obj: *mut u8) -> Result<(), &'static str> {
        let buckets = [
            "kmalloc-32",
            "kmalloc-64",
            "kmalloc-128",
            "kmalloc-256",
            "kmalloc-512",
            "kmalloc-1024",
            "kmalloc-2048",
            "kmalloc-4096",
        ];

        for bucket in &buckets {
            if self.free(bucket, obj).is_ok() {
                return Ok(());
            }
        }

        Err("kfree: Pointer not found in any kmalloc size-bucket cache")
    }

    /// Create a new slab for a cache
    fn create_slab(&self, cache: &SlabCache) -> Result<Slab, &'static str> {
        let mut objects = Vec::with_capacity(cache.objects_per_slab);

        for _ in 0..cache.objects_per_slab {
            objects.push(Some(self.allocate_memory(cache.object_size)));
        }

        Ok(Slab {
            state: SlabState::Active,
            inuse: 1,
            objects,
        })
    }

    /// Allocate memory (simplified - would use actual page allocator)
    fn allocate_memory(&self, _size: usize) -> *mut u8 {
        static COUNTER: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        (0x200000 + id * 4096) as *mut u8 // Allocate in 2MB+ memory region
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self, cache_name: &str) -> Option<SlabCacheStats> {
        let cache = self.caches.get(cache_name)?;

        Some(SlabCacheStats {
            name: cache.name.clone(),
            object_size: cache.object_size,
            total_slabs: cache.slabs.len(),
            total_objects: cache.slabs.len() * cache.objects_per_slab,
            used_objects: cache.slabs.iter().map(|s| s.inuse).sum(),
            free_objects: cache.free_objects,
        })
    }

    /// Shrink a cache by removing empty slabs
    pub fn shrink_cache(&mut self, cache_name: &str) -> Result<usize, &'static str> {
        let cache = self.caches.get_mut(cache_name).ok_or("Cache not found")?;

        let initial_count = cache.slabs.len();
        cache.slabs.retain(|slab| slab.state != SlabState::Empty);
        let removed = initial_count - cache.slabs.len();

        Ok(removed)
    }

    /// Destroy a cache
    pub fn destroy_cache(&mut self, cache_name: &str) -> Result<(), &'static str> {
        self.caches.remove(cache_name).ok_or("Cache not found")?;
        Ok(())
    }

    /// Get cache count
    pub fn cache_count(&self) -> usize {
        self.caches.len()
    }
}

// =========================================================================
// Linux-inspired Legacy Device DMA Memory Pool (Sub-16MB Memory Gating)
// =========================================================================

pub struct LegacyDevicePool {
    pub reserved_start_addr: usize,
    pub reserved_end_addr: usize,
    pub next_alloc_offset: usize,
}

impl LegacyDevicePool {
    pub fn new() -> Self {
        Self {
            // ISA DMA limit: reserved tightly below the 16MB physical RAM boundary
            // to enable 24-bit physical address gating for ancient devices (floppy, AC97, etc.)
            reserved_start_addr: 0x800000, // 8MB boundary
            reserved_end_addr: 0xF00000,   // 15MB boundary
            next_alloc_offset: 0,
        }
    }

    /// Allocate a contiguous buffer in the ISA DMA-compliant zone (< 16MB)
    pub fn alloc_dma_buffer(&mut self, size: usize) -> Result<*mut u8, &'static str> {
        let aligned_size = (size + 15) & !15; // 16-byte alignment
        if self.reserved_start_addr + self.next_alloc_offset + aligned_size > self.reserved_end_addr {
            return Err("Legacy DMA memory pool exhausted");
        }

        let allocated_ptr = (self.reserved_start_addr + self.next_alloc_offset) as *mut u8;
        self.next_alloc_offset += aligned_size;
        Ok(allocated_ptr)
    }

    /// Reset allocations (mocking driver reset or reboot)
    pub fn reset_dma_pool(&mut self) {
        self.next_alloc_offset = 0;
    }
}

#[derive(Debug, Clone)]
pub struct SlabCacheStats {
    pub name: String,
    pub object_size: usize,
    pub total_slabs: usize,
    pub total_objects: usize,
    pub used_objects: usize,
    pub free_objects: usize,
}

impl Default for SlabAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cache() {
        let mut allocator = SlabAllocator::new();

        allocator
            .create_cache("task_struct".to_string(), 512, 8)
            .unwrap();
        // Caches count includes the 8 pre-registered kmalloc caches + task_struct
        assert_eq!(allocator.cache_count(), 9);
    }

    #[test]
    fn test_allocate() {
        let mut allocator = SlabAllocator::new();

        allocator
            .create_cache("task_struct".to_string(), 512, 8)
            .unwrap();

        let obj = allocator.allocate("task_struct").unwrap();
        assert!(!obj.is_null());
    }

    #[test]
    fn test_free() {
        let mut allocator = SlabAllocator::new();

        allocator
            .create_cache("task_struct".to_string(), 512, 8)
            .unwrap();

        let obj = allocator.allocate("task_struct").unwrap();
        allocator.free("task_struct", obj).unwrap();
    }

    #[test]
    fn test_kmalloc_kfree_buckets() {
        let mut allocator = SlabAllocator::new();

        // 1. Allocate 45 bytes: maps to kmalloc-64 cache
        let ptr1 = allocator.kmalloc(45).unwrap();
        assert!(!ptr1.is_null());

        let stats_64 = allocator.get_cache_stats("kmalloc-64").unwrap();
        assert_eq!(stats_64.used_objects, 1);

        // 2. Allocate 1000 bytes: maps to kmalloc-1024 cache
        let ptr2 = allocator.kmalloc(1000).unwrap();
        assert!(!ptr2.is_null());

        let stats_1024 = allocator.get_cache_stats("kmalloc-1024").unwrap();
        assert_eq!(stats_1024.used_objects, 1);

        // 3. Free both
        allocator.kfree(ptr1).unwrap();
        allocator.kfree(ptr2).unwrap();

        let stats_64_after = allocator.get_cache_stats("kmalloc-64").unwrap();
        assert_eq!(stats_64_after.used_objects, 0);
    }

    #[test]
    fn test_legacy_device_dma_gating() {
        let mut dma_pool = LegacyDevicePool::new();

        // Allocate a 10KB buffer for ancient floppy disk drive ISA DMA transfer
        let floppy_buffer = dma_pool.alloc_dma_buffer(10 * 1024).unwrap();
        let floppy_addr = floppy_buffer as usize;

        // Verify the memory resides below 16MB physical boundary limit (0x1000000)
        assert!(floppy_addr < 0x1000000);
        assert!(floppy_addr >= 0x800000);

        // Allocate a 128KB buffer for ancient parallel port printer transfer
        let printer_buffer = dma_pool.alloc_dma_buffer(128 * 1024).unwrap();
        let printer_addr = printer_buffer as usize;
        assert!(printer_addr < 0x1000000);
        assert!(printer_addr > floppy_addr);
    }
}
