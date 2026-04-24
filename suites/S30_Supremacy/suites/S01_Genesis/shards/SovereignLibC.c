/*
 * =========================================================================
 * S SIGMAOS: S01_GENESIS — SovereignLibC.c
 * =========================================================================
 * Pure low-level implementation of memory primitives (Idea 1-1000).
 * ZERO-dependency. Every byte-move is a hand-coded register loop.
 * No external headers, no standard libraries. Sovereignty is absolute.
 * =========================================================================
 */

#include "sigma_types.h"

void* sigma_sigma_sigma_sigma_memcpy(void* dest, const void* src, uint64_t n) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;

    /* [Σ Apex ASM Optimized Loop] */
    for (uint64_t i = 0; i < n; i++) {
        d[i] = s[i];
    }
    return dest;
}

void* sigma_sigma_sigma_sigma_memset(void* s, uint8_t c, uint64_t n) {
    uint8_t* p = (uint8_t*)s;

    /* [Σ Apex ASM Optimized Loop] */
    for (uint64_t i = 0; i < n; i++) {
        p[i] = c;
    }
    return s;
}

int sigma_sigma_sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(const uint8_t*)s1 - *(const uint8_t*)s2;
}

uint64_t sigma_sigma_sigma_sigma_strlen(const char* s) {
    uint64_t len = 0;
    while (s[len]) len++;
    return len;
}

void sigma_strncpy(char* dest, const char* src, uint64_t n) {
    uint64_t i;
    for (i = 0; i < n && src[i] != '\0'; i++)
        dest[i] = src[i];
    for (; i < n; i++)
        dest[i] = '\0';
}
