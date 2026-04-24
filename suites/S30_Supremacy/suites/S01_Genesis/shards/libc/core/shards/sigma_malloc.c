#include "sigma_kernel.h"
static void* g_heap_start = SIGMA_NULL;
static sigma_sz_t g_heap_used = 0;
static const sigma_sz_t HEAP_SIZE = 1024 * 1024 * 128;
void* sigma_sigma_malloc(sigma_sz_t size) {
    if (g_heap_start == SIGMA_NULL) {
        g_heap_start = sigma_mmap(SIGMA_NULL, HEAP_SIZE, 3, 0x22, -1, 0);
        if (g_heap_start == (void*)-1) { g_heap_start = SIGMA_NULL; return SIGMA_NULL; }
    }
    if (g_heap_used + size > HEAP_SIZE) return SIGMA_NULL;
    void* ptr = (sigma_u8*)g_heap_start + g_heap_used;
    g_heap_used += size;
    return ptr;
}
void sigma_sigma_free(void* ptr) { (void)ptr; }
