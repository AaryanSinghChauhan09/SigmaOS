// sigma_alloc.rs — Cache-Aware Memory Allocator (SigmaAlloc)
// An advanced memory allocator tailored for modern CPU cache hierarchies (L1/L2/L3),
// NUMA node awareness, and thread-local caching to prevent lock contention.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::{vec::Vec, string::String};

// ── NUMA & Cache Topologies ────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct NumaNode {
    pub node_id: u32,
    pub cpu_mask: u64,
    pub mem_total: u64,
    pub mem_free: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum CacheLineSize {
    L1(u32),
    L2(u32),
    L3(u32),
}

const L1_CACHE_LINE: usize = 64;   // 64 bytes is standard for x86_64/ARM64

// ── Allocation Primitives ──────────────────────────────────────────────────

#[derive(Debug)]
pub struct ThreadLocalCache {
    pub cpu_id: u32,
    pub numa_node: u32,
    // Pre-allocated slabs for small allocations (< 4KB)
    pub small_slabs: Vec<usize>,
    pub free_count: usize,
}

#[derive(Debug)]
pub struct SigmaAlloc {
    pub nodes: Vec<NumaNode>,
    pub global_fallback: usize, // Represents slow-path global memory pool
}

impl SigmaAlloc {
    pub fn new() -> Self {
        SigmaAlloc {
            nodes: Vec::new(),
            global_fallback: 0,
        }
    }

    /// Register a NUMA node into the allocator awareness map
    pub fn register_numa_node(&mut self, id: u32, cpu_mask: u64, size: u64) {
        self.nodes.push(NumaNode {
            node_id: id,
            cpu_mask,
            mem_total: size,
            mem_free: size,
        });
    }

    /// Allocate a block of memory explicitly pinned to a NUMA node
    pub fn alloc_numa(&mut self, size: usize, align: usize, node_id: u32) -> Result<*mut u8, &'static str> {
        let node = self.nodes.iter_mut().find(|n| n.node_id == node_id)
            .ok_or("Invalid NUMA node")?;

        // Cache alignment enforcement
        let actual_align = if align < L1_CACHE_LINE { L1_CACHE_LINE } else { align };
        let actual_size = (size + actual_align - 1) & !(actual_align - 1);

        if node.mem_free < actual_size as u64 {
            return Err("OOM on requested NUMA node");
        }

        node.mem_free -= actual_size as u64;
        
        // In production: Return actual physical/virtual mapped address pointer
        Ok(actual_align as *mut u8) // Mock pointer
    }

    /// Allocate using thread-local cache to avoid lock contention
    pub fn alloc_tls(&mut self, tls: &mut ThreadLocalCache, size: usize) -> Result<*mut u8, &'static str> {
        if size <= 4096 && tls.free_count > 0 {
            tls.free_count -= 1;
            // Pop from thread local slab
            if let Some(addr) = tls.small_slabs.pop() {
                return Ok(addr as *mut u8);
            }
        }
        // Fallback to NUMA-aware global allocation
        self.alloc_numa(size, L1_CACHE_LINE, tls.numa_node)
    }

    /// Free a block of memory, attempting to return it to the thread-local cache
    pub fn dealloc_tls(&mut self, tls: &mut ThreadLocalCache, ptr: *mut u8, size: usize) {
        if size <= 4096 && tls.small_slabs.len() < 1024 {
            tls.small_slabs.push(ptr as usize);
            tls.free_count += 1;
        } else {
            // In production: Return to global global NUMA pool
            if let Some(node) = self.nodes.iter_mut().find(|n| n.node_id == tls.numa_node) {
                node.mem_free += size as u64;
            }
        }
    }
}
