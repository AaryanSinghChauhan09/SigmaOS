// SigmaOS Linux & BSD Inspired Memory Innovations Subsystem
// Incorporates memory management features from Linux and BSD distributions:
// 1. Linux DAMON Data Access Monitor engine (LinuxDamonMemoryMonitor)
// 2. OpenBSD otto-malloc security guard allocator (OpenBsdOttoMallocGuardEngine)
// 3. FreeBSD UMA Universal Memory Allocator per-CPU bucket cache engine (FreeBsdUmaZoneCacheEngine)

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ============================================================================
// 1. Linux DAMON Data Access Monitor Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct DamonRegion {
    pub start_addr: usize,
    pub end_addr: usize,
    pub access_frequency: u32, // Access count per sampling interval
    pub age: u32,              // Intervals since last access change
}

pub struct LinuxDamonMemoryMonitor {
    pub target_pid: u32,
    pub regions: Vec<DamonRegion>,
    pub sample_interval_ms: u32,
    pub max_regions: usize,
}

impl LinuxDamonMemoryMonitor {
    pub fn new(pid: u32, start_addr: usize, end_addr: usize, max_regions: usize) -> Self {
        let initial_region = DamonRegion {
            start_addr,
            end_addr,
            access_frequency: 0,
            age: 0,
        };

        Self {
            target_pid: pid,
            regions: vec![initial_region],
            sample_interval_ms: 5,
            max_regions,
        }
    }

    pub fn sample_accesses(&mut self, accessed_addrs: &[usize]) {
        for region in self.regions.iter_mut() {
            let mut accesses = 0;
            for &addr in accessed_addrs {
                if addr >= region.start_addr && addr < region.end_addr {
                    accesses += 1;
                }
            }

            if accesses > 0 {
                region.access_frequency += accesses;
                region.age = 0;
            } else {
                region.age += 1;
            }
        }
    }

    pub fn split_region(&mut self, region_idx: usize) -> Result<(), &'static str> {
        if region_idx >= self.regions.len() {
            return Err("DAMON: Region index out of bounds");
        }

        if self.regions.len() >= self.max_regions {
            return Err("DAMON: Max regions limit reached");
        }

        let region = &self.regions[region_idx];
        let mid = region.start_addr + (region.end_addr - region.start_addr) / 2;

        if mid <= region.start_addr {
            return Err("DAMON: Region too small to split");
        }

        let r1 = DamonRegion {
            start_addr: region.start_addr,
            end_addr: mid,
            access_frequency: region.access_frequency / 2,
            age: region.age,
        };

        let r2 = DamonRegion {
            start_addr: mid,
            end_addr: region.end_addr,
            access_frequency: region.access_frequency / 2,
            age: region.age,
        };

        self.regions.remove(region_idx);
        self.regions.insert(region_idx, r1);
        self.regions.insert(region_idx + 1, r2);

        Ok(())
    }

    pub fn merge_adjacent_regions(&mut self, freq_threshold: u32) {
        if self.regions.len() < 2 {
            return;
        }

        let mut i = 0;
        while i < self.regions.len() - 1 {
            let freq_diff = (self.regions[i].access_frequency as i32 - self.regions[i + 1].access_frequency as i32).abs();
            if (freq_diff as u32) <= freq_threshold {
                let merged_end = self.regions[i + 1].end_addr;
                let merged_freq = (self.regions[i].access_frequency + self.regions[i + 1].access_frequency) / 2;

                self.regions[i].end_addr = merged_end;
                self.regions[i].access_frequency = merged_freq;
                self.regions.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
}

impl Default for LinuxDamonMemoryMonitor {
    fn default() -> Self {
        Self::new(1000, 0x10000, 0x90000, 10)
    }
}

// ============================================================================
// 2. OpenBSD otto-malloc Security Guard Allocator Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct OttoAllocation {
    pub ptr: usize,
    pub size: usize,
    pub guard_page_before: usize,
    pub guard_page_after: usize,
    pub is_freed: bool,
}

pub struct OpenBsdOttoMallocGuardEngine {
    pub heap_base: usize,
    pub next_alloc_addr: usize,
    pub allocations: BTreeMap<usize, OttoAllocation>,
    pub junk_fill_on_free: bool,
    pub double_free_detected: bool,
}

impl OpenBsdOttoMallocGuardEngine {
    pub fn new(heap_start: usize) -> Self {
        Self {
            heap_base: heap_start,
            next_alloc_addr: heap_start,
            allocations: BTreeMap::new(),
            junk_fill_on_free: true,
            double_free_detected: false,
        }
    }

    pub fn malloc(&mut self, size: usize) -> Result<usize, &'static str> {
        if size == 0 {
            return Err("OttoMalloc: Allocation size cannot be zero");
        }

        // Align size to page (4096)
        let aligned_size = (size + 4095) & !4095;

        let guard_before = self.next_alloc_addr;
        let user_ptr = guard_before + 4096; // 1 page PROT_NONE guard page before
        let guard_after = user_ptr + aligned_size;

        self.next_alloc_addr = guard_after + 4096; // 1 page PROT_NONE guard page after

        let alloc = OttoAllocation {
            ptr: user_ptr,
            size: aligned_size,
            guard_page_before: guard_before,
            guard_page_after: guard_after,
            is_freed: false,
        };

        self.allocations.insert(user_ptr, alloc);
        Ok(user_ptr)
    }

    pub fn free(&mut self, ptr: usize) -> Result<(), &'static str> {
        if let Some(alloc) = self.allocations.get_mut(&ptr) {
            if alloc.is_freed {
                self.double_free_detected = true;
                return Err("OttoMalloc: Double-free vulnerability detected!");
            }

            alloc.is_freed = true;
            Ok(())
        } else {
            Err("OttoMalloc: Invalid pointer free attempt")
        }
    }

    pub fn verify_guard_pages(&self, ptr: usize) -> Result<bool, &'static str> {
        let alloc = self
            .allocations
            .get(&ptr)
            .ok_or("OttoMalloc: Allocation pointer not found")?;

        let valid_before = alloc.guard_page_before + 4096 == alloc.ptr;
        let valid_after = alloc.ptr + alloc.size == alloc.guard_page_after;

        Ok(valid_before && valid_after && !alloc.is_freed)
    }
}

impl Default for OpenBsdOttoMallocGuardEngine {
    fn default() -> Self {
        Self::new(0x2000000)
    }
}

// ============================================================================
// 3. FreeBSD UMA Universal Memory Allocator Per-CPU Bucket Cache Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct UmaCpuBucket {
    pub cpu_id: u32,
    pub cached_ptrs: Vec<usize>,
    pub max_capacity: usize,
}

pub struct FreeBsdUmaZoneCacheEngine {
    pub zone_name: String,
    pub item_size: usize,
    pub cpu_buckets: BTreeMap<u32, UmaCpuBucket>,
    pub zone_slab_ptrs: Vec<usize>,
    pub total_allocations: u64,
    pub bucket_hits: u64,
}

impl FreeBsdUmaZoneCacheEngine {
    pub fn new(zone_name: &str, item_size: usize, num_cpus: u32, bucket_cap: usize) -> Self {
        let mut buckets = BTreeMap::new();
        for cpu in 0..num_cpus {
            buckets.insert(
                cpu,
                UmaCpuBucket {
                    cpu_id: cpu,
                    cached_ptrs: Vec::new(),
                    max_capacity: bucket_cap,
                },
            );
        }

        Self {
            zone_name: zone_name.to_string(),
            item_size,
            cpu_buckets: buckets,
            zone_slab_ptrs: Vec::new(),
            total_allocations: 0,
            bucket_hits: 0,
        }
    }

    pub fn zalloc_cpu(&mut self, cpu_id: u32) -> Result<usize, &'static str> {
        self.total_allocations += 1;

        let bucket = self
            .cpu_buckets
            .get_mut(&cpu_id)
            .ok_or("UMA: Invalid CPU ID")?;

        if let Some(ptr) = bucket.cached_ptrs.pop() {
            self.bucket_hits += 1;
            Ok(ptr)
        } else {
            // Allocate from backing slab
            let new_ptr = 0x5000000 + (self.total_allocations as usize * self.item_size);
            self.zone_slab_ptrs.push(new_ptr);
            Ok(new_ptr)
        }
    }

    pub fn zfree_cpu(&mut self, cpu_id: u32, ptr: usize) -> Result<(), &'static str> {
        let bucket = self
            .cpu_buckets
            .get_mut(&cpu_id)
            .ok_or("UMA: Invalid CPU ID")?;

        if bucket.cached_ptrs.len() < bucket.max_capacity {
            bucket.cached_ptrs.push(ptr);
            Ok(())
        } else {
            // Bucket full, drain to slab
            Ok(())
        }
    }

    pub fn reclaim_zone_memory(&mut self) -> usize {
        let mut reclaimed = 0;
        for bucket in self.cpu_buckets.values_mut() {
            reclaimed += bucket.cached_ptrs.len();
            bucket.cached_ptrs.clear();
        }
        reclaimed
    }
}

impl Default for FreeBsdUmaZoneCacheEngine {
    fn default() -> Self {
        Self::new("uma_zone_default", 128, 4, 16)
    }
}

// ============================================================================
// 4. Master Suite Coordinator
// ============================================================================

pub struct SovereignDistroMemoryInnovationsSuite {
    pub damon: LinuxDamonMemoryMonitor,
    pub otto_malloc: OpenBsdOttoMallocGuardEngine,
    pub uma_cache: FreeBsdUmaZoneCacheEngine,
}

impl SovereignDistroMemoryInnovationsSuite {
    pub fn new() -> Self {
        Self {
            damon: LinuxDamonMemoryMonitor::default(),
            otto_malloc: OpenBsdOttoMallocGuardEngine::default(),
            uma_cache: FreeBsdUmaZoneCacheEngine::default(),
        }
    }
}

impl Default for SovereignDistroMemoryInnovationsSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_damon_memory_monitor() {
        let mut damon = LinuxDamonMemoryMonitor::new(1001, 0x10000, 0x90000, 10);
        damon.sample_accesses(&[0x15000, 0x20000, 0x50000]);

        assert_eq!(damon.regions[0].access_frequency, 3);

        assert!(damon.split_region(0).is_ok());
        assert_eq!(damon.regions.len(), 2);
        assert_eq!(damon.regions[0].start_addr, 0x10000);
        assert_eq!(damon.regions[0].end_addr, 0x50000);

        damon.merge_adjacent_regions(10);
        assert_eq!(damon.regions.len(), 1);
    }

    #[test]
    fn test_openbsd_otto_malloc_guard() {
        let mut otto = OpenBsdOttoMallocGuardEngine::new(0x2000000);
        let ptr1 = otto.malloc(1024).unwrap();

        assert!(otto.verify_guard_pages(ptr1).unwrap());
        assert!(otto.free(ptr1).is_ok());

        // Double free test
        assert!(otto.free(ptr1).is_err());
        assert!(otto.double_free_detected);
    }

    #[test]
    fn test_freebsd_uma_zone_cache() {
        let mut uma = FreeBsdUmaZoneCacheEngine::new("uma_mbufs", 256, 2, 8);

        // First allocation from slab
        let ptr1 = uma.zalloc_cpu(0).unwrap();
        assert_eq!(uma.bucket_hits, 0);

        // Free to CPU 0 bucket
        assert!(uma.zfree_cpu(0, ptr1).is_ok());

        // Second allocation hits CPU 0 bucket
        let ptr2 = uma.zalloc_cpu(0).unwrap();
        assert_eq!(ptr1, ptr2);
        assert_eq!(uma.bucket_hits, 1);

        // Test zone reclamation
        assert!(uma.zfree_cpu(1, 0x9999).is_ok());
        assert_eq!(uma.reclaim_zone_memory(), 1);
    }
}
