// =============================================================================
// SigmaOS  kernel/core/libc  SovereignLibC.cpp  v2.0
// Industrial LibC implementation for Ring-0 stability.
// =============================================================================
#include "libc/SovereignLibC.h"
#include "sigma_log.h"
#include "core/sigma_types.h"

extern "C" {

/* FIX: Infinite recursion fix - use sigma_log directly or define internal logger */
void sigma_log_info_industrial(const char* format, ...) {
    /* Direct kernel console output simulation */
    (void)format;
    sigma_log("[LIBC] Industrial log dispatch.");
}

void* sigma_mmap(void* addr, sigma_u64 length, int prot, int flags, int fd, sigma_u64 offset) {
    (void)addr; (void)length; (void)prot; (void)flags; (void)fd; (void)offset;
    sigma_log("[LIBC] mmap() -> 0xDEADBEEF (Stub)");
    return (void*)0xDEADBEEF; 
}

void sigma_print(const char* msg) {
    if (msg) {
        sigma_log(msg);
    }
}

void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
    return dest;
}

void* sigma_memset(void* s, int c, sigma_size_t n) {
    sigma_u8* p = (sigma_u8*)s;
    while (n--) *p++ = (sigma_u8)c;
    return s;
}

/* Canonical strcmp implementation */
int sigma_hardened_strcmp(const char* s1, const char* s2) {
    if (!s1 || !s2) return (s1 == s2) ? 0 : (s1 ? 1 : -1);
    while (*s1 && (*s1 == *s2)) {
        s1++; s2++;
    }
    return *(const sigma_u8*)s1 - *(const sigma_u8*)s2;
}

/* Alias for compatibility */
int sigma_strcmp(const char* s1, const char* s2) {
    return sigma_hardened_strcmp(s1, s2);
}

sigma_size_t sigma_strlen(const char* s) {
    sigma_size_t len = 0;
    if (!s) return 0;
    while (s[len]) len++;
    return len;
}

void sigma_hardened_strcpy(char* dest, const char* src, sigma_size_t n) {
    if (!dest || !src) return;
    sigma_size_t i;
    for (i = 0; i < n && src[i] != '\0'; i++)
        dest[i] = src[i];
    for (; i < n; i++)
        dest[i] = '\0';
}

sigma_u32 sigma_crc32(const void* data, sigma_size_t n) {
    const sigma_u8* p = (const sigma_u8*)data;
    sigma_u32 crc = 0xFFFFFFFF;
    for (sigma_size_t i = 0; i < n; i++) {
        sigma_u8 ch = p[i];
        for (int j = 0; j < 8; j++) {
            sigma_u32 b = (sigma_u32)((ch ^ crc) & 1);
            crc >>= 1;
            if (b) crc ^= 0xEDB88320;
            ch >>= 1;
        }
    }
    return ~crc;
}

} // extern "C"
