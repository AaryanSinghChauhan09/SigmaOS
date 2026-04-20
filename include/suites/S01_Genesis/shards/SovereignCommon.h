/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN COMMON SHARD (v2.0 — PRIORITY DEFINES)
 * =========================================================================
 * Strictly no includes. Immediate type definitions.
 * =========================================================================
 */

#ifndef SOVEREIGN_COMMON_H
#define SOVEREIGN_COMMON_H

/* ── Priority Core Types (must be first) ──────────────────────────────── */
#ifdef SIGMA_EXCLUDE_STD_ALIASES
#include <stddef.h>
typedef size_t             sigma_sz_t;
#else
typedef unsigned long long sigma_sz_t;
#endif
typedef sigma_sz_t         sigma_size_t;
typedef signed long long   sigma_ssz_t;
typedef signed int         sigma_err_t;

/* ── Primitives ───────────────────────────────────────────────────────── */
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_i8;
typedef signed short       sigma_i16;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;

typedef unsigned long long sigma_uptr;
typedef unsigned char      sigma_bool;

#define SIGMA_TRUE         ((sigma_bool)1)
#define SIGMA_FALSE        ((sigma_bool)0)
#define SIGMA_NULL         ((void*)0)

/* ── Sovereign Object-Oriented Foundation (SOOF) ─────────────────────── */
struct sigma_obj;
typedef void (*sigma_obj_init_f)(struct sigma_obj* obj);
typedef void (*sigma_obj_deinit_f)(struct sigma_obj* obj);
typedef int  (*sigma_obj_tostring_f)(struct sigma_obj* obj, char* buf, sigma_sz_t size);

typedef struct sigma_obj {
    const char*         name;
    sigma_u32           id;
    sigma_u32           cls;
    void*               priv;
    sigma_obj_init_f    init;
    sigma_obj_deinit_f  deinit;
    sigma_obj_tostring_f tostring;
} sigma_obj_t;

#define SIGMA_OBJ_INIT(n, c) { .name = n, .cls = c, .init = SIGMA_NULL, .deinit = SIGMA_NULL }

#endif /* SOVEREIGN_COMMON_H */
