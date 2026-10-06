// SPDX-License-Identifier: MIT
// SigmaOS Zig-Inspired Memory Management
// Zig-inspired memory management with low-level optimizations

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Zig-inspired memory allocation strategies
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZigAllocationStrategy {
    BumpAllocator,
    ArenaAllocator,
    PoolAllocator,
    GeneralPurpose,
    Custom,
}

/// Zig-inspired memory block information
#[derive(Debug, Clone)]
pub struct ZigMemoryBlock {
    pub address: u64,
    pub size: u64,
    pub strategy: ZigAllocationStrategy,
    pub used: bool,
    pub alignment: u32,
    pub metadata: BTreeMap<String, String>,
}

impl ZigMemoryBlock {
    pub fn new(address: u64, size: u64, strategy: ZigAllocationStrategy) -> Self {
        Self {
            address,
            size,
            strategy,
            used: false,
            alignment: 8,
            metadata: BTreeMap::new(),
        }
    }

    /// Mark block as used
    pub fn mark_used(&mut self) {
        self.used = true;
    }

    /// Mark block as free
    pub fn mark_free(&mut self) {
        self.used = false;
    }

    /// Set alignment
    pub fn set_alignment(&mut self, alignment: u32) {
        self.alignment = alignment;
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Zig-inspired memory manager
#[derive(Debug, Clone)]
pub struct ZigMemoryManager {
    pub blocks: BTreeMap<u64, ZigMemoryBlock>,
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub default_strategy: ZigAllocationStrategy,
    pub allocation_count: u64,
    pub deallocation_count: u64,
}

impl ZigMemoryManager {
    pub fn new(total_memory: u64) -> Self {
        Self {
            blocks: BTreeMap::new(),
            total_memory,
            used_memory: 0,
            free_memory: total_memory,
            default_strategy: ZigAllocationStrategy::GeneralPurpose,
            allocation_count: 0,
            deallocation_count: 0,
        }
    }

    /// Allocate memory block
    pub fn allocate(&mut self, size: u64, strategy: ZigAllocationStrategy) -> Result<u64, &'static str> {
        if size > self.free_memory {
            return Err("Insufficient memory");
        }

        let address = self.find_free_block(size, strategy)?;
        let mut block = ZigMemoryBlock::new(address, size, strategy);
        block.mark_used();
        block.add_metadata(String::from("allocated_at"), 
            format!("{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()));

        self.blocks.insert(address, block);
        self.used_memory += size;
        self.free_memory -= size;
        self.allocation_count += 1;

        Ok(address)
    }

    /// Deallocate memory block
    pub fn deallocate(&mut self, address: u64) -> Result<(), &'static str> {
        if let Some(block) = self.blocks.get_mut(&address) {
            if !block.used {
                return Err("Block already free");
            }
            
            block.mark_free();
            self.used_memory -= block.size;
            self.free_memory += block.size;
            self.deallocation_count += 1;
            Ok(())
        } else {
            Err("Block not found")
        }
    }

    /// Find free block
    fn find_free_block(&self, _size: u64, _strategy: ZigAllocationStrategy) -> Result<u64, &'static str> {
        // In real implementation, would use actual memory layout
        // For now, return a simple address
        Ok(self.used_memory)
    }

    /// Set default allocation strategy
    pub fn set_default_strategy(&mut self, strategy: ZigAllocationStrategy) {
        self.default_strategy = strategy;
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> ZigMemoryStats {
        ZigMemoryStats {
            total: self.total_memory,
            used: self.used_memory,
            free: self.free_memory,
            fragmentation: self.calculate_fragmentation(),
            allocation_count: self.allocation_count,
            deallocation_count: self.deallocation_count,
        }
    }

    /// Calculate memory fragmentation
    fn calculate_fragmentation(&self) -> f32 {
        if self.used_memory == 0 {
            return 0.0;
        }
        
        let used_blocks: Vec<_> = self.blocks.values()
            .filter(|b| b.used)
            .collect();
        
        if used_blocks.is_empty() {
            return 0.0;
        }

        // Simple fragmentation calculation
        let total_blocks = used_blocks.len() as f32;
        let ideal_blocks = (self.used_memory / 4096) as f32; // Assume 4KB blocks
        
        if ideal_blocks == 0.0 {
            return 0.0;
        }

        (total_blocks / ideal_blocks - 1.0).max(0.0)
    }

    /// Compact memory
    pub fn compact(&mut self) -> Result<(), &'static str> {
        // In real implementation, would defragment memory
        Ok(())
    }

    /// Get block by address
    pub fn get_block(&self, address: u64) -> Option<&ZigMemoryBlock> {
        self.blocks.get(&address)
    }
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct ZigMemoryStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub fragmentation: f32,
    pub allocation_count: u64,
    pub deallocation_count: u64,
}

impl Default for ZigMemoryManager {
    fn default() -> Self {
        Self::new(1024 * 1024 * 1024) // 1GB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zig_memory_block() {
        let mut block = ZigMemoryBlock::new(0x1000, 4096, ZigAllocationStrategy::GeneralPurpose);
        block.mark_used();
        block.set_alignment(16);
        
        assert!(block.used);
        assert_eq!(block.alignment, 16);
    }

    #[test]
    fn test_zig_memory_manager() {
        let mut manager = ZigMemoryManager::new(1024 * 1024 * 1024);
        assert_eq!(manager.total_memory, 1024 * 1024 * 1024);
        
        let address = manager.allocate(4096, ZigAllocationStrategy::GeneralPurpose).unwrap();
        assert!(manager.get_block(address).is_some());
    }

    #[test]
    fn test_allocation_deallocation() {
        let mut manager = ZigMemoryManager::new(1024 * 1024);
        
        let address = manager.allocate(4096, ZigAllocationStrategy::BumpAllocator).unwrap();
        assert_eq!(manager.allocation_count, 1);
        
        manager.deallocate(address).unwrap();
        assert_eq!(manager.deallocation_count, 1);
    }

    #[test]
    fn test_memory_stats() {
        let manager = ZigMemoryManager::new(1024 * 1024);
        let stats = manager.get_stats();
        
        assert_eq!(stats.total, 1024 * 1024);
        assert_eq!(stats.free, 1024 * 1024);
    }

    #[test]
    fn test_strategy_switching() {
        let mut manager = ZigMemoryManager::new(1024 * 1024);
        manager.set_default_strategy(ZigAllocationStrategy::ArenaAllocator);
        
        assert_eq!(manager.default_strategy, ZigAllocationStrategy::ArenaAllocator);
    }

    #[test]
    fn test_metadata() {
        let mut block = ZigMemoryBlock::new(0x2000, 8192, ZigAllocationStrategy::PoolAllocator);
        block.add_metadata(String::from("owner"), String::from("kernel"));
        
        assert_eq!(block.metadata.get("owner"), Some(&String::from("kernel")));
    }
}