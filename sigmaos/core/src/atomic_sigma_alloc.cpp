#include "libc/sigma_libc.h"

extern "C" {

// Custom Memory Pool (Static allocation to avoid dynamic runtime dependency)
#define SIGMA_POOL_SIZE 1048576 * 64 // 64 MB Pool
static sigma_u8 memory_pool[SIGMA_POOL_SIZE];
static sigma_size_t pool_offset = 0;

void* sigma_alloc(sigma_size_t size) {
    if (pool_offset + size > SIGMA_POOL_SIZE) {
        sigma_kprint("[SigmaAlloc] FATAL: Out of Silicon Memory Pool.\n");
        return 0; // Null pointer equivalent
    }
    
    void* ptr = &memory_pool[pool_offset];
    pool_offset += size;
    
    // Zero out allocated memory natively
    sigma_memset(ptr, 0, size);
    
    sigma_kprint("[SigmaAlloc] Allocated ");
    sigma_kprint_int(size);
    sigma_kprint(" bytes from Sovereign Pool.\n");
    
    return ptr;
}

void sigma_free(void* ptr) {
    // Basic arena allocator doesn't support individual frees
    // Future expansion: Add block tracking
    sigma_kprint("[SigmaAlloc] Memory free requested (Arena mode active).\n");
}

}

} // extern "C"
