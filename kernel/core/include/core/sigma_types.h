/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TYPES (v2.0)
 * =========================================================================
 * Mission: Absolute bit-perfect type definitions for Zenith Supreme.
 * =========================================================================
 */

#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_i8;
typedef signed short       sigma_i16;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;

typedef sigma_u64          sigma_size_t;
typedef sigma_i64          sigma_ssize_t;

typedef int                sigma_bool;

#define SIGMA_TRUE  1
#define SIGMA_FALSE 0
#ifdef __cplusplus
#define SIGMA_NULL  nullptr
#else
#define SIGMA_NULL  ((void*)0)
#endif

#endif
