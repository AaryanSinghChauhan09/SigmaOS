#include "SovereignLibC.h"

static void*       g_heap_start = SIGMA_NULL;
static sigma_size_t g_heap_used  = 0;

#define SIGMA_HEAP_SIZE (128ULL * 1024ULL * 1024ULL)

void* sigma_slab_alloc_raw(sigma_size_t size) {
    if (g_heap_start == SIGMA_NULL) {
        g_heap_start = sigma_mmap(SIGMA_NULL, SIGMA_HEAP_SIZE, 3, 0x22, -1, 0);
    }
    if (g_heap_used + size > SIGMA_HEAP_SIZE) return SIGMA_NULL;
    void* ptr = (void*)((sigma_u8*)g_heap_start + g_heap_used);
    g_heap_used += size;
    return ptr;
}

void* sigma_malloc(sigma_size_t size) {
    return sigma_slab_alloc_raw(size);
}

void sigma_free(void* ptr) {
    (void)ptr;
}
