/*
 * =========================================================================
 * Σ SIGMAOS: SUPREME TYPE LATTICE (v4.0 — FLATTENED SOVEREIGNTY)
 * =========================================================================
 * Mission: Absolute diagnostic purity. No modular shards. No recursion.
 * =========================================================================
 */

#ifndef SIGMAOS_SUPREME_TYPES_H
#define SIGMAOS_SUPREME_TYPES_H

/* Core Integers */
typedef unsigned char      sigma_u8;
typedef signed   char      sigma_i8;
typedef unsigned short     sigma_u16;
typedef signed   short     sigma_i16;
typedef unsigned int       sigma_u32;
typedef signed   int       sigma_i32;
typedef unsigned long long sigma_u64;
typedef signed   long long sigma_i64;

/* Architecture Word Sizes */
typedef sigma_u64          sigma_uptr;
typedef sigma_i64          sigma_iptr;
typedef sigma_u64          sigma_size_t;
typedef sigma_i64          sigma_ssize_t;

/* Status Primitives */
typedef signed int         sigma_err_t;
#define SIGMA_OK           ((sigma_err_t)0)
#define SIGMA_ERROR        ((sigma_err_t)-1)

/* Boolean Logic */
typedef unsigned char      sigma_bool;
#define SIGMA_TRUE         ((sigma_bool)1)
#define SIGMA_FALSE        ((sigma_bool)0)

/* Master Purity Aliases */
#ifndef SIGMA_EXCLUDE_STD_ALIASES
  #if !defined(_STDINT_H) && !defined(_STDINT_H_) && !defined(__uint8_t_defined)
    typedef sigma_u8       uint8_t;
    typedef sigma_u16      uint16_t;
    typedef sigma_u32      uint32_t;
    typedef sigma_u64      uint64_t;
    typedef sigma_i32      int32_t;
    typedef sigma_i64      int64_t;
  #endif
  
  #if !defined(_STDBOOL_H) && !defined(__bool_true_false_are_defined)
    typedef sigma_bool     bool;
    #define true           SIGMA_TRUE
    #define false          SIGMA_FALSE
  #endif

  #if !defined(_SIZE_T_DEFINED) && !defined(_SIZE_T) && !defined(__size_t_defined)
    typedef sigma_size_t   size_t;
  #endif
#endif

#define SIGMA_NULL ((void*)0)

#endif /* SIGMAOS_SUPREME_TYPES_H */
