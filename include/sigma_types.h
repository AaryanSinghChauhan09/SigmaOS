/*
 * =========================================================================
 * Σ SIGMAOS: SUPREME TYPES (v3.0 — LATTICE SHARD)
 * =========================================================================
 * Mission: Universal type definitions for all Sovereign Shards.
 * Design: Wraps SovereignCommon.h to ensure zero-include purity.
 * =========================================================================
 */

#ifndef SIGMAOS_SUPREME_TYPES_H
#define SIGMAOS_SUPREME_TYPES_H

#include "SovereignCommon.h"

/* ── Compatibility Aliases ────────────────────────────────────────────── */
#ifndef SIGMA_EXCLUDE_STD_ALIASES
  /* 
   * These aliases are provided for industrial-grade parity with C99/C11.
   * We guard against toolchain stdint.h to prevent recursive preamble loops.
   */
  #if !defined(_STDINT_H) && !defined(_STDINT_H_) && !defined(_GCC_STDINT_H)
    typedef sigma_u8   uint8_t;
    typedef sigma_u16  uint16_t;
    typedef sigma_u32  uint32_t;
    typedef sigma_u64  uint64_t;
    typedef sigma_i8   int8_t;
    typedef sigma_i16  int16_t;
    typedef sigma_i32  int32_t;
    typedef sigma_i64  int64_t;
  #endif

  #if !defined(_SIZE_T) && !defined(_SIZE_T_DEFINED) && !defined(__SIZE_TYPE__)
    typedef sigma_size_t  size_t;
    typedef sigma_ssize_t ssize_t;
  #endif
#endif

#endif /* SIGMAOS_SUPREME_TYPES_H */
