/*
 * Σ SigmaOS Zenith — Hardware Paging & Virtual Memory Manager
 * Absorbs: Linux x86_64 4-level paging, Arch Linux minimal VM design
 * Zero-Dependency: No libc, no stdlib, no predefined headers or functions.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef u64 phys_addr_t;
typedef u64 virt_addr_t;
typedef u64 pte_t;

/* ─────────────── Page Table Flags ─────────────── */
/* Inspired by Linux arch/x86/include/asm/pgtable_types.h */
#define PAGE_PRESENT     (1ULL << 0)   /* P: Page present in physical memory */
#define PAGE_WRITABLE    (1ULL << 1)   /* R/W: Read/Write allowed */
#define PAGE_USER        (1ULL << 2)   /* U/S: User-mode accessible */
#define PAGE_PWT         (1ULL << 3)   /* Write-Through caching */
#define PAGE_PCD         (1ULL << 4)   /* Cache Disabled */
#define PAGE_ACCESSED    (1ULL << 5)   /* CPU sets this on access */
#define PAGE_DIRTY       (1ULL << 6)   /* CPU sets this on write */
#define PAGE_HUGE        (1ULL << 7)   /* 2MB huge page if in PD */
#define PAGE_NX          (1ULL << 63)  /* No-Execute bit */

#define PAGE_SIZE        4096ULL
#define PAGE_ALIGN(addr) (((addr) + PAGE_SIZE - 1) & ~(PAGE_SIZE - 1))
#define PAGE_FRAME(pte)  ((pte) & 0x000FFFFFFFFFF000ULL)

/* ─────────────── 4-Level Page Table Indices ─────────────── */
/* PML4 → PDPT → PD → PT */
#define PML4_IDX(vaddr)  (((vaddr) >> 39) & 0x1FF)
#define PDPT_IDX(vaddr)  (((vaddr) >> 30) & 0x1FF)
#define PD_IDX(vaddr)    (((vaddr) >> 21) & 0x1FF)
#define PT_IDX(vaddr)    (((vaddr) >> 12) & 0x1FF)

/* ─────────────── Sovereign Page Frame Allocator ─────────────── */
#define PAGE_POOL_BASE   0x200000ULL  /* 2 MB — reserved pool start */
#define PAGE_POOL_PAGES  512          /* 2 MB of page tables */

static u64 page_pool_ptr = PAGE_POOL_BASE;

static pte_t* alloc_page_table() {
    if (page_pool_ptr + PAGE_SIZE > PAGE_POOL_BASE + (PAGE_POOL_PAGES * PAGE_SIZE))
        return (pte_t*)0; /* OOM */

    pte_t* pt = (pte_t*)page_pool_ptr;
    page_pool_ptr += PAGE_SIZE;

    /* Clear the page table — 512 entries */
    for (int i = 0; i < 512; i++) pt[i] = 0;

    return pt;
}

/* ─────────────── x86_64 CR3 Operations ─────────────── */
static inline u64 sigma_read_cr3() {
    u64 val;
    __asm__ volatile("mov %%cr3, %0" : "=r"(val));
    return val;
}

static inline void sigma_write_cr3(u64 val) {
    __asm__ volatile("mov %0, %%cr3" : : "r"(val) : "memory");
}

/* Flush TLB for a single page */
static inline void sigma_invlpg(virt_addr_t addr) {
    __asm__ volatile("invlpg (%0)" : : "r"(addr) : "memory");
}

static inline bool is_canonical_address(virt_addr_t addr) {
    u64 temp = addr >> 47;
    return (temp == 0 || temp == 0x1FFFFULL);
}

/* ─────────────── API: Map Virtual → Physical Page ─────────────── */
extern "C" bool sigma_map_page(virt_addr_t vaddr, phys_addr_t paddr, u64 flags) {
    if (!is_canonical_address(vaddr)) return false;
    u64 cr3 = sigma_read_cr3();
    pte_t* pml4 = (pte_t*)PAGE_FRAME(cr3);

    /* Walk and create PML4 → PDPT */
    pte_t* pdpt;
    if (!(pml4[PML4_IDX(vaddr)] & PAGE_PRESENT)) {
        pdpt = alloc_page_table();
        if (!pdpt) return false;
        pml4[PML4_IDX(vaddr)] = (pte_t)(u64)pdpt | PAGE_PRESENT | PAGE_WRITABLE;
    } else {
        pdpt = (pte_t*)PAGE_FRAME(pml4[PML4_IDX(vaddr)]);
    }

    /* Walk and create PDPT → PD */
    pte_t* pd;
    if (!(pdpt[PDPT_IDX(vaddr)] & PAGE_PRESENT)) {
        pd = alloc_page_table();
        if (!pd) return false;
        pdpt[PDPT_IDX(vaddr)] = (pte_t)(u64)pd | PAGE_PRESENT | PAGE_WRITABLE;
    } else {
        pd = (pte_t*)PAGE_FRAME(pdpt[PDPT_IDX(vaddr)]);
    }

    /* Walk and create PD → PT */
    pte_t* pt;
    if (!(pd[PD_IDX(vaddr)] & PAGE_PRESENT)) {
        pt = alloc_page_table();
        if (!pt) return false;
        pd[PD_IDX(vaddr)] = (pte_t)(u64)pt | PAGE_PRESENT | PAGE_WRITABLE;
    } else {
        pt = (pte_t*)PAGE_FRAME(pd[PD_IDX(vaddr)]);
    }

    /* Write the final PTE */
    pt[PT_IDX(vaddr)] = PAGE_FRAME(paddr) | flags | PAGE_PRESENT;
    sigma_invlpg(vaddr);
    return true;
}

/* ─────────────── API: Identity Map a Region ─────────────── */
/* Maps physical == virtual (used during boot, like Linux init_memory_mapping) */
extern "C" void sigma_identity_map(phys_addr_t start, phys_addr_t end) {
    u64 flags = PAGE_PRESENT | PAGE_WRITABLE;
    for (phys_addr_t addr = start; addr < end; addr += PAGE_SIZE) {
        sigma_map_page((virt_addr_t)addr, addr, flags);
    }
}

/* ─────────────── API: Page Fault Handler ─────────────── */
/* Called from Interrupt 14 (Page Fault) IDT entry */
extern "C" void sigma_page_fault_handler(u64 error_code, virt_addr_t fault_addr) {
    /* Read CR2 which contains the faulting address */
    virt_addr_t cr2;
    __asm__ volatile("mov %%cr2, %0" : "=r"(cr2));

    /* Bit 0 of error code: 0=Non-present page, 1=Protection violation */
    bool not_present   = !(error_code & 0x1);
    bool was_write     = (error_code >> 1) & 0x1;
    bool user_mode     = (error_code >> 2) & 0x1;

    if (not_present) {
        /* Demand paging: Allocate a physical frame and map it */
        /* (Physical frame allocator integration point) */
        sigma_map_page(cr2 & ~(PAGE_SIZE - 1), cr2 & ~(PAGE_SIZE - 1), PAGE_PRESENT | PAGE_WRITABLE);
    } else {
        /* Protection fault — halt the offending task */
        __asm__ volatile("hlt");
    }
}
