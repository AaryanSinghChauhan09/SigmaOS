#include "libc/SovereignLibC.h"
/*
 * =============================================================================
 * Σ SIGMAOS: DMA COHERENT BUFFER ALLOCATOR (v1.0)
 * =============================================================================
 * Provides physically contiguous, cache-coherent memory buffers for 
 * hardware accelerators (GPUs, NPUs, DMA controllers).
 *
 * Design:
 *   - Reserves a dedicated region of physical memory at boot (e.g. upper RAM).
 *   - Ensures 4KB page alignment (required by most DMA engines).
 *   - Marks the virtual mappings as Uncached/Device memory in the MMU to 
 *     prevent CPU cache from desyncing with hardware DMA writes.
 *
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_features.h"

/* =========================================================================
 * DMA Memory Pool State
 * ========================================================================= */

#define DMA_POOL_SIZE       (16 * 1024 * 1024) /* 16 MB reserved for DMA */
#define PAGE_SIZE           4096

typedef struct DmaBlock {
    u32             size_pages;
    bool_t          is_free;
    struct DmaBlock* next;
} DmaBlock;

static vaddr_t g_dma_base_vaddr = 0;
static paddr_t g_dma_base_paddr = 0;
static DmaBlock* g_dma_free_list = (void*)0;

/* External VMM calls */
extern k_status vmm_map_uncached(vaddr_t va, paddr_t pa, u64 npages);
extern paddr_t pmm_alloc_contiguous(u64 npages);

/* =========================================================================
 * Initialization
 * ========================================================================= */

k_status sigma_dma_init(void) {
    extern void ksigma_printf(const char* fmt, ...);

    /* 1. Allocate a large, physically contiguous chunk of RAM from the PMM */
    u64 npages = DMA_POOL_SIZE / PAGE_SIZE;
    g_dma_base_paddr = pmm_alloc_contiguous(npages);
    if (!g_dma_base_paddr) {
        ksigma_printf("[DMA] Failed to allocate contiguous physical memory.\n");
        return K_ERR_NOMEM;
    }

    /* 2. Map into kernel virtual space as UNCACHED to ensure coherency */
    g_dma_base_vaddr = KERNEL_VMA + g_dma_base_paddr; 
    k_status res = vmm_map_uncached(g_dma_base_vaddr, g_dma_base_paddr, npages);
    if (res != K_OK) return res;

    /* 3. Initialize the free list (simple first-fit allocator for DMA blocks) */
    g_dma_free_list = (DmaBlock*)(usize)g_dma_base_vaddr;
    g_dma_free_list->size_pages = npages;
    g_dma_free_list->is_free = TRUE;
    g_dma_free_list->next = (void*)0;

    ksigma_printf("[DMA] Coherent Allocator initialized: %u MB at PADDR 0x%llx\n", 
            DMA_POOL_SIZE / (1024*1024), g_dma_base_paddr);

    return K_OK;
}

/* =========================================================================
 * Allocation (First-Fit)
 * ========================================================================= */

/**
 * sigma_dma_alloc - Allocate a DMA-coherent buffer.
 * 
 * @param size_bytes Requested size in bytes.
 * @param out_paddr  Pointer to receive the physical address (for hardware registers).
 * @return Virtual address pointer for CPU access, or SIGMA_NULL if out of memory.
 */
void* sigma_dma_alloc(u64 size_bytes, paddr_t* out_paddr) {
    if (size_bytes == 0 || !out_paddr) return (void*)0;

    /* Enforce strict PAGE_SIZE alignment */
    u64 npages = (size_bytes + PAGE_SIZE - 1) / PAGE_SIZE;
    if (npages == 0) npages = 1;

    DmaBlock* current = g_dma_free_list;
    while (current) {
        if (current->is_free && current->size_pages >= npages) {
            /* Split the block if there's leftover space */
            if (current->size_pages > npages + 1) { // +1 to fit the header
                u64 byte_offset = npages * PAGE_SIZE;
                DmaBlock* split = (DmaBlock*)((u8*)current + byte_offset);
                
                split->size_pages = current->size_pages - npages;
                split->is_free = TRUE;
                split->next = current->next;
                
                current->size_pages = npages;
                current->next = split;
            }

            current->is_free = FALSE;

            /* Calculate physical address for the hardware */
            vaddr_t va = (vaddr_t)(usize)current;
            u64 offset = va - g_dma_base_vaddr;
            *out_paddr = g_dma_base_paddr + offset;

            /* Hardware Architecture Specific Cache Coherency:
             * Although the region is mapped UNCACHED, we explicitly invalidate 
             * and clean the cache lines for this physical range on allocation to
             * guarantee no stale CPU cache entries from prior usages corrupt the NPU.
             */
#ifdef SIGMA_ARCH_AARCH64
            {
                u64 start = va;
                u64 end = va + (npages * PAGE_SIZE);
                // Clean and Invalidate Data Cache by Virtual Address to Point of Coherency
                while (start < end) {
                    __asm__ volatile("dc civac, %0" : : "r" (start) : "memory");
                    start += 64; // Assuming 64-byte cache line
                }
                __asm__ volatile("dsb sy" : : : "memory"); // Data Synchronization Barrier
            }
#endif

            /* Return virtual address for the CPU */
            return (void*)(usize)va;
        }
        current = current->next;
    }

    return (void*)0; /* OOM */
}

/* =========================================================================
 * Free
 * ========================================================================= */

void sigma_dma_free(void* vaddr) {
    if (!vaddr) return;

    DmaBlock* block = (DmaBlock*)vaddr;
    block->is_free = TRUE;

    /* Hardware Architecture Specific Cache Coherency:
     * Flush cache before returning to pool to ensure all CPU writes are flushed to RAM.
     */
#ifdef SIGMA_ARCH_AARCH64
    {
        u64 start = (u64)vaddr;
        u64 end = start + (block->size_pages * PAGE_SIZE);
        while (start < end) {
            __asm__ volatile("dc civac, %0" : : "r" (start) : "memory");
            start += 64;
        }
        __asm__ volatile("dsb sy" : : : "memory");
    }
#endif

    /* Coalesce contiguous free blocks */
    DmaBlock* current = g_dma_free_list;
    while (current && current->next) {
        if (current->is_free && current->next->is_free) {
            current->size_pages += current->next->size_pages;
            current->next = current->next->next;
        } else {
            current = current->next;
        }
    }
}
