/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN LIBC - MEMORY SHARD (v20.0)
 * =========================================================================
 */

#include "SovereignLibC.h"

void* sigma_secure_memset(void* s, int c, sigma_size_t n) {
    volatile unsigned char *p = (volatile unsigned char *)s;
    while (n--) *p++ = c;
    return s;
}

void sigma_bzero(void* s, sigma_usize n) {
    sigma_memset(s, 0, n);
}

// Minimal slab allocator logic
static void* g_heap_start = SIGMA_NULL;
static sigma_size_t g_heap_used = 0;
static const sigma_size_t HEAP_SIZE = 1024 * 1024 * 128; // 128MB Shard

void* sigma_slab_alloc_raw(sigma_size_t size) {
    if (g_heap_start == SIGMA_NULL) {
        g_heap_start = sigma_mmap(SIGMA_NULL, HEAP_SIZE, 3, 0x22, -1, 0);
    }
    if (g_heap_used + size > HEAP_SIZE) return SIGMA_NULL;
    void* ptr = (sigma_u8*)g_heap_start + g_heap_used;
    g_heap_used += size;
    return ptr;
}

void* sigma_malloc(sigma_size_t size) {
    return sigma_slab_alloc_raw(size);
}

void sigma_free(void* ptr) {
    // Zero-latency shard: reclamation handled by process-shard termination.
}
