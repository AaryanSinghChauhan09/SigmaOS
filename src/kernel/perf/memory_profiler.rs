// SigmaOS Kernel Perf - Memory Profiler
// Tracks heap allocation patterns, memory fragmentation index, and slab size histograms.

use std::collections::BTreeMap;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct MemoryAllocationSnapshot {
    pub total_allocated_bytes: usize,
    pub total_free_bytes: usize,
    pub active_allocations: usize,
    pub largest_free_block_bytes: usize,
}

pub struct MemoryProfiler {
    pub snapshots: Vec<MemoryAllocationSnapshot>,
    pub slab_histogram: BTreeMap<usize, usize>, // Size class -> allocation count
}

impl MemoryProfiler {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            slab_histogram: BTreeMap::new(),
        }
    }

    pub fn record_allocation(&mut self, size: usize) {
        *self.slab_histogram.entry(size).or_insert(0) += 1;
    }

    pub fn record_snapshot(&mut self, allocated: usize, free: usize, active_allocs: usize, largest_free: usize) {
        self.snapshots.push(MemoryAllocationSnapshot {
            total_allocated_bytes: allocated,
            total_free_bytes: free,
            active_allocations: active_allocs,
            largest_free_block_bytes: largest_free,
        });
    }

    /// Calculates fragmentation index percentage: 100 * (1 - (largest_free_block / total_free_bytes))
    pub fn calculate_fragmentation_percentage(&self) -> f32 {
        if let Some(latest) = self.snapshots.last() {
            if latest.total_free_bytes == 0 {
                return 0.0;
            }
            let ratio = (latest.largest_free_block_bytes as f32) / (latest.total_free_bytes as f32);
            (1.0 - ratio).max(0.0) * 100.0
        } else {
            0.0
        }
    }
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_profiler_fragmentation() {
        let mut profiler = MemoryProfiler::new();

        profiler.record_allocation(64);
        profiler.record_allocation(64);
        profiler.record_allocation(128);

        assert_eq!(profiler.slab_histogram.get(&64), Some(&2));

        // 1000 total free bytes, largest contiguous free block is 600 bytes
        profiler.record_snapshot(400, 1000, 3, 600);
        let frag = profiler.calculate_fragmentation_percentage();
        assert!((frag - 40.0).abs() < 0.1); // 1 - 600/1000 = 0.4 -> 40%
    }
}
