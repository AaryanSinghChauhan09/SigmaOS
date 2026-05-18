/*
 * =========================================================================
 * Σ SIGMAOS: CANONICAL SovereignLibC HEADER (v3.0 - SOVEREIGN SHIM)
 * =========================================================================
 * Canonical global shim for SovereignLibC.h — accessible via -Isuites/include
 * or -Isuites/S01_Genesis/include/libc
 * =========================================================================
 */
#ifndef SOVEREIGN_LIBC_H
#define SOVEREIGN_LIBC_H

#include "core/sigma_types.h"

/* Forward declarations for all SovereignLibC primitives */
void          sigma_print(const char* str);
void          sigma_print_num(sigma_u64 val);
void          sigma_print_hex(sigma_u64 val);
int           sigma_atoi(const char* s);
int           sigma_streq(const char* s1, const char* s2);
int           sigma_compare(const char* s1, const char* s2);
void          sigma_strcat(char* dest, const char* src);
sigma_u64     sigma_strlen(const char* s);
void*         sigma_mmap(void* addr, sigma_u64 length, int prot, int flags, int fd, sigma_u64 offset);
void          sigma_exit(int code);
sigma_i64     sigma_write(int fd, const void* buf, sigma_u64 count);
void*         sigma_malloc(sigma_u64 size);
void          sigma_free(void* ptr);

#endif /* SOVEREIGN_LIBC_H */
