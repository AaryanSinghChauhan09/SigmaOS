/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NEURAL SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_NEURAL_SHARD_H
#define SOVEREIGN_NEURAL_SHARD_H

#include "sigma_types.h"

sigma_err_t sigma_neural_load          (const char* name, sigma_u32 layers, sigma_u64 params);
void        sigma_neural_infer         (const char* model_name);
void        SovereignNeuralShard_Init  (void);
void        SovereignNeural_Audit      (void);

#endif /* SOVEREIGN_NEURAL_SHARD_H */
