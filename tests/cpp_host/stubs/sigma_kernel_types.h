#include "../../../include/sigma_kernel_types.h"
/*
 * tests/cpp_host/stubs/sigma_kernel_types.h
 * Host-mode stub — replaces bare-metal ASM intrinsics for unit testing.
 * All kernel types are preserved; CPU primitives become no-ops.
 */
#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <string.h>
#include <stdio.h>

typedef uint8_t   sigma_u8;
typedef uint16_t  sigma_u16;
typedef uint32_t  sigma_u32;
typedef uint64_t  sigma_u64;
typedef int32_t   sigma_i32;
typedef int64_t   sigma_i64;
typedef size_t    sigma_usize;
typedef ptrdiff_t sigma_isize;
typedef uint64_t  sigma_paddr_t;
typedef uint64_t  sigma_vaddr_t;
typedef int       sigma_bool;
typedef int32_t   sigma_status;

#define SIGMA_TRUE  1
#define SIGMA_FALSE 0
#define SIGMA_NULL  nullptr

#define PAGE_SIZE  4096ULL
#define K_OK        0
#define K_ERR_NOMEM -1
#define K_ERR_INVAL -2

/* Stub intrinsics */
static inline uint64_t cpu_rdtsc() { return 0xDEADBEEFCAFEULL; }
static inline void* sigma_memcpy(void* d, const void* s, size_t n) { return memcpy(d,s,n); }
static inline void* sigma_memset(void* s, int c, size_t n) { return memset(s,c,n); }
static inline size_t sigma_strlen(const char* s) { return strlen(s); }
static inline int sigma_strcmp(const char* a, const char* b) { return strcmp(a,b); }
static inline void sigma_panic(const char* m, uint64_t, uint64_t) { fprintf(stderr,"PANIC: %s\n",m); }
static inline void kprintf(const char* fmt, ...) { (void)fmt; }
