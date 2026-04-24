/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: PHYSICAL MEMORY MANAGER (v1.0 - PURE C11)
 * =============================================================================
 * Algorithm: Buddy allocator (order 0..10, i.e. 4KB..4MB blocks)
 * Features:
 *   - Free-list per order (doubly-linked)
 *   - O(log n) alloc/free
 *   - Coalescing (buddies merged on free)
 *   - Multiboot2 memory map bootstrapped
 *   - Zero external dependencies
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "sigma_kernel_types.h"

/* =========================================================================
 * Constants
 * ========================================================================= */
#define PMM_MAX_ORDER     10u           /* max block = 4KB << 10 = 4MB */
#define PMM_MAX_PAGES     (1u << 20)    /* support up to 4GB RAM */
#define PMM_BITMAP_WORDS  (PMM_MAX_PAGES / 64u)

/* =========================================================================
 * Buddy Free-List Node (embedded in free page)
 * ========================================================================= */
typedef struct BuddyNode {
    struct BuddyNode* prev;
    struct BuddyNode* next;
} BuddyNode;

/* =========================================================================
 * PMM State
 * ========================================================================= */
typedef struct SigmaPMM {
    BuddyNode  free_lists[PMM_MAX_ORDER + 1]; /* sentinel heads */
    u64        bitmap[PMM_BITMAP_WORDS];       /* 1=allocated, 0=free */
    u64        total_pages;
    u64        free_pages;
    u64        alloc_calls;
    u64        free_calls;
} SigmaPMM;

static SigmaPMM g_pmm;

/* =========================================================================
 * Internal: bitmap helpers
 * ========================================================================= */
static void bitmap_set(u64 pfn) {
    g_pmm.bitmap[pfn / 64] |=  (1ULL << (pfn % 64));
}
static void bitmap_clr(u64 pfn) {
    g_pmm.bitmap[pfn / 64] &= ~(1ULL << (pfn % 64));
}
static bool_t bitmap_get(u64 pfn) {
    return !!(g_pmm.bitmap[pfn / 64] & (1ULL << (pfn % 64)));
}

/* =========================================================================
 * Internal: free-list helpers
 * ========================================================================= */
static void fl_push(u32 order, paddr_t paddr) {
    BuddyNode* node = (BuddyNode*)(usize)paddr;
    BuddyNode* head = &g_pmm.free_lists[order];
    node->next = head->next;
    node->prev = head;
    if (head->next) head->next->prev = node;
    head->next = node;
}

static paddr_t fl_pop(u32 order) {
    BuddyNode* head = &g_pmm.free_lists[order];
    if (!head->next) return 0;
    BuddyNode* node = head->next;
    head->next = node->next;
    if (node->next) node->next->prev = head;
    return (paddr_t)(usize)node;
}

static void fl_remove(u32 order, paddr_t paddr) {
    BuddyNode* node = (BuddyNode*)(usize)paddr;
    if (node->prev) node->prev->next = node->next;
    if (node->next) node->next->prev = node->prev;
}

/* =========================================================================
 * PMM Init — called from sigma_kernel_main with multiboot2 memory map
 * ========================================================================= */
k_status pmm_init(paddr_t mem_start, paddr_t mem_end) {
    u32 i;
    /* Init free-list sentinels */
    for (i = 0; i <= PMM_MAX_ORDER; i++) {
        g_pmm.free_lists[i].prev = NULL;
        g_pmm.free_lists[i].next = NULL;
    }
    /* Clear bitmap (all allocated by default) */
    for (i = 0; i < PMM_BITMAP_WORDS; i++) g_pmm.bitmap[i] = ~0ULL;

    g_pmm.total_pages = (mem_end - mem_start) / PAGE_SIZE;
    if (g_pmm.total_pages == 0) return K_ERR_INVAL;

    g_pmm.free_pages  = 0;
    g_pmm.alloc_calls = 0;
    g_pmm.free_calls  = 0;

    /* Free available pages in max-order blocks */
    paddr_t addr = ALIGN_UP(mem_start, PAGE_SIZE);
    while (addr + PAGE_SIZE <= mem_end) {
        /* Find highest order that fits */
        u32 order = PMM_MAX_ORDER;
        u64 block_size = (u64)PAGE_SIZE << order;
        while (order > 0 && (addr + block_size > mem_end ||
               (addr & (block_size - 1)))) {
            order--;
            block_size >>= 1;
        }
        /* Mark pages free in bitmap */
        u64 pages = (u64)1 << order;
        u64 pfn   = addr / PAGE_SIZE;
        u64 pi;
        for (pi = 0; pi < pages; pi++) bitmap_clr(pfn + pi);
        fl_push(order, addr);
        g_pmm.free_pages += pages;
        addr += block_size;
    }
    return K_OK;
}

/* =========================================================================
 * PMM Alloc — allocate 2^order pages, returns physical address or 0
 * ========================================================================= */
paddr_t pmm_alloc(u32 order) {
    if (order > PMM_MAX_ORDER) return 0;
    u32 o = order;
    /* Find smallest sufficient free block */
    while (o <= PMM_MAX_ORDER && !g_pmm.free_lists[o].next) o++;
    if (o > PMM_MAX_ORDER) return 0;

    paddr_t block = fl_pop(o);

    /* Split down to requested order */
    while (o > order) {
        o--;
        paddr_t buddy = block + ((u64)PAGE_SIZE << o);
        fl_push(o, buddy);
        /* Mark buddy's pages as free */
        u64 buddy_pfn = buddy / PAGE_SIZE;
        u64 pi;
        for (pi = 0; pi < (u64)(1u << o); pi++) bitmap_clr(buddy_pfn + pi);
    }

    /* Mark allocated pages */
    u64 pfn   = block / PAGE_SIZE;
    u64 pages = (u64)1 << order;
    u64 pi;
    for (pi = 0; pi < pages; pi++) bitmap_set(pfn + pi);
    g_pmm.free_pages  -= pages;
    g_pmm.alloc_calls++;
    return block;
}

/* Convenience: alloc single 4KB page */
paddr_t pmm_alloc_page(void) { return pmm_alloc(0); }

/* =========================================================================
 * PMM Free — free block at paddr of 2^order pages
 * ========================================================================= */
void pmm_free(paddr_t paddr, u32 order) {
    if (!paddr || order > PMM_MAX_ORDER) return;

    u64 pfn   = paddr / PAGE_SIZE;
    u64 pages = (u64)1 << order;
    u64 pi;
    for (pi = 0; pi < pages; pi++) bitmap_clr(pfn + pi);
    g_pmm.free_pages  += pages;
    g_pmm.free_calls++;

    /* Coalesce with buddy */
    while (order < PMM_MAX_ORDER) {
        paddr_t buddy = paddr ^ ((u64)PAGE_SIZE << order);
        u64 buddy_pfn = buddy / PAGE_SIZE;

        /* Check if entire buddy block is free */
        bool_t buddy_free = TRUE;
        for (pi = 0; pi < pages; pi++) {
            if (bitmap_get(buddy_pfn + pi)) { buddy_free = FALSE; break; }
        }
        if (!buddy_free) break;

        /* Remove buddy from its free list and merge */
        fl_remove(order, buddy);
        paddr = (paddr < buddy) ? paddr : buddy;
        order++;
        pages <<= 1;
    }
    fl_push(order, paddr);
}

void pmm_free_page(paddr_t pa) { pmm_free(pa, 0); }

/* =========================================================================
 * PMM Audit
 * ========================================================================= */
void pmm_audit(void) {
    /* kernel serial print shim */
    extern void ksigma_printf(const char* fmt, ...);
    ksigma_printf("[PMM]: Free=%llu/%llu pages | Allocs=%llu | Frees=%llu\n",
            g_pmm.free_pages, g_pmm.total_pages,
            g_pmm.alloc_calls, g_pmm.free_calls);
}
