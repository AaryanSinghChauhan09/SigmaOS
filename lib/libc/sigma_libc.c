/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-LIBC (Zero-Dependency)
 * =============================================================================
 */
#include "../../include/core/sigma_kernel_types.h"

void sigma_memcpy(void* dest, const void* src, sigma_u32 n) {
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
}

void sigma_memset(void* s, sigma_u8 c, sigma_u32 n) {
    sigma_u8* p = (sigma_u8*)s;
    while (n--) *p++ = c;
}

sigma_u32 sigma_strlen(const char* s) {
    sigma_u32 len = 0;
    while (s[len]) len++;
    return len;
}

int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++; s2++;
    }
    return *(sigma_u8*)s1 - *(sigma_u8*)s2;
}

void sigma_strncpy(char* dest, const char* src, sigma_u32 n) {
    sigma_u32 i;
    for (i = 0; i < n - 1 && src[i] != '\0'; i++) dest[i] = src[i];
    dest[i] = '\0';
}
