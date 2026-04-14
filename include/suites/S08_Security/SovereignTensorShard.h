/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TENSOR SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_TENSOR_SHARD_H
#define SOVEREIGN_TENSOR_SHARD_H

#include "sigma_types.h"
#include "SigmaOOP.h"

typedef struct {
    float* data;
    sigma_u32 rows;
    sigma_u32 cols;
} SigmaTensor_t;

typedef struct {
    SigmaObject_t core;
    sigma_u32     ops_completed;
} SovereignTensorShard_t;

SovereignTensorShard_t SovereignTensorShard_Create(void);
void sigma_tensor_gemm(SigmaTensor_t* A, SigmaTensor_t* B, SigmaTensor_t* C);
void SovereignTensorShard_Audit(SovereignTensorShard_t* self);
void SovereignTensorShard_Init(void);

#endif /* SOVEREIGN_TENSOR_SHARD_H */
