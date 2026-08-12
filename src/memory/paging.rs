// Virtual Memory & Paging Implementation
// 4-level paging architecture (PML4 → PDPT → PD → PT)
// Enhanced with Huge Pages (2MB/1GB), advanced page protection attributes,
// VMA demand paging simulation, and Clock (Second-Chance) replacement tracking.
//
// Linux, FreeBSD, and OpenBSD Parity Features Incorporated:
// 1. KSM (Kernel Samepage Merging): Deduplicates identical physical page frames to point to a shared read-only page.
// 2. Copy-on-Write (CoW) Fault Handling: Generates a writable clone of a shared/KSM page upon write intents.
// 3. zram/zswap (Compressed Memory Swap): Automatically compresses page contents when evicted, reducing swap I/O latency.
// 4. FreeBSD-style Wired (Pinned) Pages: Pinned kernel pages (`is_wired`) that are absolutely immune to page reclamation or swaps.
// 5. OpenBSD-style W^X (Write XOR Execute) Security Gate: Strict page-table check preventing any page from being simultaneously writable and executable.
// 6. Linux-style kswapd Page Reclaimer Daemon: Automated page reclamation sweeps (`SovereignPageReclaimer`) based on low/high page watermarks.

#![no_std]

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

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
    WxViolation, // OpenBSD-style Write XOR Execute security violation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub is_wired: bool,              // FreeBSD-style wired/pinned page indicator (cannot be swapped)
    pub physical_address: PhysicalAddress,
}

impl PageTableEntry {
    pub fn new(phys: PhysicalAddress) -> Self {
        Self {
            present: true,
            writable: true,
            user_accessible: true,
            is_huge: false,
            execute_disable: true, // safe default conforming to W^X
            cache_disable: false,
            write_through: false,
            dirty: false,
            accessed: false,
            is_ksm_shared: false,
            is_wired: false,
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
            is_wired: false,
            physical_address: phys,
        }
    }

    pub fn with_wired(
        phys: PhysicalAddress,
        writable: bool,
        is_huge: bool,
        execute_disable: bool,
        is_wired: bool,
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
            is_wired,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZramAlgorithm {
    Lzo,
    Lz4,
    Zstd,
}

/// Simulated Zram page block for compressed paging swap (Linux zram compression concept)
#[derive(Clone)]
pub struct ZramPage {
    pub virt_addr: VirtualAddress,
    pub compressed_data: Vec<u8>,
    pub original_size: usize,
    pub algorithm: ZramAlgorithm,
}

/// Simulated KSM Registry entry tracking content hashes for page merging (Linux KSM concept)
#[derive(Clone)]
pub struct KsmRegistryEntry {
    pub content_hash: u32,
    pub shared_phys_addr: PhysicalAddress,
    pub references: Vec<VirtualAddress>,
}

pub struct SimpleVMM {
    pub pml4_table: Vec<Option<PageDirectoryPointerTable>>,
    pub vmas: Vec<VirtualMemoryArea>, // Virtual memory regions for demand paging
    pub active_pages_for_clock: Vec<VirtualAddress>, // Swapping tracker for Clock replacement
    pub clock_hand: usize,
    pub zram_pool: Vec<ZramPage>,       // Swapped compressed pages
    pub ksm_registry: Vec<KsmRegistryEntry>, // Tracked KSM hashes and reference vectors
    pub swappiness: u8,                 // Linux swappiness (0-100)
    pub zram_algorithm: ZramAlgorithm,  // Selected zram compression algorithm
    pub zswap_max_pool_pages: usize,    // Max compressed pages before writing to disk
    pub swap_disk: Vec<(VirtualAddress, Vec<u8>)>, // Secondary Tier 2 disk swap
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
            swappiness: 60,
            zram_algorithm: ZramAlgorithm::Zstd,
            zswap_max_pool_pages: 4,
            swap_disk: Vec::new(),
        }
    }

    /// Add a Virtual Memory Area for demand paging
    pub fn register_vma(&mut self, vma: VirtualMemoryArea) {
        self.vmas.push(vma);
    }

    /// Maps a standard 4KB page (W^X safe default)
    pub fn map_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
    ) -> Result<(), MemoryError> {
        self.map_page_with_flags(virt, phys, true, true)
    }

    /// Maps a standard 4KB page with flags
    pub fn map_page_with_flags(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        writable: bool,
        execute_disable: bool,
    ) -> Result<(), MemoryError> {
        self.map_page_with_full_flags(virt, phys, writable, execute_disable, false)
    }

    /// Maps a standard 4KB page with full flags, supporting wired/pinned and W^X security checks
    pub fn map_page_with_full_flags(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        writable: bool,
        execute_disable: bool,
        is_wired: bool,
    ) -> Result<(), MemoryError> {
        // Alignment verification check (4KB = 4096 bytes = 0xFFF mask)
        if (virt.0 & 0xFFF) != 0 || (phys.0 & 0xFFF) != 0 {
            return Err(MemoryError::InvalidAddress);
        }

        // OpenBSD-style W^X Security Gate: if writable is true, execute_disable MUST be true
        if writable && !execute_disable {
            return Err(MemoryError::WxViolation);
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
        let pte = PageTableEntry::with_wired(phys, writable, false, execute_disable, is_wired);
        pd.set_entry(pt_idx, pte)?;

        // Register in active list for Clock paging tracker
        if !self.active_pages_for_clock.contains(&virt) {
            self.active_pages_for_clock.push(virt);
        }

        Ok(())
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
        let pte = PageTableEntry::with_attributes(phys, writable, true, true); // W^X safe
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

        let pte = PageTableEntry::with_attributes(phys, writable, true, true); // W^X safe
        pml4.set_huge_entry(pdpt_idx, pte)?;

        Ok(())
    }

    /// Resolves virtual address to physical address, handling huge pages and demand paging VMAs
    pub fn get_physical_address(
        &mut self,
        virt: VirtualAddress,
    ) -> Result<PhysicalAddress, MemoryError> {
        self.get_physical_address_with_access(virt, false, false)
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
        // Tier 1: Check if the page was swapped to zram pool (zswap cache layer)
        for i in 0..self.zram_pool.len() {
            if self.zram_pool[i].virt_addr.0 == virt.0 {
                // Decompress page and map it back on demand (zram decompression swap-in)
                let decompressed_phys = PhysicalAddress(virt.0); // mapped back
                self.zram_pool.remove(i);
                self.map_page_with_flags(virt, decompressed_phys, true, true).unwrap(); // W^X safe
                return Ok(decompressed_phys);
            }
        }

        // Tier 2: Check if the page was swapped out to secondary swap disk
        for i in 0..self.swap_disk.len() {
            if self.swap_disk[i].0.0 == virt.0 {
                let decompressed_phys = PhysicalAddress(virt.0); // read back from disk
                self.swap_disk.remove(i);
                self.map_page_with_flags(virt, decompressed_phys, true, true).unwrap(); // W^X safe
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
        // OpenBSD W^X check on runtime access
        if write_intent && !pte.execute_disable {
            return Err(MemoryError::WxViolation);
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

                            // FreeBSD-style Wired check: wired pages are absolutely immune to reclamation/swaps
                            if pte.is_wired {
                                self.clock_hand += 1;
                                continue;
                            }

                            if pte.accessed {
                                // Give second chance
                                pte.accessed = false;
                                self.clock_hand += 1;
                            } else {
                                // Linux swappiness check: if swappiness is 0, we bypass/avoid swapping out
                                if self.swappiness == 0 {
                                    return None;
                                }

                                // Evict this page and compress its simulated content to ZramPool
                                let evicted = self.active_pages_for_clock.remove(idx);
                                self.clock_hand = idx; // Next start point

                                // Compress page contents depending on selected algorithm (simulated)
                                let ratio = match self.zram_algorithm {
                                    ZramAlgorithm::Lzo => 0.60,
                                    ZramAlgorithm::Lz4 => 0.50,
                                    ZramAlgorithm::Zstd => 0.35,
                                };
                                let compressed_len = (4096.0 * ratio) as usize;
                                let mut compressed_data = vec![0; compressed_len];
                                if compressed_len >= 3 {
                                    compressed_data[0] = 0xAB;
                                    compressed_data[1] = 0xCD;
                                    compressed_data[2] = 0xEF;
                                }

                                self.zram_pool.push(ZramPage {
                                    virt_addr: evicted,
                                    compressed_data,
                                    original_size: 4096,
                                    algorithm: self.zram_algorithm,
                                });

                                // Write-Back Eviction Policy (zswap to secondary disk swap):
                                // If Tier 1 (zram) pool size exceeds the threshold, unstage the oldest page to Tier 2 (swap_disk)
                                if self.zram_pool.len() > self.zswap_max_pool_pages {
                                    let oldest = self.zram_pool.remove(0);
                                    self.swap_disk.push((oldest.virt_addr, oldest.compressed_data));
                                }

                                // Unmap the page frame
                                pd.entries[pt_idx] = None;

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

        Ok(())
    }
}

impl Default for SimpleVMM {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux-style kswapd Page Reclaimer Daemon
pub struct SovereignPageReclaimer {
    pub low_watermark: usize,
    pub high_watermark: usize,
}

impl SovereignPageReclaimer {
    pub fn new(low_watermark: usize, high_watermark: usize) -> Self {
        Self {
            low_watermark,
            high_watermark,
        }
    }

    /// Triggers page reclaiming sweeps when free memory blocks hit low_watermark limits
    pub fn reclaim_pages(&self, vmm: &mut SimpleVMM, current_free_pages: usize) -> usize {
        if current_free_pages >= self.low_watermark {
            return 0; // Watermark is fine; no reclaiming needed
        }

        let mut reclaimed = 0;
        let mut simulated_free_pages = current_free_pages;

        // Evict pages utilizing our Clock paging reclaimer until we reach high_watermark limits
        while simulated_free_pages < self.high_watermark && !vmm.active_pages_for_clock.is_empty() {
            if let Some(_evicted) = vmm.perform_clock_replacement_step() {
                reclaimed += 1;
                simulated_free_pages += 1;
            } else {
                break; // No more evictable pages (e.g. all remaining are wired or swappiness=0)
            }
        }
        reclaimed
    }
}

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

        vmm.map_page_with_flags(virt, phys, true, true).unwrap(); // W^X safe (writable, executable disabled)
        let resolved = vmm.get_physical_address(virt).unwrap();

        assert_eq!(resolved.0, 0x2000);
    }

    #[test]
    fn test_vmm_unmap_page() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x1000);
        let phys = PhysicalAddress(0x2000);

        vmm.map_page_with_flags(virt, phys, true, true).unwrap();
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

        vmm.map_page_with_flags(page1, PhysicalAddress(0x10000), true, true).unwrap();
        vmm.map_page_with_flags(page2, PhysicalAddress(0x20000), true, true).unwrap();

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
        assert!(vmm.map_page_with_flags(VirtualAddress(0x1000), PhysicalAddress(0x2000), true, true).is_ok());

        // 2MB alignment checks
        assert!(vmm.map_huge_2mb(VirtualAddress(0x200100), PhysicalAddress(0x800000), true).is_err());
        assert!(vmm.map_huge_2mb(VirtualAddress(0x200000), PhysicalAddress(0x800100), true).is_err());
        assert!(vmm.map_huge_2mb(VirtualAddress(0x200000), PhysicalAddress(0x800000), true).is_ok());

        // 1GB alignment checks
        assert!(vmm.map_huge_1gb(VirtualAddress(0x40001000), PhysicalAddress(0xC0000000), true).is_err());
        assert!(vmm.map_huge_1gb(VirtualAddress(0x40000000), PhysicalAddress(0xC0100000), true).is_err());
        assert!(vmm.map_huge_1gb(VirtualAddress(0x40000000), PhysicalAddress(0xC0000000), true).is_ok());
    }

    // ==========================================
    // Linux-Parity Paging & Memory Tests
    // ==========================================

    #[test]
    fn test_linux_ksm_deduplication_and_cow_faults() {
        let mut vmm = SimpleVMM::new();
        let virt_a = VirtualAddress(0x1000);
        let virt_b = VirtualAddress(0x2000);

        vmm.map_page_with_flags(virt_a, PhysicalAddress(0x10000), true, true).unwrap();
        vmm.map_page_with_flags(virt_b, PhysicalAddress(0x20000), true, true).unwrap();

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

        vmm.map_page_with_flags(virt, PhysicalAddress(0x10000), true, true).unwrap();

        // Evicting via Clock replacement triggers in-memory swap zram compression
        let evicted = vmm.perform_clock_replacement_step().unwrap();
        assert_eq!(evicted, virt);

        // Verify page was unmapped from page tables into zram_pool
        assert_eq!(vmm.zram_pool.len(), 1);

        // Accessing the page triggers zram decompression restore fault handler
        let restored_phys = vmm.get_physical_address(virt).unwrap();
        assert_eq!(restored_phys.0, virt.0);
        assert_eq!(vmm.zram_pool.len(), 0); // removed from zram compressed pool
    }

    #[test]
    fn test_advanced_multi_tier_swap_and_swappiness() {
        let mut vmm = SimpleVMM::new();
        vmm.swappiness = 0; // Disable swapping!

        let virt1 = VirtualAddress(0x1000);
        vmm.map_page_with_flags(virt1, PhysicalAddress(0x10000), true, true).unwrap();

        // Evicting with swappiness = 0 should bypass swapping
        let evicted = vmm.perform_clock_replacement_step();
        assert!(evicted.is_none());

        // Re-enable swappiness and change algorithm to Zstd (high compression)
        vmm.swappiness = 60;
        vmm.zram_algorithm = ZramAlgorithm::Zstd;
        vmm.zswap_max_pool_pages = 2; // threshold of 2

        let virt2 = VirtualAddress(0x2000);
        let virt3 = VirtualAddress(0x3000);
        let virt4 = VirtualAddress(0x4000);

        vmm.map_page_with_flags(virt1, PhysicalAddress(0x10000), true, true).unwrap();
        vmm.map_page_with_flags(virt2, PhysicalAddress(0x20000), true, true).unwrap();
        vmm.map_page_with_flags(virt3, PhysicalAddress(0x30000), true, true).unwrap();
        vmm.map_page_with_flags(virt4, PhysicalAddress(0x40000), true, true).unwrap();

        // Evict 3 pages
        vmm.perform_clock_replacement_step().unwrap();
        vmm.perform_clock_replacement_step().unwrap();
        vmm.perform_clock_replacement_step().unwrap();

        // Since max zswap size is 2 pages, the 3rd eviction should push the oldest to Tier 2 swap disk
        assert_eq!(vmm.zram_pool.len(), 2);
        assert_eq!(vmm.swap_disk.len(), 1);

        // Verify that the oldest page is located on swap disk
        let oldest_virt = vmm.swap_disk[0].0;
        // Restoring it from Tier 2 swap disk should succeed and remove it from swap disk
        let restored_phys = vmm.get_physical_address(oldest_virt).unwrap();
        assert_eq!(restored_phys.0, oldest_virt.0);
        assert_eq!(vmm.swap_disk.len(), 0);
    }

    // ==========================================
    // Advanced BSD/Competitor Paging Tests
    // ==========================================

    #[test]
    fn test_freebsd_wired_pages_prevent_eviction() {
        let mut vmm = SimpleVMM::new();
        let page1 = VirtualAddress(0x1000);
        let page2 = VirtualAddress(0x2000);

        // page1 is wired (pinned), page2 is standard unpinned
        vmm.map_page_with_full_flags(page1, PhysicalAddress(0x10000), true, true, true).unwrap();
        vmm.map_page_with_full_flags(page2, PhysicalAddress(0x20000), true, true, false).unwrap();

        // Attempt page reclamation step
        let evicted = vmm.perform_clock_replacement_step().unwrap();
        // Since page1 is wired, the reclaimer is forced to bypass it and evict page2 instead
        assert_eq!(evicted, page2);

        // Try reclaiming again - since only page1 (wired) is left, it returns None
        assert!(vmm.perform_clock_replacement_step().is_none());
    }

    #[test]
    fn test_openbsd_wx_violation() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x1000);
        let phys = PhysicalAddress(0x2000);

        // Mapping a page that is BOTH writable and executable (execute_disable = false) must trigger W^X violation
        assert_eq!(
            vmm.map_page_with_flags(virt, phys, true, false), // writable: true, execute_disable: false
            Err(MemoryError::WxViolation)
        );
    }

    #[test]
    fn test_linux_page_reclaimer_kswapd() {
        let mut vmm = SimpleVMM::new();
        let reclaimer = SovereignPageReclaimer::new(5, 10);

        // Add 8 pages
        for i in 1..=8 {
            let virt = VirtualAddress(i * 0x1000);
            vmm.map_page_with_flags(virt, PhysicalAddress(i * 0x10000), true, true).unwrap();
        }

        assert_eq!(vmm.active_pages_for_clock.len(), 8);

        // Current free pages is 4, which is below low_watermark (5)
        // It must reclaim until free pages reaches high_watermark (10).
        // Since we need to go from 4 to 10 free pages, it attempts to reclaim 6 pages.
        let reclaimed = reclaimer.reclaim_pages(&mut vmm, 4);
        assert_eq!(reclaimed, 6);
        assert_eq!(vmm.active_pages_for_clock.len(), 2); // 8 - 6 = 2 remaining active pages
    }
}
