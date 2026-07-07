/// SigmaOS: x86-64 Page Table Walker
/// Phase G Blocker #4: x86-64 page table walker
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── x86-64 Paging Constants ───────────────────────────────────────────────

pub const PAGE_SIZE: SigmaU64 = 4096;
pub const PAGE_SHIFT: SigmaU8 = 12;
pub const PT_ENTRY_COUNT: usize = 512;

// Page table entry flags
pub const PT_PRESENT: SigmaU64 = 1 << 0;
pub const PT_WRITE: SigmaU64 = 1 << 1;
pub const PT_USER: SigmaU64 = 1 << 2;
pub const PT_PWT: SigmaU64 = 1 << 3;
pub const PT_PCD: SigmaU64 = 1 << 4;
pub const PT_ACCESSED: SigmaU64 = 1 << 5;
pub const PT_DIRTY: SigmaU64 = 1 << 6;
pub const PT_PS: SigmaU64 = 1 << 7;
pub const PT_GLOBAL: SigmaU64 = 1 << 8;
pub const PT_NX: SigmaU64 = 1u64 << 63;

// ─── Page Table Entry ─────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PageTableEntry {
    pub raw: SigmaU64,
}

impl PageTableEntry {
    pub const fn new() -> Self {
        Self { raw: 0 }
    }

    pub fn is_present(&self) -> SigmaBool {
        (self.raw & PT_PRESENT) != 0
    }

    pub fn is_writable(&self) -> SigmaBool {
        (self.raw & PT_WRITE) != 0
    }

    pub fn is_user(&self) -> SigmaBool {
        (self.raw & PT_USER) != 0
    }

    pub fn get_address(&self) -> SigmaU64 {
        self.raw & 0x000FFFFFFFFFF000
    }

    pub fn set_address(&mut self, addr: SigmaU64) {
        self.raw = (self.raw & 0xFFF0000000000FFF) | (addr & 0x000FFFFFFFFFF000);
    }

    pub fn set_flags(&mut self, flags: SigmaU64) {
        self.raw |= flags;
    }

    pub fn clear_flags(&mut self, flags: SigmaU64) {
        self.raw &= !flags;
    }
}

// ─── Page Table ───────────────────────────────────────────────────────────

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; PT_ENTRY_COUNT],
}

impl PageTable {
    pub const fn new() -> Self {
        Self {
            entries: [PageTableEntry::new(); PT_ENTRY_COUNT],
        }
    }

    pub fn get_entry(&mut self, index: usize) -> Option<&mut PageTableEntry> {
        if index < PT_ENTRY_COUNT {
            Some(&mut self.entries[index])
        } else {
            None
        }
    }
}

// ─── Virtual Address Structure ─────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtualAddress {
    pub raw: SigmaU64,
}

impl VirtualAddress {
    pub const fn new(addr: SigmaU64) -> Self {
        Self { raw: addr }
    }

    pub fn pml4_index(&self) -> usize {
        ((self.raw >> 39) & 0x1FF) as usize
    }

    pub fn pdpt_index(&self) -> usize {
        ((self.raw >> 30) & 0x1FF) as usize
    }

    pub fn pd_index(&self) -> usize {
        ((self.raw >> 21) & 0x1FF) as usize
    }

    pub fn pt_index(&self) -> usize {
        ((self.raw >> 12) & 0x1FF) as usize
    }

    pub fn page_offset(&self) -> SigmaU64 {
        self.raw & 0xFFF
    }
}

// ─── Page Table Walker ───────────────────────────────────────────────────

pub struct PageTableWalker {
    pml4: SigmaU64,
    enabled: SigmaBool,
}

impl PageTableWalker {
    pub const fn new() -> Self {
        Self {
            pml4: 0,
            enabled: false,
        }
    }

    /// Initialize page table walker with PML4 address
    pub unsafe fn init(&mut self, pml4_addr: SigmaU64) {
        self.pml4 = pml4_addr;
        self.enabled = true;
    }

    /// Walk page tables and get physical address
    pub unsafe fn walk(&self, virt_addr: SigmaU64) -> Result<SigmaU64, &'static str> {
        if !self.enabled {
            return Err("Page table walker not enabled");
        }

        let va = VirtualAddress::new(virt_addr);

        // Get PML4
        let pml4 = &*(self.pml4 as *const PageTable);
        let pml4_entry = pml4.entries.get(va.pml4_index())
            .ok_or("Invalid PML4 index")?;

        if !pml4_entry.is_present() {
            return Err("PML4 entry not present");
        }

        // Get PDPT
        let pdpt_addr = pml4_entry.get_address();
        let pdpt = &*(pdpt_addr as *const PageTable);
        let pdpt_entry = pdpt.entries.get(va.pdpt_index())
            .ok_or("Invalid PDPT index")?;

        if !pdpt_entry.is_present() {
            return Err("PDPT entry not present");
        }

        // Check for 1GB page
        if pdpt_entry.raw & PT_PS != 0 {
            let page_addr = (pdpt_entry.get_address() & !0x3FFFFFFF) | (va.raw & 0x3FFFFFFF);
            return Ok(page_addr);
        }

        // Get PD
        let pd_addr = pdpt_entry.get_address();
        let pd = &*(pd_addr as *const PageTable);
        let pd_entry = pd.entries.get(va.pd_index())
            .ok_or("Invalid PD index")?;

        if !pd_entry.is_present() {
            return Err("PD entry not present");
        }

        // Check for 2MB page
        if pd_entry.raw & PT_PS != 0 {
            let page_addr = (pd_entry.get_address() & !0x1FFFFF) | (va.raw & 0x1FFFFF);
            return Ok(page_addr);
        }

        // Get PT
        let pt_addr = pd_entry.get_address();
        let pt = &*(pt_addr as *const PageTable);
        let pt_entry = pt.entries.get(va.pt_index())
            .ok_or("Invalid PT index")?;

        if !pt_entry.is_present() {
            return Err("PT entry not present");
        }

        // Get physical address
        let phys_addr = pt_entry.get_address() | va.page_offset();
        Ok(phys_addr)
    }

    /// Map virtual address to physical address
    pub unsafe fn map(&mut self, virt_addr: SigmaU64, phys_addr: SigmaU64, flags: SigmaU64) -> Result<(), &'static str> {
        if !self.enabled {
            return Err("Page table walker not enabled");
        }

        let va = VirtualAddress::new(virt_addr);

        // Get PML4
        let pml4 = &mut *(self.pml4 as *mut PageTable);
        let pml4_entry = pml4.entries.get_mut(va.pml4_index())
            .ok_or("Invalid PML4 index")?;

        // Allocate PDPT if not present
        if !pml4_entry.is_present() {
            let pdpt_addr = self.allocate_page_table()?;
            pml4_entry.set_address(pdpt_addr);
            pml4_entry.set_flags(PT_PRESENT | PT_WRITE | PT_USER);
        }

        // Get PDPT
        let pdpt_addr = pml4_entry.get_address();
        let pdpt = &mut *(pdpt_addr as *mut PageTable);
        let pdpt_entry = pdpt.entries.get_mut(va.pdpt_index())
            .ok_or("Invalid PDPT index")?;

        // Allocate PD if not present
        if !pdpt_entry.is_present() {
            let pd_addr = self.allocate_page_table()?;
            pdpt_entry.set_address(pd_addr);
            pdpt_entry.set_flags(PT_PRESENT | PT_WRITE | PT_USER);
        }

        // Get PD
        let pd_addr = pdpt_entry.get_address();
        let pd = &mut *(pd_addr as *mut PageTable);
        let pd_entry = pd.entries.get_mut(va.pd_index())
            .ok_or("Invalid PD index")?;

        // Allocate PT if not present
        if !pd_entry.is_present() {
            let pt_addr = self.allocate_page_table()?;
            pd_entry.set_address(pt_addr);
            pd_entry.set_flags(PT_PRESENT | PT_WRITE | PT_USER);
        }

        // Get PT
        let pt_addr = pd_entry.get_address();
        let pt = &mut *(pt_addr as *mut PageTable);
        let pt_entry = pt.entries.get_mut(va.pt_index())
            .ok_or("Invalid PT index")?;

        // Map page
        pt_entry.set_address(phys_addr);
        pt_entry.set_flags(flags);

        Ok(())
    }

    /// Unmap virtual address
    pub unsafe fn unmap(&mut self, virt_addr: SigmaU64) -> Result<(), &'static str> {
        if !self.enabled {
            return Err("Page table walker not enabled");
        }

        let va = VirtualAddress::new(virt_addr);

        // Get PML4
        let pml4 = &mut *(self.pml4 as *mut PageTable);
        let pml4_entry = pml4.entries.get_mut(va.pml4_index())
            .ok_or("Invalid PML4 index")?;

        if !pml4_entry.is_present() {
            return Err("PML4 entry not present");
        }

        // Get PDPT
        let pdpt_addr = pml4_entry.get_address();
        let pdpt = &mut *(pdpt_addr as *mut PageTable);
        let pdpt_entry = pdpt.entries.get_mut(va.pdpt_index())
            .ok_or("Invalid PDPT index")?;

        if !pdpt_entry.is_present() {
            return Err("PDPT entry not present");
        }

        // Get PD
        let pd_addr = pdpt_entry.get_address();
        let pd = &mut *(pd_addr as *mut PageTable);
        let pd_entry = pd.entries.get_mut(va.pd_index())
            .ok_or("Invalid PD index")?;

        if !pd_entry.is_present() {
            return Err("PD entry not present");
        }

        // Get PT
        let pt_addr = pd_entry.get_address();
        let pt = &mut *(pt_addr as *mut PageTable);
        let pt_entry = pt.entries.get_mut(va.pt_index())
            .ok_or("Invalid PT index")?;

        if !pt_entry.is_present() {
            return Err("PT entry not present");
        }

        // Clear entry
        pt_entry.raw = 0;

        Ok(())
    }

    /// Allocate a new page table (BUG-001 Fix)
    unsafe fn allocate_page_table(&self) -> Result<SigmaU64, &'static str> {
        // Call buddy allocator to allocate a page (order 0 = 1 page)
        extern "C" {
            fn sigma_buddy_alloc(order: SigmaU8) -> SigmaU64;
        }
        
        let page_addr = sigma_buddy_alloc(0);
        if page_addr == 0 {
            return Err("Failed to allocate page table");
        }
        
        // Clear the page
        let page_ptr = page_addr as *mut u8;
        for i in 0..PAGE_SIZE {
            *page_ptr.add(i as usize) = 0;
        }
        
        Ok(page_addr)
    }

    /// Get PML4 address
    pub unsafe fn get_pml4(&self) -> SigmaU64 {
        self.pml4
    }

    /// Enable page table walker
    pub unsafe fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable page table walker
    pub unsafe fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if enabled
    pub unsafe fn is_enabled(&self) -> SigmaBool {
        self.enabled
    }
}

// ─── Global Page Table Walker Instance ─────────────────────────────────────

static mut PAGE_TABLE_WALKER: PageTableWalker = PageTableWalker::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_pt_walk(virt_addr: SigmaU64) -> SigmaU64 {
    match PAGE_TABLE_WALKER.walk(virt_addr) {
        Ok(phys_addr) => phys_addr,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pt_map(virt_addr: SigmaU64, phys_addr: SigmaU64, flags: SigmaU64) -> SigmaI32 {
    match PAGE_TABLE_WALKER.map(virt_addr, phys_addr, flags) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pt_unmap(virt_addr: SigmaU64) -> SigmaI32 {
    match PAGE_TABLE_WALKER.unmap(virt_addr) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pt_init(pml4_addr: SigmaU64) {
    PAGE_TABLE_WALKER.init(pml4_addr);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pt_get_pml4() -> SigmaU64 {
    PAGE_TABLE_WALKER.get_pml4()
}
