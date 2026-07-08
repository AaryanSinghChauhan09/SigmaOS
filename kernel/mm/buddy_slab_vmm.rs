//! SigmaOS — Memory Manager: Buddy Allocator + Slab Cache
//! Pure no_std, zero-alloc bootstrapping implementation.
//! Inspired by Linux's buddy system and Solaris slab allocator.

#![no_std]
#![allow(dead_code)]

type U8    = u8;
type U16   = u16;
type U32   = u32;
type U64   = u64;
type Usize = usize;
type I32   = i32;
type Bool  = bool;

// ── Constants ────────────────────────────────────────────────────────────────
const PAGE_SIZE:      Usize = 4096;          // 4 KiB
const MAX_ORDER:      Usize = 11;            // 2^11 pages = 8 MiB max block
const MAX_PAGES:      Usize = 1024 * 1024;   // support up to 4 GiB
const BUDDY_MAP_BITS: Usize = MAX_PAGES / 8; // 128 KiB bitmap
const SLAB_CLASSES:   Usize = 12;            // 8, 16, 32, 64, 128, 256, 512, 1024,
                                              // 2048, 4096, 8192, 16384 bytes
const SLAB_OBJ_SIZES: [Usize; SLAB_CLASSES] = [
    8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384,
];

// ── Buddy Allocator ──────────────────────────────────────────────────────────

/// Bit-map tracking free blocks at each order.
/// Bit i set → page-frame i at this order is free.
static mut BUDDY_BITMAP: [[U8; BUDDY_MAP_BITS]; MAX_ORDER] =
    [[0u8; BUDDY_MAP_BITS]; MAX_ORDER];

static mut TOTAL_PAGES: Usize = 0;
static mut BASE_ADDR:   U64   = 0;   // physical base address

/// Set/clear bit `page` in order `ord` bitmap.
unsafe fn bitmap_set(ord: Usize, page: Usize) {
    BUDDY_BITMAP[ord][page / 8] |= 1 << (page % 8);
}
unsafe fn bitmap_clear(ord: Usize, page: Usize) {
    BUDDY_BITMAP[ord][page / 8] &= !(1 << (page % 8));
}
unsafe fn bitmap_test(ord: Usize, page: Usize) -> Bool {
    (BUDDY_BITMAP[ord][page / 8] >> (page % 8)) & 1 != 0
}

/// Initialise buddy allocator with `total_pages` starting at `base`.
#[no_mangle]
pub unsafe extern "C" fn mm_buddy_init(base: U64, total: Usize) {
    BASE_ADDR = base;
    TOTAL_PAGES = total.min(MAX_PAGES);
    // Clear all bitmaps
    for ord in 0..MAX_ORDER {
        for b in 0..BUDDY_MAP_BITS { BUDDY_BITMAP[ord][b] = 0; }
    }
    // Mark all pages as free at order 0, then merge
    let mut page = 0;
    while page < TOTAL_PAGES {
        mm_free_pages(base + (page * PAGE_SIZE) as U64, 0);
        page += 1;
    }
}

// BUG-001 Fix: Add VMM integration helper functions

/// Allocate pages for VMM with automatic buddy allocator integration
#[no_mangle]
pub unsafe extern "C" fn vmm_alloc_pages_for_vmm(order: Usize) -> U64 {
    mm_alloc_pages(order)
}

/// Free pages from VMM with automatic buddy allocator integration
#[no_mangle]
pub unsafe extern "C" fn vmm_free_pages_for_vmm(addr: U64, order: Usize) {
    mm_free_pages(addr, order)
}

/// Get total free pages for VMM statistics
#[no_mangle]
pub unsafe extern "C" fn vmm_get_free_pages() -> Usize {
    mm_free_page_count()
}

/// Check if buddy allocator is initialized (BUG-001 Fix)
#[no_mangle]
pub unsafe extern "C" fn mm_buddy_is_initialized() -> Bool {
    TOTAL_PAGES > 0
}

/// Free `2^order` pages starting at physical address `addr`.
#[no_mangle]
pub unsafe extern "C" fn mm_free_pages(addr: U64, order: Usize) {
    if order >= MAX_ORDER { return; }
    let page = ((addr - BASE_ADDR) as Usize) / PAGE_SIZE;
    let mut p = page;
    let mut ord = order;
    loop {
        // Toggle buddy bit
        bitmap_set(ord, p);
        if ord + 1 >= MAX_ORDER { break; }
        let buddy = p ^ (1 << ord);
        if !bitmap_test(ord, buddy) { break; }
        // Merge: clear both, promote
        bitmap_clear(ord, p);
        bitmap_clear(ord, buddy);
        p = p.min(buddy);
        ord += 1;
    }
}

/// Allocate `2^order` contiguous pages. Returns physical address or 0 on OOM.
#[no_mangle]
pub unsafe extern "C" fn mm_alloc_pages(order: Usize) -> U64 {
    if order >= MAX_ORDER { return 0; }
    // Find a free block at `order` or higher
    for ord in order..MAX_ORDER {
        // Scan bitmap for a free bit
        let block_pages = 1usize << ord;
        let pages_at_ord = TOTAL_PAGES / block_pages;
        for p in 0..pages_at_ord {
            if bitmap_test(ord, p * block_pages) {
                bitmap_clear(ord, p * block_pages);
                // Split down to requested order
                let mut cur_ord = ord;
                let mut cur_page = p * block_pages;
                while cur_ord > order {
                    cur_ord -= 1;
                    let buddy = cur_page + (1 << cur_ord);
                    bitmap_set(cur_ord, buddy);
                }
                return BASE_ADDR + (cur_page * PAGE_SIZE) as U64;
            }
        }
    }
    0 // OOM
}

/// Return the number of free pages.
#[no_mangle]
pub unsafe extern "C" fn mm_free_page_count() -> Usize {
    let mut count = 0;
    for ord in 0..MAX_ORDER {
        let block_pages = 1usize << ord;
        let pages_at_ord = TOTAL_PAGES / block_pages;
        for p in 0..pages_at_ord {
            if bitmap_test(ord, p * block_pages) {
                count += block_pages;
            }
        }
    }
    count
}

// ── Slab Allocator ────────────────────────────────────────────────────────────

/// One slab = one 4 KiB page of fixed-size objects with a free-list.
#[repr(C)]
struct Slab {
    next:       U32,    // index of next slab in list (u32::MAX = none)
    class:      U8,     // index into SLAB_OBJ_SIZES
    nr_free:    U16,
    nr_total:   U16,
    free_head:  U16,    // offset of first free object (in units of obj_size)
    _pad:       U8,
}

const MAX_SLABS: Usize = 4096;
static mut SLABS:      [Slab; MAX_SLABS] = unsafe { core::mem::zeroed() };
static mut SLAB_BASES: [U64; MAX_SLABS]  = [0u64; MAX_SLABS];
static mut SLAB_HEADS: [U32; SLAB_CLASSES] = [U32::MAX; SLAB_CLASSES];
static mut SLAB_COUNT: Usize = 0;

unsafe fn slab_obj_size(class: Usize) -> Usize { SLAB_OBJ_SIZES[class] }

unsafe fn slab_new(class: Usize) -> U32 {
    if SLAB_COUNT >= MAX_SLABS { return U32::MAX; }
    let phys = mm_alloc_pages(0);
    if phys == 0 { return U32::MAX; }
    let idx = SLAB_COUNT as U32;
    SLAB_COUNT += 1;
    let obj_size = slab_obj_size(class);
    let nr = PAGE_SIZE / obj_size;
    SLABS[idx as Usize] = Slab {
        next: SLAB_HEADS[class],
        class: class as U8,
        nr_free: nr as U16,
        nr_total: nr as U16,
        free_head: 0,
        _pad: 0,
    };
    SLAB_BASES[idx as Usize] = phys;
    // Initialise free list: each object contains the index of next free obj
    let base = phys as *mut U16;
    for i in 0..nr - 1 {
        *base.add(i * obj_size / 2) = (i + 1) as U16;
    }
    *base.add((nr - 1) * obj_size / 2) = U16::MAX;
    SLAB_HEADS[class] = idx;
    idx
}

/// Allocate an object of at least `size` bytes from the slab cache.
/// Returns physical address or 0 on failure.
#[no_mangle]
pub unsafe extern "C" fn slab_alloc(size: Usize) -> U64 {
    // Find appropriate class
    let mut class = SLAB_CLASSES;
    for c in 0..SLAB_CLASSES {
        if SLAB_OBJ_SIZES[c] >= size { class = c; break; }
    }
    if class == SLAB_CLASSES { return 0; }

    // Walk slab list for this class
    let mut slab_idx = SLAB_HEADS[class];
    if slab_idx == U32::MAX || SLABS[slab_idx as Usize].nr_free == 0 {
        slab_idx = slab_new(class);
        if slab_idx == U32::MAX { return 0; }
    }

    let slab = &mut SLABS[slab_idx as Usize];
    let obj_size = slab_obj_size(class);
    let offset = slab.free_head as Usize * obj_size;
    let base = SLAB_BASES[slab_idx as Usize] as *mut U16;
    // Read next free pointer
    let next_free = *base.add(offset / 2);
    slab.free_head = next_free;
    slab.nr_free -= 1;
    SLAB_BASES[slab_idx as Usize] + offset as U64
}

/// Free an object back to the slab cache.
#[no_mangle]
pub unsafe extern "C" fn slab_free(ptr: U64) {
    // Find which slab owns this address
    for i in 0..SLAB_COUNT {
        let base = SLAB_BASES[i];
        if ptr >= base && ptr < base + PAGE_SIZE as U64 {
            let class = SLABS[i].class as Usize;
            let obj_size = slab_obj_size(class);
            let offset = (ptr - base) as Usize / obj_size;
            let p = base as *mut U16;
            *p.add(offset * obj_size / 2) = SLABS[i].free_head;
            SLABS[i].free_head = offset as U16;
            SLABS[i].nr_free += 1;
            return;
        }
    }
}

// ── Virtual Memory Manager ────────────────────────────────────────────────────

/// Simple page-table entry (x86-64 4-level, 64-bit).
pub type Pte = U64;

/// Flags
pub const PTE_PRESENT:  U64 = 1 << 0;
pub const PTE_WRITABLE: U64 = 1 << 1;
pub const PTE_USER:     U64 = 1 << 2;
pub const PTE_NX:       U64 = 1 << 63;
pub const PTE_ADDR_MASK:U64 = 0x000F_FFFF_FFFF_F000;

/// Map a virtual page to a physical page in a PML4 page table.
/// `pml4_phys`: physical address of PML4.
/// `virt`:      virtual address to map.
/// `phys`:      physical address to map to.
/// `flags`:     PTE flags (PTE_PRESENT | PTE_WRITABLE | etc.)
#[no_mangle]
pub unsafe extern "C" fn vmm_map_page(
    pml4_phys: U64, virt: U64, phys: U64, flags: U64,
) -> I32 {
    let pml4_idx = ((virt >> 39) & 0x1FF) as Usize;
    let pdpt_idx = ((virt >> 30) & 0x1FF) as Usize;
    let pd_idx   = ((virt >> 21) & 0x1FF) as Usize;
    let pt_idx   = ((virt >> 12) & 0x1FF) as Usize;

    let pml4 = pml4_phys as *mut Pte;

    // Allocate/walk PDPT
    let mut pdpt_phys = (*pml4.add(pml4_idx)) & PTE_ADDR_MASK;
    if pdpt_phys == 0 {
        pdpt_phys = mm_alloc_pages(0);
        if pdpt_phys == 0 { return -1; }
        core::ptr::write_bytes(pdpt_phys as *mut U8, 0, PAGE_SIZE);
        *pml4.add(pml4_idx) = pdpt_phys | PTE_PRESENT | PTE_WRITABLE;
    }
    let pdpt = pdpt_phys as *mut Pte;

    // Allocate/walk PD
    let mut pd_phys = (*pdpt.add(pdpt_idx)) & PTE_ADDR_MASK;
    if pd_phys == 0 {
        pd_phys = mm_alloc_pages(0);
        if pd_phys == 0 { return -1; }
        core::ptr::write_bytes(pd_phys as *mut U8, 0, PAGE_SIZE);
        *pdpt.add(pdpt_idx) = pd_phys | PTE_PRESENT | PTE_WRITABLE;
    }
    let pd = pd_phys as *mut Pte;

    // Allocate/walk PT
    let mut pt_phys = (*pd.add(pd_idx)) & PTE_ADDR_MASK;
    if pt_phys == 0 {
        pt_phys = mm_alloc_pages(0);
        if pt_phys == 0 { return -1; }
        core::ptr::write_bytes(pt_phys as *mut U8, 0, PAGE_SIZE);
        *pd.add(pd_idx) = pt_phys | PTE_PRESENT | PTE_WRITABLE;
    }
    let pt = pt_phys as *mut Pte;
    *pt.add(pt_idx) = (phys & PTE_ADDR_MASK) | flags | PTE_PRESENT;
    0
}

/// Unmap a virtual page; returns physical address that was mapped (or 0).
#[no_mangle]
pub unsafe extern "C" fn vmm_unmap_page(pml4_phys: U64, virt: U64) -> U64 {
    let pml4 = pml4_phys as *mut Pte;
    let pml4_idx = ((virt >> 39) & 0x1FF) as Usize;
    let pdpt_phys = (*pml4.add(pml4_idx)) & PTE_ADDR_MASK;
    if pdpt_phys == 0 { return 0; }
    let pdpt = pdpt_phys as *mut Pte;
    let pdpt_idx = ((virt >> 30) & 0x1FF) as Usize;
    let pd_phys = (*pdpt.add(pdpt_idx)) & PTE_ADDR_MASK;
    if pd_phys == 0 { return 0; }
    let pd = pd_phys as *mut Pte;
    let pd_idx = ((virt >> 21) & 0x1FF) as Usize;
    let pt_phys = (*pd.add(pd_idx)) & PTE_ADDR_MASK;
    if pt_phys == 0 { return 0; }
    let pt = pt_phys as *mut Pte;
    let pt_idx = ((virt >> 12) & 0x1FF) as Usize;
    let old = *pt.add(pt_idx);
    *pt.add(pt_idx) = 0;
    old & PTE_ADDR_MASK
}

/// Translate virtual address to physical address.
/// Returns physical address or 0 if not mapped.
#[no_mangle]
pub unsafe extern "C" fn vmm_virt_to_phys(pml4_phys: U64, virt: U64) -> U64 {
    let pml4 = pml4_phys as *mut Pte;
    let pml4_idx = ((virt >> 39) & 0x1FF) as Usize;
    let pml4e = *pml4.add(pml4_idx);
    if pml4e & PTE_PRESENT == 0 { return 0; }
    let pdpt = (pml4e & PTE_ADDR_MASK) as *mut Pte;
    let pdpt_idx = ((virt >> 30) & 0x1FF) as Usize;
    let pdpte = *pdpt.add(pdpt_idx);
    if pdpte & PTE_PRESENT == 0 { return 0; }
    let pd = (pdpte & PTE_ADDR_MASK) as *mut Pte;
    let pd_idx = ((virt >> 21) & 0x1FF) as Usize;
    let pde = *pd.add(pd_idx);
    if pde & PTE_PRESENT == 0 { return 0; }
    // Check for 2 MiB huge page
    if pde & (1 << 7) != 0 { return (pde & 0x000F_FFFF_FFE0_0000) | (virt & 0x1F_FFFF); }
    let pt = (pde & PTE_ADDR_MASK) as *mut Pte;
    let pt_idx = ((virt >> 12) & 0x1FF) as Usize;
    let pte = *pt.add(pt_idx);
    if pte & PTE_PRESENT == 0 { return 0; }
    (pte & PTE_ADDR_MASK) | (virt & 0xFFF)
}
