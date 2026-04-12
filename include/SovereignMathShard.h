/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MATH SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_MATH_SHARD_H
#define SOVEREIGN_MATH_SHARD_H

#include "sigma_types.h"
#include "SigmaOOP.h"

typedef struct {
    SigmaObject_t core;
    sigma_u64 total_calcs;
} SovereignMathShard_t;

SovereignMathShard_t SovereignMath_Create(void);
float sigma_math_fast_inv_sqrt(float number);
void SovereignMathShard_Audit(SovereignMathShard_t* self);
void SovereignMathShard_Init(void);

#endif /* SOVEREIGN_MATH_SHARD_H */
