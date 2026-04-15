/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN MATH PRIMITIVES (v1.0)
 * =========================================================================
 */

#ifndef SIGMA_MATH_H
#define SIGMA_MATH_H

#include "suites/S01_Genesis/shards/sigma_types.h"

static inline sigma_f64 sigma_fabs(sigma_f64 x) {
    return (x < 0) ? -x : x;
}

#endif
