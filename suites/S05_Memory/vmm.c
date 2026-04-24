/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: VIRTUAL MEMORY MANAGER (v1.0 - PURE C11)
 * =============================================================================
 * Architecture: x86_64 4-level paging (PML4→PDP→PD→PT)
 * Features:
 *   - Map/unmap 4KB pages with permission bits
 *   - Demand-mapped kernel heap (vmalloc)
 *   - INVLPG TLB shootdown per entry
 *   - Kernel higher-half map (KERNEL_VMA = 0xFFFFFFFF80000000)
 *   - No external allocator calls beyond pmm_alloc_page()
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "sigma_kernel_types.h"

/* =========================================================================
 * External: physical allocator
 * ========================================================================= */
extern paddr_t pmm_alloc_page(void);
extern void    pmm_free_page(paddr_t pa);

/* =========================================================================
 * Paging entry flags
 * ========================================================================= */
#define PTE_PRESENT   BIT(0)
#define PTE_WRITABLE  BIT(1)
#define PTE_USER      BIT(2)
#define PTE_WT        BIT(3)
#define PTE_NOCACHE   BIT(4)
#define PTE_ACCESSED  BIT(5)
#define PTE_DIRTY     BIT(6)
#define PTE_HUGE      BIT(7)
#define PTE_GLOBAL    BIT(8)
#define PTE_NX        BIT(63)           /* No-Execute (IA32_EFER.NXE must be set) */

#define PTE_ADDR_MASK 0x000FFFFFFFFFF000ULL

/* Extract 9-bit index for each level */
#define PML4_IDX(va) (((va) >> 39) & 0x1FF)
#define PDP_IDX(va)  (((va) >> 30) & 0x1FF)
#define PD_IDX(va)   (((va) >> 21) & 0x1FF)
#define PT_IDX(va)   (((va) >> 12) & 0x1FF)

typedef u64 pte_t;

/* =========================================================================
 * VMM State
 * ========================================================================= */
typedef struct SigmaVMM {
    paddr_t pml4_phys;      /* physical addr of kernel PML4 */
    vaddr_t vmalloc_next;   /* next vmalloc address */
    u64     map_calls;
    u64     unmap_calls;
} SigmaVMM;

static SigmaVMM g_vmm;

/* =========================================================================
 * Internal helpers
 * ========================================================================= */

/* Read a page table entry */
static pte_t pte_read(paddr_t table_pa, u32 idx) {
    return ((volatile pte_t*)(usize)table_pa)[idx];
}

/* Write a page table entry */
static void pte_write(paddr_t table_pa, u32 idx, pte_t val) {
    ((volatile pte_t*)(usize)table_pa)[idx] = val;
}

/* Allocate a zeroed page table page */
static paddr_t alloc_table(void) {
    paddr_t pa = pmm_alloc_page();
    if (!pa) return 0;
    /* Zero the page (512 × u64 entries) */
    volatile u64* p = (volatile u64*)(usize)pa;
    u32 i;
    for (i = 0; i < 512; i++) p[i] = 0;
    return pa;
}

/* Walk/create PML4→PDP→PD, return PD physical address */
static paddr_t walk_to_pd(vaddr_t va, bool_t create) {
    /* PML4 */
    u32 l4 = PML4_IDX(va);
    pte_t e4 = pte_read(g_vmm.pml4_phys, l4);
    paddr_t pdp_pa;
    if (!(e4 & PTE_PRESENT)) {
        if (!create) return 0;
        pdp_pa = alloc_table();
        if (!pdp_pa) return 0;
        pte_write(g_vmm.pml4_phys, l4, pdp_pa | PTE_PRESENT | PTE_WRITABLE);
    } else {
        pdp_pa = e4 & PTE_ADDR_MASK;
    }
    /* PDP */
    u32 l3 = PDP_IDX(va);
    pte_t e3 = pte_read(pdp_pa, l3);
    paddr_t pd_pa;
    if (!(e3 & PTE_PRESENT)) {
        if (!create) return 0;
        pd_pa = alloc_table();
        if (!pd_pa) return 0;
        pte_write(pdp_pa, l3, pd_pa | PTE_PRESENT | PTE_WRITABLE);
    } else {
        pd_pa = e3 & PTE_ADDR_MASK;
    }
    return pd_pa;
}

/* Walk/create to PT, return PT physical address */
static paddr_t walk_to_pt(vaddr_t va, bool_t create) {
    paddr_t pd_pa = walk_to_pd(va, create);
    if (!pd_pa) return 0;
    u32 l2 = PD_IDX(va);
    pte_t e2 = pte_read(pd_pa, l2);
    paddr_t pt_pa;
    if (!(e2 & PTE_PRESENT)) {
        if (!create) return 0;
        pt_pa = alloc_table();
        if (!pt_pa) return 0;
        pte_write(pd_pa, l2, pt_pa | PTE_PRESENT | PTE_WRITABLE);
    } else {
        if (e2 & PTE_HUGE) return 0; /* 2MB page — no PT */
        pt_pa = e2 & PTE_ADDR_MASK;
    }
    return pt_pa;
}

/* =========================================================================
 * VMM Init — create kernel PML4, identity-map first 4GB, map kernel VMA
 * ========================================================================= */
void vmm_init(void) {
    g_vmm.pml4_phys  = alloc_table();
    g_vmm.vmalloc_next = KERNEL_VMA + (512ULL * 1024ULL * 1024ULL); /* +512MB */
    g_vmm.map_calls   = 0;
    g_vmm.unmap_calls = 0;

    /* Identity-map first 4GB using 2MB huge pages (PD level) */
    /* This ensures early kernel code (at physical address) still works */
    paddr_t pdp_id = alloc_table();
    pte_write(g_vmm.pml4_phys, 0, pdp_id | PTE_PRESENT | PTE_WRITABLE);
    u32 i;
    for (i = 0; i < 4; i++) {   /* 4 × 1GB PDP entries */
        paddr_t pd_id = alloc_table();
        pte_write(pdp_id, i, pd_id | PTE_PRESENT | PTE_WRITABLE);
        u32 j;
        for (j = 0; j < 512; j++) {  /* 512 × 2MB pages */
            paddr_t phys = ((paddr_t)i << 30) + ((paddr_t)j << 21);
            pte_write(pd_id, j, phys | PTE_PRESENT | PTE_WRITABLE | PTE_GLOBAL | PTE_HUGE);
        }
    }

    /* Also map KERNEL_VMA → physical 0 (higher-half kernel) */
    u32 kvma_l4 = PML4_IDX(KERNEL_VMA);
    paddr_t kpdp = alloc_table();
    pte_write(g_vmm.pml4_phys, kvma_l4, kpdp | PTE_PRESENT | PTE_WRITABLE);
    for (i = 0; i < 4; i++) {
        paddr_t kpd = alloc_table();
        pte_write(kpdp, i, kpd | PTE_PRESENT | PTE_WRITABLE);
        u32 j;
        for (j = 0; j < 512; j++) {
            paddr_t phys = ((paddr_t)i << 30) + ((paddr_t)j << 21);
            pte_write(kpd, j, phys | PTE_PRESENT | PTE_WRITABLE | PTE_GLOBAL | PTE_HUGE);
        }
    }

    /* Load our new PML4 */
    cpu_write_cr3(g_vmm.pml4_phys);
}

/* =========================================================================
 * Map a single 4KB page: va → pa with given flags
 * ========================================================================= */
k_status vmm_map(vaddr_t va, paddr_t pa, u64 flags) {
    paddr_t pt_pa = walk_to_pt(va, TRUE);
    if (!pt_pa) return K_ERR_NOMEM;
    u32 idx = PT_IDX(va);
    pte_write(pt_pa, idx, (pa & PTE_ADDR_MASK) | flags | PTE_PRESENT);
    cpu_invlpg(va);
    g_vmm.map_calls++;
    return K_OK;
}

/* =========================================================================
 * Unmap a single 4KB page
 * ========================================================================= */
k_status vmm_unmap(vaddr_t va) {
    paddr_t pt_pa = walk_to_pt(va, FALSE);
    if (!pt_pa) return K_ERR_NOTFOUND;
    u32 idx = PT_IDX(va);
    pte_write(pt_pa, idx, 0);
    cpu_invlpg(va);
    g_vmm.unmap_calls++;
    return K_OK;
}

/* =========================================================================
 * Translate virtual → physical address
 * ========================================================================= */
paddr_t vmm_translate(vaddr_t va) {
    paddr_t pt_pa = walk_to_pt(va, FALSE);
    if (!pt_pa) return 0;
    pte_t pte = pte_read(pt_pa, PT_IDX(va));
    if (!(pte & PTE_PRESENT)) return 0;
    return (pte & PTE_ADDR_MASK) | (va & 0xFFF);
}

/* =========================================================================
 * vmalloc: allocate n pages in kernel virtual space, backed by physical pages
 * ========================================================================= */
vaddr_t vmalloc(u64 npages) {
    vaddr_t base = g_vmm.vmalloc_next;
    u64 i;
    for (i = 0; i < npages; i++) {
        paddr_t pa = pmm_alloc_page();
        if (!pa) return 0;
        k_status s = vmm_map(base + i * PAGE_SIZE, pa,
                              PTE_WRITABLE | PTE_GLOBAL | PTE_NX);
        if (s != K_OK) return 0;
    }
    g_vmm.vmalloc_next += npages * PAGE_SIZE;
    return base;
}

/* =========================================================================
 * Audit
 * ========================================================================= */
void vmm_audit(void) {
    extern void ksigma_printf(const char* fmt, ...);
    ksigma_printf("[VMM]: PML4=%p | Maps=%llu | Unmaps=%llu | vmalloc_next=%p\n",
            (void*)(usize)g_vmm.pml4_phys,
            g_vmm.map_calls, g_vmm.unmap_calls,
            (void*)g_vmm.vmalloc_next);
}
