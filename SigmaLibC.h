/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA LIBC HEADER (v6.0 - Full Sovereign Edition)
 * =========================================================================
 * USP Absorbed: musl (minimalism), Clear Linux (AVX optimization),
 *               uClibc-ng (embedded), Diet libc (zero waste)
 * Principle: ZERO <stdint.h>, ZERO <stddef.h>. Uses only sigma_types.h.
 * Note: This file is the top-level SigmaLibC.h in the root directory.
 *       It now delegates to our new libc/ sovereign headers.
 * =========================================================================
 */

#ifndef SIGMALIBC_H
#define SIGMALIBC_H

/* Pull in our sovereign type system and full standard library replacement */
#include "libc/sigma_types.h"
#include "libc/sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Convenience type aliases for backward compatibility with existing
 * SigmaOS code that used the older type names.
 */
/* sigma_u64 / sigma_i64 etc. are already defined in sigma_types.h */

#ifdef __cplusplus
}
#endif

#endif /* SIGMALIBC_H */

