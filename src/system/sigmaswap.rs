//! SigmaSwap - Swap Space Management System
//!
//! Provides comprehensive swap space management inspired by Linux swapon/swapoff
//! and BSD swap management. Supports swap devices, swap files, and priority configuration.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Swap type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwapType {
    Partition,
    File,
    Unknown,
}

/// Swap state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapState {
    Active,
    Inactive,
    Error,
    Unknown,
}

/// Swap entry
#[derive(Debug, Clone)]
pub struct SwapEntry {
    pub id: String,
    pub swap_type: SwapType,
    pub device_path: String,
    pub size: u64,
    pub priority: i32,
    pub state: SwapState,
    pub used: u64,
}

impl SwapEntry {
    pub fn new(id: String, swap_type: SwapType, device_path: String, size: u64) -> Self {
        Self {
            id,
            swap_type,
            device_path,
            size,
            priority: -1,
            state: SwapState::Inactive,
            used: 0,
        }
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn activate(&mut self) {
        self.state = SwapState::Active;
    }

    pub fn deactivate(&mut self) {
        self.state = SwapState::Inactive;
    }

    pub fn set_used(&mut self, used: u64) {
        self.used = used;
    }
}

/// Swap manager
pub struct SigmaSwap {
    swaps: BTreeMap<String, SwapEntry>,
    next_swap_id: u64,
    total_swap: u64,
    total_used: u64,
}

impl SigmaSwap {
    pub fn new() -> Self {
        Self {
            swaps: BTreeMap::new(),
            next_swap_id: 1,
            total_swap: 0,
            total_used: 0,
        }
    }

    pub fn add_swap(&mut self, swap_type: SwapType, device_path: String, size: u64) -> String {
        let id = format!("swap-{}", self.next_swap_id);
        self.next_swap_id += 1;
        let swap = SwapEntry::new(id.clone(), swap_type, device_path, size);
        self.total_swap += size;
        self.swaps.insert(id.clone(), swap);
        id
    }

    pub fn get_swap(&self, id: &str) -> Option<&SwapEntry> {
        self.swaps.get(id)
    }

    pub fn get_swap_mut(&mut self, id: &str) -> Option<&mut SwapEntry> {
        self.swaps.get_mut(id)
    }

    pub fn list_swaps(&self) -> Vec<&SwapEntry> {
        self.swaps.values().collect()
    }

    pub fn list_swaps_by_type(&self, swap_type: SwapType) -> Vec<&SwapEntry> {
        self.swaps.values().filter(|s| s.swap_type == swap_type).collect()
    }

    pub fn list_swaps_by_state(&self, state: SwapState) -> Vec<&SwapEntry> {
        self.swaps.values().filter(|s| s.state == state).collect()
    }

    pub fn remove_swap(&mut self, id: &str) -> bool {
        if let Some(swap) = self.swaps.remove(id) {
            self.total_swap -= swap.size;
            if swap.state == SwapState::Active {
                self.total_used -= swap.used;
            }
            true
        } else {
            false
        }
    }

    pub fn activate_swap(&mut self, id: &str) -> bool {
        if let Some(swap) = self.swaps.get_mut(id) {
            swap.activate();
            true
        } else {
            false
        }
    }

    pub fn deactivate_swap(&mut self, id: &str) -> bool {
        if let Some(swap) = self.swaps.get_mut(id) {
            swap.deactivate();
            self.total_used -= swap.used;
            swap.set_used(0);
            true
        } else {
            false
        }
    }

    pub fn set_swap_priority(&mut self, id: &str, priority: i32) -> bool {
        if let Some(swap) = self.swaps.get_mut(id) {
            swap.priority = priority;
            true
        } else {
            false
        }
    }

    pub fn set_swap_used(&mut self, id: &str, used: u64) -> bool {
        if let Some(swap) = self.swaps.get_mut(id) {
            let old_used = swap.used;
            swap.set_used(used);
            if swap.state == SwapState::Active {
                self.total_used = self.total_used.saturating_sub(old_used);
                self.total_used += used;
            }
            true
        } else {
            false
        }
    }

    pub fn swap_count(&self) -> usize {
        self.swaps.len()
    }

    pub fn active_swap_count(&self) -> usize {
        self.swaps
            .values()
            .filter(|s| s.state == SwapState::Active)
            .count()
    }

    pub fn get_total_swap(&self) -> u64 {
        self.total_swap
    }

    pub fn get_total_used(&self) -> u64 {
        self.total_used
    }

    pub fn get_swap_usage(&self) -> f64 {
        if self.total_swap == 0 {
            0.0
        } else {
            (self.total_used as f64 / self.total_swap as f64) * 100.0
        }
    }
}

impl Default for SigmaSwap {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMASWAP_INSTANCE: Option<SigmaSwap> = None;

pub fn init_sigmaswap() {
    unsafe {
        SIGMASWAP_INSTANCE = Some(SigmaSwap::new());
    }
}

pub fn get_sigmaswap() -> &'static SigmaSwap {
    unsafe {
        SIGMASWAP_INSTANCE
            .as_ref()
            .expect("SigmaSwap not initialized")
    }
}

pub fn get_sigmaswap_mut() -> &'static mut SigmaSwap {
    unsafe {
        SIGMASWAP_INSTANCE
            .as_mut()
            .expect("SigmaSwap not initialized")
    }
}

pub fn add_swap(swap_type: SwapType, device_path: String, size: u64) -> String {
    let manager = get_sigmaswap_mut();
    manager.add_swap(swap_type, device_path, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_entry_creation() {
        let entry = SwapEntry::new(
            "swap-1".to_string(),
            SwapType::Partition,
            "/dev/sda1".to_string(),
            2 * 1024 * 1024 * 1024,
        );
        assert_eq!(entry.id, "swap-1");
        assert_eq!(entry.swap_type, SwapType::Partition);
        assert_eq!(entry.device_path, "/dev/sda1");
        assert_eq!(entry.size, 2 * 1024 * 1024 * 1024);
        assert_eq!(entry.state, SwapState::Inactive);
    }

    #[test]
    fn test_swap_entry_builder() {
        let entry = SwapEntry::new(
            "swap-1".to_string(),
            SwapType::File,
            "/swapfile".to_string(),
            2 * 1024 * 1024 * 1024,
        )
        .with_priority(10);

        assert_eq!(entry.priority, 10);
    }

    #[test]
    fn test_swap_state_transitions() {
        let mut entry = SwapEntry::new(
            "swap-1".to_string(),
            SwapType::Partition,
            "/dev/sda1".to_string(),
            2 * 1024 * 1024 * 1024,
        );
        assert_eq!(entry.state, SwapState::Inactive);

        entry.activate();
        assert_eq!(entry.state, SwapState::Active);

        entry.deactivate();
        assert_eq!(entry.state, SwapState::Inactive);
    }

    #[test]
    fn test_swap_manager() {
        let manager = SigmaSwap::new();
        assert_eq!(manager.swap_count(), 0);
        assert_eq!(manager.get_total_swap(), 0);
    }

    #[test]
    fn test_add_swap() {
        let mut manager = SigmaSwap::new();
        let id = manager.add_swap(
            SwapType::Partition,
            "/dev/sda1".to_string(),
            2 * 1024 * 1024 * 1024,
        );
        assert_eq!(id, "swap-1");
        assert_eq!(manager.swap_count(), 1);
        assert_eq!(manager.get_total_swap(), 2 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_activate_deactivate_swap() {
        let mut manager = SigmaSwap::new();
        let id = manager.add_swap(
            SwapType::Partition,
            "/dev/sda1".to_string(),
            2 * 1024 * 1024 * 1024,
        );

        assert!(manager.activate_swap(&id));
        assert_eq!(manager.get_swap(&id).unwrap().state, SwapState::Active);

        assert!(manager.deactivate_swap(&id));
        assert_eq!(manager.get_swap(&id).unwrap().state, SwapState::Inactive);
    }

    #[test]
    fn test_set_swap_priority() {
        let mut manager = SigmaSwap::new();
        let id = manager.add_swap(
            SwapType::Partition,
            "/dev/sda1".to_string(),
            2 * 1024 * 1024 * 1024,
        );

        assert!(manager.set_swap_priority(&id, 10));
        assert_eq!(manager.get_swap(&id).unwrap().priority, 10);
    }

    #[test]
    fn test_set_swap_used() {
        let mut manager = SigmaSwap::new();
        let id = manager.add_swap(
            SwapType::Partition,
            "/dev/sda1".to_string(),
            2 * 1024 * 1024 * 1024,
        );
        manager.activate_swap(&id);

        assert!(manager.set_swap_used(&id, 1024 * 1024 * 1024));
        assert_eq!(manager.get_swap(&id).unwrap().used, 1024 * 1024 * 1024);
        assert_eq!(manager.get_total_used(), 1024 * 1024 * 1024);
    }

    #[test]
    fn test_list_swaps_by_type() {
        let mut manager = SigmaSwap::new();
        manager.add_swap(SwapType::Partition, "/dev/sda1".to_string(), 2 * 1024 * 1024 * 1024);
        manager.add_swap(SwapType::File, "/swapfile".to_string(), 1 * 1024 * 1024 * 1024);
        manager.add_swap(SwapType::Partition, "/dev/sda2".to_string(), 2 * 1024 * 1024 * 1024);

        let partitions = manager.list_swaps_by_type(SwapType::Partition);
        assert_eq!(partitions.len(), 2);

        let files = manager.list_swaps_by_type(SwapType::File);
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_list_swaps_by_state() {
        let mut manager = SigmaSwap::new();
        let id1 = manager.add_swap(SwapType::Partition, "/dev/sda1".to_string(), 2 * 1024 * 1024 * 1024);
        let id2 = manager.add_swap(SwapType::File, "/swapfile".to_string(), 1 * 1024 * 1024 * 1024);
        manager.activate_swap(&id1);

        let active = manager.list_swaps_by_state(SwapState::Active);
        assert_eq!(active.len(), 1);

        let inactive = manager.list_swaps_by_state(SwapState::Inactive);
        assert_eq!(inactive.len(), 1);
    }

    #[test]
    fn test_remove_swap() {
        let mut manager = SigmaSwap::new();
        let id = manager.add_swap(
            SwapType::Partition,
            "/dev/sda1".to_string(),
            2 * 1024 * 1024 * 1024,
        );
        manager.activate_swap(&id);
        manager.set_swap_used(&id, 1024 * 1024 * 1024);

        assert!(manager.remove_swap(&id));
        assert_eq!(manager.swap_count(), 0);
        assert_eq!(manager.get_total_swap(), 0);
        assert_eq!(manager.get_total_used(), 0);
    }

    #[test]
    fn test_swap_counts() {
        let mut manager = SigmaSwap::new();
        let id1 = manager.add_swap(SwapType::Partition, "/dev/sda1".to_string(), 2 * 1024 * 1024 * 1024);
        let id2 = manager.add_swap(SwapType::File, "/swapfile".to_string(), 1 * 1024 * 1024 * 1024);
        manager.activate_swap(&id1);
        manager.activate_swap(&id2);

        assert_eq!(manager.swap_count(), 2);
        assert_eq!(manager.active_swap_count(), 2);
    }

    #[test]
    fn test_swap_usage() {
        let mut manager = SigmaSwap::new();
        let id = manager.add_swap(
            SwapType::Partition,
            "/dev/sda1".to_string(),
            2 * 1024 * 1024 * 1024,
        );
        manager.activate_swap(&id);
        manager.set_swap_used(&id, 1024 * 1024 * 1024);

        let usage = manager.get_swap_usage();
        assert!(usage > 0.0 && usage <= 100.0);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaswap();
        let manager = get_sigmaswap();
        assert_eq!(manager.swap_count(), 0);

        let manager_mut = get_sigmaswap_mut();
        manager_mut.add_swap(SwapType::Partition, "/dev/sda1".to_string(), 2 * 1024 * 1024 * 1024);
        assert_eq!(manager.swap_count(), 1);
    }

    #[test]
    fn test_add_swap_convenience() {
        init_sigmaswap();
        let id = add_swap(SwapType::Partition, "/dev/sda1".to_string(), 2 * 1024 * 1024 * 1024);
        assert_eq!(id, "swap-1");

        let manager = get_sigmaswap();
        assert!(manager.get_swap(&id).is_some());
    }
}
