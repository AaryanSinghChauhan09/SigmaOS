#include "../../include/libc/SovereignLibC.h"
/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MEMORY ZENITH (v94.0 ZENITH SUPREME)
 * =========================================================================
 * Mission: Absolute Memory Sovereignty via Direct Hardware Control.
 * Principles:
 *   - Slab: High-speed fixed-size object allocation (bump-pointer).
 *   - No Libraries: Zero sigma_malloc()/sigma_free() from external libc.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =========================================================================
 */

#include "../../include/libc/sigma_libc.h"

#define MEM_SEG_MAX      1024u
#define MEM_POOL_SIZE    (128ULL * 1024ULL * 1024ULL) /* 128 MB shard */

typedef struct MemorySegment {
    sigma_u64  start_addr;
    sigma_u64  size;
    sigma_bool allocated;
} MemorySegment;

typedef struct SovereignMemoryManager {
    sigma_u8*     pool;
    sigma_size_t  used;
    MemorySegment segments[MEM_SEG_MAX];
    sigma_size_t  segment_count;
    sigma_u64     alloc_calls;
    sigma_u64     free_calls;
} SovereignMemoryManager;

static SovereignMemoryManager g_mem_manager;

void sigma_mem_audit(void) {
    sigma_printf("\n--- Σ SOVEREIGN MEMORY AUDIT (v94.0) ---\n");
    sigma_printf("| Total Pool     : %u MB\n", 
                 (unsigned int)(MEM_POOL_SIZE / 1024u / 1024u));
    sigma_printf("| Used Space     : %u KB\n", 
                 (unsigned int)(g_mem_manager.used / 1024u));
    sigma_printf("| Managed Shards : %llu\n", (sigma_u64)g_mem_manager.segment_count);
    sigma_printf("| Competitors    : jemalloc/ptmalloc neutralized.\n");
    sigma_printf("-----------------------------------------\n");
}

void* sigma_mem_allocate(sigma_size_t size) {
    if ((g_mem_manager.used + size) > MEM_POOL_SIZE) return SIGMA_NULL;
    if (g_mem_manager.segment_count >= MEM_SEG_MAX) return SIGMA_NULL;

    void* ptr = g_mem_manager.pool + g_mem_manager.used;

    g_mem_manager.segments[g_mem_manager.segment_count].start_addr = (sigma_u64)(sigma_size_t)ptr;
    g_mem_manager.segments[g_mem_manager.segment_count].size       = size;
    g_mem_manager.segments[g_mem_manager.segment_count].allocated  = SIGMA_TRUE;
    g_mem_manager.segment_count++;

    g_mem_manager.used += size;
    g_mem_manager.alloc_calls++;
    return ptr;
}

void sigma_mem_init(void) {
    sigma_printf("[MEM-ZENITH]: Mapping Raw Silicon Shard (128MB)...\n");
    
    /* Request a massive shard from the silicon */
    g_mem_manager.pool = (sigma_u8*)sigma_slab_alloc_raw(MEM_POOL_SIZE);
    if (!g_mem_manager.pool) {
        sigma_printf("[ERROR]: Failed to map sovereign heap shard.\n");
        return;
    }
    
    g_mem_manager.used          = 0;
    g_mem_manager.segment_count = 0;
    
    sigma_printf("[MEM-ZENITH]: Memory Shard Online at %p\n", g_mem_manager.pool);
}
