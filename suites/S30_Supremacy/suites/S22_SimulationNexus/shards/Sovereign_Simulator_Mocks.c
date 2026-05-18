#include "libc/SovereignLibC.h"
/**
 * @file Sovereign_Simulator_Mocks.c
 * @brief Zero-Dependency Simulation Mocks.
 */

#include "libc/sigma_libc.h"
#include "sigma_string.h"

/* Redirecting simulate.c mocks to Sovereign Atoms */
void sigma_printf_sim(const char* format, ...) {
    // Uses our atomic sigma_printf
    // (Actual va_list handling logic implemented in sigma_printf shard)
}

void* sigma_malloc_sim(sigma_sz_t size) { 
    return sigma_slab_alloc_raw(size); 
}

void sigma_free_sim(void* ptr) { 
    sigma_sigma_free(ptr); 
}

void sigma_memset_sim(void* s, sigma_u8 c, sigma_sz_t n) { 
    sigma_sigma_memset(s, c, n); 
}

void sigma_memcpy_sim(void* d, const void* s, sigma_sz_t n) { 
    sigma_sigma_memcpy(d, s, n); 
}
