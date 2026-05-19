/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SLAB ALLOCATOR
 * =============================================================================
 * Inspired by: Linux kernel mm/slab.c (and slub.c)
 *              FreeBSD sys/kern/subr_uma.c
 * =============================================================================
 * High-performance object-caching memory allocator for kernel structures.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define MAX_SLAB_CACHES 32
#define SLAB_PAGE_SIZE  4096

typedef struct sigma_slab {
    struct sigma_slab* next;
    void*  page_base;
    sigma_u32 free_count;
    sigma_u32 next_free_idx;
    sigma_u32 bitmask[4]; /* Up to 128 objects per slab for simulation */
} sigma_slab_t;

typedef struct {
    char name[32];
    sigma_u32 obj_size;
    sigma_u32 objs_per_slab;
    sigma_slab_t* partial_slabs;
    sigma_slab_t* full_slabs;
    sigma_bool active;
} sigma_kmem_cache_t;

static sigma_kmem_cache_t cache_table[MAX_SLAB_CACHES];
static sigma_u8 dummy_slab_memory[1024 * 1024]; /* 1MB dummy memory for simulation */
static sigma_u32 dummy_mem_offset = 0;

void slab_allocator_init(void) {
    sigma_memset(cache_table, 0, sizeof(cache_table));
    sigma_printf("[slab] Kernel SLAB Allocator initialized\n");
}

int kmem_cache_create(const char* name, sigma_u32 size) {
    if (size == 0 || size > SLAB_PAGE_SIZE / 2) return -1;
    
    for (sigma_u32 i = 0; i < MAX_SLAB_CACHES; i++) {
        if (!cache_table[i].active) {
            sigma_strcpy(cache_table[i].name, name, 32);
            cache_table[i].obj_size = size;
            cache_table[i].objs_per_slab = SLAB_PAGE_SIZE / size;
            cache_table[i].partial_slabs = SIGMA_NULL;
            cache_table[i].full_slabs = SIGMA_NULL;
            cache_table[i].active = SIGMA_TRUE;
            
            sigma_printf("[slab] Created cache '%s' (Size: %u, Objs/Slab: %u)\n", 
                         name, size, cache_table[i].objs_per_slab);
            return (int)i;
        }
    }
    return -1;
}

void* kmem_cache_alloc(sigma_u32 cache_id) {
    if (cache_id >= MAX_SLAB_CACHES || !cache_table[cache_id].active) return SIGMA_NULL;
    sigma_kmem_cache_t* cache = &cache_table[cache_id];
    
    /* Simulate allocation from partial slabs (simplified) */
    if (dummy_mem_offset + cache->obj_size <= sizeof(dummy_slab_memory)) {
        void* obj = &dummy_slab_memory[dummy_mem_offset];
        dummy_mem_offset += cache->obj_size;
        
        /* Align offset to next 8-byte boundary */
        dummy_mem_offset = (dummy_mem_offset + 7) & ~7;
        
        sigma_printf("[slab] Allocating from '%s' -> %p\n", cache->name, obj);
        return obj;
    }
    
    sigma_printf("[slab] ERR: Out of memory for cache '%s'\n", cache->name);
    return SIGMA_NULL;
}

void kmem_cache_free(sigma_u32 cache_id, void* obj) {
    if (cache_id >= MAX_SLAB_CACHES || !cache_table[cache_id].active) return;
    sigma_printf("[slab] Freeing object %p to '%s'\n", obj, cache_table[cache_id].name);
    /* Simulated free */
}
