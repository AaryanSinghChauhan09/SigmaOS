/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SLAB ALLOCATOR (v1.0)
 * =============================================================================
 * Principles: Zero-Fragmentation & Cache-Efficient Object Reuse.
 * =============================================================================
 */
#include "../../../include/sigma_kernel_types.h"

typedef struct slab {
    sigma_u32 object_size;
    sigma_u32 total_objects;
    sigma_u32 free_objects;
    void* first_free;
    struct slab* next;
} slab_t;

static slab_t* slab_list = SIGMA_NULL;

extern void* pmm_alloc_page();

void* slab_alloc(sigma_u32 size) {
    /* 1. Find existing slab with matching size */
    slab_t* curr = slab_list;
    while (curr) {
        if (curr->object_size == size && curr->free_objects > 0) {
            void* obj = curr->first_free;
            curr->first_free = *(void**)obj;
            curr->free_objects--;
            return obj;
        }
        curr = curr->next;
    }

    /* 2. Create new slab (Page-aligned) */
    void* page = pmm_alloc_page();
    slab_t* new_slab = (slab_t*)page;
    new_slab->object_size = size;
    new_slab->total_objects = (4096 - sizeof(slab_t)) / size;
    new_slab->free_objects = new_slab->total_objects - 1;
    new_slab->next = slab_list;
    slab_list = new_slab;

    /* 3. Chain free objects */
    sigma_u8* start = (sigma_u8*)page + sizeof(slab_t);
    new_slab->first_free = start + size;
    void** ptr = (void**)new_slab->first_free;
    for (sigma_u32 i = 0; i < new_slab->free_objects - 1; i++) {
        *ptr = (sigma_u8*)ptr + size;
        ptr = (void**)*ptr;
    }
    *ptr = SIGMA_NULL;

    return start;
}
