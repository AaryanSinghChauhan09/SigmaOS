/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TYPES (v2.0)
 * =========================================================================
 * Mission: Absolute bit-perfect type definitions for Zenith Supreme.
 * =========================================================================
 */

#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

/* --- Primitive Types --- */
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_i8;
typedef signed short       sigma_i16;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;

/* --- Native Aliases --- */
typedef sigma_u8           u8;
typedef sigma_u16          u16;
typedef sigma_u32          u32;
typedef sigma_u64          u64;
typedef sigma_i8           i8;
typedef sigma_i16          i16;
typedef sigma_i32          i32;
typedef sigma_i64          i64;

/* --- System Types --- */
typedef sigma_u64          sigma_size_t;
typedef sigma_i64          sigma_ssize_t;
typedef sigma_u64          usize;
typedef sigma_i64          isize;
typedef double             sigma_f64;
typedef int                sigma_bool;
typedef sigma_i32          pid_t;
typedef sigma_u64          virt_addr_t;

#define SIGMA_TRUE  1
#define SIGMA_FALSE 0
#define SIGMA_NULL  ((void*)0)
#define SIGMA_UNUSED(x) (void)(x)

#define SIGMA_OK      0
#define SIGMA_EIO     1
#define SIGMA_ENOMEM  2

typedef int sigma_err_t;

#ifndef SIGMA_NORETURN
#define SIGMA_NORETURN _Noreturn
#endif

#endif
