#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

#include <stddef.h>
#include <stdint.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

void sys_print(const char* fmt, ...);

#ifndef sigma_strcmp
static inline int sigma_strcmp(const char* s1, const char* s2) {
    if (!s1 || !s2) return s1 == s2 ? 0 : (s1 ? 1 : -1);
    return strcmp(s1, s2);
}
#endif

static inline char* sigma_strcpy(char* dest, const char* src) {
    if (!dest || !src) return dest;
    return strcpy(dest, src);
}

static inline size_t sigma_strlen(const char* s) {
    if (!s) return 0;
    return strlen(s);
}

static inline void* sigma_memset(void* s, int c, size_t n) {
    return memset(s, c, n);
}

static inline void* sigma_memcpy(void* dest, const void* src, size_t n) {
    return memcpy(dest, src, n);
}

static inline void* sigma_malloc(size_t size) {
    return malloc(size);
}

static inline void sigma_free(void* ptr) {
    free(ptr);
}

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LIBC_H */
