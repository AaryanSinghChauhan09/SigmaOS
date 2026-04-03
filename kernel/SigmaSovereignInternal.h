/* 
 Σ SIGMAOS ZENITH: SOVEREIGN INTERNAL UTILS (v2300.0)
 Mission: SSE-Accelerated Silicon Performance.
*/

#ifndef SIGMA_INTERNAL_H
#define SIGMA_INTERNAL_H

#include <stdint.h>
#include <stdbool.h>

// Σ BARE-METAL PRINTS
void sigma_print(const char* s);

// Σ SSE-ACCELERATED MEMORY SHARD (v2300.0)
inline void* sigma_memcpy_sse(void* dest, const void* src, uint32_t n) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;
    while (n >= 16) {
        // Σ SSE-Direct Block Transfer
        __asm__ volatile (
            "movups (%0), %%xmm0\n"
            "movups %%xmm0, (%1)\n"
            :: "r"(s), "r"(d) : "xmm0", "memory"
        );
        d += 16;
        s += 16;
        n -= 16;
    }
    // Handle alignment fragments (linear byte pass)
    while (n--) *d++ = *s++;
    return dest;
}

// Σ UTILS: STRING SHARDS
char* sigma_strcpy(char* dest, const char* src);
const char* sigma_strstr(const char* str, const char* substr);

#endif
