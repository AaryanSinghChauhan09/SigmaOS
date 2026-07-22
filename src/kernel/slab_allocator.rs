// Slab Allocator - Linux-style efficient small object allocation
// Reduces fragmentation by caching freed objects of similar sizes

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
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
    caches: BTreeMap<String, SlabCache>,
    next_slab_id: u64,
}

impl SlabAllocator {
    pub fn new() -> Self {
        Self {
            caches: BTreeMap::new(),
            next_slab_id: 0,
        }
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
        // Get object size and capacity beforehand
        let (object_size, objects_per_slab) = {
            let cache = self.caches.get(cache_name).ok_or("Cache not found")?;
            (cache.object_size, cache.objects_per_slab)
        };

        // Try to find a free object in existing slabs
        let cache = self.caches.get_mut(cache_name).ok_or("Cache not found")?;
        for slab in &mut cache.slabs {
            if slab.state != SlabState::Full {
                for obj in &mut slab.objects {
                    if obj.is_none() {
                        let next_id = self.next_slab_id;
                        let ptr = Self::allocate_memory(next_id, object_size);
                        *obj = Some(ptr);
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
                }
            }
        }

        // No free objects, create a new slab
        let next_id = self.next_slab_id;
        let new_slab = self.create_slab(next_id, objects_per_slab, object_size)?;
        let obj = new_slab.objects[0].unwrap();
        let cache = self.caches.get_mut(cache_name).ok_or("Cache not found")?;
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
                    if slab.inuse > 0 {
                        slab.inuse -= 1;
                    }
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

    /// Create a new slab for a cache
    fn create_slab(&self, next_id: u64, objects_per_slab: usize, object_size: usize) -> Result<Slab, &'static str> {
        let mut objects = Vec::with_capacity(objects_per_slab);

        for _ in 0..objects_per_slab {
            objects.push(Some(Self::allocate_memory(next_id, object_size)));
        }

        Ok(Slab {
            state: SlabState::Active,
            inuse: 1,
            objects,
        })
    }

    /// Allocate memory (simplified - would use actual allocator)
    fn allocate_memory(next_slab_id: u64, _size: usize) -> *mut u8 {
        // In a real implementation, this would use the underlying page allocator
        // For now, return a dummy pointer
        (0x2000 + next_slab_id as usize) as *mut u8
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
        assert_eq!(allocator.cache_count(), 1);
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
    fn test_cache_stats() {
        let mut allocator = SlabAllocator::new();

        allocator
            .create_cache("task_struct".to_string(), 512, 8)
            .unwrap();

        allocator.allocate("task_struct").unwrap();
        allocator.allocate("task_struct").unwrap();

        let stats = allocator.get_cache_stats("task_struct").unwrap();
        assert_eq!(stats.used_objects, 2);
    }

    #[test]
    fn test_shrink_cache() {
        let mut allocator = SlabAllocator::new();

        allocator
            .create_cache("task_struct".to_string(), 512, 8)
            .unwrap();

        let obj1 = allocator.allocate("task_struct").unwrap();
        let obj2 = allocator.allocate("task_struct").unwrap();

        allocator.free("task_struct", obj1).unwrap();
        allocator.free("task_struct", obj2).unwrap();

        let removed = allocator.shrink_cache("task_struct").unwrap();
        assert!(removed > 0);
    }

    #[test]
    fn test_destroy_cache() {
        let mut allocator = SlabAllocator::new();

        allocator
            .create_cache("task_struct".to_string(), 512, 8)
            .unwrap();
        allocator.destroy_cache("task_struct").unwrap();

        assert_eq!(allocator.cache_count(), 0);
    }
}
