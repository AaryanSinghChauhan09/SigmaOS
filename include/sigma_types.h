#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

/**
 * SIGMAOS: Sovereign Type Shards (v13.5 - Obsidian)
 * Dependencies: sigma_kernel_types.h (Industrial Source of Truth)
 */

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Standard POSIX-like types parity */
typedef sigma_u8  uint8_t;
typedef sigma_u16 uint16_t;
typedef sigma_u32 uint32_t;
typedef sigma_u64 uint64_t;

typedef sigma_i8  int8_t;
typedef sigma_i16 int16_t;
typedef sigma_i32 int32_t;
typedef sigma_i64 int64_t;

typedef sigma_i64 sigma_ssize_t;
typedef sigma_u64 sigma_size_t;
typedef sigma_u64 sigma_usize;
typedef sigma_u64 sigma_addr_t;

typedef float  sigma_f32;
typedef double sigma_f64;

typedef sigma_i32 sigma_status;

#ifndef __cplusplus
typedef unsigned char bool;
#define true  1
#define false 0
#endif

/* Industrial Constants */
#define SIGMA_TRUE  1u
#define SIGMA_FALSE 0u
#ifdef __cplusplus
#define SIGMA_NULL  nullptr
#else
#define SIGMA_NULL  ((void*)0)
#endif

#define SIGMA_OK    0x00000000u
#define SIGMA_ERROR 0xFFFFFFFFu

#define SIGMA_PACKED     __attribute__((packed))
#define SIGMA_ALIGNED(x) __attribute__((aligned(x)))

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TYPES_H */
