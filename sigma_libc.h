#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

/*
 * =========================================================
 * SIGMAOS: SOVEREIGN LIBC (INDUSTRIAL GRADE)
 * =========================================================
 * Zero-dependency, bare-metal C library for the Lattice.
 * =========================================================
 */

#include "core/sigma_kernel_types.h"

#define SIGMA_LIBC_VERSION 0x09

/* Initialization & Bootstrap */
#ifdef __cplusplus
extern "C" {
#endif

void sigma_core_init(void);

/* Industrial Shard Initialization */
static inline void sigma_shard_init_internal(void) {
    sigma_core_init();
}

#define SIGMA_SHARD_INIT() sigma_shard_init_internal()

/* Memory Orchestration */
void* sigma_memcpy(void* dest, const void* src, sigma_size_t n);
void* sigma_memset(void* s, int c, sigma_size_t n);

/* String Manipulation */
sigma_size_t sigma_strlen(const char* s);
void sigma_strncpy(char* dest, const char* src, sigma_size_t n);
int sigma_strcmp(const char* s1, const char* s2);
int sigma_atoi(const char* str);

/* Terminal I/O */
void sigma_printf(const char* format, ...);
void sigma_exit(int status);
sigma_u32 sigma_crc32(const void* data, sigma_size_t n);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_LIBC_H

