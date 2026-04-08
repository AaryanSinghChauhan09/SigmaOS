/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MEMORY SHARD (v1.0 - PURE C11)
 * =========================================================================
 */

#include "sigma_libc.h"

void* sigma_memset(void* s, int c, sigma_size_t n) {
    unsigned char* p = (unsigned char*)s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}

void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
    unsigned char* d = (unsigned char*)dest;
    const unsigned char* s = (const unsigned char*)src;
    while (n--) *d++ = *s++;
    return dest;
}

void* sigma_malloc(sigma_size_t size) {
    // Mission: Minimal slab allocator logic placeholder (actual logic in SovereignMemoryZenith)
    // For now, we use a simple static buffer or mmap syscall
    (void)size;
    return SIGMA_NULL; 
}

void sigma_free(void* ptr) {
    (void)ptr;
}
