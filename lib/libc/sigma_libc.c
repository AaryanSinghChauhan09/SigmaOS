/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-LIBC (Zero-Dependency)
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

void sigma_memcpy(void* dest, const void* src, sigma_u32 n) {
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
}

void sigma_memset(void* s, sigma_u8 c, sigma_u32 n) {
    sigma_u8* p = (sigma_u8*)s;
    while (n--) *p++ = c;
}

sigma_size_t sigma_strlen(const char* s) {
    sigma_size_t len = 0;
    while (s[len]) len++;
    return len;
}

#ifndef SIGMA_STRCMP_DEFINED
int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++; s2++;
    }
    return *(sigma_u8*)s1 - *(sigma_u8*)s2;
}
#endif

void sigma_strncpy(char* dest, const char* src, sigma_u32 n) {
    sigma_u32 i;
    for (i = 0; i < n - 1 && src[i] != '\0'; i++) dest[i] = src[i];
    dest[i] = '\0';
}

char* sigma_strstr(const char* haystack, const char* needle) {
    if (!*needle) return (char*)haystack;
    for (; *haystack; haystack++) {
        if (*haystack == *needle) {
            const char *h = haystack, *n = needle;
            while (*h && *n && *h == *n) {
                h++; n++;
            }
            if (!*n) return (char*)haystack;
        }
    }
    return (char*)0;
}

char* sigma_hardened_strstr(const char* haystack, const char* needle) {
    return sigma_strstr(haystack, needle);
}

