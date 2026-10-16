// SigmaOS Swap Core Manager
// Race-free page swap-out and swap-in operations, bounded metadata tracking, and double-swap prevention.

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwapError {
    OutOfSwapSlots,
    SlotNotFound(usize),
    AlreadySwapped(u64),
    NotSwapped(u64),
    DoubleFreeSlot(usize),
    DeviceIoError,
}

impl core::fmt::Display for SwapError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfSwapSlots => write!(f, "SwapError: no available swap slots"),
            Self::SlotNotFound(slot) => write!(f, "SwapError: swap slot {} not found", slot),
            Self::AlreadySwapped(addr) => {
                write!(f, "SwapError: address {:#x} is already swapped out", addr)
            }
            Self::NotSwapped(addr) => write!(
                f,
                "SwapError: address {:#x} is not present in swap space",
                addr
            ),
            Self::DoubleFreeSlot(slot) => {
                write!(f, "SwapError: attempted double free of swap slot {}", slot)
            }
            Self::DeviceIoError => write!(
                f,
                "SwapError: secondary storage I/O error during swap operation"
            ),
        }
    }
}

pub type SwapResult<T> = Result<T, SwapError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwapEntry {
    pub slot_id: usize,
    pub virt_addr: u64,
    pub phys_frame: u64,
    pub active: bool,
}

#[derive(Debug)]
pub struct SwapManager {
    total_slots: usize,
    free_slot_count: AtomicUsize,
    used_slots: Mutex<HashMap<usize, SwapEntry>>,
    address_to_slot: Mutex<HashMap<u64, usize>>,
    free_slots: Mutex<Vec<usize>>,
    oom_triggered: AtomicBool,
}

impl SwapManager {
    pub fn new(total_slots: usize) -> Self {
        let mut free_slots = Vec::with_capacity(total_slots);
        for i in 0..total_slots {
            free_slots.push(i);
        }

        Self {
            total_slots,
            free_slot_count: AtomicUsize::new(total_slots),
            used_slots: Mutex::new(HashMap::new()),
            address_to_slot: Mutex::new(HashMap::new()),
            free_slots: Mutex::new(free_slots),
            oom_triggered: AtomicBool::new(false),
        }
    }

    /// Perform race-free page swap-out with double-swap prevention
    pub fn swap_out_page(&self, virt_addr: u64, phys_frame: u64) -> SwapResult<usize> {
        let mut addr_map = self.address_to_slot.lock().unwrap();

        if addr_map.contains_key(&virt_addr) {
            return Err(SwapError::AlreadySwapped(virt_addr));
        }

        let mut free_list = self.free_slots.lock().unwrap();
        let slot_id = free_list.pop().ok_or_else(|| {
            self.oom_triggered.store(true, Ordering::SeqCst);
            SwapError::OutOfSwapSlots
        })?;

        let entry = SwapEntry {
            slot_id,
            virt_addr,
            phys_frame,
            active: true,
        };

        let mut slots_map = self.used_slots.lock().unwrap();
        slots_map.insert(slot_id, entry);
        addr_map.insert(virt_addr, slot_id);

        self.free_slot_count.fetch_sub(1, Ordering::SeqCst);

        Ok(slot_id)
    }

    /// Perform race-free page swap-in
    pub fn swap_in_page(&self, virt_addr: u64) -> SwapResult<(u64, usize)> {
        let mut addr_map = self.address_to_slot.lock().unwrap();
        let slot_id = addr_map
            .remove(&virt_addr)
            .ok_or(SwapError::NotSwapped(virt_addr))?;

        let mut slots_map = self.used_slots.lock().unwrap();
        let entry = slots_map
            .remove(&slot_id)
            .ok_or(SwapError::SlotNotFound(slot_id))?;

        let mut free_list = self.free_slots.lock().unwrap();
        free_list.push(slot_id);

        self.free_slot_count.fetch_add(1, Ordering::SeqCst);

        Ok((entry.phys_frame, slot_id))
    }

    pub fn total_slots(&self) -> usize {
        self.total_slots
    }

    pub fn free_slots(&self) -> usize {
        self.free_slot_count.load(Ordering::SeqCst)
    }

    pub fn is_oom_triggered(&self) -> bool {
        self.oom_triggered.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_manager_basic() {
        let sm = SwapManager::new(5);
        assert_eq!(sm.total_slots(), 5);
        assert_eq!(sm.free_slots(), 5);

        let slot = sm.swap_out_page(0x1000, 0x20000).unwrap();
        assert_eq!(sm.free_slots(), 4);

        let (phys, freed_slot) = sm.swap_in_page(0x1000).unwrap();
        assert_eq!(phys, 0x20000);
        assert_eq!(freed_slot, slot);
        assert_eq!(sm.free_slots(), 5);
    }
}
