// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_thp.cpp — Transparent Huge Pages (THP) for SigmaOS
//
// When a process allocates a contiguous 2MB virtual region and all 512
// underlying 4KB pages are mapped, this code promotes them to a single
// 2MB PTE (using the PS bit).  The hardware can then cache the whole
// 2MB region in a single TLB entry instead of 512 — reducing TLB pressure
// by 512× for large allocations (AI model buffers, video frames, etc.).
//
// Inspired by:
//   • Linux mm/huge_memory.c (THP collapse logic)
//   • Linux mm/khugepaged.c (background scanner)
//   • FreeBSD sys/vm/vm_page.c (superpage promotion)

#include "sigma_thp.h"
#include "sigma_vmm.h"
#include <stdint.h>
#include <stdbool.h>
#include <stdatomic.h>

#define PAGE_SIZE      4096UL
#define HUGE_PAGE_SIZE (2UL * 1024 * 1024)   // 2MB
#define PAGES_PER_HUGE (HUGE_PAGE_SIZE / PAGE_SIZE)  // 512

// PML4 → PDPT → PD → PT bit layout
#define PTE_PRESENT    (1ULL << 0)
#define PTE_WRITE      (1ULL << 1)
#define PTE_USER       (1ULL << 2)
#define PTE_ACCESSED   (1ULL << 5)
#define PTE_DIRTY      (1ULL << 6)
#define PTE_HUGE       (1ULL << 7)   // PS bit: 2MB page at PD level
#define PTE_NX         (1ULL << 63)
#define PTE_ADDR_MASK  0x000FFFFFFFFFF000ULL

typedef uint64_t pte_t;

// ── Physical memory allocator hook (provided by sigma_pmm.cpp) ───────────────

extern uintptr_t sigma_pmm_alloc_huge(void);  // returns 2MB-aligned PA or 0

// ── THP promotion ─────────────────────────────────────────────────────────────

// Returns the Page Directory entry pointer for a given virtual address
static pte_t *va_to_pd_entry(uintptr_t cr3, uintptr_t va) {
    uint64_t pml4i = (va >> 39) & 0x1FF;
    uint64_t pdpti = (va >> 30) & 0x1FF;
    uint64_t pdi   = (va >> 21) & 0x1FF;

    pte_t *pml4 = (pte_t *)(cr3 & PTE_ADDR_MASK);
    if (!(pml4[pml4i] & PTE_PRESENT)) return NULL;

    pte_t *pdpt = (pte_t *)(pml4[pml4i] & PTE_ADDR_MASK);
    if (!(pdpt[pdpti] & PTE_PRESENT)) return NULL;
    if (pdpt[pdpti] & PTE_HUGE) return NULL;  // already 1GB page

    pte_t *pd = (pte_t *)(pdpt[pdpti] & PTE_ADDR_MASK);
    return &pd[pdi];
}

// Check whether all 512 4KB PTEs in a 2MB range are present and contiguous
static bool region_collapsible(uintptr_t cr3, uintptr_t va_base,
                                uintptr_t *out_first_pa) {
    // va_base must be 2MB-aligned
    if (va_base & (HUGE_PAGE_SIZE - 1)) return false;

    pte_t *pd_entry = va_to_pd_entry(cr3, va_base);
    if (!pd_entry) return false;
    if (!(*pd_entry & PTE_PRESENT)) return false;
    if (*pd_entry & PTE_HUGE) return false;  // already huge

    pte_t *pt = (pte_t *)(*pd_entry & PTE_ADDR_MASK);
    uintptr_t expected_pa = pt[0] & PTE_ADDR_MASK;

    for (int i = 0; i < (int)PAGES_PER_HUGE; i++) {
        if (!(pt[i] & PTE_PRESENT)) return false;
        if ((pt[i] & PTE_ADDR_MASK) != expected_pa + (uintptr_t)i * PAGE_SIZE)
            return false;  // non-contiguous physical pages
    }
    *out_first_pa = expected_pa;
    return true;
}

// Promote a 2MB VA range to a single huge PTE
// Returns true on success.
bool sigma_thp_promote(uintptr_t cr3, uintptr_t va_base) {
    uintptr_t first_pa = 0;
    if (!region_collapsible(cr3, va_base, &first_pa)) return false;

    // first_pa must also be 2MB-aligned for a huge PTE
    if (first_pa & (HUGE_PAGE_SIZE - 1)) return false;

    pte_t *pd_entry = va_to_pd_entry(cr3, va_base);
    if (!pd_entry) return false;

    // Save old PT pointer to free it after promotion
    uintptr_t old_pt_pa = *pd_entry & PTE_ADDR_MASK;

    // Install 2MB PTE
    pte_t new_entry = first_pa | PTE_PRESENT | PTE_WRITE | PTE_HUGE |
                      PTE_ACCESSED | PTE_DIRTY;
    // Preserve NX and user bits from old entry
    if (*pd_entry & PTE_NX)   new_entry |= PTE_NX;
    if (*pd_entry & PTE_USER) new_entry |= PTE_USER;

    // Atomic write — other CPUs may be reading the PD concurrently
    __atomic_store_n(pd_entry, new_entry, __ATOMIC_SEQ_CST);

    // TLB shootdown: invalidate the old 512 mappings on all CPUs
    // (sigma_ipi_tlb_shootdown declared in sigma_ipi.h)
    extern void sigma_ipi_tlb_shootdown(uintptr_t, uintptr_t);
    sigma_ipi_tlb_shootdown(va_base, va_base + HUGE_PAGE_SIZE);

    // Free the old PT page (4KB)
    extern void sigma_pmm_free_page(uintptr_t pa);
    sigma_pmm_free_page(old_pt_pa);

    return true;
}

// ── Background khugepaged-style scanner ──────────────────────────────────────

// Called periodically (e.g., every 100ms from sigma_sched_idle())
// Scans the first @scan_limit 2MB-aligned regions in the calling process's
// address space and promotes any that are eligible.

void sigma_thp_scan(uintptr_t cr3, uintptr_t va_start, uintptr_t va_end,
                    uint32_t scan_limit) {
    uintptr_t va = va_start & ~(HUGE_PAGE_SIZE - 1);
    uint32_t  promoted = 0;

    while (va < va_end && promoted < scan_limit) {
        if (sigma_thp_promote(cr3, va)) promoted++;
        va += HUGE_PAGE_SIZE;
    }
}
