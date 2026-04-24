/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN STRING SHARD (v1.0 - PURE C11)
 * =========================================================================
 */

#include "sigma_libc.h"

sigma_sz_t sigma_sigma_strlen(const char* s) {
    if (!s) return 0;
    sigma_sz_t len = 0;
    while (s[len]) len++;
    return len;
}

int sigma_streq(const char* s1, const char* s2) {
    if (!s1 || !s2) return 0;
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return (*(unsigned char*)s1 - *(unsigned char*)s2) == 0;
}

const char* sigma_strstr(const char* haystack, const char* needle) {
    if (!*needle) return haystack;
    for (; *haystack; haystack++) {
        if (*haystack == *needle) {
            const char *h, *n;
            for (h = haystack, n = needle; *h && *n && *h == *n; h++, n++);
            if (!*n) return haystack;
        }
    }
    return SIGMA_NULL;
}
char* sigma_strncpy(char* dest, const char* src, sigma_sz_t n) {
    sigma_sz_t i;
    for (i = 0; i < n && src[i] != '\0'; i++)
        dest[i] = src[i];
    for (; i < n; i++)
        dest[i] = '\0';
    return dest;
}

char* sigma_sigma_strcpy(char* dest, const char* src) {
    char* d = dest;
    while ((*d++ = *src++));
    return dest;
}

int sigma_sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}
