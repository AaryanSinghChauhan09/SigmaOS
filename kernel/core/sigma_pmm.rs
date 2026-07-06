// SigmaOS — Physical Memory Manager (Issue #1004)
// Buddy allocator for physical pages + virtual address space management.
// No external dependencies — sovereign implementation.
#![no_std]
#![allow(dead_code)]

// ─── Constants ───────────────────────────────────────────────────────────────
pub const PAGE_SIZE:    usize = 4096;
pub const PAGE_SHIFT:   usize = 12;
pub const MAX_ORDER:    usize = 11;    // 2^11 pages = 8 MB max block
pub const MAX_PAGES:    usize = 1 << 20; // up to 4 GB with 4K pages

/// Physical address type
pub type PhysAddr = u64;
pub type VirtAddr = u64;

// ─── Page Frame Number Arithmetic ────────────────────────────────────────────

#[inline(always)] pub fn pfn(pa: PhysAddr)  -> usize { (pa >> PAGE_SHIFT) as usize }
#[inline(always)] pub fn pa(pfn: usize)    -> PhysAddr { (pfn as u64) << PAGE_SHIFT }
#[inline(always)] pub fn buddy_pfn(pfn: usize, order: usize) -> usize { pfn ^ (1 << order) }

// ─── Free List ───────────────────────────────────────────────────────────────

/// Intrusive free list entry stored inside the free page itself.
/// Each entry holds next/prev PFN (0xFFFF_FFFF = null sentinel).
const NULL_PFN: usize = usize::MAX;

#[derive(Clone, Copy)]
pub struct FreeBlock {
    pub next: usize,
    pub prev: usize,
}

pub struct BuddyAllocator {
    /// free_list[order] = head PFN of free blocks at that order
    pub free_list: [usize; MAX_ORDER + 1],
    /// page metadata: is_free, order
    pub page_free:  [bool;  MAX_PAGES],
    pub page_order: [u8;    MAX_PAGES],
    /// Free blocks stored inline in a separate metadata array
    pub blocks:     [FreeBlock; MAX_PAGES],
    pub total_pages: usize,
    pub free_pages:  usize,
}

impl BuddyAllocator {
    pub const fn new() -> Self {
        BuddyAllocator {
            free_list:   [NULL_PFN; MAX_ORDER + 1],
            page_free:   [false;    MAX_PAGES],
            page_order:  [0u8;      MAX_PAGES],
            blocks:      [FreeBlock { next: NULL_PFN, prev: NULL_PFN }; MAX_PAGES],
            total_pages:  0,
            free_pages:   0,
        }
    }

    /// Register a physical memory region with the allocator.
    pub fn add_region(&mut self, base: PhysAddr, size: usize) {
        let start = pfn(base);
        let npages = size / PAGE_SIZE;
        self.total_pages += npages;
        // Align and insert aligned blocks
        let mut i = start;
        let end = start + npages;
        while i < end && i < MAX_PAGES {
            // Find largest aligned order that fits
            let mut order = MAX_ORDER;
            while order > 0 {
                let block_size = 1 << order;
                if (i & (block_size - 1)) == 0 && i + block_size <= end { break; }
                order -= 1;
            }
            self.free_block_insert(i, order);
            i += 1 << order;
        }
    }

    fn free_block_insert(&mut self, pfn: usize, order: usize) {
        if pfn >= MAX_PAGES { return; }
        self.page_free[pfn]  = true;
        self.page_order[pfn] = order as u8;
        self.blocks[pfn].next = self.free_list[order];
        self.blocks[pfn].prev = NULL_PFN;
        if self.free_list[order] != NULL_PFN {
            self.blocks[self.free_list[order]].prev = pfn;
        }
        self.free_list[order] = pfn;
        self.free_pages += 1 << order;
    }

    fn free_block_remove(&mut self, pfn: usize, order: usize) {
        let next = self.blocks[pfn].next;
        let prev = self.blocks[pfn].prev;
        if prev != NULL_PFN { self.blocks[prev].next = next; }
        else { self.free_list[order] = next; }
        if next != NULL_PFN { self.blocks[next].prev = prev; }
        self.blocks[pfn].next = NULL_PFN;
        self.blocks[pfn].prev = NULL_PFN;
        self.page_free[pfn]   = false;
        self.free_pages -= 1 << order;
    }

    /// Allocate 2^order contiguous pages. Returns physical address or None.
    pub fn alloc(&mut self, order: usize) -> Option<PhysAddr> {
        if order > MAX_ORDER { return None; }
        // Find smallest free order >= requested
        for o in order..=MAX_ORDER {
            if self.free_list[o] != NULL_PFN {
                let block_pfn = self.free_list[o];
                self.free_block_remove(block_pfn, o);
                // Split down to requested order
                let mut cur_pfn   = block_pfn;
                let mut cur_order = o;
                while cur_order > order {
                    cur_order -= 1;
                    let buddy = cur_pfn + (1 << cur_order);
                    self.free_block_insert(buddy, cur_order);
                }
                self.page_order[cur_pfn] = order as u8;
                return Some(pa(cur_pfn));
            }
        }
        None
    }

    /// Free a previously allocated block.
    pub fn free(&mut self, addr: PhysAddr) {
        let mut pfn = pfn(addr);
        if pfn >= MAX_PAGES { return; }
        let mut order = self.page_order[pfn] as usize;
        // Coalesce with buddy
        loop {
            if order >= MAX_ORDER { break; }
            let buddy = buddy_pfn(pfn, order);
            if buddy >= MAX_PAGES { break; }
            if !self.page_free[buddy] { break; }
            if self.page_order[buddy] as usize != order { break; }
            self.free_block_remove(buddy, order);
            pfn = pfn.min(buddy); // merged block starts at lower pfn
            order += 1;
        }
        self.free_block_insert(pfn, order);
        self.page_order[pfn] = order as u8;
    }

    pub fn free_mb(&self) -> usize { self.free_pages * PAGE_SIZE / (1024 * 1024) }
    pub fn total_mb(&self) -> usize { self.total_pages * PAGE_SIZE / (1024 * 1024) }
}

// ─── Kernel Page Table (x86-64, 4-level) ─────────────────────────────────────

pub const PTE_PRESENT:  u64 = 1 << 0;
pub const PTE_WRITE:    u64 = 1 << 1;
pub const PTE_USER:     u64 = 1 << 2;
pub const PTE_NX:       u64 = 1 << 63;
pub const PTE_ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

/// Walk / allocate a 4-level page table for a virtual address.
/// `root` = physical address of PML4.
pub unsafe fn map_page(
    pmm: &mut BuddyAllocator,
    root: PhysAddr,
    va: VirtAddr,
    pa_target: PhysAddr,
    flags: u64,
) -> bool {
    let pml4_idx = ((va >> 39) & 0x1FF) as usize;
    let pdpt_idx = ((va >> 30) & 0x1FF) as usize;
    let pd_idx   = ((va >> 21) & 0x1FF) as usize;
    let pt_idx   = ((va >> 12) & 0x1FF) as usize;

    let pml4 = root as *mut u64;

    // PML4 → PDPT
    let pdpt_pa = alloc_or_get_table(pmm, pml4.add(pml4_idx));
    if pdpt_pa == 0 { return false; }

    // PDPT → PD
    let pdpt = pdpt_pa as *mut u64;
    let pd_pa = alloc_or_get_table(pmm, pdpt.add(pdpt_idx));
    if pd_pa == 0 { return false; }

    // PD → PT
    let pd = pd_pa as *mut u64;
    let pt_pa = alloc_or_get_table(pmm, pd.add(pd_idx));
    if pt_pa == 0 { return false; }

    // PT → Page
    let pt = pt_pa as *mut u64;
    pt.add(pt_idx).write_volatile((pa_target & PTE_ADDR_MASK) | flags | PTE_PRESENT);
    true
}

unsafe fn alloc_or_get_table(pmm: &mut BuddyAllocator, entry: *mut u64) -> PhysAddr {
    let e = entry.read_volatile();
    if e & PTE_PRESENT != 0 {
        return e & PTE_ADDR_MASK;
    }
    // Allocate a new page for this table
    if let Some(new_pa) = pmm.alloc(0) {
        // Zero the new page
        let ptr = new_pa as *mut u8;
        for i in 0..PAGE_SIZE { ptr.add(i).write_volatile(0); }
        entry.write_volatile((new_pa & PTE_ADDR_MASK) | PTE_PRESENT | PTE_WRITE);
        new_pa
    } else {
        0
    }
}

// ─── Virtual Memory Area (VMA) ───────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct Vma {
    pub start: VirtAddr,
    pub end:   VirtAddr,
    pub flags: u32,
    pub name:  [u8; 16],
}

pub const VMA_READ:    u32 = 1 << 0;
pub const VMA_WRITE:   u32 = 1 << 1;
pub const VMA_EXEC:    u32 = 1 << 2;
pub const VMA_ANON:    u32 = 1 << 3;
pub const VMA_FILE:    u32 = 1 << 4;
pub const VMA_STACK:   u32 = 1 << 5;

pub const MAX_VMA: usize = 256;

pub struct AddrSpace {
    pub vmas:  [Vma; MAX_VMA],
    pub count: usize,
    pub pgd:   PhysAddr,  // PML4 physical address
}

impl AddrSpace {
    pub const fn new(pgd: PhysAddr) -> Self {
        const EMPTY: Vma = Vma { start: 0, end: 0, flags: 0, name: [0u8; 16] };
        AddrSpace { vmas: [EMPTY; MAX_VMA], count: 0, pgd }
    }

    pub fn insert_vma(&mut self, vma: Vma) -> bool {
        if self.count >= MAX_VMA { return false; }
        self.vmas[self.count] = vma;
        self.count += 1;
        true
    }

    pub fn find_vma(&self, addr: VirtAddr) -> Option<&Vma> {
        for v in &self.vmas[..self.count] {
            if addr >= v.start && addr < v.end { return Some(v); }
        }
        None
    }

    pub fn remove_vma(&mut self, start: VirtAddr) -> bool {
        for i in 0..self.count {
            if self.vmas[i].start == start {
                self.vmas[i] = self.vmas[self.count - 1];
                self.count -= 1;
                return true;
            }
        }
        false
    }
}

// ─── Global PMM ──────────────────────────────────────────────────────────────
static mut PMM: BuddyAllocator = BuddyAllocator::new();

pub fn sigma_pmm_init(base: PhysAddr, size: usize) {
    unsafe { PMM.add_region(base, size); }
}

pub fn sigma_pmm_alloc(order: usize) -> Option<PhysAddr> {
    unsafe { PMM.alloc(order) }
}

pub fn sigma_pmm_free(addr: PhysAddr) {
    unsafe { PMM.free(addr); }
}

pub fn sigma_pmm_stats() -> (usize, usize) {
    unsafe { (PMM.free_mb(), PMM.total_mb()) }
}
