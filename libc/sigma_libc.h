/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LIBC HEADER (v2.0)
 * =========================================================================
 */

#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

#include "sigma_types.h"

// --- Syscall Wrappers ---
void          sigma_exit(int code);
sigma_ssize_t sigma_write(int fd, const void* buf, sigma_size_t count);
sigma_ssize_t sigma_read(int fd, void* buf, sigma_size_t count);
int           sigma_open(const char* filename, int flags, int mode);
int           sigma_close(int fd);
void*         sigma_mmap(void* addr, sigma_size_t length, int prot, int flags, int fd, sigma_u64 offset);

// --- libc utility functions ---
sigma_size_t  sigma_strlen(const char* s);
void*         sigma_memset(void* s, int c, sigma_size_t n);
void*         sigma_memcpy(void* dest, const void* src, sigma_size_t n);
int           sigma_streq(const char* s1, const char* s2);

// --- High-Level primitives implemented at Low-Level ---
void sigma_print(const char* str);
void sigma_printf(const char* format, ...);

#endif


