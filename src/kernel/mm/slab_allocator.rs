/// SigmaOS SLAB/SLUB memory allocator
/// Inspired by Bonwick's 1994 paper and the Linux kernel SLUB allocator.
/// Exposes caches for fixed-size allocations to prevent fragmentation.
use crate::klib::HashMap;
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

pub struct Slab {
    pub object_size: usize,
    pub free_list: Vec<usize>, // Indicies of free slots
    pub data: Vec<u8>,
}

impl Slab {
    pub fn new(object_size: usize, num_objects: usize) -> Self {
        let mut free_list = Vec::new();
        for i in 0..num_objects {
            free_list.push(i);
        }
        let data = vec![0u8; object_size * num_objects];
        Slab {
            object_size,
            free_list,
            data,
        }
    }

    pub fn allocate(&mut self) -> Option<usize> {
        self.free_list.pop()
    }

    pub fn deallocate(&mut self, slot: usize) {
        self.free_list.push(slot);
    }

    pub fn is_full(&self) -> bool {
        self.free_list.is_empty()
    }
}

pub struct SlabCache {
    pub object_size: usize,
    slabs: Vec<Slab>,
}

impl SlabCache {
    pub fn new(object_size: usize) -> Self {
        SlabCache {
            object_size,
            slabs: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> Option<usize> {
        for (i, slab) in self.slabs.iter_mut().enumerate() {
            if !slab.is_full() {
                let slot = slab.allocate()?;
                return Some(i * 1000 + slot); // Simple ID scheme
            }
        }

        // Add a new slab
        let mut new_slab = Slab::new(self.object_size, 64);
        let slot = new_slab.allocate()?;
        self.slabs.push(new_slab);
        Some((self.slabs.len() - 1) * 1000 + slot)
    }

    pub fn deallocate(&mut self, id: usize) {
        let slab_idx = id / 1000;
        let slot = id % 1000;
        if let Some(slab) = self.slabs.get_mut(slab_idx) {
            slab.deallocate(slot);
        }
    }
}

pub struct SlabAllocator {
    caches: HashMap<usize, SlabCache>,
}

impl SlabAllocator {
    pub fn new() -> Self {
        let mut allocator = SlabAllocator {
            caches: HashMap::new(),
        };
        // Pre-create caches for typical sizes
        allocator.create_cache(16);
        allocator.create_cache(32);
        allocator.create_cache(64);
        allocator.create_cache(128);
        allocator.create_cache(256);
        allocator
    }

    pub fn create_cache(&mut self, object_size: usize) {
        self.caches.insert(object_size, SlabCache::new(object_size));
    }

    pub fn allocate(&mut self, size: usize) -> Option<(usize, usize)> {
        // Find best fitting cache
        let mut sorted_sizes = Vec::new();
        for &k in self.caches.keys() {
            sorted_sizes.push(k);
        }

        // simple sort
        for i in 0..sorted_sizes.len() {
            for j in (i+1)..sorted_sizes.len() {
                if sorted_sizes[j] < sorted_sizes[i] {
                    let tmp = sorted_sizes[i];
                    sorted_sizes[i] = sorted_sizes[j];
                    sorted_sizes[j] = tmp;
                }
            }
        }

        for cache_size in sorted_sizes {
            if cache_size >= size {
                let cache = self.caches.get_mut(&cache_size)?;
                let id = cache.allocate()?;
                return Some((id, cache_size));
            }
        }
        None
    }

    pub fn deallocate(&mut self, cache_size: usize, id: usize) {
        if let Some(cache) = self.caches.get_mut(&cache_size) {
            cache.deallocate(id);
        }
    }
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
    fn test_slab_cache_allocations() {
        let mut allocator = SlabAllocator::new();

        let (id1, size1) = allocator.allocate(24).unwrap();
        assert_eq!(size1, 32); // should fit in 32-byte slab

        let (id2, size2) = allocator.allocate(120).unwrap();
        assert_eq!(size2, 128);

        allocator.deallocate(size1, id1);
        allocator.deallocate(size2, id2);
    }
}
