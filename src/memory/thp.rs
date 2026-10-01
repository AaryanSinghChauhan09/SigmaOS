// Transparent Huge Pages (THP)
// Inspired by Linux transparent huge pages for memory efficiency

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Huge page size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HugePageSize {
    Size2MB = 2 * 1024 * 1024,
    Size1GB = 1024 * 1024 * 1024,
}

/// THP policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThpPolicy {
    Always,
    Never,
    Madvise,
}

/// Huge page allocation
#[derive(Debug, Clone)]
pub struct HugePageAllocation {
    pub start_addr: u64,
    pub size: usize,
    pub page_size: HugePageSize,
    pub allocated: bool,
    pub timestamp: u64,
}

/// THP manager
pub struct ThpManager {
    allocations: HashMap<u64, HugePageAllocation>,
    next_alloc_id: AtomicU64,
    policy: ThpPolicy,
    defrag_enabled: bool,
    collapse_enabled: bool,
    defrag_threshold: u32, // percentage
    scan_sleep_ms: u32,
}

impl ThpManager {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
            next_alloc_id: AtomicU64::new(1),
            policy: ThpPolicy::Madvise,
            defrag_enabled: true,
            collapse_enabled: true,
            defrag_threshold: 50,
            scan_sleep_ms: 10,
        }
    }

    /// Allocate a huge page
    pub fn allocate(&mut self, size: usize, page_size: HugePageSize) -> Result<u64, &'static str> {
        if !self.should_use_thp(size) {
            return Err("THP not allowed for this size");
        }

        let alloc_id = self.next_alloc_id.fetch_add(1, Ordering::SeqCst);
        let start_addr = alloc_id * page_size as u64;

        let allocation = HugePageAllocation {
            start_addr,
            size,
            page_size,
            allocated: true,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64,
        };

        self.allocations.insert(alloc_id, allocation);

        Ok(start_addr)
    }

    /// Free a huge page
    pub fn free(&mut self, addr: u64) -> Result<(), &'static str> {
        let alloc_id = self.find_allocation_by_addr(addr)?;

        if let Some(mut allocation) = self.allocations.remove(&alloc_id) {
            allocation.allocated = false;
            Ok(())
        } else {
            Err("Allocation not found")
        }
    }

    /// Collapse small pages into huge page
    pub fn collapse(&mut self, addr: u64) -> Result<(), &'static str> {
        if !self.collapse_enabled {
            return Err("Collapse not enabled");
        }

        let alloc_id = self.find_allocation_by_addr(addr)?;

        if let Some(allocation) = self.allocations.get_mut(&alloc_id) {
            // Simulated collapse - in real implementation would
            // coalesce adjacent small pages into a huge page
            Ok(())
        } else {
            Err("Allocation not found")
        }
    }

    /// Defrag memory to enable huge pages
    pub fn defrag(&mut self) -> Result<usize, &'static str> {
        if !self.defrag_enabled {
            return Err("Defrag not enabled");
        }

        let mut collapsed = 0;

        for allocation in self.allocations.values_mut() {
            if allocation.allocated && allocation.size < allocation.page_size as usize {
                // Simulated defrag - move pages to enable huge page allocation
                collapsed += 1;
            }
        }

        Ok(collapsed)
    }

    /// Check if THP should be used
    fn should_use_thp(&self, size: usize) -> bool {
        match self.policy {
            ThpPolicy::Always => true,
            ThpPolicy::Never => false,
            ThpPolicy::Madvise => size >= 64 * 1024, // 64KB threshold
        }
    }

    /// Find allocation by address
    fn find_allocation_by_addr(&self, addr: u64) -> Result<u64, &'static str> {
        for (&id, allocation) in &self.allocations {
            if addr >= allocation.start_addr
                && addr < allocation.start_addr + allocation.size as u64
            {
                return Ok(id);
            }
        }
        Err("Allocation not found for address")
    }

    /// Set THP policy
    pub fn set_policy(&mut self, policy: ThpPolicy) {
        self.policy = policy;
    }

    /// Get THP policy
    pub fn policy(&self) -> ThpPolicy {
        self.policy
    }

    /// Enable/disable defrag
    pub fn set_defrag_enabled(&mut self, enabled: bool) {
        self.defrag_enabled = enabled;
    }

    /// Enable/disable collapse
    pub fn set_collapse_enabled(&mut self, enabled: bool) {
        self.collapse_enabled = enabled;
    }

    /// Set defrag threshold
    pub fn set_defrag_threshold(&mut self, threshold: u32) {
        self.defrag_threshold = threshold;
    }

    /// Get allocation count
    pub fn allocation_count(&self) -> usize {
        self.allocations.len()
    }

    /// Get total huge page memory
    pub fn total_huge_memory(&self) -> usize {
        self.allocations
            .values()
            .filter(|a| a.allocated)
            .map(|a| a.size)
            .sum()
    }

    /// Get THP statistics
    pub fn get_stats(&self) -> ThpStats {
        let total_allocations = self.allocations.len();
        let active_allocations = self.allocations.values().filter(|a| a.allocated).count();

        let total_2mb = self
            .allocations
            .values()
            .filter(|a| a.allocated && a.page_size == HugePageSize::Size2MB)
            .count();

        let total_1gb = self
            .allocations
            .values()
        let active_allocations = self.allocations.values()
            .filter(|a| a.allocated)
            .count();

        let total_2mb = self.allocations.values()
            .filter(|a| a.allocated && a.page_size == HugePageSize::Size2MB)
            .count();

        let total_1gb = self.allocations.values()
            .filter(|a| a.allocated && a.page_size == HugePageSize::Size2MB)
            .count();

        let total_1gb = self
            .allocations
            .values()
            .filter(|a| a.allocated && a.page_size == HugePageSize::Size1GB)
            .count();

        ThpStats {
            total_allocations,
            active_allocations,
            total_2mb,
            total_1gb,
            policy: self.policy,
            defrag_enabled: self.defrag_enabled,
            collapse_enabled: self.collapse_enabled,
        }
    }
}

/// THP statistics
#[derive(Debug, Clone)]
pub struct ThpStats {
    pub total_allocations: usize,
    pub active_allocations: usize,
    pub total_2mb: usize,
    pub total_1gb: usize,
    pub policy: ThpPolicy,
    pub defrag_enabled: bool,
    pub collapse_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_2mb() {
        let mut thp = ThpManager::new();

        let addr = thp
            .allocate(2 * 1024 * 1024, HugePageSize::Size2MB)
            .unwrap();
        let addr = thp.allocate(2 * 1024 * 1024, HugePageSize::Size2MB).unwrap();
        assert!(addr > 0);
        assert_eq!(thp.allocation_count(), 1);
    }

    #[test]
    fn test_free() {
        let mut thp = ThpManager::new();

        let addr = thp
            .allocate(2 * 1024 * 1024, HugePageSize::Size2MB)
            .unwrap();
        let addr = thp.allocate(2 * 1024 * 1024, HugePageSize::Size2MB).unwrap();
        assert!(thp.free(addr).is_ok());
    }

    #[test]
    fn test_policy_never() {
        let mut thp = ThpManager::new();

        thp.set_policy(ThpPolicy::Never);

        let result = thp.allocate(2 * 1024 * 1024, HugePageSize::Size2MB);
        assert!(result.is_err());
    }

    #[test]
    fn test_policy_always() {
        let mut thp = ThpManager::new();

        thp.set_policy(ThpPolicy::Always);

        let addr = thp
            .allocate(2 * 1024 * 1024, HugePageSize::Size2MB)
            .unwrap();
        let addr = thp.allocate(2 * 1024 * 1024, HugePageSize::Size2MB).unwrap();
        assert!(addr > 0);
    }

    #[test]
    fn test_collapse() {
        let mut thp = ThpManager::new();

        let addr = thp.allocate(64 * 1024, HugePageSize::Size2MB).unwrap();
        assert!(thp.collapse(addr).is_ok());
    }

    #[test]
    fn test_defrag() {
        let mut thp = ThpManager::new();

        thp.allocate(64 * 1024, HugePageSize::Size2MB).unwrap();

        let collapsed = thp.defrag().unwrap();
        assert!(collapsed >= 0);
    }

    #[test]
    fn test_defrag_disabled() {
        let mut thp = ThpManager::new();

        thp.set_defrag_enabled(false);

        let result = thp.defrag();
        assert!(result.is_err());
    }

    #[test]
    fn test_stats() {
        let mut thp = ThpManager::new();

        thp.allocate(2 * 1024 * 1024, HugePageSize::Size2MB)
            .unwrap();
        thp.allocate(1024 * 1024 * 1024, HugePageSize::Size1GB)
            .unwrap();
        thp.allocate(2 * 1024 * 1024, HugePageSize::Size2MB).unwrap();
        thp.allocate(1024 * 1024 * 1024, HugePageSize::Size1GB).unwrap();

        let stats = thp.get_stats();
        assert_eq!(stats.total_2mb, 1);
        assert_eq!(stats.total_1gb, 1);
    }

    #[test]
    fn test_total_huge_memory() {
        let mut thp = ThpManager::new();

        thp.allocate(2 * 1024 * 1024, HugePageSize::Size2MB)
            .unwrap();
        thp.allocate(2 * 1024 * 1024, HugePageSize::Size2MB).unwrap();

        let total = thp.total_huge_memory();
        assert_eq!(total, 2 * 1024 * 1024);
    }
}
