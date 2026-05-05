/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-LIBC (Zero-Dependency)
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

/* sigma_memcpy, sigma_memset, sigma_strlen are implemented in SovereignLibC.asm */


#ifndef SIGMA_STRCMP_DEFINED
/* sigma_strcmp in ASM */
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




