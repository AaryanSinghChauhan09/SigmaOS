#ifndef SOVEREIGN_LIBC_H
#define SOVEREIGN_LIBC_H

// SigmaOS Sovereign Core LibC (S-LIBC)
// Philosophy: Musl/Bionic - Minimal, performant, and zero-dependency.
// USP: Eliminates reliance on host toolchain headers.

typedef unsigned long size_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;

void* sigma_memset(void* s, int c, size_t n) {
    unsigned char* p = s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}

void* sigma_memcpy(void* dest, const void* src, size_t n) {
    unsigned char* d = dest;
    const unsigned char* s = src;
    while (n--) *d++ = *s++;
    return dest;
}

size_t sigma_strlen(const char* s) {
    size_t len = 0;
    while (s[len]) len++;
    return len;
}

#endif
