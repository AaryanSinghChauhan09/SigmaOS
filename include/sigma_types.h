#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

/* =========================================================================
 * SIGMAOS: SOVEREIGN TYPE SHARDS (v13.0)
 * =========================================================================
 */

typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_i8;
typedef signed short       sigma_i16;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;

typedef signed long long   sigma_ssize_t;
typedef unsigned long long sigma_size_t;
typedef sigma_size_t       sigma_usize;
typedef unsigned long long sigma_addr_t;

typedef float              sigma_f32;
typedef double             sigma_f64;

typedef unsigned long long sigma_bool;

#define SIGMA_TRUE  1u
#define SIGMA_FALSE 0u
#define SIGMA_NULL  ((void*)0)
#define SIGMA_OK    0x00000000u
#define SIGMA_ERROR 0xFFFFFFFFu

typedef sigma_u32 sigma_status;

#endif
