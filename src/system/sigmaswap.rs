// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaSwap - System Swap Manager
//!
//! SigmaSwap provides system swap management for SigmaOS, inspired by swapon/swapoff from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Swap priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
}

impl fmt::Display for SwapPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwapPriority::Low => write!(f, "low"),
            SwapPriority::Normal => write!(f, "normal"),
            SwapPriority::High => write!(f, "high"),
        }
    }
}

/// Swap state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapState {
    /// Active
    Active,
    /// Inactive
    Inactive,
}

impl fmt::Display for SwapState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwapState::Active => write!(f, "active"),
            SwapState::Inactive => write!(f, "inactive"),
        }
    }
}

/// Swap space
#[derive(Debug, Clone)]
pub struct SwapSpace {
    /// Swap ID
    pub swap_id: String,
    /// Device path
    pub device_path: String,
    /// Size in bytes
    pub size: u64,
    /// Used bytes
    pub used: u64,
    /// Priority
    pub priority: SwapPriority,
    /// State
    pub state: SwapState,
}

impl SwapSpace {
    /// Create a new swap space
    pub fn new(swap_id: String, device_path: String, size: u64, priority: SwapPriority) -> Self {
        Self {
            swap_id,
            device_path,
            size,
            used: 0,
            priority,
            state: SwapState::Inactive,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: SwapState) -> Self {
        self.state = state;
        self
    }

    /// Set used bytes
    pub fn with_used(mut self, used: u64) -> Self {
        self.used = used;
        self
    }
}

/// SigmaSwap - System Swap Manager
pub struct SigmaSwap {
    /// Swap spaces (swap_id -> space)
    spaces: BTreeMap<String, SwapSpace>,
    /// Running state
    running: bool,
    /// Space counter
    space_counter: u32,
}

impl SigmaSwap {
    /// Create a new SigmaSwap instance
    pub fn new() -> Self {
        Self {
            spaces: BTreeMap::new(),
            running: false,
            space_counter: 0,
        }
    }

    /// Start the swap manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the swap manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a swap space
    pub fn add_swap_space(&mut self, space: SwapSpace) -> Result<(), String> {
        if self.spaces.contains_key(&space.swap_id) {
            return Err(format!("Swap space '{}' already exists", space.swap_id));
        }

        self.spaces.insert(space.swap_id.clone(), space);
        self.space_counter += 1;
        Ok(())
    }

    /// Remove a swap space
    pub fn remove_swap_space(&mut self, swap_id: &str) -> Result<(), String> {
        if self.spaces.remove(swap_id).is_none() {
            return Err(format!("Swap space '{}' not found", swap_id));
        }
        Ok(())
    }

    /// Activate a swap space
    pub fn activate_swap(&mut self, swap_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaSwap not running".to_string());
        }

        match self.spaces.get_mut(swap_id) {
            Some(space) => {
                space.state = SwapState::Active;
                Ok(())
            }
            None => Err(format!("Swap space '{}' not found", swap_id)),
        }
    }

    /// Deactivate a swap space
    pub fn deactivate_swap(&mut self, swap_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaSwap not running".to_string());
        }

        match self.spaces.get_mut(swap_id) {
            Some(space) => {
                space.state = SwapState::Inactive;
                space.used = 0;
                Ok(())
            }
            None => Err(format!("Swap space '{}' not found", swap_id)),
        }
    }

    /// Get a swap space
    pub fn get_swap_space(&self, swap_id: &str) -> Option<&SwapSpace> {
        self.spaces.get(swap_id)
    }

    /// List all swap spaces
    pub fn list_swap_spaces(&self) -> Vec<&SwapSpace> {
        self.spaces.values().collect()
    }

    /// List active swap spaces
    pub fn list_active_swaps(&self) -> Vec<&SwapSpace> {
        self.spaces
            .values()
            .filter(|s| s.state == SwapState::Active)
            .collect()
    }

    /// Get total swap size
    pub fn total_swap_size(&self) -> u64 {
        self.spaces
            .values()
            .filter(|s| s.state == SwapState::Active)
            .map(|s| s.size)
            .sum()
    }

    /// Get total swap used
    pub fn total_swap_used(&self) -> u64 {
        self.spaces
            .values()
            .filter(|s| s.state == SwapState::Active)
            .map(|s| s.used)
            .sum()
    }

    /// Get swap space count
    pub fn swap_space_count(&self) -> usize {
        self.spaces.len()
    }

    /// Get swap manager statistics
    pub fn swap_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_spaces".to_string(),
            self.swap_space_count().to_string(),
        );
        stats.insert(
            "active_spaces".to_string(),
            self.list_active_swaps().len().to_string(),
        );
        stats.insert("total_size".to_string(), self.total_swap_size().to_string());
        stats.insert("total_used".to_string(), self.total_swap_used().to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaSwap {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaSwap instance (placeholder for actual global state)
static mut SIGMASWAP_INSTANCE: Option<SigmaSwap> = None;

/// Initialize the global SigmaSwap instance
pub fn init_sigmaswap() {
    unsafe {
        SIGMASWAP_INSTANCE = Some(SigmaSwap::new());
    }
}

/// Get the global SigmaSwap instance
pub fn get_sigmaswap() -> Option<&'static mut SigmaSwap> {
    unsafe { SIGMASWAP_INSTANCE.as_mut() }
}

/// Convenience function to activate a swap space
pub fn activate_swap_space(swap_id: &str) -> Result<(), String> {
    if let Some(swap) = get_sigmaswap() {
        swap.activate_swap(swap_id)
    } else {
        Err("SigmaSwap not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_priority_display() {
        assert_eq!(SwapPriority::Low.to_string(), "low");
        assert_eq!(SwapPriority::High.to_string(), "high");
    }

    #[test]
    fn test_swap_state_display() {
        assert_eq!(SwapState::Active.to_string(), "active");
        assert_eq!(SwapState::Inactive.to_string(), "inactive");
    }

    #[test]
    fn test_swap_space() {
        let space = SwapSpace::new(
            "swap1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            SwapPriority::Normal,
        )
        .with_state(SwapState::Active)
        .with_used(512 * 1024 * 1024);

        assert_eq!(space.swap_id, "swap1");
        assert_eq!(space.state, SwapState::Active);
        assert_eq!(space.used, 512 * 1024 * 1024);
    }

    #[test]
    fn test_sigmaswap() {
        let mut swap = SigmaSwap::new();
        assert!(!swap.is_running());
        swap.start();
        assert!(swap.is_running());
        swap.stop();
        assert!(!swap.is_running());
    }

    #[test]
    fn test_sigmaswap_add_swap_space() {
        let mut swap = SigmaSwap::new();

        let space = SwapSpace::new(
            "swap1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            SwapPriority::Normal,
        );

        assert!(swap.add_swap_space(space).is_ok());
        assert_eq!(swap.swap_space_count(), 1);
    }

    #[test]
    fn test_sigmaswap_activate_swap() {
        let mut swap = SigmaSwap::new();
        swap.start();

        let space = SwapSpace::new(
            "swap1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            SwapPriority::Normal,
        );

        swap.add_swap_space(space).unwrap();
        assert!(swap.activate_swap("swap1").is_ok());

        let space = swap.get_swap_space("swap1").unwrap();
        assert_eq!(space.state, SwapState::Active);
    }

    #[test]
    fn test_sigmaswap_deactivate_swap() {
        let mut swap = SigmaSwap::new();
        swap.start();

        let space = SwapSpace::new(
            "swap1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            SwapPriority::Normal,
        )
        .with_state(SwapState::Active)
        .with_used(512 * 1024 * 1024);

        swap.add_swap_space(space).unwrap();
        assert!(swap.deactivate_swap("swap1").is_ok());

        let space = swap.get_swap_space("swap1").unwrap();
        assert_eq!(space.state, SwapState::Inactive);
        assert_eq!(space.used, 0);
    }

    #[test]
    fn test_sigmaswap_list_active_swaps() {
        let mut swap = SigmaSwap::new();

        let space = SwapSpace::new(
            "swap1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            SwapPriority::Normal,
        )
        .with_state(SwapState::Active);

        swap.add_swap_space(space).unwrap();

        let active = swap.list_active_swaps();
        assert_eq!(active.len(), 1);
    }

    #[test]
    fn test_sigmaswap_total_swap_size() {
        let mut swap = SigmaSwap::new();

        let space = SwapSpace::new(
            "swap1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            SwapPriority::Normal,
        )
        .with_state(SwapState::Active);

        swap.add_swap_space(space).unwrap();

        assert_eq!(swap.total_swap_size(), 1024 * 1024 * 1024);
    }

    #[test]
    fn test_sigmaswap_stats() {
        let mut swap = SigmaSwap::new();

        let space = SwapSpace::new(
            "swap1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            SwapPriority::Normal,
        )
        .with_state(SwapState::Active)
        .with_used(512 * 1024 * 1024);

        swap.add_swap_space(space).unwrap();

        let stats = swap.swap_stats();
        assert_eq!(stats.get("total_spaces"), Some(&"1".to_string()));
        assert_eq!(stats.get("active_spaces"), Some(&"1".to_string()));
    }

    #[test]
    fn test_activate_swap_space_convenience() {
        init_sigmaswap();

        let mut swap = get_sigmaswap().unwrap();
        swap.start();

        let space = SwapSpace::new(
            "swap1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            SwapPriority::Normal,
        );

        swap.add_swap_space(space).unwrap();

        let result = activate_swap_space("swap1");
        assert!(result.is_ok());
    }
}
