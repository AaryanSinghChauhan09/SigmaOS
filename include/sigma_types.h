#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

/* =========================================================================
 * SIGMAOS: SOVEREIGN TYPE SHARDS (v13.0)
 * =========================================================================
 */

#ifdef __cplusplus
extern "C" {
#endif

typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_i8;
typedef signed short       sigma_i16;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;

// Standard types parity
typedef sigma_u8  uint8_t;
typedef sigma_u16 uint16_t;
typedef sigma_u32 uint32_t;
typedef sigma_u64 uint64_t;
typedef sigma_i8  int8_t;
typedef sigma_i16 int16_t;
typedef sigma_i32 int32_t;
typedef sigma_i64 int64_t;

typedef signed long long   sigma_ssize_t;
typedef unsigned long long sigma_size_t;
typedef sigma_size_t       sigma_usize;
typedef unsigned long long sigma_addr_t;

typedef float              sigma_f32;
typedef double             sigma_f64;

#ifndef __cplusplus
typedef unsigned char bool;
#define true 1
#define false 0
#endif

#define SIGMA_TRUE  1u
#define SIGMA_FALSE 0u
#ifdef __cplusplus
#define SIGMA_NULL  nullptr
#else
#define SIGMA_NULL  ((void*)0)
#endif
#define SIGMA_OK    0x00000000u
#define SIGMA_ERROR 0xFFFFFFFFu

#define SIGMA_PACKED __attribute__((packed))
#define SIGMA_ALIGNED(x) __attribute__((aligned(x)))

typedef sigma_u32 sigma_status;

#ifdef __cplusplus
}
#endif

#endif
