#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

/* =========================================================================
 * Σ SIGMAOS: SOVEREIGN TYPE SHARDS (v12.0)
 * =========================================================================
 * Standardized integer types for industrial-grade bare-metal stability.
 * ========================================================================= */

typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_i8;
typedef signed short       sigma_i16;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;

typedef float              sigma_f32;
typedef double             sigma_f64;

typedef unsigned long long sigma_size_t;
typedef unsigned long long sigma_addr_t;
typedef unsigned long long sigma_bool;

#define SIGMA_TRUE  1
#define SIGMA_FALSE 0
#define SIGMA_NULL  ((void*)0)

#endif
