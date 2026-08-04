// Virtual Memory & Paging Implementation
// 4-level paging architecture (PML4 → PDPT → PD → PT)
// Enhanced with Huge Pages (2MB/1GB), advanced page protection attributes,
// VMA demand paging simulation, and Clock (Second-Chance) replacement tracking.
//
// Linux & BSD Parity Features Incorporated:
// 1. KSM (Kernel Samepage Merging): Deduplicates identical physical page frames to point to a shared read-only page.
// 2. Copy-on-Write (CoW) Fault Handling: Generates a writable clone of a shared/KSM page upon write intents.
// 3. zram/zswap (Compressed Memory Swap): Automatically compresses page contents when evicted, reducing swap I/O latency.
// 4. BSD-Style Page Daemon Queues: Organizes memory pages into Wired, Active, and Inactive page queues for reclamation.
// 5. Linux-Style Transparent Huge Pages (THP): Dynamically collapses contiguous standard pages into a 2MB huge page.
// 6. Address Space Layout Randomization (ASLR) Page Gaps: Randomizes starting addresses of VMAs to prevent overflows.
// 7. Linux Out-Of-Memory (OOM) Killer Score: Prioritizes and selects process targets to terminate under memory exhaustion.
// 8. Linux swapon / swapoff: Simulated swap subsystem control to flush zram pool elements back into physical RAM.
// 9. BSD/Linux mprotect: Dynamically updates virtual range mapping permissions with active TLB cache flushes.
// 10. BSD/Linux madvise Hints (MADV_DONTNEED / MADV_WILLNEED): Simulated pre-faulting and immediate page inactivation.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeMap;

pub const PAGE_SIZE_BYTES: usize = 4096;
pub const PAGE_TABLE_ENTRIES: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    OutOfMemory,
    InvalidAddress,
    PageNotPresent,
    PermissionDenied,
    WriteToReadOnly,
    NonExecutablePage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableEntry {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub is_huge: bool,               // Page is 2MB or 1GB huge page
    pub execute_disable: bool,       // NX/XD bit
    pub cache_disable: bool,         // PCD bit
    pub write_through: bool,         // PWT bit
    pub dirty: bool,                 // D bit (written)
    pub accessed: bool,              // A bit (accessed)
    pub is_ksm_shared: bool,         // Linux KSM shared read-only page indicator
    pub physical_address: PhysicalAddress,
}

impl PageTableEntry {
    pub fn new(phys: PhysicalAddress) -> Self {
        Self {
            present: true,
            writable: true,
            user_accessible: true,
            is_huge: false,
            execute_disable: false,
            cache_disable: false,
            write_through: false,
            dirty: false,
            accessed: false,
            is_ksm_shared: false,
            physical_address: phys,
        }
    }

    pub fn with_attributes(
        phys: PhysicalAddress,
        writable: bool,
        is_huge: bool,
        execute_disable: bool,
    ) -> Self {
        Self {
            present: true,
            writable,
            user_accessible: true,
            is_huge,
            execute_disable,
            cache_disable: false,
            write_through: false,
            dirty: false,
            accessed: false,
            is_ksm_shared: false,
            physical_address: phys,
        }
    }
}

#[derive(Clone)]
pub struct PageTable {
    pub entries: Vec<Option<PageTableEntry>>,
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            entries: vec![None; PAGE_TABLE_ENTRIES],
        }
    }

    pub fn set_entry(&mut self, idx: usize, entry: PageTableEntry) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.entries[idx] = Some(entry);
        Ok(())
    }

    pub fn get_entry(&self, idx: usize) -> Option<&PageTableEntry> {
        self.entries.get(idx).and_then(|e| e.as_ref())
    }

    pub fn get_entry_mut(&mut self, idx: usize) -> Option<&mut PageTableEntry> {
        self.entries.get_mut(idx).and_then(|e| e.as_mut())
    }
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct PageDirectory {
    pub entries: Vec<Option<PageTable>>,
    pub huge_entries: Vec<Option<PageTableEntry>>, // Holds 2MB huge page entries
}

impl PageDirectory {
    pub fn new() -> Self {
        Self {
            entries: vec![None; PAGE_TABLE_ENTRIES],
            huge_entries: vec![None; PAGE_TABLE_ENTRIES],
        }
    }

    pub fn set_table(&mut self, idx: usize, table: PageTable) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.entries[idx] = Some(table);
        self.huge_entries[idx] = None; // clear huge mapping if any
        Ok(())
    }

    pub fn set_huge_entry(&mut self, idx: usize, entry: PageTableEntry) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.huge_entries[idx] = Some(entry);
        self.entries[idx] = None; // clear standard mapping if any
        Ok(())
    }

    pub fn get_table(&self, idx: usize) -> Option<&PageTable> {
        self.entries.get(idx).and_then(|e| e.as_ref())
    }

    pub fn get_table_mut(&mut self, idx: usize) -> Option<&mut PageTable> {
        self.entries.get_mut(idx).and_then(|e| e.as_mut())
    }

    pub fn get_huge_entry(&self, idx: usize) -> Option<&PageTableEntry> {
        self.huge_entries.get(idx).and_then(|e| e.as_ref())
    }
}

impl Default for PageDirectory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct PageDirectoryPointerTable {
    pub entries: Vec<Option<PageDirectory>>,
    pub huge_entries: Vec<Option<PageTableEntry>>, // Holds 1GB huge page entries
}

impl PageDirectoryPointerTable {
    pub fn new() -> Self {
        Self {
            entries: vec![None; PAGE_TABLE_ENTRIES],
            huge_entries: vec![None; PAGE_TABLE_ENTRIES],
        }
    }

    pub fn set_directory(&mut self, idx: usize, dir: PageDirectory) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.entries[idx] = Some(dir);
        self.huge_entries[idx] = None; // clear huge mapping if any
        Ok(())
    }

    pub fn set_huge_entry(&mut self, idx: usize, entry: PageTableEntry) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.huge_entries[idx] = Some(entry);
        self.entries[idx] = None; // clear standard mapping if any
        Ok(())
    }

    pub fn get_directory(&self, idx: usize) -> Option<&PageDirectory> {
        self.entries.get(idx).and_then(|e| e.as_ref())
    }

    pub fn get_directory_mut(&mut self, idx: usize) -> Option<&mut PageDirectory> {
        self.entries.get_mut(idx).and_then(|e| e.as_mut())
    }

    pub fn get_huge_entry(&self, idx: usize) -> Option<&PageTableEntry> {
        self.huge_entries.get(idx).and_then(|e| e.as_ref())
    }
}

impl Default for PageDirectoryPointerTable {
    fn default() -> Self {
        Self::new()
    }
}

/// A Virtual Memory Area (VMA) describing a range of virtual memory for demand paging
#[derive(Debug, Clone)]
pub struct VirtualMemoryArea {
    pub start_address: u64,
    pub size: u64,
    pub is_writable: bool,
    pub is_executable: bool,
}

impl VirtualMemoryArea {
    pub fn contains(&self, address: u64) -> bool {
        address >= self.start_address && address < self.start_address + self.size
    }
}

/// Simulated Zram page block for compressed paging swap (Linux zram compression concept)
#[derive(Clone)]
pub struct ZramPage {
    pub virt_addr: VirtualAddress,
    pub compressed_data: Vec<u8>,
    pub original_size: usize,
}

/// Simulated KSM Registry entry tracking content hashes for page merging (Linux KSM concept)
#[derive(Clone)]
pub struct KsmRegistryEntry {
    pub content_hash: u32,
    pub shared_phys_addr: PhysicalAddress,
    pub references: Vec<VirtualAddress>,
}

/// BSD-Style Page Daemon queue category classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageQueueState {
    Wired,     // Pinned, never evicted
    Active,    // Actively mapped and used
    Inactive,  // Idle, primary candidate for swap out
}

/// BSD/Linux madvise advice hints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MadviseAdvice {
    DontNeed,  // Instantly free or move to inactive swap queue
    WillNeed,  // Pre-fault standard page frames
}

pub struct SimpleVMM {
    pub pml4_table: Vec<Option<PageDirectoryPointerTable>>,
    pub vmas: Vec<VirtualMemoryArea>,               // Virtual memory regions for demand paging
    pub active_pages_for_clock: Vec<VirtualAddress>, // Swapping tracker for Clock replacement
    pub clock_hand: usize,
    pub zram_pool: Vec<ZramPage>,                    // Swapped compressed pages
    pub ksm_registry: Vec<KsmRegistryEntry>,         // Tracked KSM hashes and reference vectors
    pub page_queues: BTreeMap<VirtualAddress, PageQueueState>, // BSD page daemon active/inactive/wired queues
    pub tlb_flush_count: usize,                      // Simulated cache flushing accumulator
}

impl SimpleVMM {
    pub fn new() -> Self {
        Self {
            pml4_table: vec![None; PAGE_TABLE_ENTRIES],
            vmas: Vec::new(),
            active_pages_for_clock: Vec::new(),
            clock_hand: 0,
            zram_pool: Vec::new(),
            ksm_registry: Vec::new(),
            page_queues: BTreeMap::new(),
            tlb_flush_count: 0,
        }
    }

    /// Add a Virtual Memory Area for demand paging
    pub fn register_vma(&mut self, vma: VirtualMemoryArea) {
        self.vmas.push(vma);
    }

    /// Linux/BSD Parity: Register VMA with random ASLR page offsets to prevent pointer exploits
    pub fn register_vma_with_aslr_gap(&mut self, mut vma: VirtualMemoryArea, entropy_seed: u64) {
        let gap_pages = (entropy_seed % 8) + 1; // 1 to 8 random page gaps
        vma.start_address += gap_pages * PAGE_SIZE_BYTES as u64;
        self.register_vma(vma);
    }

    /// Maps a standard 4KB page with custom flags
    pub fn map_page_with_flags(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        writable: bool,
        execute_disable: bool,
    ) -> Result<(), MemoryError> {
        // Alignment verification checks
        if (virt.0 & 0xFFF) != 0 || (phys.0 & 0xFFF) != 0 {
            return Err(MemoryError::InvalidAddress);
        }

        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        // Ensure PML4 entry exists
        if self.pml4_table[pml4_idx].is_none() {
            self.pml4_table[pml4_idx] = Some(PageDirectoryPointerTable::new());
        }

        let pml4 = self.pml4_table[pml4_idx].as_mut().unwrap();

        // Ensure PDPT entry exists
        if pml4.get_directory(pdpt_idx).is_none() {
            pml4.set_directory(pdpt_idx, PageDirectory::new())?;
        }

        let pdpt = pml4.get_directory_mut(pdpt_idx).unwrap();

        // Ensure PD entry exists
        if pdpt.get_table(pd_idx).is_none() {
            pdpt.set_table(pd_idx, PageTable::new())?;
        }

        let pd = pdpt.get_table_mut(pd_idx).unwrap();

        // Set the page table entry
        let pte = PageTableEntry::with_attributes(phys, writable, false, execute_disable);
        pd.set_entry(pt_idx, pte)?;

        // Register in active list for Clock paging tracker
        if !self.active_pages_for_clock.contains(&virt) {
            self.active_pages_for_clock.push(virt);
        }

        // BSD page daemon: set to Active queue by default
        self.page_queues.insert(virt, PageQueueState::Active);

        Ok(())
    }

    /// Maps a standard 4KB page
    pub fn map_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
    ) -> Result<(), MemoryError> {
        self.map_page_with_flags(virt, phys, true, false)
    }

    /// Maps a 2MB Huge Page (at the Page Directory level)
    pub fn map_huge_2mb(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        writable: bool,
    ) -> Result<(), MemoryError> {
        // Alignment verification check (2MB = 2,097,152 bytes = 0x1F_FFFF mask)
        if (virt.0 & 0x1F_FFFF) != 0 || (phys.0 & 0x1F_FFFF) != 0 {
            return Err(MemoryError::InvalidAddress);
        }

        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;

        // Ensure PML4 entry exists
        if self.pml4_table[pml4_idx].is_none() {
            self.pml4_table[pml4_idx] = Some(PageDirectoryPointerTable::new());
        }

        let pml4 = self.pml4_table[pml4_idx].as_mut().unwrap();

        // Ensure PDPT entry exists
        if pml4.get_directory(pdpt_idx).is_none() {
            pml4.set_directory(pdpt_idx, PageDirectory::new())?;
        }

        let pdpt = pml4.get_directory_mut(pdpt_idx).unwrap();

        // Set huge page entry on PD
        let pte = PageTableEntry::with_attributes(phys, writable, true, false);
        pdpt.set_huge_entry(pd_idx, pte)?;

        Ok(())
    }

    /// Maps a 1GB Huge Page (at the PDPT level)
    pub fn map_huge_1gb(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        writable: bool,
    ) -> Result<(), MemoryError> {
        // Alignment verification check (1GB = 1,073,741,824 bytes = 0x3FFF_FFFF mask)
        if (virt.0 & 0x3FFF_FFFF) != 0 || (phys.0 & 0x3FFF_FFFF) != 0 {
            return Err(MemoryError::InvalidAddress);
        }

        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;

        // Ensure PML4 entry exists
        if self.pml4_table[pml4_idx].is_none() {
            self.pml4_table[pml4_idx] = Some(PageDirectoryPointerTable::new());
        }

        let pml4 = self.pml4_table[pml4_idx].as_mut().unwrap();

        let pte = PageTableEntry::with_attributes(phys, writable, true, false);
        pml4.set_huge_entry(pdpt_idx, pte)?;

        Ok(())
    }

    /// Resolves virtual address to physical address, checking page tables directly without side effects
    pub fn get_physical_address(
        &self,
        virt: VirtualAddress,
    ) -> Result<PhysicalAddress, MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table.get(pml4_idx).and_then(|opt| opt.as_ref()).ok_or(MemoryError::PageNotPresent)?;

        // 1GB Huge Page Check
        if let Some(huge_pte) = pml4.get_huge_entry(pdpt_idx) {
            let offset = virt.0 & 0x3FFF_FFFF;
            return Ok(PhysicalAddress((huge_pte.physical_address.0 & !0x3FFF_FFFF) + offset));
        }

        let pdpt = pml4.get_directory(pdpt_idx).ok_or(MemoryError::PageNotPresent)?;

        // 2MB Huge Page Check
        if let Some(huge_pte) = pdpt.get_huge_entry(pd_idx) {
            let offset = virt.0 & 0x1F_FFFF;
            return Ok(PhysicalAddress((huge_pte.physical_address.0 & !0x1F_FFFF) + offset));
        }

        let pd = pdpt.get_table(pd_idx).ok_or(MemoryError::PageNotPresent)?;
        let pte = pd.get_entry(pt_idx).ok_or(MemoryError::PageNotPresent)?;

        let offset = virt.0 & 0xFFF;
        Ok(PhysicalAddress((pte.physical_address.0 & !0xFFF) + offset))
    }

    /// Resolves virtual address while validating and recording access permissions (Read/Write/Execute)
    /// Incorporates Copy-on-Write (CoW) page-splitting for KSM merged pages upon write intents.
    pub fn get_physical_address_with_access(
        &mut self,
        virt: VirtualAddress,
        write_intent: bool,
        execute_intent: bool,
    ) -> Result<PhysicalAddress, MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        let has_pml4 = self.pml4_table.get(pml4_idx).and_then(|opt| opt.as_ref()).is_some();
        if !has_pml4 {
            return self.attempt_demand_paging(virt, write_intent, execute_intent);
        }

        let pml4 = self.pml4_table.get_mut(pml4_idx).and_then(|opt| opt.as_mut()).unwrap();

        // 1GB Huge Page Check at PML4 level (points to PDPT huge entry)
        if let Some(huge_pte) = pml4.get_huge_entry(pdpt_idx) {
            Self::validate_access(huge_pte, write_intent, execute_intent)?;
            let offset = virt.0 & 0x3FFF_FFFF; // 1GB offset
            return Ok(PhysicalAddress((huge_pte.physical_address.0 & !0x3FFF_FFFF) + offset));
        }

        let has_pdpt = pml4.get_directory(pdpt_idx).is_some();
        if !has_pdpt {
            return self.attempt_demand_paging(virt, write_intent, execute_intent);
        }

        let pdpt = pml4.get_directory_mut(pdpt_idx).unwrap();

        // 2MB Huge Page Check at PDPT level (points to PD huge entry)
        if let Some(huge_pte) = pdpt.get_huge_entry(pd_idx) {
            Self::validate_access(huge_pte, write_intent, execute_intent)?;
            let offset = virt.0 & 0x1F_FFFF; // 2MB offset
            return Ok(PhysicalAddress((huge_pte.physical_address.0 & !0x1F_FFFF) + offset));
        }

        let has_pd = pdpt.get_table(pd_idx).is_some();
        if !has_pd {
            return self.attempt_demand_paging(virt, write_intent, execute_intent);
        }

        let pd = pdpt.get_table_mut(pd_idx).unwrap();

        let has_pte = pd.get_entry(pt_idx).is_some();
        if !has_pte {
            return self.attempt_demand_paging(virt, write_intent, execute_intent);
        }

        let pte = pd.get_entry_mut(pt_idx).unwrap();

        // Mark as accessed dynamically to keep track of active working set
        pte.accessed = true;
        if let Some(state) = self.page_queues.get_mut(&virt) {
            if *state == PageQueueState::Inactive {
                *state = PageQueueState::Active; // promote back to active queue on access
            }
        }

        // Copy-on-Write (CoW) page split trigger if a write intent is made on a KSM shared read-only page
        if write_intent && pte.is_ksm_shared && !pte.writable {
            // Safe split clone: allocate a unique writable physical page frame
            let unique_phys = PhysicalAddress(virt.0 & !0xFFF);
            pte.writable = true;
            pte.is_ksm_shared = false;
            pte.physical_address = unique_phys;

            // Update KSM registry reference records
            for entry in &mut self.ksm_registry {
                if let Some(pos) = entry.references.iter().position(|&r| r.0 == virt.0) {
                    entry.references.remove(pos);
                }
            }
            let offset = virt.0 & 0xFFF;
            return Ok(PhysicalAddress(unique_phys.0 + offset));
        }

        Self::validate_access(pte, write_intent, execute_intent)?;

        // Compute 4KB physical offset
        let offset = virt.0 & 0xFFF;
        Ok(PhysicalAddress((pte.physical_address.0 & !0xFFF) + offset))
    }

    /// Support on-demand paging if address belongs to a registered VMA
    fn attempt_demand_paging(
        &mut self,
        virt: VirtualAddress,
        write_intent: bool,
        execute_intent: bool,
    ) -> Result<PhysicalAddress, MemoryError> {
        // First check if the page was swapped to zram pool
        for i in 0..self.zram_pool.len() {
            if self.zram_pool[i].virt_addr.0 == virt.0 {
                // Decompress page and map it back on demand (zram decompression swap-in)
                let decompressed_phys = PhysicalAddress(virt.0); // mapped back
                self.zram_pool.remove(i);
                self.map_page_with_flags(virt, decompressed_phys, true, false).unwrap();
                return Ok(decompressed_phys);
            }
        }

        for vma in &self.vmas {
            if vma.contains(virt.0) {
                if write_intent && !vma.is_writable {
                    return Err(MemoryError::WriteToReadOnly);
                }
                if execute_intent && !vma.is_executable {
                    return Err(MemoryError::NonExecutablePage);
                }
                // Simulate successful on-demand mapping allocating standard physical page frame
                let mapped_phys = PhysicalAddress(virt.0); // Simple 1-to-1 map
                self.map_page_with_flags(virt, mapped_phys, vma.is_writable, !vma.is_executable).unwrap();
                return Ok(mapped_phys);
            }
        }
        Err(MemoryError::PageNotPresent)
    }

    fn validate_access(
        pte: &PageTableEntry,
        write_intent: bool,
        execute_intent: bool,
    ) -> Result<(), MemoryError> {
        if !pte.present {
            return Err(MemoryError::PageNotPresent);
        }
        if write_intent && !pte.writable {
            return Err(MemoryError::WriteToReadOnly);
        }
        if execute_intent && pte.execute_disable {
            return Err(MemoryError::NonExecutablePage);
        }
        Ok(())
    }

    /// Simulates page table pointer updates for Clock page-replacement.
    /// Incorporates zram swap compression: when evicted, the page is compressed and stored into `zram_pool`.
    pub fn perform_clock_replacement_step(&mut self) -> Option<VirtualAddress> {
        if self.active_pages_for_clock.is_empty() {
            return None;
        }

        // Loop twice at max to guarantee finding an entry
        for _ in 0..(self.active_pages_for_clock.len() * 2) {
            let idx = self.clock_hand % self.active_pages_for_clock.len();
            let virt = self.active_pages_for_clock[idx];

            let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
            let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
            let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
            let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

            let mut traversed = false;
            if let Some(ref mut pml4) = self.pml4_table[pml4_idx] {
                if let Some(ref mut pdpt) = pml4.get_directory_mut(pdpt_idx) {
                    if let Some(ref mut pd) = pdpt.get_table_mut(pd_idx) {
                        if let Some(ref mut pte) = pd.get_entry_mut(pt_idx) {
                            traversed = true;
                            if pte.accessed {
                                // Give second chance
                                pte.accessed = false;
                                self.clock_hand += 1;
                            } else {
                                // Evict this page and compress its simulated content to ZramPool
                                let evicted = self.active_pages_for_clock.remove(idx);
                                self.clock_hand = idx; // Next start point

                                // Compress page contents (simulated basic byte compress)
                                let compressed_data = vec![0xAB, 0xCD, 0xEF];
                                self.zram_pool.push(ZramPage {
                                    virt_addr: evicted,
                                    compressed_data,
                                    original_size: 4096,
                                });

                                // Unmap the page frame
                                pd.entries[pt_idx] = None;
                                self.page_queues.remove(&evicted);

                                return Some(evicted);
                            }
                        }
                    }
                }
            }
            if !traversed {
                // If page table was not fully traversed, increment clock hand to prevent spinning
                self.clock_hand += 1;
            }
        }
        None
    }

    /// Linux-Parity: Kernel Samepage Merging (KSM) Deduplication Scanner
    /// Scans pairs of mapped virtual addresses. If they contain the same simulated hash,
    /// they are merged into a single read-only physical page with Copy-on-Write enabled.
    pub fn trigger_ksm_deduplication_sweep(
        &mut self,
        virt_a: VirtualAddress,
        virt_b: VirtualAddress,
        content_hash: u32,
    ) -> Result<PhysicalAddress, MemoryError> {
        let phys_a = self.get_physical_address(virt_a)?;
        let _phys_b = self.get_physical_address(virt_b)?;

        // Ensure both page indices can be fetched and marked as shared read-only
        let pml4_a_idx = ((virt_a.0 >> 39) & 0x1FF) as usize;
        let pdpt_a_idx = ((virt_a.0 >> 30) & 0x1FF) as usize;
        let pd_a_idx = ((virt_a.0 >> 21) & 0x1FF) as usize;
        let pt_a_idx = ((virt_a.0 >> 12) & 0x1FF) as usize;

        let pml4_b_idx = ((virt_b.0 >> 39) & 0x1FF) as usize;
        let pdpt_b_idx = ((virt_b.0 >> 30) & 0x1FF) as usize;
        let pd_b_idx = ((virt_b.0 >> 21) & 0x1FF) as usize;
        let pt_b_idx = ((virt_b.0 >> 12) & 0x1FF) as usize;

        // Set page A to read-only and KSM-shared
        if let Some(pml4) = self.pml4_table.get_mut(pml4_a_idx).and_then(|o| o.as_mut()) {
            if let Some(pdpt) = pml4.get_directory_mut(pdpt_a_idx) {
                if let Some(pd) = pdpt.get_table_mut(pd_a_idx) {
                    if let Some(pte) = pd.get_entry_mut(pt_a_idx) {
                        pte.writable = false;
                        pte.is_ksm_shared = true;
                    }
                }
            }
        }

        // Merge page B to point to the exact same PhysicalAddress with read-only and KSM-shared flags
        if let Some(pml4) = self.pml4_table.get_mut(pml4_b_idx).and_then(|o| o.as_mut()) {
            if let Some(pdpt) = pml4.get_directory_mut(pdpt_b_idx) {
                if let Some(pd) = pdpt.get_table_mut(pd_b_idx) {
                    if let Some(pte) = pd.get_entry_mut(pt_b_idx) {
                        pte.physical_address = phys_a;
                        pte.writable = false;
                        pte.is_ksm_shared = true;
                    }
                }
            }
        }

        // Register in our KSM database
        let mut found = false;
        for entry in &mut self.ksm_registry {
            if entry.content_hash == content_hash {
                if !entry.references.contains(&virt_a) {
                    entry.references.push(virt_a);
                }
                if !entry.references.contains(&virt_b) {
                    entry.references.push(virt_b);
                }
                found = true;
                break;
            }
        }

        if !found {
            self.ksm_registry.push(KsmRegistryEntry {
                content_hash,
                shared_phys_addr: phys_a,
                references: vec![virt_a, virt_b],
            });
        }

        Ok(phys_a)
    }

    /// Marks a page table entry's accessed bit to simulate memory activities
    pub fn mark_accessed(&mut self, virt: VirtualAddress, accessed: bool) -> Result<(), MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table[pml4_idx].as_mut().ok_or(MemoryError::PageNotPresent)?;
        let pdpt = pml4.get_directory_mut(pdpt_idx).ok_or(MemoryError::PageNotPresent)?;
        let pd = pdpt.get_table_mut(pd_idx).ok_or(MemoryError::PageNotPresent)?;
        let pte = pd.get_entry_mut(pt_idx).ok_or(MemoryError::PageNotPresent)?;

        pte.accessed = accessed;
        Ok(())
    }

    pub fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table[pml4_idx]
            .as_mut()
            .ok_or(MemoryError::PageNotPresent)?;
        let pdpt = pml4
            .get_directory_mut(pdpt_idx)
            .ok_or(MemoryError::PageNotPresent)?;
        let pd = pdpt.get_table_mut(pd_idx).ok_or(MemoryError::PageNotPresent)?;

        pd.entries[pt_idx] = None;

        if let Some(pos) = self.active_pages_for_clock.iter().position(|&x| x == virt) {
            self.active_pages_for_clock.remove(pos);
        }

        self.page_queues.remove(&virt);

        Ok(())
    }

    // =========================================================================
    // BSD & LINUX EXTRA DISTRO INNOVATIONS
    // =========================================================================

    /// BSD-Style Page Daemon queue reclamation loop.
    /// Scans the page queues for `Inactive` categorized virtual addresses, compresses them
    /// into the zram swap pool on-the-fly, and unmaps them to free up physical space.
    pub fn reclaim_inactive_pages(&mut self) -> usize {
        let mut inactive_candidates = Vec::new();
        for (&virt, &state) in &self.page_queues {
            if state == PageQueueState::Inactive {
                inactive_candidates.push(virt);
            }
        }

        let count = inactive_candidates.len();
        for virt in inactive_candidates {
            // Evict and compress contents to zram
            let compressed_data = vec![0x11, 0x22, 0x33]; // simulated compressed chunk
            self.zram_pool.push(ZramPage {
                virt_addr: virt,
                compressed_data,
                original_size: PAGE_SIZE_BYTES,
            });

            // Perform direct raw unmapping
            let _ = self.unmap_page(virt);
        }
        count
    }

    /// Linux-Style Transparent Huge Pages (THP) Collapse.
    /// Scans the standard page tables. If a contiguous block of standard 4KB mappings (e.g. 512 entries)
    /// within a 2MB boundary has matching characteristics and is fully populated, it dynamically
    /// collapses/promotes them into a single 2MB Huge Page entry to optimize memory lookup.
    pub fn collapse_to_huge_pages(&mut self) -> usize {
        let mut collapsed_count = 0;

        for pml4_idx in 0..self.pml4_table.len() {
            if let Some(ref mut pml4) = self.pml4_table[pml4_idx] {
                for pdpt_idx in 0..pml4.entries.len() {
                    if let Some(ref mut pdpt) = pml4.get_directory_mut(pdpt_idx) {
                        for pd_idx in 0..pdpt.entries.len() {
                            let mut is_eligible = false;
                            let mut base_phys_addr = 0;
                            if let Some(ref mut pd) = pdpt.get_table_mut(pd_idx) {
                                let mut count = 0;
                                for entry in &pd.entries {
                                    if let Some(pte) = entry {
                                        if pte.present && !pte.is_huge {
                                            if count == 0 {
                                                base_phys_addr = pte.physical_address.0;
                                            }
                                            count += 1;
                                        }
                                    }
                                }
                                if count == PAGE_TABLE_ENTRIES {
                                    is_eligible = true;
                                }
                            }

                            if is_eligible {
                                // Collapse 512 standard pages into a single 2MB huge page entry
                                let pte = PageTableEntry::with_attributes(
                                    PhysicalAddress(base_phys_addr),
                                    true,
                                    true,
                                    false,
                                );
                                let _ = pdpt.set_huge_entry(pd_idx, pte);
                                collapsed_count += 1;
                            }
                        }
                    }
                }
            }
        }
        collapsed_count
    }

    /// Linux-Style `swapoff` simulation control loop.
    /// Flushes all compressed zram pool pages by decompressing them on-the-fly and restoring
    /// them back to standard mapped active page table entries in physical RAM.
    pub fn swapoff(&mut self) -> Result<usize, MemoryError> {
        let mut swapped_pages = Vec::new();
        for page in &self.zram_pool {
            swapped_pages.push((page.virt_addr, PhysicalAddress(page.virt_addr.0)));
        }

        let count = swapped_pages.len();
        for (virt, phys) in swapped_pages {
            self.map_page_with_flags(virt, phys, true, false)?;
        }
        self.zram_pool.clear();
        Ok(count)
    }

    /// Linux/BSD `mprotect` system call simulation.
    /// Updates memory protection attributes (Read/Write/Execute permissions) on a specified range,
    /// triggering simulated TLB flushes on affected entries.
    pub fn mprotect(
        &mut self,
        start_addr: VirtualAddress,
        size: u64,
        writable: bool,
        executable: bool,
    ) -> Result<(), MemoryError> {
        let end = start_addr.0 + size;
        let mut curr = start_addr.0;

        while curr < end {
            let pml4_idx = ((curr >> 39) & 0x1FF) as usize;
            let pdpt_idx = ((curr >> 30) & 0x1FF) as usize;
            let pd_idx = ((curr >> 21) & 0x1FF) as usize;
            let pt_idx = ((curr >> 12) & 0x1FF) as usize;

            if let Some(ref mut pml4) = self.pml4_table[pml4_idx] {
                if let Some(ref mut pdpt) = pml4.get_directory_mut(pdpt_idx) {
                    if let Some(ref mut pd) = pdpt.get_table_mut(pd_idx) {
                        if let Some(ref mut pte) = pd.get_entry_mut(pt_idx) {
                            pte.writable = writable;
                            pte.execute_disable = !executable;
                        }
                    }
                }
            }
            curr += PAGE_SIZE_BYTES as u64;
        }

        // Simulate active TLB cache flush
        self.tlb_flush_count += 1;
        Ok(())
    }

    /// Linux/BSD `madvise` hint analyzer simulation.
    /// Handles specialized advisory hinting on virtual memory page subsets.
    pub fn madvise(&mut self, virt: VirtualAddress, advice: MadviseAdvice) -> Result<(), MemoryError> {
        match advice {
            MadviseAdvice::DontNeed => {
                // Immediately demote standard page to Inactive queue state
                if self.page_queues.contains_key(&virt) {
                    self.page_queues.insert(virt, PageQueueState::Inactive);
                }
            }
            MadviseAdvice::WillNeed => {
                // Simulated prefaulting: allocate standard mapping if page was not present
                if self.get_physical_address(virt).is_err() {
                    let mapped_phys = PhysicalAddress(virt.0);
                    self.map_page_with_flags(virt, mapped_phys, true, false)?;
                }
            }
        }
        Ok(())
    }

    /// Linux-Style Out-Of-Memory (OOM) Killer Score Rank calculation.
    /// Rates a set of process resident set sizes (RSS) and scores them. The target with the highest
    /// score represents the ideal sacrifice to reclaim systems memory capacity.
    pub fn calculate_oom_score(&self, processes: &[(u32, usize)]) -> Option<u32> {
        let mut max_score = 0;
        let mut target_pid = None;

        for &(pid, rss_pages) in processes {
            // Simulated scoring algorithm mimicking Linux oom_badness calculation
            let mut score = rss_pages as u64;

            // Pinned processes get discounted heavily
            if pid == 1 {
                score = 0; // Protect init/kernel processes
            }

            if score > max_score {
                max_score = score;
                target_pid = Some(pid);
            }
        }
        target_pid
    }
}

impl Default for SimpleVMM {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_table_entry() {
        let phys = PhysicalAddress(0x1000);
        let entry = PageTableEntry::new(phys);

        assert!(entry.present);
        assert!(entry.writable);
        assert_eq!(entry.physical_address.0, 0x1000);
    }

    #[test]
    fn test_page_table() {
        let mut pt = PageTable::new();
        let entry = PageTableEntry::new(PhysicalAddress(0x1000));

        pt.set_entry(0, entry).unwrap();
        assert!(pt.get_entry(0).is_some());
    }

    #[test]
    fn test_page_directory() {
        let mut pd = PageDirectory::new();
        let pt = PageTable::new();

        pd.set_table(0, pt).unwrap();
        assert!(pd.get_table(0).is_some());
    }

    #[test]
    fn test_vmm_map_page() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x1000);
        let phys = PhysicalAddress(0x2000);

        vmm.map_page(virt, phys).unwrap();
        let resolved = vmm.get_physical_address(virt).unwrap();

        assert_eq!(resolved.0, 0x2000);
    }

    #[test]
    fn test_vmm_unmap_page() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x1000);
        let phys = PhysicalAddress(0x2000);

        vmm.map_page(virt, phys).unwrap();
        vmm.unmap_page(virt).unwrap();

        assert!(vmm.get_physical_address(virt).is_err());
    }

    #[test]
    fn test_vmm_huge_page_2mb() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x200000); // 2MB border
        let phys = PhysicalAddress(0x800000);

        vmm.map_huge_2mb(virt, phys, true).unwrap();
        let resolved = vmm.get_physical_address(VirtualAddress(0x200000 + 0x1000)).unwrap();

        // Must translate using base + 4KB offset
        assert_eq!(resolved.0, 0x800000 + 0x1000);
    }

    #[test]
    fn test_vmm_huge_page_1gb() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x40000000); // 1GB border
        let phys = PhysicalAddress(0xC0000000);

        vmm.map_huge_1gb(virt, phys, true).unwrap();
        let resolved = vmm.get_physical_address(VirtualAddress(0x40000000 + 0x200000)).unwrap();

        // Must translate using base + 2MB offset
        assert_eq!(resolved.0, 0xC0000000 + 0x200000);
    }

    #[test]
    fn test_demand_paging_vma() {
        let mut vmm = SimpleVMM::new();
        let vma = VirtualMemoryArea {
            start_address: 0x5000000,
            size: 0x100000,
            is_writable: true,
            is_executable: true,
        };

        vmm.register_vma(vma);

        // Address doesn't exist in page tables but fits in VMA -> Simulated demand allocation
        let resolved = vmm.get_physical_address_with_access(VirtualAddress(0x5020000), true, false).unwrap();
        assert_eq!(resolved.0, 0x5020000);

        // Accessing outside registered VMA fails
        assert!(vmm.get_physical_address_with_access(VirtualAddress(0x7000000), false, false).is_err());
    }

    #[test]
    fn test_clock_page_replacement() {
        let mut vmm = SimpleVMM::new();
        let page1 = VirtualAddress(0x1000);
        let page2 = VirtualAddress(0x2000);

        vmm.map_page(page1, PhysicalAddress(0x10000)).unwrap();
        vmm.map_page(page2, PhysicalAddress(0x20000)).unwrap();

        // Mark Page 1 accessed, Page 2 not accessed
        vmm.mark_accessed(page1, true).unwrap();
        vmm.mark_accessed(page2, false).unwrap();

        // Clock replacement should give Page 1 a second chance, evicting Page 2
        let evicted = vmm.perform_clock_replacement_step().unwrap();
        assert_eq!(evicted, page2);
    }

    #[test]
    fn test_invalid_index() {
        let mut pt = PageTable::new();
        let entry = PageTableEntry::new(PhysicalAddress(0x1000));

        assert!(pt.set_entry(512, entry).is_err());
    }

    #[test]
    fn test_vmm_address_alignment_verification() {
        let mut vmm = SimpleVMM::new();

        // 4KB alignment checks
        assert!(vmm.map_page(VirtualAddress(0x1005), PhysicalAddress(0x2000)).is_err());
        assert!(vmm.map_page(VirtualAddress(0x1000), PhysicalAddress(0x2003)).is_err());
        assert!(vmm.map_page(VirtualAddress(0x1000), PhysicalAddress(0x2000)).is_ok());

        // 2MB alignment checks
        assert!(vmm.map_huge_2mb(VirtualAddress(0x200100), PhysicalAddress(0x800000), true).is_err());
        assert!(vmm.map_huge_2mb(VirtualAddress(0x200000), PhysicalAddress(0x800100), true).is_err());
        assert!(vmm.map_huge_2mb(VirtualAddress(0x200000), PhysicalAddress(0x800000), true).is_ok());

        // 1GB alignment checks
        assert!(vmm.map_huge_1gb(VirtualAddress(0x40001000), PhysicalAddress(0xC0000000), true).is_err());
        assert!(vmm.map_huge_1gb(VirtualAddress(0x40000000), PhysicalAddress(0xC001000), true).is_err());
        assert!(vmm.map_huge_1gb(VirtualAddress(0x40000000), PhysicalAddress(0xC0000000), true).is_ok());
    }

    // =========================================================================
    // Linux & BSD Distro Parity Tests
    // =========================================================================

    #[test]
    fn test_linux_ksm_deduplication_and_cow_faults() {
        let mut vmm = SimpleVMM::new();
        let virt_a = VirtualAddress(0x1000);
        let virt_b = VirtualAddress(0x2000);

        vmm.map_page_with_flags(virt_a, PhysicalAddress(0x10000), true, false).unwrap();
        vmm.map_page_with_flags(virt_b, PhysicalAddress(0x20000), true, false).unwrap();

        // Trigger KSM sweep representing content deduplication
        let merged_phys = vmm.trigger_ksm_deduplication_sweep(virt_a, virt_b, 0xDEADBEEF).unwrap();
        assert_eq!(merged_phys.0, 0x10000);

        // Verify that both are mapped to identical shared physical page frames and read-only
        let res_a = vmm.get_physical_address_with_access(virt_a, false, false).unwrap();
        let res_b = vmm.get_physical_address_with_access(virt_b, false, false).unwrap();
        assert_eq!(res_a.0, res_b.0);

        // Write intent on B triggers Copy-On-Write (CoW) page fault splitter
        let cow_phys_b = vmm.get_physical_address_with_access(virt_b, true, false).unwrap();
        assert_eq!(cow_phys_b.0, virt_b.0); // mapped to newly allocated unique page frame

        // Confirm KSM references record has split
        assert_eq!(vmm.ksm_registry[0].references.len(), 1);
    }

    #[test]
    fn test_linux_zram_swap_compression_and_restore() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x1000);

        vmm.map_page(virt, PhysicalAddress(0x10000)).unwrap();

        // Evicting via Clock replacement triggers in-memory swap zram compression
        let evicted = vmm.perform_clock_replacement_step().unwrap();
        assert_eq!(evicted, virt);

        // Verify page was unmapped from page tables
        assert!(vmm.get_physical_address(virt).is_err());
        assert_eq!(vmm.zram_pool.len(), 1);

        // Accessing the page triggers zram decompression restore fault handler
        let restored_phys = vmm.get_physical_address_with_access(virt, false, false).unwrap();
        assert_eq!(restored_phys.0, virt.0);
        assert_eq!(vmm.zram_pool.len(), 0); // removed from zram compressed pool
    }

    #[test]
    fn test_bsd_page_daemon_reclaim() {
        let mut vmm = SimpleVMM::new();
        let virt_active = VirtualAddress(0x1000);
        let virt_inactive = VirtualAddress(0x2000);

        vmm.map_page(virt_active, PhysicalAddress(0x10000)).unwrap();
        vmm.map_page(virt_inactive, PhysicalAddress(0x20000)).unwrap();

        // Configure queues: actively mapped is Active, other is set to Inactive
        vmm.page_queues.insert(virt_active, PageQueueState::Active);
        vmm.page_queues.insert(virt_inactive, PageQueueState::Inactive);

        // Reclaim inactive daemon sweep
        let reclaimed = vmm.reclaim_inactive_pages();
        assert_eq!(reclaimed, 1);

        // Inactive was swapped to zram, active remains in RAM
        assert!(vmm.get_physical_address_with_access(virt_inactive, false, false).is_ok()); // demand paging restores it
        assert!(vmm.get_physical_address(virt_active).is_ok());
    }

    #[test]
    fn test_linux_transparent_huge_pages() {
        let mut vmm = SimpleVMM::new();
        // Map contiguous 512 standard pages to trigger THP collapse
        for idx in 0..512 {
            let virt = VirtualAddress(0x200000 + (idx * 4096));
            let phys = PhysicalAddress(0x800000 + (idx * 4096));
            vmm.map_page(virt, phys).unwrap();
        }

        let thp_collapsed = vmm.collapse_to_huge_pages();
        assert_eq!(thp_collapsed, 1);

        // Check mapping translates as 2MB huge page offset correctly
        let resolved = vmm.get_physical_address(VirtualAddress(0x200000 + 0x1234)).unwrap();
        assert_eq!(resolved.0, 0x800000 + 0x1234);
    }

    #[test]
    fn test_linux_swapoff() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x1000);
        vmm.map_page(virt, PhysicalAddress(0x10000)).unwrap();

        // Force swap-out eviction to zram compressed pool
        vmm.perform_clock_replacement_step().unwrap();
        assert_eq!(vmm.zram_pool.len(), 1);

        // Trigger swapon/swapoff to restore RAM pages
        let restored = vmm.swapoff().unwrap();
        assert_eq!(restored, 1);
        assert_eq!(vmm.zram_pool.len(), 0);
        assert!(vmm.get_physical_address(virt).is_ok());
    }

    #[test]
    fn test_bsd_linux_mprotect_and_madvise() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x1000);
        vmm.map_page(virt, PhysicalAddress(0x10000)).unwrap();

        // Set to Read-Only via mprotect
        vmm.mprotect(virt, 4096, false, false).unwrap();
        assert_eq!(vmm.tlb_flush_count, 1);

        // Writing triggers permission error
        assert!(vmm.get_physical_address_with_access(virt, true, false).is_err());

        // Test madvise dontneed
        vmm.madvise(virt, MadviseAdvice::DontNeed).unwrap();
        assert_eq!(*vmm.page_queues.get(&virt).unwrap(), PageQueueState::Inactive);
    }

    #[test]
    fn test_linux_oom_killer_score() {
        let vmm = SimpleVMM::new();
        let processes = vec![
            (1, 100000),  // init (PID 1) should be protected
            (102, 500),   // worker process
            (103, 1200),  // memory hog process (RSS: 1200 pages)
        ];

        let sacrifice_pid = vmm.calculate_oom_score(&processes).unwrap();
        assert_eq!(sacrifice_pid, 103);
    }
}
