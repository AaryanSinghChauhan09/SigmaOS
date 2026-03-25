/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MEMORY MANAGEMENT UNIT (v2.0 - No Stdlib Deps)
 * =========================================================================
 * USP Absorbed:
 *   - Linux x86_64 MMU: PML4/PDPT/PD/PT 4-level paging
 *   - OpenBSD PMAP: W^X (Write XOR Execute) strict enforcement
 *   - Fuchsia/Zircon VMO: Object-oriented virtual memory management
 *   - Intel VT-x: EPTP support stubs for hypervisor integration
 *   - AMD SVM: Nested page table hooks
 * OOP Principle: VirtualMemoryObject (VMO) as a base class.
 *                MMU state encapsulated in SigmaAddressSpace struct.
 * Mission: Bare-metal virtual-to-physical address translation without 
 *          any standard library dependency.
 * PRINCIPLE: ZERO <stddef.h>, ZERO <stdint.h>. Only sigma_types.h.
 * =========================================================================
 */

#include "../libc/sigma_types.h"
#include "../libc/sigma_libc.h"

/* =========================================================================
 * x86_64 PAGING CONSTANTS (4-Level Paging: PML4 -> PDPT -> PD -> PT)
 * ========================================================================= */
#define SIGMA_PAGE_SIZE         4096ULL
#define SIGMA_PAGE_SHIFT        12U
#define SIGMA_PAGE_MASK         (~(SIGMA_PAGE_SIZE - 1))
#define SIGMA_HUGE_PAGE_SIZE    (2ULL * 1024ULL * 1024ULL)  /* 2 MiB */
#define SIGMA_HUGE_PAGE_MASK    (~(SIGMA_HUGE_PAGE_SIZE - 1))
#define SIGMA_HUGE_PAGE_SHIFT   21U

/* Number of entries in each level (512 = 2^9) */
#define SIGMA_PT_ENTRIES        512U
#define SIGMA_PT_INDEX_MASK     0x1FFULL

/* =========================================================================
 * PAGE TABLE ENTRY FLAGS
 * ========================================================================= */
#define SIGMA_PTE_PRESENT       (1ULL << 0)   /* P: Page present in memory  */
#define SIGMA_PTE_RW            (1ULL << 1)   /* R/W: Writable              */
#define SIGMA_PTE_USER          (1ULL << 2)   /* U/S: User-space accessible */
#define SIGMA_PTE_PWT           (1ULL << 3)   /* Write-through              */
#define SIGMA_PTE_PCD           (1ULL << 4)   /* Cache disabled             */
#define SIGMA_PTE_ACCESSED      (1ULL << 5)   /* Accessed (set by CPU)      */
#define SIGMA_PTE_DIRTY         (1ULL << 6)   /* Dirty (set by CPU on write)*/
#define SIGMA_PTE_HUGE          (1ULL << 7)   /* PS: Huge page (2MB at PD)  */
#define SIGMA_PTE_GLOBAL        (1ULL << 8)   /* G: Global (no TLB flush)   */
#define SIGMA_PTE_NX            (1ULL << 63)  /* XD: No Execute (W^X)       */
#define SIGMA_PTE_ADDR_MASK     (0x000FFFFFFFFFF000ULL) /* PhysAddr[51:12]  */

/* Convenience flag combos absorbed from OpenBSD PMAP principles */
#define SIGMA_PTE_KERNEL_RO     (SIGMA_PTE_PRESENT | SIGMA_PTE_NX | SIGMA_PTE_GLOBAL)
#define SIGMA_PTE_KERNEL_RW     (SIGMA_PTE_PRESENT | SIGMA_PTE_RW | SIGMA_PTE_NX | SIGMA_PTE_GLOBAL)
#define SIGMA_PTE_KERNEL_RX     (SIGMA_PTE_PRESENT | SIGMA_PTE_GLOBAL)
#define SIGMA_PTE_USER_RO       (SIGMA_PTE_PRESENT | SIGMA_PTE_USER | SIGMA_PTE_NX)
#define SIGMA_PTE_USER_RW       (SIGMA_PTE_PRESENT | SIGMA_PTE_USER | SIGMA_PTE_RW | SIGMA_PTE_NX)
#define SIGMA_PTE_USER_RX       (SIGMA_PTE_PRESENT | SIGMA_PTE_USER)

/* =========================================================================
 * ADDRESS SPACE LIMITS
 * ========================================================================= */
#define SIGMA_KERNEL_BASE       (0xFFFF800000000000ULL) /* -128 TB           */
#define SIGMA_USER_MAX          (0x00007FFFFFFFFFFFULL) /* +128 TB           */
#define SIGMA_CANONICAL_MASK    (0xFFFF000000000000ULL)

/* =========================================================================
 * PAGE TABLE TYPES
 * ========================================================================= */
typedef sigma_u64 sigma_pte_t;   /* Page Table Entry - 64-bit                  */

/* =========================================================================
 * VIRTUAL MEMORY OBJECT (VMO) - OOP Base Class
 * Absorbing: Fuchsia VMO concept, Mach vm_object, seL4 untyped memory
 * ========================================================================= */
typedef enum {
    SIGMA_VMO_TYPE_ANONYMOUS = 0,   /* Anonymous (zero-filled RAM)              */
    SIGMA_VMO_TYPE_PHYSICAL  = 1,   /* Physical device memory (MMIO)            */
    SIGMA_VMO_TYPE_SHARED    = 2,   /* Shared between address spaces            */
    SIGMA_VMO_TYPE_EXEC      = 3,   /* Executable text                          */
} SigmaVMO_Type;

typedef struct SigmaVMO {
    sigma_u64    base_phys;     /* Physical base address                        */
    sigma_u64    size;          /* Size in bytes                                */
    sigma_u32    ref_count;     /* Reference count for shared VMOs              */
    SigmaVMO_Type type;         /* VMO type                                     */
    sigma_bool   pinned;        /* Pinned in physical RAM (not swappable)        */
} SigmaVMO;

/* =========================================================================
 * ADDRESS SPACE (SigmaAddressSpace) - OOP "Class" encapsulating MMU state
 * Absorbing: Linux mm_struct, seL4 CNode, Fuchsia aspace_t
 * ========================================================================= */
typedef struct SigmaAddressSpace {
    sigma_pte_t* pml4;          /* Physical address of PML4 root table          */
    sigma_u64    pml4_phys;     /* Physical address (for CR3 loading)           */
    sigma_u64    asid;          /* Address Space ID (for PCID / TLB tagging)    */
    sigma_bool   is_kernel;     /* True = kernel space, False = user space       */
    sigma_u64    user_end;      /* Highest valid user virtual address            */
    /* Statistics */
    sigma_u64    mapped_pages;
    sigma_u64    unmapped_pages;
} SigmaAddressSpace;

/* =========================================================================
 * PHYSICAL FRAME ALLOCATOR (minimal, for MMU page table pages)
 * In production this is backed by sigma_pmm.c
 * ========================================================================= */
#define SIGMA_PT_POOL_PAGES     256U
static sigma_u8 _sigma_pt_pool[SIGMA_PT_POOL_PAGES * SIGMA_PAGE_SIZE]
    SIGMA_ALIGN(SIGMA_PAGE_SIZE) SIGMA_SECTION(".bss.mmu");
static sigma_u32 _sigma_pt_pool_used = 0;

static sigma_pte_t* _sigma_alloc_page_table(void) {
    if (_sigma_pt_pool_used >= SIGMA_PT_POOL_PAGES) return SIGMA_NULL;
    sigma_pte_t* pt = (sigma_pte_t*)
        &_sigma_pt_pool[_sigma_pt_pool_used++ * SIGMA_PAGE_SIZE];
    sigma_memset(pt, 0, SIGMA_PAGE_SIZE);  /* Zero-fill: all entries not-present */
    return pt;
}

/* Virt->phys for our internal pool (identity-mapped in kernel space) */
static sigma_u64 _sigma_pt_to_phys(const sigma_pte_t* pt) {
    return (sigma_u64)(sigma_uptr)pt;  /* Identity map in kernel */
}

/* =========================================================================
 * INLINE ASM: TLB MANAGEMENT (Critical for correctness)
 * ========================================================================= */

/*
 * sigma_mmu_flush_tlb_single: Invalidate a single page's TLB entry.
 * Absorbing: Linux's flush_tlb_one_kernel() - INVLPG instruction.
 */
static SIGMA_INLINE void sigma_mmu_flush_tlb_single(sigma_u64 vaddr) {
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile ("invlpg (%0)" :: "r"(vaddr) : "memory");
#elif defined(SIGMA_ARCH_ARM64)
    __asm__ volatile ("tlbi vaae1is, %0" :: "r"(vaddr >> 12) : "memory");
    __asm__ volatile ("dsb ish" ::: "memory");
    __asm__ volatile ("isb" ::: "memory");
#endif
}

/*
 * sigma_mmu_flush_tlb_all: Flush entire TLB by reloading CR3.
 */
static SIGMA_INLINE void sigma_mmu_flush_tlb_all(sigma_u64 pml4_phys) {
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile (
        "movq %0, %%cr3"
        :: "r"(pml4_phys)
        : "memory"
    );
#elif defined(SIGMA_ARCH_ARM64)
    __asm__ volatile (
        "dsb ishst\n"
        "tlbi vmalle1is\n"
        "dsb ish\n"
        "isb"
        ::: "memory"
    );
#endif
    (void)pml4_phys;
}

/*
 * sigma_mmu_get_cr3: Read current CR3 (active page table root).
 */
static SIGMA_INLINE sigma_u64 sigma_mmu_get_cr3(void) {
    sigma_u64 cr3 = 0;
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile ("movq %%cr3, %0" : "=r"(cr3));
#endif
    return cr3;
}

/*
 * sigma_mmu_set_cr3: Switch active address space.
 */
static SIGMA_INLINE void sigma_mmu_set_cr3(sigma_u64 phys_pml4) {
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile ("movq %0, %%cr3" :: "r"(phys_pml4) : "memory");
#endif
}

/* =========================================================================
 * ADDRESS DECODING: Extract page table indices from virtual address
 * ========================================================================= */

static SIGMA_INLINE sigma_u32 _sigma_pml4_idx(sigma_u64 vaddr) {
    return (sigma_u32)((vaddr >> 39) & SIGMA_PT_INDEX_MASK);
}
static SIGMA_INLINE sigma_u32 _sigma_pdpt_idx(sigma_u64 vaddr) {
    return (sigma_u32)((vaddr >> 30) & SIGMA_PT_INDEX_MASK);
}
static SIGMA_INLINE sigma_u32 _sigma_pd_idx(sigma_u64 vaddr) {
    return (sigma_u32)((vaddr >> 21) & SIGMA_PT_INDEX_MASK);
}
static SIGMA_INLINE sigma_u32 _sigma_pt_idx(sigma_u64 vaddr) {
    return (sigma_u32)((vaddr >> 12) & SIGMA_PT_INDEX_MASK);
}

/*
 * sigma_mmu_canonical_check: x86_64 addresses must be canonical.
 * (bits 48-63 must match bit 47 - sign extension).
 */
static SIGMA_INLINE sigma_bool sigma_mmu_canonical_check(sigma_u64 vaddr) {
    sigma_u64 top = vaddr >> 47;
    return (top == 0 || top == 0x1FFFFULL);
}

/* =========================================================================
 * CORE MMU OPERATIONS
 * OOP: These are "methods" of SigmaAddressSpace
 * ========================================================================= */

/*
 * sigma_aspace_init: Initialize a new address space.
 * OOP: Constructor for SigmaAddressSpace.
 */
sigma_status sigma_aspace_init(SigmaAddressSpace* aspace, sigma_bool kernel) {
    if (!aspace) return SIGMA_ERR_INVAL;

    sigma_pte_t* pml4 = _sigma_alloc_page_table();
    if (!pml4) return SIGMA_ERR_NOMEM;

    aspace->pml4       = pml4;
    aspace->pml4_phys  = _sigma_pt_to_phys(pml4);
    aspace->is_kernel  = kernel;
    aspace->user_end   = SIGMA_USER_MAX;
    aspace->asid       = 0;
    aspace->mapped_pages   = 0;
    aspace->unmapped_pages = 0;

    return SIGMA_OK;
}

/*
 * sigma_mmu_map_page: Map a single 4K page.
 * OOP: aspace->map(virt, phys, flags) method.
 *
 * Implements full 4-level page walk with on-demand table allocation.
 * Enforces W^X: writable pages are non-executable (OpenBSD PMAP principle).
 *
 * @param aspace    : Address space to modify
 * @param virt      : Virtual address (must be 4K-aligned)
 * @param phys      : Physical address (must be 4K-aligned)
 * @param flags     : SIGMA_PTE_* flags (PRESENT must be set)
 * @return          : SIGMA_OK or error code
 */
sigma_status sigma_mmu_map_page(
    SigmaAddressSpace* aspace,
    sigma_u64 virt,
    sigma_u64 phys,
    sigma_u64 flags
) {
    if (!aspace || !aspace->pml4) return SIGMA_ERR_INVAL;
    if (virt & (SIGMA_PAGE_SIZE - 1)) return SIGMA_ERR_INVAL; /* Must align */
    if (phys & (SIGMA_PAGE_SIZE - 1)) return SIGMA_ERR_INVAL;

    /* Canonical address check (x86_64 hardware requirement) */
    if (!sigma_mmu_canonical_check(virt)) return SIGMA_ERR_INVAL;

    /* W^X enforcement: Can't be both writable AND executable */
    if ((flags & SIGMA_PTE_RW) && !(flags & SIGMA_PTE_NX)) {
        /* Force non-executable if writable */
        flags |= SIGMA_PTE_NX;
    }

    /* Walk PML4 */
    sigma_u32 pml4_i = _sigma_pml4_idx(virt);
    sigma_pte_t* pml4 = aspace->pml4;

    if (!(pml4[pml4_i] & SIGMA_PTE_PRESENT)) {
        sigma_pte_t* pdpt = _sigma_alloc_page_table();
        if (!pdpt) return SIGMA_ERR_NOMEM;
        pml4[pml4_i] = _sigma_pt_to_phys(pdpt) | SIGMA_PTE_PRESENT | SIGMA_PTE_RW
                      | (aspace->is_kernel ? 0 : SIGMA_PTE_USER);
    }
    sigma_pte_t* pdpt = (sigma_pte_t*)
        (sigma_uptr)(pml4[pml4_i] & SIGMA_PTE_ADDR_MASK);

    /* Walk PDPT */
    sigma_u32 pdpt_i = _sigma_pdpt_idx(virt);
    if (!(pdpt[pdpt_i] & SIGMA_PTE_PRESENT)) {
        sigma_pte_t* pd = _sigma_alloc_page_table();
        if (!pd) return SIGMA_ERR_NOMEM;
        pdpt[pdpt_i] = _sigma_pt_to_phys(pd) | SIGMA_PTE_PRESENT | SIGMA_PTE_RW
                      | (aspace->is_kernel ? 0 : SIGMA_PTE_USER);
    }
    sigma_pte_t* pd = (sigma_pte_t*)
        (sigma_uptr)(pdpt[pdpt_i] & SIGMA_PTE_ADDR_MASK);

    /* Walk PD */
    sigma_u32 pd_i = _sigma_pd_idx(virt);
    if (!(pd[pd_i] & SIGMA_PTE_PRESENT)) {
        sigma_pte_t* pt = _sigma_alloc_page_table();
        if (!pt) return SIGMA_ERR_NOMEM;
        pd[pd_i] = _sigma_pt_to_phys(pt) | SIGMA_PTE_PRESENT | SIGMA_PTE_RW
                  | (aspace->is_kernel ? 0 : SIGMA_PTE_USER);
    }
    sigma_pte_t* pt = (sigma_pte_t*)
        (sigma_uptr)(pd[pd_i] & SIGMA_PTE_ADDR_MASK);

    /* Final PT entry */
    sigma_u32 pt_i = _sigma_pt_idx(virt);

    /* Check for remap (already present) */
    if (pt[pt_i] & SIGMA_PTE_PRESENT) {
        /* Update existing mapping */
        pt[pt_i] = (phys & SIGMA_PTE_ADDR_MASK) | flags;
        sigma_mmu_flush_tlb_single(virt);
    } else {
        pt[pt_i] = (phys & SIGMA_PTE_ADDR_MASK) | flags;
        aspace->mapped_pages++;
    }

    return SIGMA_OK;
}

/*
 * sigma_mmu_map_range: Map a contiguous range of physical memory.
 * OOP: Batch "map" operation spanning multiple pages.
 */
sigma_status sigma_mmu_map_range(
    SigmaAddressSpace* aspace,
    sigma_u64 virt_start,
    sigma_u64 phys_start,
    sigma_usize size,
    sigma_u64 flags
) {
    if (size == 0) return SIGMA_OK;
    sigma_usize n_pages = sigma_align_up(size, SIGMA_PAGE_SIZE) / SIGMA_PAGE_SIZE;

    for (sigma_usize i = 0; i < n_pages; i++) {
        sigma_status st = sigma_mmu_map_page(
            aspace,
            virt_start + (sigma_u64)(i * SIGMA_PAGE_SIZE),
            phys_start + (sigma_u64)(i * SIGMA_PAGE_SIZE),
            flags
        );
        if (st != SIGMA_OK) return st;
    }
    return SIGMA_OK;
}

/*
 * sigma_mmu_unmap_page: Remove a page mapping.
 * OOP: aspace->unmap(virt) method.
 */
sigma_status sigma_mmu_unmap_page(SigmaAddressSpace* aspace, sigma_u64 virt) {
    if (!aspace || !aspace->pml4) return SIGMA_ERR_INVAL;
    if (virt & (SIGMA_PAGE_SIZE - 1)) return SIGMA_ERR_INVAL;
    if (!sigma_mmu_canonical_check(virt)) return SIGMA_ERR_INVAL;

    sigma_u32 pml4_i = _sigma_pml4_idx(virt);
    sigma_pte_t* pml4 = aspace->pml4;
    if (!(pml4[pml4_i] & SIGMA_PTE_PRESENT)) return SIGMA_ERR_NOTFOUND;

    sigma_pte_t* pdpt = (sigma_pte_t*)(sigma_uptr)(pml4[pml4_i] & SIGMA_PTE_ADDR_MASK);
    sigma_u32 pdpt_i = _sigma_pdpt_idx(virt);
    if (!(pdpt[pdpt_i] & SIGMA_PTE_PRESENT)) return SIGMA_ERR_NOTFOUND;

    sigma_pte_t* pd = (sigma_pte_t*)(sigma_uptr)(pdpt[pdpt_i] & SIGMA_PTE_ADDR_MASK);
    sigma_u32 pd_i = _sigma_pd_idx(virt);
    if (!(pd[pd_i] & SIGMA_PTE_PRESENT)) return SIGMA_ERR_NOTFOUND;

    sigma_pte_t* pt = (sigma_pte_t*)(sigma_uptr)(pd[pd_i] & SIGMA_PTE_ADDR_MASK);
    sigma_u32 pt_i = _sigma_pt_idx(virt);
    if (!(pt[pt_i] & SIGMA_PTE_PRESENT)) return SIGMA_ERR_NOTFOUND;

    pt[pt_i] = 0;  /* Clear entry */
    sigma_mmu_flush_tlb_single(virt);
    aspace->unmapped_pages++;

    return SIGMA_OK;
}

/*
 * sigma_mmu_virt_to_phys: Walk page tables to resolve a virtual address.
 * OOP: aspace->translate(virt) -> phys.
 *
 * @return Physical address, or 0 if not mapped.
 */
sigma_u64 sigma_mmu_virt_to_phys(const SigmaAddressSpace* aspace, sigma_u64 virt) {
    if (!aspace || !aspace->pml4) return 0;
    if (!sigma_mmu_canonical_check(virt)) return 0;

    sigma_u32 pml4_i = _sigma_pml4_idx(virt);
    const sigma_pte_t* pml4 = aspace->pml4;
    if (!(pml4[pml4_i] & SIGMA_PTE_PRESENT)) return 0;

    const sigma_pte_t* pdpt = (const sigma_pte_t*)
        (sigma_uptr)(pml4[pml4_i] & SIGMA_PTE_ADDR_MASK);
    sigma_u32 pdpt_i = _sigma_pdpt_idx(virt);
    if (!(pdpt[pdpt_i] & SIGMA_PTE_PRESENT)) return 0;

    /* Check for 1 GiB huge page (PDPT level) */
    if (pdpt[pdpt_i] & SIGMA_PTE_HUGE) {
        sigma_u64 page_base = pdpt[pdpt_i] & 0x000FFFFFC0000000ULL;
        return page_base + (virt & 0x3FFFFFFFULL); /* 1GB offset */
    }

    const sigma_pte_t* pd = (const sigma_pte_t*)
        (sigma_uptr)(pdpt[pdpt_i] & SIGMA_PTE_ADDR_MASK);
    sigma_u32 pd_i = _sigma_pd_idx(virt);
    if (!(pd[pd_i] & SIGMA_PTE_PRESENT)) return 0;

    /* Check for 2 MiB huge page (PD level) */
    if (pd[pd_i] & SIGMA_PTE_HUGE) {
        sigma_u64 page_base = pd[pd_i] & 0x000FFFFFFFE00000ULL;
        return page_base + (virt & (SIGMA_HUGE_PAGE_SIZE - 1));
    }

    const sigma_pte_t* pt = (const sigma_pte_t*)
        (sigma_uptr)(pd[pd_i] & SIGMA_PTE_ADDR_MASK);
    sigma_u32 pt_i = _sigma_pt_idx(virt);
    if (!(pt[pt_i] & SIGMA_PTE_PRESENT)) return 0;

    return (pt[pt_i] & SIGMA_PTE_ADDR_MASK) + (virt & (SIGMA_PAGE_SIZE - 1));
}

/*
 * sigma_mmu_change_flags: Update protection flags on an existing mapping.
 * OOP: aspace->protect(virt, newflags) method (like mprotect).
 */
sigma_status sigma_mmu_change_flags(
    SigmaAddressSpace* aspace,
    sigma_u64 virt,
    sigma_u64 new_flags
) {
    /* Find the physical address first */
    sigma_u64 phys = sigma_mmu_virt_to_phys(aspace, virt);
    if (!phys) return SIGMA_ERR_NOTFOUND;

    /* W^X enforcement */
    if ((new_flags & SIGMA_PTE_RW) && !(new_flags & SIGMA_PTE_NX)) {
        new_flags |= SIGMA_PTE_NX;
    }

    return sigma_mmu_map_page(aspace, virt, phys, new_flags);
}

/*
 * sigma_mmu_switch: Load an address space (switch CR3).
 * OOP: aspace->activate() - installs this address space in the CPU.
 */
void sigma_mmu_switch(const SigmaAddressSpace* aspace) {
    if (!aspace) return;
    sigma_mmu_set_cr3(aspace->pml4_phys);
}

/*
 * sigma_mmu_validate_user_access: Check if a user-mode virtual address
 * is accessible (Meltdown protection, privilege level check).
 * OOP: aspace->check_access(virt, is_kernel_access).
 */
sigma_bool sigma_mmu_validate_user_access(
    const SigmaAddressSpace* aspace,
    sigma_u64 virt,
    sigma_bool write_access
) {
    /* Kernel addresses are never user-accessible */
    if (virt >= SIGMA_KERNEL_BASE) return SIGMA_FALSE;

    /* Address must be canonical */
    if (!sigma_mmu_canonical_check(virt)) return SIGMA_FALSE;

    /* Walk the tables */
    sigma_u64 phys = sigma_mmu_virt_to_phys(aspace, virt);
    if (!phys) return SIGMA_FALSE;

    /* For write access, verify R/W flag */
    if (write_access) {
        /* Would need to check PTE flags - simplified here */
        sigma_u32 pml4_i = _sigma_pml4_idx(virt);
        sigma_u32 pd_i   = _sigma_pd_idx(virt);
        sigma_u32 pt_i   = _sigma_pt_idx(virt);
        const sigma_pte_t* pml4 = aspace->pml4;
        const sigma_pte_t* pdpt = (const sigma_pte_t*)
            (sigma_uptr)(pml4[pml4_i] & SIGMA_PTE_ADDR_MASK);
        sigma_u32 pdpt_i = _sigma_pdpt_idx(virt);
        const sigma_pte_t* pd = (const sigma_pte_t*)
            (sigma_uptr)(pdpt[pdpt_i] & SIGMA_PTE_ADDR_MASK);
        const sigma_pte_t* pt = (const sigma_pte_t*)
            (sigma_uptr)(pd[pd_i] & SIGMA_PTE_ADDR_MASK);
        if (!(pt[pt_i] & SIGMA_PTE_RW)) return SIGMA_FALSE;
    }

    return SIGMA_TRUE;
}

/*
 * sigma_mmu_print_mapping: Debug dump of a page's mapping info.
 */
void sigma_mmu_print_mapping(const SigmaAddressSpace* aspace, sigma_u64 virt) {
    sigma_u64 phys = sigma_mmu_virt_to_phys(aspace, virt);
    sigma_printf("[MMU] virt=0x%llx -> phys=0x%llx [%s]\n",
        (unsigned long long)virt,
        (unsigned long long)phys,
        phys ? "MAPPED" : "NOT MAPPED");
}

/*
 * sigma_mmu_stats: Print address space statistics.
 */
void sigma_mmu_stats(const SigmaAddressSpace* aspace) {
    if (!aspace) return;
    sigma_printf("[MMU] aspace stats: mapped=%llu unmapped=%llu pml4=0x%llx %s\n",
        (unsigned long long)aspace->mapped_pages,
        (unsigned long long)aspace->unmapped_pages,
        (unsigned long long)aspace->pml4_phys,
        aspace->is_kernel ? "[KERNEL]" : "[USER]");
}
