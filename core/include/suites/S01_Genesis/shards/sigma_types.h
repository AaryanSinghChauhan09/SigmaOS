/*
 * =========================================================================
 * S SIGMAOS: SUPREME TYPES (v4.0 — DEFENSIVE LATTICE)
 * =========================================================================
 * Mission: Universal type definitions for all Sovereign Shards.
 * Design: Hardened against toolchain stdint.h preamble recursion.
 * =========================================================================
 */

#ifndef SIGMAOS_SUPREME_TYPES_H
#define SIGMAOS_SUPREME_TYPES_H

#include "suites/S01_Genesis/shards/SovereignCommon.h"

/* ── Compatibility Aliases ────────────────────────────────────────────── */
#ifndef SIGMA_EXCLUDE_STD_ALIASES
    typedef sigma_u8   uint8_t;
    typedef sigma_u16  uint16_t;
    typedef sigma_u32  uint32_t;
    typedef sigma_u64  uint64_t;
    typedef sigma_i8   int8_t;
    typedef sigma_i16  int16_t;
    typedef sigma_i32  int32_t;
    typedef sigma_i64  int64_t;

    typedef sigma_sz_t  size_t;
    typedef sigma_ssz_t ssize_t;
#endif

#endif /* SIGMAOS_SUPREME_TYPES_H */
