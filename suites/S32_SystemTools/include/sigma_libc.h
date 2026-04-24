/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN HOST BRIDGE (DEFENSIVE EDITION)
 * =========================================================================
 * Maps sigma_ calls to standard C for host-based tools.
 * Zero-recursion design (does not include SovereignCommon.h).
 * =========================================================================
 */

#ifndef SIGMA_HOST_BRIDGE_H
#define SIGMA_HOST_BRIDGE_H

/* Pre-empt kernel headers to avoid redefinition conflicts */
#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H
#endif
#ifndef SOVEREIGN_COMMON_H
#define SOVEREIGN_COMMON_H
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "sigma_libc.h"
#include "sigma_libc.h"
#include "sigma_libc.h"
#include <unistd.h>
#include "sigma_libc.h"

// ── Manual Type Definitions (Avoiding Common Header Recursion) ───────────
typedef size_t             sigma_sz_t;
typedef size_t             sigma_size_t;
typedef uint8_t            sigma_u8;
typedef uint16_t           sigma_u16;
typedef uint32_t           sigma_u32;
typedef uint64_t           sigma_u64;
typedef int8_t             sigma_i8;
typedef int16_t            sigma_i16;
typedef int32_t            sigma_i32;
typedef int64_t            sigma_i64;
typedef uint64_t           sigma_uptr;
typedef int64_t            sigma_ssz_t;
typedef int32_t            sigma_err_t;
typedef bool               sigma_bool;

#define SIGMA_TRUE         true
#define SIGMA_FALSE        false
#define SIGMA_NULL         NULL

// ── I/O Bridge ───────────────────────────────────────────────────────────
static inline int sigma_printf(const char* format, ...) {
    va_list args; va_start(args, format);
    int ret = vprintf(format, args);
    va_end(args); return ret;
}

static inline FILE* sigma_open(const char* path, const char* mode) {
    return fopen(path, mode);
}

static inline int sigma_close(FILE* stream) {
    if (!stream) return -1;
    return fclose(stream);
}

static inline size_t sigma_read(void* ptr, size_t size, size_t count, FILE* stream) {
    return fread(ptr, size, count, stream);
}

static inline size_t sigma_write(const void* ptr, size_t size, size_t count, FILE* stream) {
    return fwrite(ptr, size, count, stream);
}

static inline int sigma_sprintf(char* str, const char* format, ...) {
    va_list args; va_start(args, format);
    int ret = vsprintf(str, format, args);
    va_end(args); return ret;
}

static inline int sigma_snprintf(char* str, size_t size, const char* format, ...) {
    va_list args; va_start(args, format);
    int ret = vsnprintf(str, size, format, args);
    va_end(args); return ret;
}

static inline int sigma_fprintf(FILE* stream, const char* format, ...) {
    va_list args; va_start(args, format);
    int ret = vfprintf(stream, format, args);
    va_end(args); return ret;
}

static inline void sigma_exit(int status) {
    exit(status);
}

static inline char* sigma_getcwd(char* buf, size_t size) {
    return getcwd(buf, size);
}

// ── Memory Bridge ────────────────────────────────────────────────────────
static inline void* sigma_malloc(size_t size) {
    return malloc(size);
}

static inline void sigma_free(void* ptr) {
    free(ptr);
}

static inline void* sigma_memset(void* s, int c, size_t n) {
    return memset(s, c, n);
}

static inline void* sigma_memcpy(void* dest, const void* src, size_t n) {
    return memcpy(dest, src, n);
}

// ── String Bridge ────────────────────────────────────────────────────────
static inline int sigma_strcmp(const char* s1, const char* s2) {
    return strcmp(s1, s2);
}

static inline int sigma_strncmp(const char* s1, const char* s2, size_t n) {
    return strncmp(s1, s2, n);
}

static inline size_t sigma_strlen(const char* s) {
    return strlen(s);
}

static inline char* sigma_strcpy(char* dest, const char* src) {
    return strcpy(dest, src);
}

static inline char* sigma_strncpy(char* dest, const char* src, size_t n) {
    return strncpy(dest, src, n);
}

static inline char* sigma_strrchr(const char* s, int c) {
    return (char*)strrchr(s, c);
}

static inline char* sigma_strstr(const char* haystack, const char* needle) {
    return (char*)strstr(haystack, needle);
}

static inline char* sigma_strncat(char* dest, const char* src, size_t n) {
    return strncat(dest, src, n);
}

// ── Kernel Compatibility ─────────────────────────────────────────────────
#define sigma_sigma_sigma_printf sigma_printf
#define sigma_sigma_printf       sigma_printf
#define sigma_sigma_malloc       sigma_malloc
#define sigma_sigma_free         sigma_free
#define sigma_sigma_memset       sigma_memset
#define sigma_sigma_memcpy       sigma_memcpy
#define sigma_sigma_strlen       sigma_strlen
#define sigma_sigma_strcmp       sigma_strcmp
#define sigma_sigma_strcpy       sigma_strcpy
#define sigma_sigma_strncpy      sigma_strncpy
#define sigma_sigma_strrchr      sigma_strrchr

#define SIGMA_OK    0

#endif // SIGMA_HOST_BRIDGE_H
