/*
 * =========================================================================
 * S SIGMAOS: SIGMA C11 FOUNDATION (v20.0 - PURE C11 REPLACEMENT)
 * =========================================================================
 * Replaces SigmaOOP.hpp — all C++ OOP constructs eliminated.
 * Provides: core types, sigma_status, sigma_log, fundamental macros.
 * Standard: C11 (ISO/IEC 9899:2011). No namespaces. No classes. No vtables.
 *
 * Migration map:
 *   SigmaObject (C++ base class)  -> sigma_obj_header_t (C struct field)
 *   SigmaMemory::allocate()       -> sigma_malloc()
 *   namespace SigmaOS             -> sigma_ prefix convention
 *   sigma_log()                   -> sigma_log() in SovereignLibC.h
 * =========================================================================
 */

#ifndef SIGMA_C11_H
#define SIGMA_C11_H

#include "sigma_libc.h"

/* =========================================================================
 * Common typedefs used across shards
 * ========================================================================= */
typedef sigma_u32 sigma_status_t;

#define SIGMA_C11_VERSION  20u

/* =========================================================================
 * Minimal object header (C11 equivalent of SigmaObject base class)
 * Usage: embed as first field in any shard struct for introspection.
 *
 *   typedef struct MyShard {
 *       sigma_obj_header_t hdr;   // must be first
 *       ...fields...
 *   } MyShard;
 * ========================================================================= */
typedef struct sigma_obj_header_t {
    const char* type_name;   /* String tag for the shard type */
    sigma_u32   version;     /* Shard version */
} sigma_obj_header_t;

#define SIGMA_OBJ_INIT(name, ver) { (name), (ver) }

/* =========================================================================
 * Sovereign memory shim (forwards to SovereignLibC slab allocator)
 * ========================================================================= */
static inline void* sigma_alloc(sigma_sz_t size) {
    return sigma_malloc(size);
}

static inline void sigma_dealloc(void* ptr) {
    sigma_free(ptr);
}

/* =========================================================================
 * Compile-time assertion (C11 _Static_assert)
 * ========================================================================= */
#define SIGMA_STATIC_ASSERT(cond, msg) _Static_assert(cond, msg)

/* =========================================================================
 * Utility: clamp a value between lo and hi
 * ========================================================================= */
#define SIGMA_CLAMP(v, lo, hi) \
    (((v) < (lo)) ? (lo) : (((v) > (hi)) ? (hi) : (v)))

/* =========================================================================
 * Utility: min / max
 * ========================================================================= */
#define SIGMA_MIN(a, b)  (((a) < (b)) ? (a) : (b))
#define SIGMA_MAX(a, b)  (((a) > (b)) ? (a) : (b))

/* =========================================================================
 * Utility: array length
 * ========================================================================= */
#define SIGMA_ARRLEN(arr) (sizeof(arr) / sizeof((arr)[0]))

/* =========================================================================
 * Utility: compile-time likely/unlikely branch hints
 * ========================================================================= */
#define SIGMA_LIKELY(x)   __builtin_expect(!!(x), 1)
#define SIGMA_UNLIKELY(x) __builtin_expect(!!(x), 0)

/* =========================================================================
 * Utility: compiler barrier
 * ========================================================================= */
#define SIGMA_BARRIER() __asm__ __volatile__("" ::: "memory")

#endif /* SIGMA_C11_H */


