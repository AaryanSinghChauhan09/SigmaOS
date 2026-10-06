// SPDX-License-Identifier: MIT
// SigmaOS Nim-Inspired System Components
// Nim programming language-inspired memory management and garbage collection

use std::collections::BTreeMap;
use std::vec::Vec;

/// Nim-inspired garbage collection strategies
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NimGCStrategy {
    DeferredGC,
    IncrementalGC,
    GenerationalGC,
    ConcurrentGC,
    ManualGC,
}

/// Nim-inspired memory allocation
#[derive(Debug, Clone)]
pub struct NimMemoryAllocation {
    pub address: u64,
    pub size: u64,
    pub allocated: bool,
    pub ref_count: u32,
    pub marked: bool,
    pub allocation_type: NimAllocationType,
}

/// Nim-inspired allocation types
#[derive(Debug, Clone, PartialEq)]
pub enum NimAllocationType {
    Stack,
    Heap,
    Static,
    Closure,
    Sequence,
    Object,
}

impl NimMemoryAllocation {
    pub fn new(address: u64, size: u64, allocation_type: NimAllocationType) -> Self {
        Self {
            address,
            size,
            allocated: true,
            ref_count: 1,
            marked: false,
            allocation_type,
        }
    }

    /// Increment reference count
    pub fn increment_ref(&mut self) {
        self.ref_count += 1;
    }

    /// Decrement reference count
    pub fn decrement_ref(&mut self) {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
    }

    /// Mark for garbage collection
    pub fn mark(&mut self) {
        self.marked = true;
    }

    /// Unmark
    pub fn unmark(&mut self) {
        self.marked = false;
    }

    /// Check if allocation is safe to free
    pub fn can_free(&self) -> bool {
        self.ref_count == 0 && !self.marked
    }
}

/// Nim-inspired garbage collector
#[derive(Debug, Clone)]
pub struct NimGarbageCollector {
    pub allocations: BTreeMap<u64, NimMemoryAllocation>,
    pub gc_strategy: NimGCStrategy,
    pub gc_threshold: u64,
    pub gc_counter: u64,
    pub collections_performed: u64,
    pub memory_freed: u64,
}

impl NimGarbageCollector {
    pub fn new(gc_strategy: NimGCStrategy) -> Self {
        Self {
            allocations: BTreeMap::new(),
            gc_strategy,
            gc_threshold: 1024 * 1024, // 1MB default
            gc_counter: 0,
            collections_performed: 0,
            memory_freed: 0,
        }
    }

    /// Allocate memory
    pub fn allocate(&mut self, size: u64, allocation_type: NimAllocationType) -> Result<u64, &'static str> {
        let address = self.find_free_address(size)?;
        let allocation = NimMemoryAllocation::new(address, size, allocation_type);
        self.allocations.insert(address, allocation);
        
        // Check if GC should run
        self.check_gc_trigger();
        
        Ok(address)
    }

    /// Deallocate memory
    pub fn deallocate(&mut self, address: u64) -> Result<(), &'static str> {
        if let Some(allocation) = self.allocations.get_mut(&address) {
            allocation.decrement_ref();
            
            if allocation.can_free() {
                let freed_size = allocation.size;
                self.allocations.remove(&address);
                self.memory_freed += freed_size;
            }
            Ok(())
        } else {
            Err("Allocation not found")
        }
    }

    /// Find free address
    fn find_free_address(&self, size: u64) -> Result<u64, &'static str> {
        // In real implementation, would use actual memory layout
        let address = self.allocations.len() as u64 * size;
        Ok(address)
    }

    /// Run garbage collection
    pub fn run_gc(&mut self) -> Result<u64, &'static str> {
        match self.gc_strategy {
            NimGCStrategy::DeferredGC => self.deferred_gc(),
            NimGCStrategy::IncrementalGC => self.incremental_gc(),
            NimGCStrategy::GenerationalGC => self.generational_gc(),
            NimGCStrategy::ConcurrentGC => self.concurrent_gc(),
            NimGCStrategy::ManualGC => self.manual_gc(),
        }
    }

    /// Deferred garbage collection
    fn deferred_gc(&mut self) -> Result<u64, &'static str> {
        let mut freed = 0;
        let mut to_remove = Vec::new();
        
        for (address, allocation) in &self.allocations {
            if allocation.can_free() {
                to_remove.push(*address);
                freed += allocation.size;
            }
        }
        
        for address in to_remove {
            if let Some(allocation) = self.allocations.remove(&address) {
                self.memory_freed += allocation.size;
            }
        }
        
        self.collections_performed += 1;
        Ok(freed)
    }

    /// Incremental garbage collection
    fn incremental_gc(&mut self) -> Result<u64, &'static str> {
        // Process a subset of allocations
        let max_to_process = (self.allocations.len() / 10).max(1);
        let mut processed = 0;
        let mut freed = 0;
        let mut to_remove = Vec::new();
        
        for (address, allocation) in &self.allocations {
            if processed >= max_to_process {
                break;
            }
            
            if allocation.can_free() {
                to_remove.push(*address);
                freed += allocation.size;
            }
            processed += 1;
        }
        
        for address in to_remove {
            if let Some(allocation) = self.allocations.remove(&address) {
                self.memory_freed += allocation.size;
            }
        }
        
        self.collections_performed += 1;
        Ok(freed)
    }

    /// Generational garbage collection
    fn generational_gc(&mut self) -> Result<u64, &'static str> {
        // Collect young generation first
        let mut freed = 0;
        let mut to_remove = Vec::new();
        
        for (address, allocation) in &self.allocations {
            if allocation.can_free() && allocation.allocation_type == NimAllocationType::Stack {
                to_remove.push(*address);
                freed += allocation.size;
            }
        }
        
        for address in to_remove {
            if let Some(allocation) = self.allocations.remove(&address) {
                self.memory_freed += allocation.size;
            }
        }
        
        self.collections_performed += 1;
        Ok(freed)
    }

    /// Concurrent garbage collection
    fn concurrent_gc(&mut self) -> Result<u64, &'static str> {
        // Simulated concurrent GC
        let freed = self.deferred_gc()?;
        self.collections_performed += 1;
        Ok(freed)
    }

    /// Manual garbage collection
    fn manual_gc(&mut self) -> Result<u64, &'static str> {
        let freed = self.deferred_gc()?;
        self.collections_performed += 1;
        Ok(freed)
    }

    /// Check if GC should be triggered
    fn check_gc_trigger(&mut self) {
        self.gc_counter += 1;
        
        if self.gc_counter >= self.gc_threshold {
            let _ = self.run_gc();
            self.gc_counter = 0;
        }
    }

    /// Set GC threshold
    pub fn set_gc_threshold(&mut self, threshold: u64) {
        self.gc_threshold = threshold;
    }

    /// Get GC statistics
    pub fn get_gc_stats(&self) -> NimGCStats {
        let total_allocations = self.allocations.len();
        let total_memory: u64 = self.allocations.values().map(|a| a.size).sum();
        
        NimGCStats {
            total_allocations,
            total_memory,
            collections_performed: self.collections_performed,
            memory_freed: self.memory_freed,
            gc_strategy: self.gc_strategy,
        }
    }

    /// Mark allocation as reachable
    pub fn mark_allocation(&mut self, address: u64) -> Result<(), &'static str> {
        if let Some(allocation) = self.allocations.get_mut(&address) {
            allocation.mark();
            Ok(())
        } else {
            Err("Allocation not found")
        }
    }

    /// Sweep unmarked allocations
    pub fn sweep(&mut self) -> Result<u64, &'static str> {
        let mut to_remove = Vec::new();
        let mut freed = 0;
        
        for (address, allocation) in &self.allocations {
            if !allocation.marked && allocation.can_free() {
                to_remove.push(*address);
                freed += allocation.size;
            }
        }
        
        for address in to_remove {
            if let Some(allocation) = self.allocations.remove(&address) {
                self.memory_freed += allocation.size;
            }
        }
        
        // Unmark all remaining allocations
        for allocation in self.allocations.values_mut() {
            allocation.unmark();
        }
        
        Ok(freed)
    }
}

/// Nim-inspired GC statistics
#[derive(Debug, Clone)]
pub struct NimGCStats {
    pub total_allocations: usize,
    pub total_memory: u64,
    pub collections_performed: u64,
    pub memory_freed: u64,
    pub gc_strategy: NimGCStrategy,
}

/// Nim-inspired reference tracking
#[derive(Debug, Clone)]
pub struct NimReferenceTracker {
    pub references: BTreeMap<u64, Vec<u64>>,
}

impl NimReferenceTracker {
    pub fn new() -> Self {
        Self {
            references: BTreeMap::new(),
        }
    }

    /// Add reference
    pub fn add_reference(&mut self, from: u64, to: u64) {
        self.references.entry(from).or_insert_with(Vec::new).push(to);
    }

    /// Remove reference
    pub fn remove_reference(&mut self, from: u64, to: u64) {
        if let Some(refs) = self.references.get_mut(&from) {
            refs.retain(|&r| r != to);
        }
    }

    /// Get all references from allocation
    pub fn get_references(&self, from: u64) -> Option<&Vec<u64>> {
        self.references.get(&from)
    }

    /// Check for cycles
    pub fn detect_cycles(&self) -> Vec<Vec<u64>> {
        let mut cycles = Vec::new();
        let mut visited = Vec::new();
        
        for &start in self.references.keys() {
            if !visited.contains(&start) {
                if let Some(cycle) = self.find_cycle(start, &mut visited) {
                    cycles.push(cycle);
                }
            }
        }
        
        cycles
    }

    /// Find cycle starting from node
    fn find_cycle(&self, start: u64, visited: &mut Vec<u64>) -> Option<Vec<u64>> {
        let mut path = Vec::new();
        let mut current = start;
        let mut seen = BTreeMap::new();
        
        loop {
            if seen.contains_key(&current) {
                let cycle_start = seen[&current];
                let cycle = path[cycle_start..].to_vec();
                return Some(cycle);
            }
            
            if visited.contains(&current) {
                return None;
            }
            
            seen.insert(current, path.len());
            path.push(current);
            visited.push(current);
            
            if let Some(refs) = self.references.get(&current) {
                if let Some(&next) = refs.first() {
                    current = next;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
}

impl Default for NimReferenceTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nim_memory_allocation() {
        let mut alloc = NimMemoryAllocation::new(
            0x1000,
            4096,
            NimAllocationType::Heap
        );
        
        alloc.increment_ref();
        assert_eq!(alloc.ref_count, 2);
        
        alloc.decrement_ref();
        assert_eq!(alloc.ref_count, 1);
        
        alloc.mark();
        assert!(alloc.marked);
    }

    #[test]
    fn test_garbage_collector() {
        let mut gc = NimGarbageCollector::new(NimGCStrategy::DeferredGC);
        
        let addr1 = gc.allocate(4096, NimAllocationType::Heap).unwrap();
        let _addr2 = gc.allocate(8192, NimAllocationType::Stack).unwrap();
        
        assert_eq!(gc.allocations.len(), 2);
        
        // Set allocation to be freeable
        if let Some(alloc) = gc.allocations.get_mut(&addr1) {
            alloc.ref_count = 0;
            alloc.marked = false;
        }
        
        let freed = gc.run_gc().unwrap();
        
        assert!(freed > 0);
        assert_eq!(gc.allocations.len(), 1);
    }

    #[test]
    fn test_reference_tracking() {
        let mut tracker = NimReferenceTracker::new();
        
        tracker.add_reference(0x1000, 0x2000);
        tracker.add_reference(0x1000, 0x3000);
        
        let refs = tracker.get_references(0x1000);
        assert!(refs.is_some());
        assert_eq!(refs.unwrap().len(), 2);
    }

    #[test]
    fn test_gc_strategies() {
        let mut gc = NimGarbageCollector::new(NimGCStrategy::IncrementalGC);
        gc.allocate(4096, NimAllocationType::Heap).unwrap();
        
        let stats = gc.get_gc_stats();
        assert_eq!(stats.gc_strategy, NimGCStrategy::IncrementalGC);
    }

    #[test]
    fn test_mark_sweep() {
        let mut gc = NimGarbageCollector::new(NimGCStrategy::ManualGC);
        
        let addr1 = gc.allocate(4096, NimAllocationType::Heap).unwrap();
        let addr2 = gc.allocate(8192, NimAllocationType::Heap).unwrap();
        
        gc.mark_allocation(addr1).unwrap();
        
        // Set addr2 to be freeable
        if let Some(alloc) = gc.allocations.get_mut(&addr2) {
            alloc.ref_count = 0;
            alloc.marked = false;
        }
        
        let freed = gc.sweep().unwrap();
        assert!(freed > 0);
        assert_eq!(gc.allocations.len(), 1);
    }
}