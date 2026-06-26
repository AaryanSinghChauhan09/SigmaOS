/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN LIBC — PUBLIC API
 * =========================================================================
 * Zero-dependency C runtime for SigmaOS kernel and userland.
 * Do NOT include <string.h>, <stdlib.h>, or <stdio.h> anywhere in the kernel.
 *
 * Implementation: kernel/libc/sigma_libc_impl.c
 * =========================================================================
 */

#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

#include "./sigma_kernel_types.h"
/* sigma_log.h is NOT included here — include it explicitly if needed. */

#ifdef __cplusplus
extern "C" {
#endif

/* -------------------------------------------------------------------------
 * Memory Management
 * ------------------------------------------------------------------------- */
void*        sigma_malloc(sigma_size_t size);
void         sigma_free(void* ptr);
void*        sigma_realloc(void* ptr, sigma_size_t size);
void*        sigma_memset(void* dst, sigma_u8 val, sigma_size_t n);
void*        sigma_memcpy(void* dst, const void* src, sigma_size_t n);
void*        sigma_memmove(void* dst, const void* src, sigma_size_t n);
int          sigma_memcmp(const void* a, const void* b, sigma_size_t n);
int          sigma_posix_memalign(void** memptr, sigma_size_t alignment, sigma_size_t size);

/* -------------------------------------------------------------------------
 * String Operations
 * ------------------------------------------------------------------------- */
sigma_size_t sigma_strlen(const char* s);
int          sigma_strcmp(const char* a, const char* b);
int          sigma_strncmp(const char* a, const char* b, sigma_size_t n);
char*        sigma_strcpy(char* dst, const char* src);
char*        sigma_strncpy(char* dst, const char* src, sigma_size_t n);
char*        sigma_strcat(char* dst, const char* src);
const char*  sigma_strchr(const char* s, char c);
const char*  sigma_strstr(const char* haystack, const char* needle);

/* -------------------------------------------------------------------------
 * Number Conversion
 * ------------------------------------------------------------------------- */
sigma_i32    sigma_atoi(const char* str);
char*        sigma_itoa(sigma_i32 val, char* buf, sigma_u32 base);
sigma_i32    sigma_abs(sigma_i32 val);

/* -------------------------------------------------------------------------
 * I/O — Syscall-backed, no FILE*, no glibc
 * ------------------------------------------------------------------------- */
void         sys_print(const char* fmt, ...);     /* %s %d %u %x %p %c %% %ld %lu %lx */
sigma_size_t sigma_snprintf(char* buf, sigma_size_t max, const char* fmt, ...);
sigma_status sys_ipc_send(sigma_u32 target_shard, sigma_u32 msg_id,
                           const void* data, sigma_size_t len);

/* ---- CPU Feature Detection (call once at boot) ---- */
void         sigma_libc_detect_cpu_features(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LIBC_H */
