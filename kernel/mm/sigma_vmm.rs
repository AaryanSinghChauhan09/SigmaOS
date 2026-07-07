//! SigmaOS — x86-64 Page Table Walker / Virtual Memory Manager
//! 4-level paging: PML4 → PDPT → PD → PT
//! No std, no allocator — static page table structures.

#![no_std]
#![allow(dead_code)]

type U64 = u64;
type Usize = usize;

// ── Page Table Constants ────────────────────────────────────────────────────
const PAGE_SIZE:      Usize = 4096;
const ENTRIES_PER_TABLE: usize = 512;

// Page table entry flags (x86-64)
const PTE_PRESENT:    U64 = 1 << 0;
const PTE_WRITABLE:   U64 = 1 << 1;
const PTE_USER:       U64 = 1 << 2;
const PTE_PWT:        U64 = 1 << 3;  // Page Write-Through
const PTE_PCD:        U64 = 1 << 4;  // Page Cache Disable
const PTE_ACCESSED:   U64 = 1 << 5;
const PTE_DIRTY:      U64 = 1 << 6;
const PTE_HUGE:       U64 = 1 << 7;  // 2MB or 1GB page
const PTE_GLOBAL:     U64 = 1 << 8;
const PTE_NX:         U64 = 1 << 63; // No Execute

const ADDR_MASK: U64 = 0x000F_FFFF_FFFF_F000; // Physical address bits 12-51

// Virtual address decomposition
const PML4_SHIFT: u32 = 39;
const PDPT_SHIFT: u32 = 30;
const PD_SHIFT:   u32 = 21;
const PT_SHIFT:   u32 = 12;
const INDEX_MASK: U64 = 0x1FF; // 9 bits = 512 entries

// ── Page Table Entry ────────────────────────────────────────────────────────
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PageTableEntry(U64);

impl PageTableEntry {
    pub const fn empty() -> Self { PageTableEntry(0) }

    pub fn is_present(&self) -> bool { self.0 & PTE_PRESENT != 0 }
    pub fn is_writable(&self) -> bool { self.0 & PTE_WRITABLE != 0 }
    pub fn is_user(&self) -> bool { self.0 & PTE_USER != 0 }
    pub fn is_huge(&self) -> bool { self.0 & PTE_HUGE != 0 }
    pub fn is_nx(&self) -> bool { self.0 & PTE_NX != 0 }
    pub fn is_accessed(&self) -> bool { self.0 & PTE_ACCESSED != 0 }
    pub fn is_dirty(&self) -> bool { self.0 & PTE_DIRTY != 0 }

    pub fn address(&self) -> U64 { self.0 & ADDR_MASK }

    pub fn set(&mut self, phys_addr: U64, flags: U64) {
        self.0 = (phys_addr & ADDR_MASK) | flags;
    }

    pub fn clear(&mut self) { self.0 = 0; }

    pub fn add_flags(&mut self, flags: U64) { self.0 |= flags; }
    pub fn remove_flags(&mut self, flags: U64) { self.0 &= !flags; }
}

// ── Page Table (one level) ──────────────────────────────────────────────────
#[repr(C, align(4096))]
#[derive(Copy, Clone)]
pub struct PageTable {
    pub entries: [PageTableEntry; ENTRIES_PER_TABLE],
}

impl PageTable {
    pub const fn new() -> Self {
        PageTable {
            entries: [PageTableEntry::empty(); ENTRIES_PER_TABLE],
        }
    }
}

// ── Static Page Tables (for early boot) ─────────────────────────────────────
const MAX_PML4: usize = 1;
const MAX_PDPT: usize = 4;
const MAX_PD:   usize = 16;
const MAX_PT:   usize = 512;

static mut PML4:  [PageTable; MAX_PML4] = [PageTable::new(); MAX_PML4];
static mut PDPTS: [PageTable; MAX_PDPT] = [PageTable::new(); MAX_PDPT];
static mut PDS:   [PageTable; MAX_PD]   = [PageTable::new(); MAX_PD];
static mut PTS:   [PageTable; MAX_PT]   = [PageTable::new(); MAX_PT];
static mut PT_NEXT: usize = 0;
static mut PD_NEXT: usize = 0;
static mut PDPT_NEXT: usize = 0;

// ── VMM State ───────────────────────────────────────────────────────────────
pub struct VmmState {
    pub pml4_phys: U64,
    pub mapped_pages: U64,
    pub page_faults: U64,
}

static mut VMM: VmmState = VmmState {
    pml4_phys: 0,
    mapped_pages: 0,
    page_faults: 0,
};

// ── Internal Helpers ────────────────────────────────────────────────────────
fn virt_pml4_idx(vaddr: U64) -> usize { ((vaddr >> PML4_SHIFT) & INDEX_MASK) as usize }
fn virt_pdpt_idx(vaddr: U64) -> usize { ((vaddr >> PDPT_SHIFT) & INDEX_MASK) as usize }
fn virt_pd_idx(vaddr: U64)   -> usize { ((vaddr >> PD_SHIFT) & INDEX_MASK) as usize }
fn virt_pt_idx(vaddr: U64)   -> usize { ((vaddr >> PT_SHIFT) & INDEX_MASK) as usize }
fn page_offset(vaddr: U64)   -> usize { (vaddr & 0xFFF) as usize }

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize the VMM with identity mapping for the first `num_pages` pages.
#[no_mangle]
pub unsafe extern "C" fn sigma_vmm_init(num_pages: u32) -> i32 {
    PT_NEXT = 0;
    PD_NEXT = 0;
    PDPT_NEXT = 0;
    VMM.mapped_pages = 0;

    // Store PML4 physical address (for cr3)
    VMM.pml4_phys = PML4.as_ptr() as U64;

    // Identity map: virtual address == physical address
    let mut page: u32 = 0;
    while page < num_pages {
        let vaddr = (page as U64) * PAGE_SIZE as U64;
        let phys  = vaddr;
        sigma_vmm_map(vaddr, phys, PTE_PRESENT | PTE_WRITABLE);
        page += 1;
    }

    0
}

/// Map a single 4KB page: vaddr → phys_addr with given flags.
#[no_mangle]
pub unsafe extern "C" fn sigma_vmm_map(vaddr: U64, phys_addr: U64, flags: U64) -> i32 {
    let pml4_idx = virt_pml4_idx(vaddr);
    let pdpt_idx = virt_pdpt_idx(vaddr);
    let pd_idx   = virt_pd_idx(vaddr);
    let pt_idx   = virt_pt_idx(vaddr);

    // Ensure PML4 entry points to a PDPT
    if !PML4[0].entries[pml4_idx].is_present() {
        if PDPT_NEXT >= MAX_PDPT { return -1; }
        let pdpt_phys = PDPTS[PDPT_NEXT..].as_ptr() as U64;
        PML4[0].entries[pml4_idx].set(pdpt_phys, PTE_PRESENT | PTE_WRITABLE | PTE_USER);
        PDPT_NEXT += 1;
    }

    // Get the PDPT
    let pdpt_base = PML4[0].entries[pml4_idx].address();
    let pdpt = &mut *(pdpt_base as *mut PageTable);

    // Ensure PDPT entry points to a PD
    if !pdpt.entries[pdpt_idx].is_present() {
        if PD_NEXT >= MAX_PD { return -2; }
        let pd_phys = PDS[PD_NEXT..].as_ptr() as U64;
        pdpt.entries[pdpt_idx].set(pd_phys, PTE_PRESENT | PTE_WRITABLE | PTE_USER);
        PD_NEXT += 1;
    }

    // Get the PD
    let pd_base = pdpt.entries[pdpt_idx].address();
    let pd = &mut *(pd_base as *mut PageTable);

    // Ensure PD entry points to a PT
    if !pd.entries[pd_idx].is_present() {
        if PT_NEXT >= MAX_PT { return -3; }
        let pt_phys = PTS[PT_NEXT..].as_ptr() as U64;
        pd.entries[pd_idx].set(pt_phys, PTE_PRESENT | PTE_WRITABLE | PTE_USER);
        PT_NEXT += 1;
    }

    // Get the PT
    let pt_base = pd.entries[pd_idx].address();
    let pt = &mut *(pt_base as *mut PageTable);

    // Set the page table entry
    pt.entries[pt_idx].set(phys_addr, flags | PTE_PRESENT);
    VMM.mapped_pages += 1;

    // Invalidate TLB for this page
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("invlpg [{}]", in(reg) vaddr, options(nostack, preserves_flags));

    0
}

/// Unmap a single 4KB page.
#[no_mangle]
pub unsafe extern "C" fn sigma_vmm_unmap(vaddr: U64) -> i32 {
    let pml4_idx = virt_pml4_idx(vaddr);
    let pdpt_idx = virt_pdpt_idx(vaddr);
    let pd_idx   = virt_pd_idx(vaddr);
    let pt_idx   = virt_pt_idx(vaddr);

    if !PML4[0].entries[pml4_idx].is_present() { return -1; }
    let pdpt = &mut *(PML4[0].entries[pml4_idx].address() as *mut PageTable);

    if !pdpt.entries[pdpt_idx].is_present() { return -1; }
    let pd = &mut *(pdpt.entries[pdpt_idx].address() as *mut PageTable);

    if !pd.entries[pd_idx].is_present() { return -1; }
    let pt = &mut *(pd.entries[pd_idx].address() as *mut PageTable);

    if !pt.entries[pt_idx].is_present() { return -1; }

    pt.entries[pt_idx].clear();
    VMM.mapped_pages -= 1;

    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("invlpg [{}]", in(reg) vaddr, options(nostack, preserves_flags));

    0
}

/// Translate a virtual address to a physical address.
/// Returns the physical address or U64::MAX on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_vmm_translate(vaddr: U64) -> U64 {
    let pml4_idx = virt_pml4_idx(vaddr);
    if !PML4[0].entries[pml4_idx].is_present() { return U64::MAX; }
    let pdpt = &*(PML4[0].entries[pml4_idx].address() as *const PageTable);

    let pdpt_idx = virt_pdpt_idx(vaddr);
    if !pdpt.entries[pdpt_idx].is_present() { return U64::MAX; }
    // Check for 1GB huge page
    if pdpt.entries[pdpt_idx].is_huge() {
        return pdpt.entries[pdpt_idx].address() | (vaddr & 0x3FFF_FFFF);
    }
    let pd = &*(pdpt.entries[pdpt_idx].address() as *const PageTable);

    let pd_idx = virt_pd_idx(vaddr);
    if !pd.entries[pd_idx].is_present() { return U64::MAX; }
    // Check for 2MB huge page
    if pd.entries[pd_idx].is_huge() {
        return pd.entries[pd_idx].address() | (vaddr & 0x1F_FFFF);
    }
    let pt = &*(pd.entries[pd_idx].address() as *const PageTable);

    let pt_idx = virt_pt_idx(vaddr);
    if !pt.entries[pt_idx].is_present() { return U64::MAX; }

    pt.entries[pt_idx].address() | (vaddr & 0xFFF)
}

/// Get the PML4 physical address (for loading into CR3).
#[no_mangle]
pub unsafe extern "C" fn sigma_vmm_get_cr3() -> U64 {
    VMM.pml4_phys
}

/// Get the total number of mapped pages.
#[no_mangle]
pub unsafe extern "C" fn sigma_vmm_mapped_pages() -> U64 {
    VMM.mapped_pages
}

/// Flush the entire TLB by reloading CR3.
#[no_mangle]
pub unsafe extern "C" fn sigma_vmm_flush_tlb() {
    #[cfg(target_arch = "x86_64")]
    {
        let cr3 = VMM.pml4_phys;
        core::arch::asm!(
            "mov cr3, {}",
            in(reg) cr3,
            options(nostack, preserves_flags)
        );
    }
}
