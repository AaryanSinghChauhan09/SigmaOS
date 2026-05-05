/*
 * ==========================================================================
 * SIGMAOS: SOVEREIGN LIBC (Pure C11 Implementation)
 * ==========================================================================
 * This file provides the pure-C fallback implementations for the SovereignLibC
 * primitives declared in include/SovereignLibC.h.
 *
 * NOTE: High-performance hot-paths (sigma_memcpy, sigma_memset, sigma_strlen)
 *       are implemented as static inline ASM in sigma_kernel_types.h.
 *       This file only implements the higher-level string and format utilities.
 * ==========================================================================
 */

#include "../../../include/SovereignLibC.h"
#include "../../../include/sigma_types.h"

/* va_list support in freestanding mode via compiler builtins */
#ifndef va_list
typedef __builtin_va_list va_list;
#define va_start(ap, last) __builtin_va_start(ap, last)
#define va_arg(ap, type)   __builtin_va_arg(ap, type)
#define va_end(ap)         __builtin_va_end(ap)
#endif

/* =========================================================================
 * STRING UTILITIES
 * ========================================================================= */

int sigma_streq(const char* s1, const char* s2) {
    if (!s1 || !s2) return 0;
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return (*s1 == *s2);
}

int sigma_compare(const char* s1, const char* s2) {
    return sigma_streq(s1, s2) ? 0 : ((unsigned char)*s1 - (unsigned char)*s2);
}

void sigma_strcat(char* dest, const char* src) {
    if (!dest || !src) return;
    while (*dest) dest++;
    while (*src) *dest++ = *src++;
    *dest = '\0';
}

void sigma_strncat(char* dest, const char* src, sigma_size_t n) {
    if (!dest || !src) return;
    while (*dest) dest++;
    sigma_size_t i = 0;
    while (i < n && src[i]) { *dest++ = src[i++]; }
    *dest = '\0';
}

/* Bounded copy — safer replacement for legacy sigma_strcpy */
void sigma_strcpy(char* dest, const char* src, sigma_size_t n) {
    if (!dest || !src) return;
    sigma_size_t i = 0;
    for (; i < n - 1 && src[i] != '\0'; i++) dest[i] = src[i];
    dest[i] = '\0';
}


int sigma_atoi(const char* s) {
    if (!s) return 0;
    int result = 0;
    int sign = 1;
    if (*s == '-') { sign = -1; s++; }
    while (*s >= '0' && *s <= '9') {
        result = result * 10 + (*s - '0');
        s++;
    }
    return sign * result;
}

/* =========================================================================
 * OUTPUT UTILITIES
 * ========================================================================= */

/* Write a single hex digit */
static void _write_hex_digit(char* buf, int* idx, sigma_u8 d) {
    buf[(*idx)++] = (d < 10) ? ('0' + d) : ('a' + d - 10);
}

void sigma_print(const char* str) {
    if (!str) return;
    sigma_write(1, str, sigma_strlen(str));
}

void sigma_print_num(sigma_u64 val) {
    char buf[32];
    int i = 30;
    buf[31] = '\0';
    if (val == 0) {
        buf[i--] = '0';
    } else {
        while (val > 0 && i > 0) {
            buf[i--] = (char)((val % 10) + '0');
            val /= 10;
        }
    }
    sigma_print(&buf[i + 1]);
}

void sigma_print_hex(sigma_u64 val) {
    char buf[20];
    int idx = 0;
    buf[idx++] = '0'; buf[idx++] = 'x';
    for (int shift = 60; shift >= 0; shift -= 4)
        _write_hex_digit(buf, &idx, (sigma_u8)((val >> shift) & 0xF));
    buf[idx] = '\0';
    sigma_print(buf);
}

/* Minimal freestanding printf — supports %s, %d, %u, %x, %llu, %p, %c */
void sigma_printf(const char* format, ...) {
    if (!format) return;
    char buf[512];
    int bi = 0;
    va_list ap;
    va_start(ap, format);
    for (const char* f = format; *f && bi < 510; f++) {
        if (*f != '%') { buf[bi++] = *f; continue; }
        f++;
        if (!*f) break;
        if (*f == 's') {
            const char* s = va_arg(ap, const char*);
            if (!s) s = "(null)";
            while (*s && bi < 510) buf[bi++] = *s++;
        } else if (*f == 'd' || *f == 'i') {
            sigma_i64 v = va_arg(ap, int);
            if (v < 0) { buf[bi++] = '-'; v = -v; }
            char nb[24]; int ni = 0;
            do { nb[ni++] = (char)('0' + v % 10); v /= 10; } while (v);
            while (ni > 0) buf[bi++] = nb[--ni];
        } else if (*f == 'u') {
            sigma_u64 v = (sigma_u64)va_arg(ap, unsigned int);
            char nb[24]; int ni = 0;
            if (v == 0) { buf[bi++] = '0'; } else {
                do { nb[ni++] = (char)('0' + v % 10); v /= 10; } while (v);
                while (ni > 0) buf[bi++] = nb[--ni];
            }
        } else if (*f == 'x' || *f == 'X') {
            sigma_u64 v = (sigma_u64)va_arg(ap, unsigned int);
            char nb[18]; int ni = 0;
            if (v == 0) { buf[bi++] = '0'; } else {
                do { nb[ni++] = "0123456789abcdef"[v & 0xF]; v >>= 4; } while (v);
                while (ni > 0) buf[bi++] = nb[--ni];
            }
        } else if (*f == 'l') {
            f++;
            if (*f == 'l') {
                f++;
                sigma_u64 v = va_arg(ap, sigma_u64);
                char nb[24]; int ni = 0;
                if (v == 0) { buf[bi++] = '0'; } else {
                    do { nb[ni++] = (char)('0' + v % 10); v /= 10; } while (v);
                    while (ni > 0) buf[bi++] = nb[--ni];
                }
            }
        } else if (*f == 'p') {
            sigma_u64 v = (sigma_u64)(sigma_usize)va_arg(ap, void*);
            buf[bi++] = '0'; buf[bi++] = 'x';
            for (int shift = 60; shift >= 0; shift -= 4)
                buf[bi++] = "0123456789abcdef"[(v >> shift) & 0xF];
        } else if (*f == 'c') {
            buf[bi++] = (char)va_arg(ap, int);
        } else if (*f == '%') {
            buf[bi++] = '%';
        }
    }
    va_end(ap);
    buf[bi] = '\0';
    sigma_write(1, buf, (sigma_size_t)bi);
}

void sigma_log(const char* msg) {
    sigma_print("[SIGMA] ");
    sigma_print(msg);
    sigma_print("\n");
}

/* =========================================================================
 * SECURITY-HARDENED PRIMITIVES
 * ========================================================================= */

void* sigma_secure_memset(void* s, int c, sigma_size_t n) {
    volatile unsigned char* p = (volatile unsigned char*)s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}

void sigma_hardened_strcpy(char* dest, const char* src, sigma_size_t dest_size) {
    sigma_strcpy(dest, src, dest_size);
}

int sigma_hardened_strcmp(const char* s1, const char* s2) {
    return sigma_compare(s1, s2);
}

int sigma_hardened_strncmp(const char* s1, const char* s2, sigma_size_t n) {
    if (!s1 || !s2) return 0;
    sigma_size_t i = 0;
    while (i < n && s1[i] && s2[i] && s1[i] == s2[i]) i++;
    return (i == n) ? 0 : ((unsigned char)s1[i] - (unsigned char)s2[i]);
}

/* =========================================================================
 * SOVEREIGN MEMORY MANAGEMENT (bump-pointer slab, 128 MB arena)
 * ========================================================================= */

#define SIGMA_SLAB_SIZE (128ULL * 1024ULL * 1024ULL)
static char   _sigma_slab[SIGMA_SLAB_SIZE];
static sigma_size_t _sigma_slab_ptr = 0;

void* sigma_slab_alloc_raw(sigma_size_t size) {
    size = (size + 7u) & ~7u;  /* 8-byte alignment */
    if (_sigma_slab_ptr + size > SIGMA_SLAB_SIZE) return (void*)0;
    void* ret = &_sigma_slab[_sigma_slab_ptr];
    _sigma_slab_ptr += size;
    return ret;
}

void* sigma_malloc(sigma_size_t size) {
    return sigma_slab_alloc_raw(size);
}

/* Slab allocator has no per-object free — lifecycle managed by OS */
void sigma_free(void* ptr) {
    (void)ptr; /* No-op in bump-pointer design */
}
