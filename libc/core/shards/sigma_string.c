#include "sigma_kernel.h"
int sigma_streq(const char* s1, const char* s2) {
    sigma_size_t i = 0;
    while(s1[i] != '\0' && s2[i] != '\0') {
        if(s1[i] != s2[i]) return SIGMA_FALSE;
        i++;
    }
    return (s1[i] == s2[i]) ? SIGMA_TRUE : SIGMA_FALSE;
}
int sigma_compare(const char* s1, const char* s2) { return sigma_streq(s1, s2); }
void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
    return dest;
}
int sigma_memcmp(const void* s1, const void* s2, sigma_size_t n) {
    const sigma_u8* p1 = (const sigma_u8*)s1;
    const sigma_u8* p2 = (const sigma_u8*)s2;
    while (n--) { if (*p1 != *p2) return (int)(*p1 - *p2); p1++; p2++; }
    return 0;
}
void* sigma_memmove(void* dest, const void* src, sigma_size_t n) {
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    if (d < s) { while (n--) *d++ = *s++; }
    else { d += n; s += n; while (n--) *--d = *--s; }
    return dest;
}
void sigma_strcat(char* dest, const char* src) {
    char* rd = dest; while (*rd) rd++;
    while (*src) { *rd = *src; rd++; src++; }
    *rd = '\0';
}
