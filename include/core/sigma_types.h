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
#if !defined(SIGMA_STDINT_DECLARED) && !defined(_STDINT_H) && !defined(_SYS_TYPES_H)
#define uint8_t  sigma_u8
#define uint16_t sigma_u16
#define uint32_t sigma_u32
#define uint64_t sigma_u64

#define int8_t   sigma_i8
#define int16_t  sigma_i16
#define int32_t  sigma_i32
#define int64_t  sigma_i64

#define SIGMA_STDINT_DECLARED
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
