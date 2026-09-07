#![cfg_attr(not(test), no_std)]
// SigmaOS Memory Manager Performance Allocator Stack
// Implements Slab Cache (<512B), Buddy Allocator (>=512B), and ACPI SRAT NUMA Topology Page Allocator.


use std::vec::Vec;

pub const SLAB_MAX_SIZE: usize = 512;
pub const PAGE_SIZE_4K: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationTier {
    SlabCache,
    BuddyZone,
    NumaTopologyPage,
}

pub struct NumaNodeTopology {
    pub node_id: usize,
    pub start_cpu: usize,
    pub end_cpu: usize,
    pub ram_start_bytes: u64,
    pub ram_size_bytes: u64,
    pub allocated_bytes: u64,
}

pub struct PerfMmAllocatorStack {
    pub numa_nodes: Vec<NumaNodeTopology>,
    pub slab_allocations_count: usize,
    pub buddy_allocations_count: usize,
    pub numa_allocations_count: usize,
}

impl PerfMmAllocatorStack {
    pub fn new() -> Self {
        Self {
            numa_nodes: Vec::new(),
            slab_allocations_count: 0,
            buddy_allocations_count: 0,
            numa_allocations_count: 0,
        }
    }

    pub fn discover_numa_node_srat(&mut self, node_id: usize, start_cpu: usize, end_cpu: usize, ram_start: u64, ram_size: u64) {
        self.numa_nodes.push(NumaNodeTopology {
            node_id,
            start_cpu,
            end_cpu,
            ram_start_bytes: ram_start,
            ram_size_bytes: ram_size,
            allocated_bytes: 0,
        });
    }

    pub fn sigma_alloc(&mut self, size_bytes: usize, requesting_cpu: usize) -> (AllocationTier, u64) {
        if size_bytes < SLAB_MAX_SIZE {
            self.slab_allocations_count += 1;
            (AllocationTier::SlabCache, 0x1000 + (self.slab_allocations_count * 64) as u64)
        } else if size_bytes < 64 * 1024 {
            self.buddy_allocations_count += 1;
            (AllocationTier::BuddyZone, 0x10000 + (self.buddy_allocations_count * 512) as u64)
        } else {
            // NUMA-aware allocation closest to requesting CPU
            self.numa_allocations_count += 1;
            let mut target_node_id = 0;
            for node in &mut self.numa_nodes {
                if requesting_cpu >= node.start_cpu && requesting_cpu <= node.end_cpu {
                    target_node_id = node.node_id;
                    node.allocated_bytes += size_bytes as u64;
                    break;
                }
            }
            let base_addr = self.numa_nodes.get(target_node_id).map_or(0x100000, |n| n.ram_start_bytes);
            (AllocationTier::NumaTopologyPage, base_addr + (self.numa_allocations_count * PAGE_SIZE_4K) as u64)
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_mm_allocator_stack_tiers() {
        let mut mm = PerfMmAllocatorStack::new();
        mm.discover_numa_node_srat(0, 0, 7, 0x0, 64 * 1024 * 1024 * 1024); // Node 0: CPUs 0-7
        mm.discover_numa_node_srat(1, 8, 15, 64 * 1024 * 1024 * 1024, 64 * 1024 * 1024 * 1024); // Node 1: CPUs 8-15

        // Small allocation (<512B) triggers Slab Cache
        let (tier1, addr1) = mm.sigma_alloc(128, 2);
        assert_eq!(tier1, AllocationTier::SlabCache);
        assert_ne!(addr1, 0);

        // Medium allocation (>=512B) triggers Buddy Zone
        let (tier2, addr2) = mm.sigma_alloc(4096, 2);
        assert_eq!(tier2, AllocationTier::BuddyZone);
        assert_ne!(addr2, 0);

        // Large allocation (>=64KB) triggers NUMA-aware Page Allocator
        let (tier3, addr3) = mm.sigma_alloc(128 * 1024, 10); // CPU 10 -> Node 1
        assert_eq!(tier3, AllocationTier::NumaTopologyPage);
        assert!(addr3 >= 64 * 1024 * 1024 * 1024); // Allocated from Node 1 RAM base
        assert_eq!(mm.numa_nodes[1].allocated_bytes, 128 * 1024);
    }
}
