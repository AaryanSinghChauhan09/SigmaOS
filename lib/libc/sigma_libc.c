/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-LIBC (Zero-Dependency)
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

void sigma_memcpy(void* dest, const void* src, u32 n) {
    u8* d = (u8*)dest;
    const u8* s = (const u8*)src;
    while (n--) *d++ = *s++;
}

void sigma_memset(void* s, u8 c, u32 n) {
    u8* p = (u8*)s;
    while (n--) *p++ = c;
}

u32 sigma_strlen(const char* s) {
    u32 len = 0;
    while (s[len]) len++;
    return len;
}

int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++; s2++;
    }
    return *(u8*)s1 - *(u8*)s2;
}

void sigma_strncpy(char* dest, const char* src, u32 n) {
    u32 i;
    for (i = 0; i < n - 1 && src[i] != '\0'; i++) dest[i] = src[i];
    dest[i] = '\0';
}
