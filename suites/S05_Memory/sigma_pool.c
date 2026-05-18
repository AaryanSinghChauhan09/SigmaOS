#include "libc/SovereignLibC.h"
/*
 * =============================================================================
 * Σ SIGMAOS: PER-MODULE MEMORY POOL DELEGATION (v1.0)
 * =============================================================================
 * Gives each Lattice Shard its own dedicated slab pool.
 * If one module leaks or crashes, it cannot exhaust the memory of another.
 *
 * Design:
 *   - Each pool owns a contiguous range of pages obtained via vmalloc().
 *   - Pools are registered by name (e.g., "S07_Network", "S08_Security").
 *   - Allocations within a pool are O(1) slab-style.
 *   - Pool exhaustion triggers a per-pool OOM handler, NOT a kernel panic.
 *   - Audit functions report per-pool usage for observability.
 *
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_features.h"

#if SIGMA_FEATURE_SLAB_POOLS

/* =========================================================================
 * External dependencies
 * ========================================================================= */

extern vaddr_t vmalloc(u64 npages);
extern void    ksigma_printf(const char* fmt, ...);

/* =========================================================================
 * Pool structures
 * ========================================================================= */

#define POOL_NAME_MAX   32
#define POOL_MAGIC      0x504F4F4CUL  /* "POOL" */

typedef void (*pool_oom_handler_t)(const char* pool_name, u64 requested);

typedef struct SigmaPoolBlock {
    struct SigmaPoolBlock* next;
} SigmaPoolBlock;

typedef struct SigmaMemPool {
    u32             magic;
    char            name[POOL_NAME_MAX];
    vaddr_t         base;               /* start of pool region */
    u64             total_pages;
    u64             block_size;          /* allocation granularity */
    u64             total_blocks;
    u64             used_blocks;
    u64             peak_blocks;         /* high-water mark */
    u64             alloc_calls;
    u64             free_calls;
    SigmaPoolBlock* free_list;
    pool_oom_handler_t oom_handler;
    bool_t          active;
} SigmaMemPool;

static SigmaMemPool g_pools[SIGMA_MAX_SLAB_POOLS];
static u32 g_pool_count = 0;

/* =========================================================================
 * Pool Creation
 * ========================================================================= */

/**
 * sigma_pool_create — Allocate a new isolated memory pool.
 *
 * @param name          Human-readable owner name (e.g., "S07_Network").
 * @param npages        Number of 4KB pages to reserve.
 * @param block_size    Allocation granularity in bytes (must be ≥ sizeof(void*)).
 * @return              Pool ID (≥ 0) on success, -1 on failure.
 */
int sigma_pool_create(const char* name, u64 npages, u64 block_size) {
    if (g_pool_count >= SIGMA_MAX_SLAB_POOLS) return -1;
    if (block_size < sizeof(SigmaPoolBlock))
        block_size = sizeof(SigmaPoolBlock);

    /* Allocate backing pages */
    vaddr_t base = vmalloc(npages);
    if (!base) return -1;

    SigmaMemPool* p = &g_pools[g_pool_count];
    p->magic        = POOL_MAGIC;
    p->base         = base;
    p->total_pages  = npages;
    p->block_size   = block_size;
    p->total_blocks = (npages * PAGE_SIZE) / block_size;
    p->used_blocks  = 0;
    p->peak_blocks  = 0;
    p->alloc_calls  = 0;
    p->free_calls   = 0;
    p->oom_handler  = (pool_oom_handler_t)0;
    p->active       = TRUE;

    /* Copy name */
    u32 i;
    for (i = 0; i < POOL_NAME_MAX - 1 && name[i]; i++)
        p->name[i] = name[i];
    p->name[i] = '\0';

    /* Build free list */
    p->free_list = (SigmaPoolBlock*)(usize)base;
    u8* cursor = (u8*)(usize)base;
    u64 b;
    for (b = 0; b < p->total_blocks - 1; b++) {
        SigmaPoolBlock* blk = (SigmaPoolBlock*)(cursor + b * block_size);
        blk->next = (SigmaPoolBlock*)(cursor + (b + 1) * block_size);
    }
    SigmaPoolBlock* last = (SigmaPoolBlock*)(cursor + (p->total_blocks - 1) * block_size);
    last->next = (SigmaPoolBlock*)0;

    int id = (int)g_pool_count;
    g_pool_count++;
    return id;
}

/* =========================================================================
 * Per-Pool Allocation / Free
 * ========================================================================= */

void* sigma_pool_alloc(int pool_id) {
    if (pool_id < 0 || (u32)pool_id >= g_pool_count) return (void*)0;
    SigmaMemPool* p = &g_pools[pool_id];
    if (p->magic != POOL_MAGIC || !p->active) return (void*)0;

    if (!p->free_list) {
        /* Pool exhausted — invoke per-pool OOM instead of kernel panic */
        if (p->oom_handler)
            p->oom_handler(p->name, p->block_size);
        return (void*)0;
    }

    SigmaPoolBlock* blk = p->free_list;
    p->free_list = blk->next;
    p->used_blocks++;
    p->alloc_calls++;
    if (p->used_blocks > p->peak_blocks)
        p->peak_blocks = p->used_blocks;
    return (void*)blk;
}

void sigma_pool_free(int pool_id, void* ptr) {
    if (pool_id < 0 || (u32)pool_id >= g_pool_count || !ptr) return;
    SigmaMemPool* p = &g_pools[pool_id];
    if (p->magic != POOL_MAGIC) return;

    /* Bounds check: ptr must fall within [base, base + total_pages * PAGE_SIZE) */
    vaddr_t addr = (vaddr_t)(usize)ptr;
    vaddr_t end  = p->base + p->total_pages * PAGE_SIZE;
    if (addr < p->base || addr >= end) return;  /* foreign pointer — reject */

    SigmaPoolBlock* blk = (SigmaPoolBlock*)ptr;
    blk->next = p->free_list;
    p->free_list = blk;
    p->used_blocks--;
    p->free_calls++;
}

/* =========================================================================
 * OOM Handler Registration
 * ========================================================================= */

void sigma_pool_set_oom(int pool_id, pool_oom_handler_t handler) {
    if (pool_id < 0 || (u32)pool_id >= g_pool_count) return;
    g_pools[pool_id].oom_handler = handler;
}

/* =========================================================================
 * Audit / Observability
 * ========================================================================= */

void sigma_pool_audit(int pool_id) {
    if (pool_id < 0 || (u32)pool_id >= g_pool_count) return;
    const SigmaMemPool* p = &g_pools[pool_id];
    ksigma_printf("[POOL:%s] Blocks: %llu/%llu (peak %llu) | Allocs: %llu | Frees: %llu\n",
            p->name, p->used_blocks, p->total_blocks, p->peak_blocks,
            p->alloc_calls, p->free_calls);
}

void sigma_pool_audit_all(void) {
    ksigma_printf("[POOL] === Sovereign Memory Pool Audit (%u pools) ===\n", g_pool_count);
    u32 i;
    for (i = 0; i < g_pool_count; i++) {
        sigma_pool_audit((int)i);
    }
}

/* =========================================================================
 * Pool Destruction (for teardown / hot-unload)
 * ========================================================================= */

k_status sigma_pool_destroy(int pool_id) {
    if (pool_id < 0 || (u32)pool_id >= g_pool_count) return K_ERR_INVAL;
    SigmaMemPool* p = &g_pools[pool_id];
    if (p->used_blocks > 0) return K_ERR_INVAL;  /* cannot destroy while in use */
    p->active = FALSE;
    p->magic  = 0;
    /* Pages would be freed via vmfree() when implemented */
    return K_OK;
}

#endif /* SIGMA_FEATURE_SLAB_POOLS */
