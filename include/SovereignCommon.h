/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN COMMON SHARD (v1.0)
 * =========================================================================
 * Purpose: The absolute base of the hierarchical lattice.
 *          Zero-dependency. Pure C11 primitives.
 *          Strictly no includes allowed.
 * =========================================================================
 */

#ifndef SOVEREIGN_COMMON_H
#define SOVEREIGN_COMMON_H

/* ── Sovereign Primitives ─────────────────────────────────────────────── */
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_i8;
typedef signed short       sigma_i16;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;

typedef unsigned long long sigma_uptr;
typedef unsigned long long sigma_size_t;
typedef signed long long   sigma_ssize_t;
typedef signed int         sigma_err_t;

typedef unsigned char      sigma_bool;
#define SIGMA_TRUE         ((sigma_bool)1)
#define SIGMA_FALSE        ((sigma_bool)0)

#define SIGMA_NULL         ((void*)0)

/* ── Error Codes ──────────────────────────────────────────────────────── */
#define SIGMA_OK            0
#define SIGMA_ERROR        -1
#define SIGMA_NOMEM        -2
#define SIGMA_INVAL        -3
#define SIGMA_BUSY         -4
#define SIGMA_NOTSUPP      -5

#endif /* SOVEREIGN_COMMON_H */
