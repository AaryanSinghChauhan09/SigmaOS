/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN LIBC & TYPES (v1000.1.6)
 * =========================================================================
 * Mission: Absolute Zero-Dependency Silicon Parity.
 * Standard: ISO C11. No External Headers.
 * =========================================================================
 */

#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

/* Scalar Primitive Resolution */
typedef unsigned char      uint8_t;
typedef unsigned short     uint16_t;
typedef unsigned int       uint32_t;
typedef unsigned long long uint64_t;
typedef char               int8_t;
typedef short              int16_t;
typedef int                int32_t;
typedef long long          int64_t;
typedef unsigned long long size_t;
typedef unsigned long long uintptr_t;

#define NULL ((void*)0)

/* Sovereign I/O Prototypes */
void sigma_printf(const char* format, ...);
void* sigma_malloc(size_t size);
void sigma_free(void* ptr);
void sigma_memset(void* s, int c, size_t n);
void sigma_memcpy(void* dest, const void* src, size_t n);
int sigma_strcmp(const char* s1, const char* s2);
size_t sigma_strlen(const char* s);

#endif
