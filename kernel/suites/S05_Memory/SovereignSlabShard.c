#include "../../../include/SovereignMemory.h"
#include "../../../include/sigma_kernel.h"
#include "../../../libc/sigma_libc.h"

#define SLAB_SIGNATURE 0x516D4105

typedef struct sigma_slab {
    sigma_u32 magic;
    sigma_size_t obj_size;
    sigma_size_t count;
    void* free_list;
    struct sigma_slab* next;
} sigma_slab_t;

static sigma_slab_t* slab_caches[32];

void* sigma_slab_malloc_ext(sigma_size_t size) {
    int idx = 0;
    while ((1ULL << (idx + 3)) < size && idx < 31) idx++;
    
    sigma_slab_t* cache = slab_caches[idx];
    if (!cache) {
        sigma_size_t page_size = 4096;
        void* mem = sigma_mmap(SIGMA_NULL, page_size, 3, 0x22, -1, 0);
        if (mem == (void*)-1) return SIGMA_NULL;
        
        cache = (sigma_slab_t*)mem;
        cache->magic = SLAB_SIGNATURE;
        cache->obj_size = (1ULL << (idx + 3));
        cache->count = (page_size - sizeof(sigma_slab_t)) / cache->obj_size;
        cache->free_list = (void*)((sigma_u8*)mem + sizeof(sigma_slab_t));
        
        sigma_u8* ptr = (sigma_u8*)cache->free_list;
        for (sigma_size_t i = 0; i < cache->count - 1; i++) {
            *(void**)ptr = (void*)(ptr + cache->obj_size);
            ptr += cache->obj_size;
        }
        *(void**)ptr = SIGMA_NULL;
        slab_caches[idx] = cache;
    }
    
    if (cache->free_list) {
        void* obj = cache->free_list;
        cache->free_list = *(void**)obj;
        return obj;
    }
    return SIGMA_NULL;
}

void sigma_slab_free_ext(void* ptr, sigma_size_t size) {
    int idx = 0;
    while ((1ULL << (idx + 3)) < size && idx < 31) idx++;
    sigma_slab_t* cache = slab_caches[idx];
    if (cache) {
        *(void**)ptr = cache->free_list;
        cache->free_list = ptr;
    }
}

void SovereignSlab_Register(void) {
    SovereignMemory_Register("slab", sigma_slab_malloc_ext, sigma_slab_free_ext);
}
