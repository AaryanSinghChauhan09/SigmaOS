// Physical Demand Paging and Swap System
// Implements demand paging with swap backing for SigmaOS

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

/// Page frame number (PFN)
pub type Pfn = u64;

/// Virtual address
pub type VirtualAddress = u64;

/// Physical address
pub type PhysicalAddress = u64;

/// Page table entry flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableFlags {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub write_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub global: bool,
}

impl Default for PageTableFlags {
    fn default() -> Self {
        PageTableFlags {
            present: false,
            writable: false,
            user_accessible: false,
            write_through: false,
            cache_disable: false,
            accessed: false,
            dirty: false,
            global: false,
        }
    }
}

/// Page table entry
#[derive(Debug, Clone)]
pub struct PageTableEntry {
    pub pfn: Pfn,
    pub flags: PageTableFlags,
}

impl PageTableEntry {
    pub fn new(pfn: Pfn) -> Self {
        PageTableEntry {
            pfn,
            flags: PageTableFlags {
                present: true,
                writable: true,
                user_accessible: true,
                ..Default::default()
            },
        }
    }

    pub fn not_present() -> Self {
        PageTableEntry {
            pfn: 0,
            flags: PageTableFlags::default(),
        }
    }
}

/// Page fault error codes (x86_64)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultError {
    Present = 0x01,
    Write = 0x02,
    User = 0x04,
    ReservedWrite = 0x08,
    InstructionFetch = 0x10,
}

/// Swap entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwapEntry {
    pub swap_slot: u64,
    pub backing_file: PathBuf,
    pub offset: u64,
}

/// Memory page state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageState {
    /// Page is present in physical memory
    Present,
    /// Page is swapped out
    Swapped(SwapEntry),
    /// Page is not allocated
    NotPresent,
}

/// Memory page
#[derive(Debug, Clone)]
pub struct MemoryPage {
    pub pfn: Pfn,
    pub state: PageState,
    pub virtual_address: VirtualAddress,
    pub flags: PageTableFlags,
}

impl MemoryPage {
    pub fn new(pfn: Pfn, virtual_address: VirtualAddress) -> Self {
        MemoryPage {
            pfn,
            state: PageState::Present,
            virtual_address,
            flags: PageTableFlags {
                present: true,
                writable: true,
                user_accessible: true,
                ..Default::default()
            },
        }
    }

    pub fn is_present(&self) -> bool {
        matches!(self.state, PageState::Present)
    }

    pub fn is_swapped(&self) -> bool {
        matches!(self.state, PageState::Swapped(_))
    }
}

/// LRU (Least Recently Used) page replacement algorithm
pub struct LruPageReplacer {
    access_order: VecDeque<Pfn>,
}

impl LruPageReplacer {
    pub fn new() -> Self {
        LruPageReplacer {
            access_order: VecDeque::new(),
        }
    }

    /// Record page access
    pub fn record_access(&mut self, pfn: Pfn) {
        // Remove from current position if exists
        if let Some(pos) = self.access_order.iter().position(|&x| x == pfn) {
            self.access_order.remove(pos);
        }
        // Add to front (most recently used)
        self.access_order.push_front(pfn);
    }

    /// Get least recently used page for eviction
    pub fn get_lru(&self) -> Option<Pfn> {
        self.access_order.back().copied()
    }

    /// Remove page from tracking
    pub fn remove(&mut self, pfn: Pfn) {
        if let Some(pos) = self.access_order.iter().position(|&x| x == pfn) {
            self.access_order.remove(pos);
        }
    }
}

impl Default for LruPageReplacer {
    fn default() -> Self {
        Self::new()
    }
}

/// Swap device configuration
#[derive(Debug, Clone)]
pub struct SwapDevice {
    pub backing_file: PathBuf,
    pub size_pages: u64,
    pub used_slots: u64,
}

impl SwapDevice {
    pub fn new(backing_file: PathBuf, size_pages: u64) -> Self {
        SwapDevice {
            backing_file,
            size_pages,
            used_slots: 0,
        }
    }

    pub fn allocate_slot(&mut self) -> Option<u64> {
        if self.used_slots < self.size_pages {
            let slot = self.used_slots;
            self.used_slots += 1;
            Some(slot)
        } else {
            None
        }
    }

    pub fn free_slot(&mut self, slot: u64) {
        // Simplified - in real implementation would track free slots
        if slot < self.used_slots {
            self.used_slots -= 1;
        }
    }
}

/// Demand paging manager
pub struct DemandPagingManager {
    /// Page table mapping virtual addresses to page table entries
    page_table: HashMap<VirtualAddress, PageTableEntry>,
    /// Physical memory pages
    physical_pages: HashMap<Pfn, MemoryPage>,
    /// Virtual to physical mapping
    virtual_to_physical: HashMap<VirtualAddress, Pfn>,
    /// LRU page replacer
    lru_replacer: LruPageReplacer,
    /// Swap device
    swap_device: Option<SwapDevice>,
    /// Next available PFN
    next_pfn: Pfn,
    /// Total physical pages
    total_physical_pages: u64,
}

impl DemandPagingManager {
    pub fn new(total_physical_pages: u64) -> Self {
        DemandPagingManager {
            page_table: HashMap::new(),
            physical_pages: HashMap::new(),
            virtual_to_physical: HashMap::new(),
            lru_replacer: LruPageReplacer::new(),
            swap_device: None,
            next_pfn: 1,
            total_physical_pages,
        }
    }

    /// Configure swap device
    pub fn configure_swap(&mut self, backing_file: PathBuf, size_pages: u64) {
        self.swap_device = Some(SwapDevice::new(backing_file, size_pages));
    }

    /// Handle page fault
    pub fn handle_page_fault(
        &mut self,
        virtual_address: VirtualAddress,
        error_code: PageFaultError,
    ) -> Result<(), &'static str> {
        // Check if page is in page table
        if let Some(entry) = self.page_table.get(&virtual_address) {
            if entry.flags.present {
                // Page is present - this might be a protection fault
                if error_code.contains(PageFaultError::Write) && !entry.flags.writable {
                    return Err("Write violation - page not writable");
                }
                if error_code.contains(PageFaultError::User) && !entry.flags.user_accessible {
                    return Err("User access violation - page not user accessible");
                }
                return Ok(());
            }

            // Page is not present - need to page in
            if let Some(pfn) = self.virtual_to_physical.get(&virtual_address) {
                if let Some(page) = self.physical_pages.get(pfn) {
                    if page.is_swapped() {
                        // Page in from swap
                        self.page_in_from_swap(virtual_address, *pfn)?;
                        return Ok(());
                    }
                }
            }
        }

        // Allocate new page
        self.allocate_page(virtual_address)?;
        Ok(())
    }

    /// Allocate a new page
    fn allocate_page(&mut self, virtual_address: VirtualAddress) -> Result<(), &'static str> {
        // Check if we have physical memory available
        if self.physical_pages.len() as u64 >= self.total_physical_pages {
            // Need to evict a page
            self.evict_page()?;
        }

        // Allocate new PFN
        let pfn = self.next_pfn;
        self.next_pfn += 1;

        // Create new page
        let page = MemoryPage::new(pfn, virtual_address);
        self.physical_pages.insert(pfn, page);
        self.virtual_to_physical.insert(virtual_address, pfn);

        // Update page table
        let entry = PageTableEntry::new(pfn);
        self.page_table.insert(virtual_address, entry);

        // Record access
        self.lru_replacer.record_access(pfn);

        Ok(())
    }

    /// Evict a page to swap
    fn evict_page(&mut self) -> Result<(), &'static str> {
        // Get LRU page
        let lru_pfn = self.lru_replacer.get_lru()
            .ok_or("No pages to evict")?;

        // Find the page
        if let Some(page) = self.physical_pages.get(&lru_pfn) {
            if !page.is_present() {
                return Err("LRU page is not present");
            }

            // Check if swap device is available
            if self.swap_device.is_none() {
                return Err("No swap device configured");
            }

            // Allocate swap slot
            let swap_device = self.swap_device.as_mut().unwrap();
            let swap_slot = swap_device.allocate_slot()
                .ok_or("Swap device full")?;

            // Create swap entry
            let swap_entry = SwapEntry {
                swap_slot,
                backing_file: swap_device.backing_file.clone(),
                offset: swap_slot * 4096, // Assume 4KB pages
            };

            // Update page state
            let virtual_address = page.virtual_address;
            if let Some(p) = self.physical_pages.get_mut(&lru_pfn) {
                p.state = PageState::Swapped(swap_entry.clone());
            }

            // Update page table entry to not present
            if let Some(entry) = self.page_table.get_mut(&virtual_address) {
                entry.flags.present = false;
            }

            // Remove from LRU tracking
            self.lru_replacer.remove(lru_pfn);

            Ok(())
        } else {
            Err("LRU page not found")
        }
    }

    /// Page in a page from swap
    fn page_in_from_swap(&mut self, virtual_address: VirtualAddress, pfn: Pfn) -> Result<(), &'static str> {
        // Find the page
        if let Some(page) = self.physical_pages.get_mut(&pfn) {
            if let PageState::Swapped(_swap_entry) = &page.state {
                // In a real implementation, this would read from the swap file
                // For now, we just mark the page as present

                // Update page state
                page.state = PageState::Present;

                // Update page table entry to present
                if let Some(entry) = self.page_table.get_mut(&virtual_address) {
                    entry.flags.present = true;
                }

                // Record access
                self.lru_replacer.record_access(pfn);

                Ok(())
            } else {
                Err("Page is not swapped")
            }
        } else {
            Err("Page not found")
        }
    }

    /// Get page table entry for a virtual address
    pub fn get_page_table_entry(&self, virtual_address: VirtualAddress) -> Option<&PageTableEntry> {
        self.page_table.get(&virtual_address)
    }

    /// Get memory statistics
    pub fn get_memory_stats(&self) -> DemandPagingStats {
        let present_pages = self.physical_pages.values()
            .filter(|p| p.is_present())
            .count() as u64;
        let swapped_pages = self.physical_pages.values()
            .filter(|p| p.is_swapped())
            .count() as u64;

        DemandPagingStats {
            total_physical_pages: self.total_physical_pages,
            present_pages,
            swapped_pages,
            free_pages: self.total_physical_pages - present_pages,
        }
    }
}

impl PageFaultError {
    pub fn contains(&self, flag: PageFaultError) -> bool {
        (*self as u8) & (flag as u8) != 0
    }
}

/// Demand paging memory statistics
#[derive(Debug, Clone)]
pub struct DemandPagingStats {
    pub total_physical_pages: u64,
    pub present_pages: u64,
    pub swapped_pages: u64,
    pub free_pages: u64,
}

/// Type alias for compatibility
pub type DemandPagingMemoryStats = DemandPagingStats;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_page_replacer() {
        let mut lru = LruPageReplacer::new();

        lru.record_access(1);
        lru.record_access(2);
        lru.record_access(3);

        assert_eq!(lru.get_lru(), Some(1));

        lru.record_access(1);
        assert_eq!(lru.get_lru(), Some(2));
    }

    #[test]
    fn test_swap_device() {
        let mut swap = SwapDevice::new(PathBuf::from("/swapfile"), 100);

        assert_eq!(swap.allocate_slot(), Some(0));
        assert_eq!(swap.allocate_slot(), Some(1));
        assert_eq!(swap.used_slots, 2);

        swap.free_slot(0);
        assert_eq!(swap.used_slots, 1);
    }

    #[test]
    fn test_demand_paging_allocate() {
        let mut dpm = DemandPagingManager::new(10);

        dpm.allocate_page(0x1000).unwrap();

        let stats = dpm.get_memory_stats();
        assert_eq!(stats.present_pages, 1);
        assert_eq!(stats.free_pages, 9);
    }

    #[test]
    fn test_demand_paging_fault_handling() {
        let mut dpm = DemandPagingManager::new(10);

        // Handle page fault for new page
        dpm.handle_page_fault(0x1000, PageFaultError::User).unwrap();

        let entry = dpm.get_page_table_entry(0x1000);
        assert!(entry.is_some());
        assert!(entry.unwrap().flags.present);
    }

    #[test]
    fn test_demand_paging_with_swap() {
        let mut dpm = DemandPagingManager::new(2);
        dpm.configure_swap(PathBuf::from("/swapfile"), 100);

        // Fill physical memory
        dpm.allocate_page(0x1000).unwrap();
        dpm.allocate_page(0x2000).unwrap();

        let stats = dpm.get_memory_stats();
        assert_eq!(stats.present_pages, 2);
        assert_eq!(stats.free_pages, 0);

        // Allocate another page - should trigger eviction
        dpm.allocate_page(0x3000).unwrap();

        let stats = dpm.get_memory_stats();
        assert_eq!(stats.present_pages, 2);
        assert_eq!(stats.swapped_pages, 1);
    }

    #[test]
    fn test_page_table_flags() {
        let entry = PageTableEntry::new(100);
        assert!(entry.flags.present);
        assert!(entry.flags.writable);
        assert!(entry.flags.user_accessible);
    }

    #[test]
    fn test_page_not_present() {
        let entry = PageTableEntry::not_present();
        assert!(!entry.flags.present);
    }
}
