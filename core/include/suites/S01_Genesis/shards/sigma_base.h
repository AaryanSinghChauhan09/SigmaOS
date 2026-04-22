/*
 * =========================================================================
 * S SIGMAOS: CORE FOUNDATION (v2.0 — ZERO DEPENDENCY)
 * =========================================================================
 * Mission: Minimal dependencies for all Sovereign Shards.
 * =========================================================================
 */

#ifndef SIGMA_BASE_H
#define SIGMA_BASE_H

#include "sigma_types.h"
#include "sigma_libc.h"
#include "SovereignModule.h"

#ifndef SIGMA_ASSERT
 #ifdef __SIGMAOS__
  #define SIGMA_ASSERT(cond, msg) \
    do { if (!(cond)) { sigma_sigma_printf("S [PANIC]: %s (%s:%d)\n", msg, __FILE__, __LINE__); for(;;); } } while(0)
 #else
  #define SIGMA_ASSERT(cond, msg) \
    do { if (!(cond)) { sigma_sigma_printf("S [ASSERTION FAILED]: %s (%s:%d)\n", msg, __FILE__, __LINE__); sigma_exit(1); } } while(0)
 #endif
#endif

#endif /* SIGMA_BASE_H */
