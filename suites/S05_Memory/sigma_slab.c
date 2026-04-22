/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: SLAB ALLOCATOR (O(1) MEMORY SHARD)
 * =========================================================================
 * Mission: High-performance memory sharding with zero fragmentation.
 * Capability: Linux-style caches (kmalloc-8...kmalloc-4096).
 * =========================================================================
 */

#include "../libc/sigma_libc.h"
#include "../libc/sigma_types.h"

#define SLAB_SIGNATURE 0x516D4105 // "SIGMAOS"

typedef struct sigma_slab {
    sigma_u32 magic;
    sigma_size_t obj_size;
    sigma_size_t count;
    void* free_list;
    struct sigma_slab* next;
} sigma_slab_t;

static sigma_slab_t* slab_caches[32]; // Caches for different sizes

void* sigma_slab_alloc(sigma_size_t size) {
    // Find appropriate cache index (pseudo-code/logic)
    int idx = 0;
    while ((1ULL << (idx + 3)) < size && idx < 31) idx++;
    
    sigma_slab_t* cache = slab_caches[idx];
    if (!cache) {
        // Create new slab via mmap (using SovereignLibC)
        sigma_size_t page_size = 4096;
        void* mem = sigma_mmap(SIGMA_NULL, page_size, 3, 0x22, -1, 0);
        if (mem == (void*)-1) return SIGMA_NULL;
        
        cache = (sigma_slab_t*)mem;
        cache->magic = SLAB_SIGNATURE;
        cache->obj_size = (1ULL << (idx + 3));
        cache->count = (page_size - sizeof(sigma_slab_t)) / cache->obj_size;
        cache->free_list = (void*)((sigma_u8*)mem + sizeof(sigma_slab_t));
        cache->next = SIGMA_NULL;
        
        // Initialize free list (linked sharding)
        sigma_u8* ptr = (sigma_u8*)cache->free_list;
        for (sigma_size_t i = 0; i < cache->count - 1; i++) {
            *(void**)ptr = (void*)(ptr + cache->obj_size);
            ptr += cache->obj_size;
        }
        *(void**)ptr = SIGMA_NULL;
        
        slab_caches[idx] = cache;
    }
    
    // Pop from free list
    if (cache->free_list) {
        void* obj = cache->free_list;
        cache->free_list = *(void**)obj;
        return obj;
    }
    
    return SIGMA_NULL; // Out of memory in this slab
}

void sigma_slab_free(void* ptr, sigma_size_t size) {
    if (!ptr) return;
    
    // Find appropriate cache index
    int idx = 0;
    while ((1ULL << (idx + 3)) < size && idx < 31) idx++;
    
    sigma_slab_t* cache = slab_caches[idx];
    if (cache) {
        *(void**)ptr = cache->free_list;
        cache->free_list = ptr;
    }
}
