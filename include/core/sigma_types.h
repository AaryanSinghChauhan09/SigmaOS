#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

/**
 * SIGMAOS: Sovereign Type Shards (v13.5 - Obsidian)
 * Dependencies: sigma_kernel_types.h (Industrial Source of Truth)
 */

#include "core/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Standard POSIX-like types parity - only if not already defined */
#if !defined(_UINT8_T_DECLARED) && !defined(_STDINT_H) && !defined(_SYS_TYPES_H)
typedef sigma_u8  uint8_t;
typedef sigma_u16 uint16_t;
typedef sigma_u32 uint32_t;
typedef sigma_u64 uint64_t;

typedef sigma_i8  int8_t;
typedef sigma_i16 int16_t;
typedef sigma_i32 int32_t;
typedef sigma_i64 int64_t;

#define _UINT8_T_DECLARED
#define _UINT16_T_DECLARED
#define _UINT32_T_DECLARED
#define _UINT64_T_DECLARED
#define _INT8_T_DECLARED
#define _INT16_T_DECLARED
#define _INT32_T_DECLARED
#define _INT64_T_DECLARED
#endif

typedef sigma_i64 sigma_ssize_t;
typedef sigma_u64 sigma_size_t;
typedef sigma_u64 sigma_addr_t;

typedef float  sigma_f32;
typedef double sigma_f64;

#ifndef SIGMA_STATUS_DEFINED
#define SIGMA_STATUS_DEFINED
typedef sigma_i32 sigma_status;
#endif

#ifndef __cplusplus
#ifndef bool
typedef unsigned char bool;
#define true  1
#define false 0
#endif
#endif

/* Industrial Constants */
#ifndef SIGMA_TRUE
#define SIGMA_TRUE  1u
#endif
#ifndef SIGMA_FALSE
#define SIGMA_FALSE 0u
#endif
#ifndef SIGMA_NULL
#ifdef __cplusplus
#define SIGMA_NULL  nullptr
#else
#define SIGMA_NULL  ((void*)0)
#endif
#endif

#define SIGMA_OK    0x00000000u
#define SIGMA_ERROR 0xFFFFFFFFu

#define SIGMA_PACKED     __attribute__((packed))
#define SIGMA_ALIGNED(x) __attribute__((aligned(x)))

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TYPES_H */
