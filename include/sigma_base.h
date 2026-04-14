/*
 * =========================================================================
 * Σ SIGMAOS: CORE FOUNDATION (v2.0 — ZERO DEPENDENCY)
 * =========================================================================
 * Mission: Minimal dependencies for all Sovereign Shards.
 * =========================================================================
 */

#ifndef SIGMA_BASE_H
#define SIGMA_BASE_H

#include "sigma_types.h"
#include "SovereignLibC.h"
#include "SovereignModule.h"

#define SIGMA_ASSERT(cond, msg) \
    do { if (!(cond)) { sigma_printf("Σ [ASSERTION FAILED]: %s (%s:%d)\n", msg, __FILE__, __LINE__); sigma_exit(1); } } while(0)

#endif /* SIGMA_BASE_H */
