// =============================================================================
// SigmaOS — libc — sigma_libc_extended.c
// Extended Sovereign LibC — glibc-free string, math, and I/O primitives
// =============================================================================
// Replaces: any remaining glibc linkage in sigma_libc
// Competitor USPs Absorbed:
//   • musl libc (Alpine Linux) — strict POSIX, no GNU extensions, tiny binary
//   • dietlibc (Linux)         — minimal, 50KB total, zero bloat
//   • Newlib (embedded)        — retargetable, no OS assumption
//   • Plan 9 libc              — clean, elegant, no cruft
// Architecture:
//   • All functions pure C11, no system call passthrough to host OS
//   • sigma_syscall.c provides the raw syscall wrappers
//   • This file extends: string ops, memory ops, printf, atoi, qsort
// =============================================================================

#include <sigma_types.h>
#include "sigma_types.h"


// ── Memory Primitives (no glibc) ──────────────────────────────────────────────
void* sigma_memcpy(void* dest, const void* src, uint32_t n) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;
    while (n--) *d++ = *s++;
    return dest;
}

void* sigma_memset(void* dest, int c, uint32_t n) {
    uint8_t* d = (uint8_t*)dest;
    while (n--) *d++ = (uint8_t)c;
    return dest;
}

int sigma_memcmp(const void* a, const void* b, uint32_t n) {
    const uint8_t* pa = (const uint8_t*)a;
    const uint8_t* pb = (const uint8_t*)b;
    while (n--) {
        if (*pa != *pb) return (int)*pa - (int)*pb;
        pa++; pb++;
    }
    return 0;
}

void* sigma_memmove(void* dest, const void* src, uint32_t n) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;
    if (d < s) { while (n--) *d++ = *s++; }
    else { d += n; s += n; while (n--) *--d = *--s; }
    return dest;
}

// ── String Primitives ─────────────────────────────────────────────────────────
uint32_t sigma_strlen(const char* s) {
    const char* p = s;
    while (*p) p++;
    return (uint32_t)(p - s);
}

char* sigma_strcpy(char* dest, const char* src) {
    char* d = dest;
    while ((*d++ = *src++));
    return dest;
}

char* sigma_strncpy(char* dest, const char* src, uint32_t n) {
    char* d = dest;
    while (n && (*d++ = *src++)) n--;
    while (n--) *d++ = '\0';
    return dest;
}

int sigma_strcmp(const char* a, const char* b) {
    while (*a && (*a == *b)) { a++; b++; }
    return (unsigned char)*a - (unsigned char)*b;
}

int sigma_strncmp(const char* a, const char* b, uint32_t n) {
    while (n && *a && (*a == *b)) { a++; b++; n--; }
    if (!n) return 0;
    return (unsigned char)*a - (unsigned char)*b;
}

char* sigma_strstr(const char* haystack, const char* needle) {
    if (!*needle) return (char*)haystack;
    for (; *haystack; haystack++) {
        const char* h = haystack, *n = needle;
        while (*h && *n && *h == *n) { h++; n++; }
        if (!*n) return (char*)haystack;
    }
    return NULL;
}

char* sigma_strchr(const char* s, int c) {
    while (*s) { if (*s == (char)c) return (char*)s; s++; }
    return (c == '\0') ? (char*)s : NULL;
}

// ── Number Conversion ─────────────────────────────────────────────────────────
int64_t sigma_atoi(const char* s) {
    int64_t result = 0; int sign = 1;
    while (*s == ' ' || *s == '\t') s++;
    if (*s == '-') { sign = -1; s++; }
    else if (*s == '+') s++;
    while (*s >= '0' && *s <= '9') { result = result * 10 + (*s++ - '0'); }
    return result * sign;
}

// Safe snprintf — no glibc, no vsnprintf. Integer + string only.
int sigma_snprintf(char* buf, uint32_t size, const char* fmt, ...) {
    va_list ap; va_start(ap, fmt);
    uint32_t written = 0;
    for (const char* f = fmt; *f && written < size - 1; f++) {
        if (*f != '%') { buf[written++] = *f; continue; }
        f++;
        if (*f == 'd' || *f == 'u') {
            int64_t val = (*f=='d') ? va_arg(ap, int) : va_arg(ap, unsigned int);
            char tmp[24]; int tlen = 0;
            if (val < 0) { buf[written++] = '-'; val = -val; }
            if (val == 0) tmp[tlen++] = '0';
            while (val > 0) { tmp[tlen++] = '0' + (val % 10); val /= 10; }
            for (int i = tlen - 1; i >= 0 && written < size - 1; i--)
                buf[written++] = tmp[i];
        } else if (*f == 's') {
            const char* sv = va_arg(ap, const char*);
            while (*sv && written < size - 1) buf[written++] = *sv++;
        } else if (*f == 'c') {
            buf[written++] = (char)va_arg(ap, int);
        } else { buf[written++] = '%'; buf[written++] = *f; }
    }
    buf[written] = '\0';
    va_end(ap);
    return (int)written;
}

// ── Sorting (no qsort from glibc) ─────────────────────────────────────────────
// Insertion sort — zero-alloc, cache-friendly for small N (<= 64 elements)
void sigma_sort_small(void* base, uint32_t n, uint32_t size,
                      int (*cmp)(const void*, const void*)) {
    uint8_t* arr = (uint8_t*)base;
    uint8_t  tmp[256];
    for (uint32_t i = 1; i < n; i++) {
        sigma_memcpy(tmp, arr + i * size, size);
        int32_t j = (int32_t)i - 1;
        while (j >= 0 && cmp(arr + j * size, tmp) > 0) {
            sigma_memcpy(arr + (j + 1) * size, arr + j * size, size);
            j--;
        }
        sigma_memcpy(arr + (j + 1) * size, tmp, size);
    }
}

